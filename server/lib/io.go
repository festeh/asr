package lib

import (
	base64 "encoding/base64"
	"fmt"
	wav "github.com/go-audio/wav"
	"io"
	"strings"
)

func decodeBase64(rawData string) (string, error) {
	reader := strings.NewReader(rawData)
	decoder := base64.NewDecoder(base64.StdEncoding, reader)
	result, err := io.ReadAll(decoder)
	if err != nil {
		return "", err
	}
	resultStr := string(result)
	return resultStr, nil
}

func DecodeAudio(rawData string) ([]float32, error) {
	decompressedData, err := decodeBase64(rawData)
	decoder := wav.NewDecoder(strings.NewReader(decompressedData))
	buf, err := decoder.FullPCMBuffer()
	if err != nil {
		return nil, err
	}
	// TODO: should be done on client
	bufFloat := buf.AsFloat32Buffer()
	fmt.Println(buf.Format.SampleRate)
	fmt.Println(buf.Format.NumChannels)
	return bufFloat.Data, nil
}
