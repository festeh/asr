package lib

import (
	"encoding/json"
	"fmt"
	"net/http"
)

type Server struct {
	queue        chan int
	model        *WhisperModel
	googleSpeech *GoogleSpeechClient
}

func NewServer(model *WhisperModel) (*Server, error) {
	queue := make(chan int, 1)
	googleSpeech, err := NewGoogleSpeechClient()
	if err != nil {
		return nil, fmt.Errorf("failed to create Google Speech client: %v", err)
	}
	return &Server{
		queue:        queue,
		model:        model,
		googleSpeech: googleSpeech,
	}, nil
}

type RequestData struct {
	AudioEncoded string `json:"audio"`
	Lang         string `json:"lang"`
}

type handler func(http.ResponseWriter, *http.Request)

func parseData(r *http.Request) (*RequestData, error) {
	defer r.Body.Close()
	data := json.NewDecoder(r.Body)
	var reqData RequestData
	err := data.Decode(&reqData)
	if err != nil {
		return nil, err
	}
	if reqData.Lang == "" {
		reqData.Lang = "auto"
	}
	return &reqData, nil
}

func (s *Server) handleRecognition() handler {
	return func(w http.ResponseWriter, r *http.Request) {
		fmt.Println("Received request")
		defer func() { <-s.queue }()
		s.queue <- 1
		fmt.Println("Processing...")
		parsed, err := parseData(r)
		if err != nil {
			http.Error(w, "Error parsing request", http.StatusBadRequest)
			return
		}
		fmt.Println("Parsed data")
		audio, err := DecodeAudio(parsed.AudioEncoded)
		fmt.Println("Decoded audio")
		if err != nil {
			http.Error(w, "Error processing audio", http.StatusInternalServerError)
			return
		}
		s.model.Predict(audio, parsed.Lang)
		fmt.Println("Predicted")
		fmt.Fprint(w, "done")
	}
}

func (s *Server) handleGoogleStreaming() handler {
	return func(w http.ResponseWriter, r *http.Request) {
		fmt.Println("Received Google streaming request")
		defer func() { <-s.queue }()
		s.queue <- 1

		lang := r.URL.Query().Get("lang")
		if lang == "" {
			lang = "en-US"
		}

		resultChan, errChan := s.googleSpeech.StreamingRecognize(r.Context(), r.Body, lang)

		w.Header().Set("Content-Type", "text/event-stream")
		w.Header().Set("Cache-Control", "no-cache")
		w.Header().Set("Connection", "keep-alive")

		flusher, ok := w.(http.Flusher)
		if !ok {
			http.Error(w, "Streaming unsupported", http.StatusInternalServerError)
			return
		}

		for {
			select {
			case result, ok := <-resultChan:
				if !ok {
					return
				}
				fmt.Fprintf(w, "data: %s\n\n", result)
				flusher.Flush()
			case err, ok := <-errChan:
				if !ok {
					return
				}
				fmt.Fprintf(w, "event: error\ndata: %s\n\n", err.Error())
				flusher.Flush()
				return
			case <-r.Context().Done():
				return
			}
		}
	}
}

func (s *Server) handleWhisperLocal() handler {
	return func(w http.ResponseWriter, r *http.Request) {
		fmt.Println("Received Whisper local request")
		defer func() { <-s.queue }()
		s.queue <- 1

		parsed, err := parseData(r)
		if err != nil {
			http.Error(w, "Error parsing request", http.StatusBadRequest)
			return
		}

		audio, err := DecodeAudio(parsed.AudioEncoded)
		if err != nil {
			http.Error(w, "Error processing audio", http.StatusInternalServerError)
			return
		}

		result := make(chan string)
		go func() {
			s.model.Predict(audio, parsed.Lang)
			result <- "Transcription completed"
		}()

		w.Header().Set("Content-Type", "text/event-stream")
		w.Header().Set("Cache-Control", "no-cache")
		w.Header().Set("Connection", "keep-alive")

		flusher, ok := w.(http.Flusher)
		if !ok {
			http.Error(w, "Streaming unsupported", http.StatusInternalServerError)
			return
		}

		select {
		case msg := <-result:
			fmt.Fprintf(w, "data: %s\n\n", msg)
			flusher.Flush()
		case <-r.Context().Done():
			return
		}
	}
}

func (s *Server) ListenAndServe(addr string) error {
	http.HandleFunc("/recognize", s.handleRecognition())
	http.HandleFunc("/google/streaming", s.handleGoogleStreaming())
	http.HandleFunc("/whisper/local", s.handleWhisperLocal())

	return http.ListenAndServe(addr, nil)
}
