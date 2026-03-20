// src/lib/auth.ts
import {
	createAuth0Client,
	type Auth0Client,
	type Auth0ClientOptions,
	type User
} from '@auth0/auth0-spa-js';
import { writable, type Writable } from 'svelte/store';

// -- Types --
interface AuthState {
	isAuthenticated: boolean;
	user: User | null;
	loading: boolean;
	error: string | null;
}

// -- Svelte Stores --
export const authState: Writable<AuthState> = writable({
	isAuthenticated: false,
	user: null,
	loading: true,
	error: null
});

let auth0: Auth0Client;

// -- Helper: Get Environment Vars Securely --
function getEnv(key: string): string {
	// VITE_ prefix required for Vite to expose to client!
	const val = (import.meta as any).env[key];
	if (!val) throw new Error(`${key} is missing in .env!`);
	return val;
}

// -- Init Auth0 Client --
export async function initAuth(): Promise<void> {
	authState.update((v) => ({ ...v, loading: true, error: null }));
	const options: Auth0ClientOptions = {
		domain: getEnv('VITE_AUTH0_DOMAIN'),
		clientId: getEnv('VITE_AUTH0_CLIENT_ID'),
		authorizationParams: {
			redirect_uri: window.location.origin
		},
		cacheLocation: 'localstorage',
		useRefreshTokens: true
	};
	auth0 = await createAuth0Client(options);

	try {
		// Handle redirect callback after login
		if (window.location.search.includes('code=') && window.location.search.includes('state=')) {
			await auth0.handleRedirectCallback();
			window.history.replaceState({}, document.title, window.location.pathname);
		}
		const isAuthenticated = await auth0.isAuthenticated();
		const user = isAuthenticated ? ((await auth0.getUser()) ?? null) : null;
		authState.set({ isAuthenticated, user, loading: false, error: null });
	} catch (error: any) {
		authState.set({
			isAuthenticated: false,
			user: null,
			loading: false,
			error: error.message || 'Auth error'
		});
	}
}

// -- Auth Actions --
export async function login(): Promise<void> {
	authState.update((v) => ({ ...v, loading: true, error: null }));
	await auth0.loginWithRedirect({
		authorizationParams: { redirect_uri: window.location.origin }
	});
}

export async function logout(): Promise<void> {
	await auth0.logout({ logoutParams: { returnTo: window.location.origin } });
	authState.set({ isAuthenticated: false, user: null, loading: false, error: null });
}
