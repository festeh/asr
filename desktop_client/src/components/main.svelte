<script lang="ts">
	import Recording from './recording.svelte';
  import Result from './result.svelte';
  import LanguageSelector from './language_select.svelte';
	import Control from './control.svelte';
</script>

<div class="flex h-screen flex-col items-center text-lg font-sans">
	<div class="h-2/5">
		<div class="flex h-full flex-col  justify-center">
			<Recording />
		</div>
	</div>
	<div class="h-1/5">
		<div class="flex h-full flex-col justify-start">
			<Control />
		</div>
	</div>
	<div class="h-1/5">
		<div class="flex h-full flex-col justify-start">
			<Result />
		</div>
	</div>

	<div class="h-1/5">
		<div class="flex h-full flex-col justify-start">
			<LanguageSelector />
		</div>
	</div>
</div>
