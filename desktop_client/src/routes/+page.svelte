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
					audio = wavFromBase64(response as string);
				});
			});
		}
		status = status === RecordingStatus.IDLE ? RecordingStatus.RECORDING : RecordingStatus.IDLE;
	}

	function playAudio() {
		if (audio === undefined) {
			console.log('No audio to play');
		}
    audio!.currentTime = 0;
    audio!.currentTime += 0;
		audio!.play();
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
