<script lang="ts">
	import Sidebar from '$lib/components/Sidebar.svelte';
	import RecentSnipesWidget from '$lib/components/RecentSnipesWidget.svelte';
	import SnipeStatsWidget from '$lib/components/SnipeStatsWidget.svelte';

	import { onMount } from 'svelte';
	let sidebarOpen = $state(false);
	let isMdOrLarger = false;

	function checkScreen() {
		isMdOrLarger = window.matchMedia('(min-width: 768px)').matches;
		if (isMdOrLarger) sidebarOpen = false; // Always show sidebar at md+
	}

	onMount(() => {
		checkScreen();
		window.addEventListener('resize', checkScreen);
		return () => window.removeEventListener('resize', checkScreen);
	});
</script>

<!-- Hamburger for mobile only, over content -->
<button
	class="top-4 left-4 md:hidden p-2 rounded-lg bg-gray-700 text-gray-50 hover:bg-gray-600 focus-visible:ring-primary absolute z-50 flex items-center focus:outline-none focus-visible:ring-2"
	aria-label="Open navigation menu"
	aria-controls="sidebar"
	aria-expanded={sidebarOpen}
	onclick={() => (sidebarOpen = !sidebarOpen)}
>
	<svg
		width="24"
		height="24"
		fill="none"
		stroke="currentColor"
		stroke-width="2"
		stroke-linecap="round"
		stroke-linejoin="round"
		viewBox="0 0 24 24"
	>
		<line x1="3" y1="12" x2="21" y2="12" />
		<line x1="3" y1="6" x2="21" y2="6" />
		<line x1="3" y1="18" x2="21" y2="18" />
	</svg>
</button>

<!-- Sidebar fixed on desktop, overlays on mobile. Remove from normal flow -->
<Sidebar mobileOpen={sidebarOpen} onClose={() => (sidebarOpen = false)} />

<!-- Main content, scrollable, offset for sidebar -->
<main
	class="gap-8 md:ml-64 md:mt-6 pt-14 bg-gray-50 md:rounded-xl relative flex h-screen flex-col items-stretch overflow-y-auto transition-all duration-200"
>
	<RecentSnipesWidget />
	<SnipeStatsWidget />
</main>
