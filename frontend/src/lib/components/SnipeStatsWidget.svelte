<script lang="ts">
import { writable } from 'svelte/store';
import { onMount } from 'svelte';
import { getGlobalStats, type GlobalStats } from '$lib/api';

const stats = writable<GlobalStats | null>(null);
const isLoading = writable(true);
const error = writable<string | null>(null);

onMount(async () => {
	try {
		isLoading.set(true);
		error.set(null);
		const res = await getGlobalStats();
		stats.set(res.data);
	} catch (e) {
		const errMsg = e instanceof Error ? e.message : 'Failed to load stats';
		error.set(errMsg);
		stats.set(null);
	} finally {
		isLoading.set(false);
	}
});
</script>

<div class="bg-white rounded-lg shadow-md p-6 mb-6 w-full border">
	<h2 class="mb-4 text-xl font-bold gap-2 flex items-center">
		<span class="h-5 w-1 rounded bg-primary mr-2 inline-block"></span>
		<span class="text-primary">Snipe Stats</span>
	</h2>
	{#if $isLoading}
		<div class="flex justify-center items-center py-8"><span class="loader animate-spin w-8 h-8 border-4 border-primary border-t-transparent rounded-full"></span></div>
	{:else if $error}
		<div class="text-red-500 py-8 text-center">Failed to load: {$error}</div>
	{:else if $stats}
		<div class="gap-4 grid grid-cols-2">
			<div>
				<div class="text-3xl font-extrabold text-gray-900">{$stats.total_snipes}</div>
				<div class="text-sm text-gray-500">Total Snipes</div>
			</div>
			<div>
				{#if $stats.top_sniper}
					<div class="text-3xl font-extrabold text-primary">{$stats.top_sniper.person.display_name || $stats.top_sniper.person.username}</div>
					<div class="text-sm text-gray-500">Top Sniper ({$stats.top_sniper.count})</div>
				{:else}
					<div class="text-gray-400">No snipes yet</div>
				{/if}
			</div>
		</div>
	{/if}
</div>

<style>
.loader {
	border-top-color: transparent;
	border-radius: 50%;
}
</style>
