package main

import (
	"flag"
	"fmt"
	"log"
	"whisper_server/lib"
)

func main() {
	port := flag.Int("port", 8080, "port number to run the server on")
	modelPath := flag.String("model", "/home/dlipin/projects/audiorec/whisper.cpp/models/ggml-small.bin", "Path to the Whisper model file")
	flag.Parse()
	var model *lib.WhisperModel
	var err error
	if *modelPath != "" {
		model, err = lib.LoadWhisperModel(*modelPath)
		if err != nil {
			log.Fatalf("Failed to load Whisper model: %v", err)
		}
	}

	if err != nil {
		log.Fatal(err)
	}

	server, err := lib.NewServer(model)
	if err != nil {
		log.Fatalf("Failed to create server: %v", err)
	}

	addr := fmt.Sprintf(":%d", *port)
	fmt.Printf("Server is running on %s\n", addr)
	log.Fatal(server.ListenAndServe(addr))
}
