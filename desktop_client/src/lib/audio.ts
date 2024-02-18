

export function wavFromBase64(base64: string): HTMLAudioElement {
  const wav = new Audio(`data:audio/wav;base64,${base64}`);
  wav.preload = 'auto';
  return wav;
}
