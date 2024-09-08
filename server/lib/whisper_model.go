package lib

import (
	"fmt"
	"os"

	whisper "github.com/ggerganov/whisper.cpp/bindings/go"
)

func LoadWhisperModel(modelPath string) (*WhisperModel, error) {
	if _, err := os.Stat(modelPath); os.IsNotExist(err) {
		return nil, fmt.Errorf("model file does not exist: %s", modelPath)
	}

	model, err := whisper.New(modelPath)
	if err != nil {
		return nil, fmt.Errorf("failed to load model: %v", err)
	}

	context, err := model.NewContext()
	if err != nil {
		return nil, fmt.Errorf("failed to create context: %v", err)
	}

	context.SetTranslate(false)

	return &WhisperModel{
		model:   model,
		context: context,
	}, nil
}
