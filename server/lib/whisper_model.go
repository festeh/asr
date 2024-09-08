package lib

import (
	"fmt"
	"os"
	"strings"

	whisper "github.com/ggerganov/whisper.cpp/bindings/go/pkg/whisper"
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

type WhisperModel struct {
	model   whisper.Model
	context whisper.Context
}

func (w *WhisperModel) Predict(data []float32, lang string) (string, error) {
	println("Predicting...")
	w.context.SetLanguage(lang)
	var result string
	var cb whisper.SegmentCallback = func(segment whisper.Segment) {
		result += segment.Text + " "
	}
	err := w.context.Process(data, cb, nil)
	if err != nil {
		return "", err
	}
	return strings.TrimSpace(result), nil
}
