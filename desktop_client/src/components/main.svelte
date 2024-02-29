<script lang="ts">
	import Recording from './recording.svelte';
	import Playback from './playback.svelte';
  import Result from './result.svelte';
  import Language from './language.svelte';
</script>

<div class="flex h-screen flex-col items-center">
	<div class="h-2/5">
		<div class="flex h-full flex-col  justify-center">
			<Recording />
		</div>
	</div>
	<div class="h-1/5">
		<div class="flex h-full flex-col justify-start">
			<Playback />
		</div>
	</div>
	<div class="h-1/5">
		<div class="flex h-full flex-col justify-start">
			<Result />
		</div>
	</div>

	<div class="h-1/5">
		<div class="flex h-full flex-col justify-start">
			<Language />
		</div>
	</div>
</div>
