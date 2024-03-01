<script lang="ts">
	import { RecordingStatus } from '$lib/types';
	import { invoke } from '@tauri-apps/api/tauri';
	import { wavFromBase64 } from '$lib/audio';
	import { audioStore } from '../stores/audio';
	import Button from './button.svelte';

	let status: RecordingStatus = RecordingStatus.IDLE;
	$: buttonText = status === RecordingStatus.IDLE ? 'Record' : 'Stop';

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
					audioStore.set(wavFromBase64(audioStr));
				});
			});
		}
		status = status === RecordingStatus.IDLE ? RecordingStatus.RECORDING : RecordingStatus.IDLE;
	}

</script>

<div class="flex">
  <Button on:click={toggleStatus}>{buttonText}</Button>
	<div class="border flex justify-center items-center rounded-lg p-2 font-bold bg-secondary-700">
		{status}
	</div>
</div>
