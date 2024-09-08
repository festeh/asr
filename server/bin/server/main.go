package main

import (
	"flag"
	"fmt"
	"log"
	"whisper_server/lib"
)

func main() {
	port := flag.Int("port", 8080, "port number to run the server on")
	flag.Parse()

	model, err := lib.NewWhisperModel("/home/dlipin/projects/audiorec/whisper.cpp/models/ggml-small.bin")
	if err != nil {
		log.Fatal(err)
	}
	server, err := lib.NewServer(model)
	if err != nil {
		log.Fatal(err)
	}
	
	addr := fmt.Sprintf(":%d", *port)
	fmt.Printf("Server is running on %s\n", addr)
	server.ListenAndServe(addr)
}
