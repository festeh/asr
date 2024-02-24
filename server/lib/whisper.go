package lib

import (
	whisper "github.com/ggerganov/whisper.cpp/bindings/go/pkg/whisper"
)

type WhisperModel struct {
	model   whisper.Model
	context whisper.Context
}

func NewWhisperModel(modelPath string) (*WhisperModel, error) {
	model, err := whisper.New(modelPath)
	if err != nil {
		return nil, err
	}
	context, err := model.NewContext()
	if err != nil {
		return nil, err
	}
	context.SetTranslate(false)
	return &WhisperModel{
		model:   model,
		context: context,
	}, nil
}

func (w *WhisperModel) Predict(data []float32) error {
	w.context.SetLanguage("de")
	var cb whisper.SegmentCallback = func(segment whisper.Segment) {
		// Do something with the Segment
		println(segment.Text)
	}
	err := w.context.Process(data, cb, nil)
	if err != nil {
		return err
	}
	return nil
}
