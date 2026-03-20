<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { authState, initAuth, login, logout } from '$lib/auth';
	import { derived } from 'svelte/store';

	const user = derived(authState, ($s) => $s.user);
	const loading = derived(authState, ($s) => $s.loading);
	const isAuthenticated = derived(authState, ($s) => $s.isAuthenticated);
	const error = derived(authState, ($s) => $s.error);

	onMount(() => {
		initAuth();
	});
</script>

<div class="bg-gray-50 flex min-h-screen flex-col">
	<header
		class="top-0 left-0 h-14 bg-gray-900 text-gray-50 shadow px-6 fixed z-50 flex w-full items-center justify-between"
	>
		<div class="font-bold text-xl tracking-wide">PhotoSnipe</div>
		<!-- AUTH NAV -->
		{#if $loading}
			<span class="animate-pulse text-gray-400">Loading...</span>
		{:else if $isAuthenticated}
			<div class="gap-3 flex items-center">
				<span class="sm:inline-block hidden">{$user?.name || $user?.email}</span>
				{#if $user?.picture}
					<img
						src={$user.picture}
						alt="avatar"
						class="w-8 h-8 border-indigo-400 shadow-sm rounded-full border-2"
					/>
				{/if}
				<button
					class="ml-4 px-3 py-1 rounded bg-indigo-700 hover:bg-indigo-600 text-white shadow transition"
					on:click={logout}
				>
					Log out
				</button>
			</div>
		{:else}
			<button
				class="px-4 py-2 rounded-lg bg-indigo-500 hover:bg-indigo-400 shadow-lg font-semibold text-white text-base transition"
				on:click={login}
			>
				Log in
			</button>
		{/if}
	</header>
	<main class="flex flex-1 flex-col items-stretch justify-start">
		<!-- AUTH GATE: lock content if not authenticated! -->
		{#if $loading}
			<div class="flex flex-1 items-center justify-center">
				<div
					class="rounded-xl p-8 bg-white shadow animate-pulse text-2xl font-semibold text-gray-500"
				>
					Signing in with Auth0...
				</div>
			</div>
		{:else if !$isAuthenticated}
			<div class="flex flex-1 items-center justify-center">
				<!-- Branded login card -->
				<div
					class="shadow-xl bg-white rounded-2xl px-10 py-12 max-w-sm animate-fadeIn relative flex w-full flex-col items-center border"
				>
					<img
						src="https://cdn.auth0.com/brand/assets/logo-dark-bg.svg"
						class="w-20 mb-4"
						alt="Auth0 Logo"
					/>
					<h1 class="text-2xl font-bold mb-2 tracking-wider text-gray-900">
						Welcome to PhotoSnipe
					</h1>
					<p class="text-gray-500 mb-6 text-center">
						Sign in with Auth0 to access and snipe photos!
					</p>
					{#if $error}
						<div class="mb-2 text-red-500 font-semibold">{$error}</div>
					{/if}
					<button
						class="px-6 py-2 mt-4 bg-orange-600 hover:bg-orange-500 text-white font-bold rounded-lg shadow-lg gap-2 flex items-center transition"
						on:click={login}
					>
						<img
							src="https://cdn.auth0.com/website/auth0-logo.svg"
							class="w-6 h-6"
							alt="Auth0 Icon"
						/>
						Continue with Auth0
					</button>
					<div class="mt-6 text-xs text-gray-400 text-center">
						Secured by Auth0. Your data is safe.
					</div>
				</div>
			</div>
		{:else}
			<!-- Main app content: show slot for authenticated -->
			<slot></slot>
		{/if}
	</main>
</div>
