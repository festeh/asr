<script lang="ts">
	import { RecordingStatus } from '$lib/types';
	import { onMount } from 'svelte';

	import { invoke } from '@tauri-apps/api/tauri';
	import { wavFromBase64 } from '$lib/audio';

	let status: RecordingStatus = RecordingStatus.IDLE;
	$: buttonText = status === RecordingStatus.IDLE ? 'Start Recording' : 'Stop Recording';
	let audio: HTMLAudioElement | undefined = undefined;

	function toggleStatus() {
		if (status === RecordingStatus.IDLE) {
			invoke('start_recording').then((response) => {
				console.log('Recording started', response);
			});
		} else {
			invoke('stop_recording').then((response) => {
				console.log('Recording stopped', response);
				invoke('get_audio').then((response) => {
					const audioStr = response as string;
					console.log('Audio received', audioStr.slice(0, 100));
					audio = wavFromBase64(audioStr);
				});
			});
		}
		status = status === RecordingStatus.IDLE ? RecordingStatus.RECORDING : RecordingStatus.IDLE;
	}

	function playAudio() {
		if (audio === undefined) {
			console.log('Audio not loaded, fetching from backend');
			invoke('get_audio').then((response) => {
				const audioStr = response as string;
				console.log('Audio received from backend', audioStr.slice(0, 100));
				audio = wavFromBase64(audioStr);
			});
		}
		// audio!.currentTime = 0;
		// audio!.currentTime += 0;
		audio.play().then(() => {
      console.log('Audio played');
    }).catch((error) => {
      console.error('Audio play error', error);
    })
	}

	function pauseAudio() {
		audio!.pause();
	}
</script>

<div class="flex h-screen flex-col items-center justify-evenly">
	{status}
	<button on:click={() => toggleStatus()} class="btn btn-lg border">{buttonText}</button>
	<div class="flex">
		<button on:click={() => playAudio()} class="btn btn-lg mr-5 border">Play Audio </button>
		<button on:click={() => pauseAudio()} class="btn btn-lg border">Pause Audio </button>
	</div>
</div>
