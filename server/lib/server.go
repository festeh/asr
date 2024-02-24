package lib

import (
	"encoding/json"
	"fmt"
	"net/http"
)

type Server struct {
	queue chan int
	model *WhisperModel
}

func NewServer(model *WhisperModel) *Server {
	queue := make(chan int, 1)
	return &Server{
		queue: queue,
		model: model,
	}
}

type RequestData struct {
	Audio string `json:"audio"`
}

type handler func(http.ResponseWriter, *http.Request)

func extractAudioKey(r *http.Request) (string, error) {
	defer r.Body.Close()
	data := json.NewDecoder(r.Body)
	var reqData RequestData
	err := data.Decode(&reqData)
	if err != nil {
		return "", err
	}
	return reqData.Audio, nil
}

func (s *Server) handleRecognition() handler {
	return func(w http.ResponseWriter, r *http.Request) {
		fmt.Println("Received request")
		defer func() { <-s.queue }()
		s.queue <- 1
		fmt.Println("Processing...")
		rawData, err := extractAudioKey(r)
		if err != nil {
			http.Error(w, "Error parsing request", http.StatusBadRequest)
			return
		}
		data, err := ProcessWavStream(rawData)
		if err != nil {
			http.Error(w, "Error processing audio", http.StatusInternalServerError)
			return
		}
		s.model.Predict(data)
		fmt.Fprint(w, "done")
	}
}

func (s *Server) ListenAndServe(addr string) error {
	http.HandleFunc("/recognize", s.handleRecognition())

	return http.ListenAndServe(addr, nil)
}
