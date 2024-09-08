package lib

import (
	"context"
	"fmt"
	"io"

	speech "cloud.google.com/go/speech/apiv1"
	"cloud.google.com/go/speech/apiv1/speechpb"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

type GoogleSpeechClient struct {
	client *speech.Client
}

func NewGoogleSpeechClient() (*GoogleSpeechClient, error) {
	ctx := context.Background()
	client, err := speech.NewClient(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to create client: %v", err)
	}
	return &GoogleSpeechClient{client: client}, nil
}

func (g *GoogleSpeechClient) StreamingRecognize(ctx context.Context, stream io.Reader, lang string) (chan string, chan error) {
	resultChan := make(chan string)
	errChan := make(chan error)

	go func() {
		defer close(resultChan)
		defer close(errChan)

		streamingConfig := &speechpb.StreamingRecognitionConfig{
			Config: &speechpb.RecognitionConfig{
				Encoding:        speechpb.RecognitionConfig_LINEAR16,
				SampleRateHertz: 16000,
				LanguageCode:    lang,
			},
			InterimResults: true,
		}

		client, err := g.client.StreamingRecognize(ctx)
		if err != nil {
			errChan <- fmt.Errorf("failed to start streaming: %v", err)
			return
		}

		err = client.Send(&speechpb.StreamingRecognizeRequest{
			StreamingRequest: &speechpb.StreamingRecognizeRequest_StreamingConfig{
				StreamingConfig: streamingConfig,
			},
		})
		if err != nil {
			errChan <- fmt.Errorf("failed to send streaming config: %v", err)
			return
		}

		go func() {
			buffer := make([]byte, 1024)
			for {
				n, err := stream.Read(buffer)
				if err == io.EOF {
					break
				}
				if err != nil {
					errChan <- fmt.Errorf("failed to read audio data: %v", err)
					return
				}
				err = client.Send(&speechpb.StreamingRecognizeRequest{
					StreamingRequest: &speechpb.StreamingRecognizeRequest_AudioContent{
						AudioContent: buffer[:n],
					},
				})
				if err != nil {
					errChan <- fmt.Errorf("failed to send audio data: %v", err)
					return
				}
			}
			client.CloseSend()
		}()

		for {
			resp, err := client.Recv()
			if err == io.EOF {
				break
			}
			if err != nil {
				if statusErr, ok := status.FromError(err); ok {
					if statusErr.Code() == codes.OutOfRange {
						// End of audio
						break
					}
				}
				errChan <- fmt.Errorf("failed to receive response: %v", err)
				return
			}

			for _, result := range resp.Results {
				for _, alt := range result.Alternatives {
					resultChan <- alt.Transcript
				}
			}
		}
	}()

	return resultChan, errChan
}

func (g *GoogleSpeechClient) Close() error {
	return g.client.Close()
}
