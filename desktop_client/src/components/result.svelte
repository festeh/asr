<script lang="ts">
	import { resultStore } from '../stores/result';

	let resultMsg = '';
	$: {
		const result = $resultStore;
		if (result === '') {
			resultMsg = 'Waiting...';
		} else {
			resultMsg = result;
		}
	}
</script>

<div class="flex">
	<span>
		{resultMsg}
	</span>
</div>
