package main

import (
	"fmt"
	"log"
	"whisper_server/lib"
)

func main() {
	model, err := lib.NewWhisperModel("/home/dlipin/projects/audiorec/whisper.cpp/models/ggml-tiny.bin")
	if err != nil {
		log.Fatal(err)
	}
	server := lib.NewServer(model)
	fmt.Println("Server is running on :8080")
	server.ListenAndServe(":8080")
}
