<script lang="ts">
	// Import global Tailwind CSS foundation
	import '../app.css';
</script>

<div class="bg-background flex min-h-screen flex-col">
	<!-- Global Card UI shell -->
	<header class="bg-sidebar text-sidebar-text shadow px-4 py-2 flex items-center justify-between">
		<div class="font-bold text-xl tracking-wide">PhotoSnipe</div>
		<!-- Add nav links, user icon, etc., as desired -->
	</header>
	<main class="p-4 gap-8 flex flex-1 flex-col items-center justify-start">
		<div class="max-w-4xl gap-8 flex w-full flex-col">
			<!-- All your routed content will appear here: -->
			<slot></slot>
		</div>
	</main>
	<footer class="bg-sidebar text-sidebar-text px-4 py-2 text-sm text-center opacity-70">
		&copy; {new Date().getFullYear()} PhotoSnipe &mdash; by FTCScout.org
	</footer>
</div>
