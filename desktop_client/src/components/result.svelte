<script lang="ts">
	import { audioStore } from "../stores/audio";

	let result = '';

	function onSubmit() {
		fetch('http://localhost:8000/recognize', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ audio: audioStore })
		})
			.then((response) => response.json())
			.then((data) => {
				console.log('Success:', data);
				result = data.result;
			})
			.catch((error) => {
				console.error('Error:', error);
			});
	}
</script>

<div class="flex">
	<button class="btn-blue btn bg-secondary-700" on:click={() => onSubmit()}> Recognize </button>
	<span>
		{result}
	</span>
</div>
