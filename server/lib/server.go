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
	AudioEncoded string `json:"audio"`
	Lang	string `json:"lang"`
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

func (s *Server) ListenAndServe(addr string) error {
	http.HandleFunc("/recognize", s.handleRecognition())

	return http.ListenAndServe(addr, nil)
}
