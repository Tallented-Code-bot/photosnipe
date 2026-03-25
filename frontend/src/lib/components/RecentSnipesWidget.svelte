<script lang="ts">
import { writable } from 'svelte/store';
import { onMount } from 'svelte';
import { getRecentSnipes, type Snipe, getPersonsByIds, type Person } from '$lib/api';

const snipes = writable<Snipe[]>([]);
const isLoading = writable(true);
const error = writable<string | null>(null);
const people = writable<Record<string, Person>>({});

onMount(async () => {
  try {
    isLoading.set(true);
    error.set(null);
    people.set({});
    const res = await getRecentSnipes(5);
    snipes.set(res.data);
    // Collect all unique IDs
    const ids = Array.from(new Set(res.data.flatMap(snipe => [snipe.sniper_id, snipe.snipee_id])));
    if (ids.length > 0) {
      const pres = await getPersonsByIds(ids);
      if (pres.success) {
        const map: Record<string, Person> = {};
        for(const p of pres.data) {
          map[p.id.toString()] = p;
        }
        people.set(map);
      }
    }
  } catch (e) {
    const errMsg = e instanceof Error ? e.message : 'Failed to load';
    error.set(errMsg);
    snipes.set([]);
    people.set({});
  } finally {
    isLoading.set(false);
  }
});
</script>

<div class="bg-white rounded-lg shadow-md p-6 mb-6 w-full border">
	<h2 class="mb-4 text-xl font-bold gap-2 flex items-center">
		<span class="h-5 w-1 rounded bg-primary mr-2 inline-block"></span>
		<span class="text-primary">Recent Snipes</span>
	</h2>

	<ul>
		{#if $isLoading}
			<li class="flex justify-center py-8">
				<span class="loader animate-spin w-8 h-8 border-4 border-primary border-t-transparent rounded-full"></span>
			</li>
		{:else if $error}
			<li class="text-red-500 py-8 text-center">Failed to load: {$error}</li>
		{:else if $snipes.length === 0}
			<li class="text-gray-400 text-base py-8 text-center">
				<svg
					class="mb-2 h-10 w-10 text-gray-300 mx-auto"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					stroke-width="2"
				>
					<circle cx="12" cy="12" r="10" />
					<path stroke-linecap="round" stroke-linejoin="round" d="M9 10h.01M15 10h.01M8 16h8" />
				</svg>
				No recent snipes yet!
			</li>
		{:else}
			{#each $snipes as snipe}
				<li class="mb-2 flex flex-col md:flex-row md:items-end md:gap-2">
					<div class="flex items-center gap-3 w-full">
						<span class="font-semibold text-gray-800">
							{#if $people[snipe.sniper_id]}
								{$people[snipe.sniper_id].display_name || $people[snipe.sniper_id].username}
							{:else}
								Sniper #{snipe.sniper_id}
							{/if}
						</span>
						<span class="mx-2 text-lg">→</span>
						<span class="font-semibold text-gray-800">
							{#if $people[snipe.snipee_id]}
								{$people[snipe.snipee_id].display_name || $people[snipe.snipee_id].username}
							{:else}
								Snipee #{snipe.snipee_id}
							{/if}
						</span>
					</div>
					{#if snipe.picture_url}
						<img src={snipe.picture_url} alt="snipe image" class="my-1 md:my-0 w-24 h-24 object-cover border rounded-md" />
					{/if}
					{#if snipe.text}
						<span class="text-sm text-gray-500 ml-1 italic">“{snipe.text}”</span>
					{/if}
				</li>
			{/each}
		{/if}
	</ul>
</div>

<style>
.loader {
	border-top-color: transparent;
	border-radius: 50%;
}
</style>
