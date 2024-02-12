<script lang="ts">
	import { RecordingStatus } from '$lib/types';
  import { onMount } from 'svelte';

	let status: RecordingStatus = RecordingStatus.IDLE;

	function toggleStatus() {
		status = status === RecordingStatus.IDLE ? RecordingStatus.RECORDING : RecordingStatus.IDLE;
	}

  onMount(async () => {
    const constraints = {
      audio: true,
      video: false
    };
    const res = await navigator.mediaDevices.getUserMedia(constraints)
    console.log(res);
  });
</script>

<div class="flex h-screen flex-col items-center justify-center">
	<button on:click={() => toggleStatus()} class="btn btn-lg border">Start</button>
	{status}
</div>
