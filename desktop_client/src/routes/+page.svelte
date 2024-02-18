<script lang="ts">
	import { RecordingStatus } from '$lib/types';
	import { onMount } from 'svelte';

	import { invoke } from '@tauri-apps/api/tauri';

	let status: RecordingStatus = RecordingStatus.IDLE;
  $: buttonText = status === RecordingStatus.IDLE ? 'Start Recording' : 'Stop Recording';
	let audio;

	function toggleStatus() {
		if (status === RecordingStatus.IDLE) {
			invoke('start_recording').then((response) => {
				console.log('Recording started');
				console.log(response);
			});
		} else {
			invoke('stop_recording').then((response) => {
				console.log('Recording stopped');
				console.log(response);
				data = response as string;
			});
		}
		status = status === RecordingStatus.IDLE ? RecordingStatus.RECORDING : RecordingStatus.IDLE;
	}


	function playAudio() {
		audio.play();
	}

	function pauseAudio() {
		audio.pause();
	}

	let data = 'Loading...';
</script>

<div class="flex h-screen flex-col items-center justify-center">
	<button on:click={() => toggleStatus()} class="btn btn-lg border">{buttonText}</button>
	<div>{data}</div>

	<button on:click={() => playAudio()} class="btn btn-lg border">Play Audio </button>
</div>
