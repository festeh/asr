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

func (s *Server) modelLoaded() bool {
	return s.model != nil
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

		if !s.modelLoaded() {
			http.Error(w, "Whisper model not loaded", http.StatusServiceUnavailable)
			return
		}

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

		recognizedText, err := s.model.Predict(audio, parsed.Lang)
		if err != nil {
			http.Error(w, fmt.Sprintf("Error predicting: %v", err), http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]string{"result": recognizedText})
	}
}

func (s *Server) ListenAndServe(addr string) error {
	http.HandleFunc("/google/streaming", s.handleGoogleStreaming())
	http.HandleFunc("/whisper/local", s.handleWhisperLocal())

	return http.ListenAndServe(addr, nil)
}
