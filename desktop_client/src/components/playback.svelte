<script lang="ts">
	import { audioStore } from '../stores/audio';
	import { get } from 'svelte/store';
	import { invoke } from '@tauri-apps/api/tauri';
	import { wavFromBase64 } from '$lib/audio';
	import Button from './button.svelte';

	type State = 'empty' | 'nonempty';
	let state: State = 'empty';

	$: {
		const audio = $audioStore;
		if (audio.src === '') {
			state = 'empty';
		} else {
			state = 'nonempty';
		}
	}

	function playAudio() {
		let audio = get(audioStore);

		if (audio.src === '') {
			console.log('Audio not loaded, fetching from backend');
			invoke('get_audio').then((response) => {
				const audioStr = response as string;
				console.log('Audio received from backend', audioStr.slice(0, 100));
				audioStore.set(wavFromBase64(audioStr));
				audio = get(audioStore);
			});
		}
		audio
			.play()
			.then(() => {
				console.log('Audio played');
			})
			.catch((error) => {
				console.error('Audio play error', error);
			});
	}

	function pauseAudio() {
		let audio = get(audioStore);
		audio.pause();
	}

</script>

{#if state === 'empty'}
	No audio available
{/if}

{#if state === 'nonempty'}
	<div class="flex">
		<Button on:click={() => playAudio()}>Play</Button>
		<Button on:click={() => pauseAudio()}>Pause</Button>
	</div>
{/if}
