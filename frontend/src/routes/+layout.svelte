<script lang="ts">
	// Import global Tailwind CSS foundation
	import '../app.css';
</script>

<div class="bg-gray-50 flex min-h-screen flex-col">
	<!-- Global Card UI shell -->
	<header class="fixed top-0 left-0 w-full h-14 z-50 bg-gray-900 text-gray-50 shadow px-6 flex items-center justify-between">
		<div class="font-bold text-xl tracking-wide">PhotoSnipe</div>
		<!-- Add nav links, user icon, etc., as desired -->
	</header>
	<main class="flex-1 flex flex-col justify-start items-stretch">
		<!-- All your routed content will appear here: -->
		<slot></slot>
	</main>
	</div>
