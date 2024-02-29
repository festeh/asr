<script lang="ts">
	import { RecordingStatus } from '$lib/types';
	import { invoke } from '@tauri-apps/api/tauri';
	import { wavFromBase64 } from '$lib/audio';
	import { audioStore } from '../stores/audio';

	let status: RecordingStatus = RecordingStatus.IDLE;
	$: buttonText = status === RecordingStatus.IDLE ? 'Start Recording' : 'Stop Recording';

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
	<button on:click={() => toggleStatus()} class="btn btn-lg border">{buttonText}</button>
	<div class="border flex justify-center items-center rounded-lg ml-4 p-2 font-bold bg-secondary-900">
		{status}
	</div>
</div>
