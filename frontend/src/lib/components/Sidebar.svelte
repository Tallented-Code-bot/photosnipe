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
	const currentPath = derived(page, $page => $page.url.pathname);
</script>

<!-- Sidebar overlay for mobile (only shown if mobileOpen) -->
<div
	class="fixed inset-0 z-40 bg-black bg-opacity-40 md:hidden"
	class:hidden={!mobileOpen}
	role="presentation"
	on:click={() => onClose()}
	on:keyup={(e) => e.key === 'Escape' && onClose()}
	tabindex="-1"
	aria-hidden={!mobileOpen}
/>

<nav
	class="bg-sidebar text-sidebar flex min-h-screen w-64 flex-col p-4
	   fixed z-50 inset-y-0 left-0 transition-transform md:static md:translate-x-0
	   md:flex
	   {mobileOpen ? 'translate-x-0' : '-translate-x-full'}"
	style="will-change: transform;"
	aria-label="Main Navigation"
	aria-modal={mobileOpen}
	role="navigation"
	class:hidden={!mobileOpen && !('md' in window.matchMedia('(min-width: 768px)') && window.matchMedia('(min-width: 768px)').matches)}
>
	<ul class="flex flex-col gap-2" role="list">
		{#each navs as nav}
			<li>
				<a
					href={nav.href}
					class="flex items-center gap-3 rounded-lg px-3 py-2 font-semibold focus:outline-none focus-visible:ring-2 focus-visible:ring-primary transition-colors {($page.url.pathname === nav.href) ? 'bg-primary/10 text-primary' : 'hover:bg-sidebar-text/10'}"
					aria-current={($page.url.pathname === nav.href) ? 'page' : undefined}
					role="link"
					tabindex="0"
					on:keydown={(e) => {if(e.key==='Enter'){ e.currentTarget.click(); onClose(); }}}
					on:click={() => onClose()}
				>
					{@html nav.icon}
					<span>{nav.label}</span>
				</a>
			</li>
		{/each}
	</ul>
</nav>
