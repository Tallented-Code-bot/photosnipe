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

<div class="bg-background bg-red-900 relative flex min-h-screen">
	<!-- Hamburger button, only on mobile -->
	<button
		class="top-4 left-4 md:hidden p-2 rounded-lg bg-sidebar-text/10 text-sidebar-text hover:bg-sidebar-text/20 focus-visible:ring-primary absolute z-50 flex items-center focus:outline-none focus-visible:ring-2"
		aria-label="Open navigation menu"
		aria-controls="sidebar"
		aria-expanded={sidebarOpen}
		onclick={() => (sidebarOpen = !sidebarOpen)}
	>
		<!-- Hamburger icon -->
		<svg
			width="24"
			height="24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			viewBox="0 0 24 24"
			><line x1="3" y1="12" x2="21" y2="12" /><line x1="3" y1="6" x2="21" y2="6" /><line
				x1="3"
				y1="18"
				x2="21"
				y2="18"
			/></svg
		>
	</button>

	<!-- Sidebar -->
	<Sidebar mobileOpen={sidebarOpen} onClose={() => (sidebarOpen = false)} />

	<main
		class="p-8 gap-8 md:ml-0 ml-0 flex flex-1 flex-col items-stretch overflow-y-auto transition-all duration-200"
	>
		<RecentSnipesWidget />
		<SnipeStatsWidget />
	</main>
</div>
