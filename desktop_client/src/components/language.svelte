<script lang="ts">
	import { languageStore, type Language } from '../stores/language';

	export let language = 'de';
  let isSelected = false;

	function setLanguage(lang: string) {
		languageStore.set(lang as Language);
	}

  $: {
    const currentLanguage = $languageStore;
    isSelected = currentLanguage === language;
  }

</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	class="m-2 flex items-center justify-center border p-3"
	on:click={() => setLanguage(language)}
  class:bg-secondary-600={isSelected}
>
	{language.toUpperCase()}
</div>
