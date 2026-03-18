<script lang="ts">
	// Placeholder for recent snipes
</script>

<div class="bg-white rounded-lg border shadow-md p-6 mb-6 w-full">
	<h2 class="mb-4 text-xl font-bold gap-2 flex items-center">
		<span class="h-5 w-1 rounded bg-primary mr-2 inline-block"></span>
		<span class="text-primary">Recent Snipes</span>
	</h2>
	<ul>
		<!-- Example: show an empty state illustration if empty, otherwise map the snipes -->
		{#if false}
			<li class="text-gray-400 text-base py-8 text-center">
				<svg
					class="mb-2 h-10 w-10 text-gray-300 mx-auto"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					stroke-width="2"
					><circle cx="12" cy="12" r="10" /><path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M9 10h.01M15 10h.01M8 16h8"
					/></svg
				>
				No recent snipes yet!
			</li>
		{:else}
			<li class="mb-2">
				<span class="text-base font-semibold text-gray-800">Snipe 1</span>
				<span class="text-sm text-gray-500">- Placeholder data</span>
			</li>
			<li class="mb-2">
				<span class="text-base font-semibold text-gray-800">Snipe 2</span>
				<span class="text-sm text-gray-500">- Placeholder data</span>
			</li>
			<li>
				<span class="text-base font-semibold text-gray-800">Snipe 3</span>
				<span class="text-sm text-gray-500">- Placeholder data</span>
			</li>
		{/if}
	</ul>
</div>
