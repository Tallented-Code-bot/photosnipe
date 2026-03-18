<script lang="ts">
	import { page } from '$app/stores';
	import { derived } from 'svelte/store';

	const { mobileOpen = false, onClose = () => {} } = $props();

	type NavItem = {
		href: string;
		label: string;
		icon: string;
	};

	const navs: NavItem[] = [
		{
			href: '/',
			label: 'Home',
			icon: `<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"20\" height=\"20\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" class=\"feather feather-home\" viewBox=\"0 0 24 24\"><path d=\"M3 9.5L12 4l9 5.5V20a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V9.5z\"/><polyline points=\"9 22 9 12 15 12 15 22\"></polyline></svg>`
		},
		{
			href: '/snipes',
			label: 'Snipes',
			icon: `<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"20\" height=\"20\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" class=\"feather feather-target\" viewBox=\"0 0 24 24\"><circle cx=\"12\" cy=\"12\" r=\"10\"/><circle cx=\"12\" cy=\"12\" r=\"6\"/><circle cx=\"12\" cy=\"12\" r=\"2\"/></svg>`
		},
		{
			href: '/stats',
			label: 'Stats',
			icon: `<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"20\" height=\"20\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" class=\"feather feather-bar-chart-2\" viewBox=\"0 0 24 24\"><line x1=\"18\" y1=\"20\" x2=\"18\" y2=\"10\"/><line x1=\"12\" y1=\"20\" x2=\"12\" y2=\"4\"/><line x1=\"6\" y1=\"20\" x2=\"6\" y2=\"14\"/></svg>`
		}
	];

	// Compute active nav based on current page
	const currentPath = derived(page, ($page) => $page.url.pathname);
</script>

<!-- Sidebar overlay for mobile (only shown if mobileOpen) -->
<div
	class="inset-0 bg-black bg-opacity-40 md:hidden fixed z-40"
	class:hidden={!mobileOpen}
	role="presentation"
	onclick={() => onClose()}
	onkeyup={(e) => e.key === 'Escape' && onClose()}
	tabindex="-1"
	aria-hidden={!mobileOpen}
></div>

<nav
	class="bg-gray-800 text-gray-100 w-64 p-6 left-0 top-14 transition-transform z-40 h-[calc(100vh-3.5rem)] hidden md:fixed md:block flex-col"
	aria-label="Main Navigation"
>

	<ul class="gap-2 flex flex-col" role="list">
		{#each navs as nav}
			<li>
				<a
					href={nav.href}
					class="gap-3 rounded-lg px-3 py-2 font-semibold focus-visible:ring-primary flex items-center transition-colors focus:outline-none focus-visible:ring-2
      {$page.url.pathname === nav.href
						? 'bg-primary/10 text-primary'
						: 'hover:bg-primary/20 hover:text-primary'}"
					aria-current={$page.url.pathname === nav.href ? 'page' : undefined}
					
					tabindex="0"
onkeydown={(e) => {
					if (e.key === 'Enter') {
						e.currentTarget.click();
						onClose();
					}
				}}
					onclick={() => onClose()}
				>
					{@html nav.icon}
					<span>{nav.label}</span>
				</a>
			</li>
		{/each}
	</ul>
</nav>
