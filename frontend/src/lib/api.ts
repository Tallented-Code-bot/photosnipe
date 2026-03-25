// src/lib/api.ts
// Generic API utility for PhotoSnipe frontend

// --- Types ---
export interface ApiResponse<T> {
  data: T;
  meta?: unknown; // Use more specific typing if meta shape is known/per-endpoint
  success: boolean;
}

export interface Snipe {
  _id: string;
  sniper_id: number;
  snipee_id: number;
  picture_url: string;
  text?: string | null;
  channel_id: number;
  guild_id?: number | null;
}

export interface Person {
  _id: string;
  id: number;
  username: string;
  display_name?: string | null;
}

export interface LeaderboardEntry {
  person: Person;
  count: number;
}

export interface GlobalStats {
  total_persons: number;
  total_snipes: number;
  top_sniper?: LeaderboardEntry | null;
  top_snipee?: LeaderboardEntry | null;
}

// --- API Calls ---
const API_BASE = '/api'; // Adjust for dev proxy if needed

export async function getRecentSnipes(limit = 5) {
  const url = `${API_BASE}/snipes?limit=${limit}`;
  const res = await fetch(url);
  if (!res.ok) throw new Error(`Failed to fetch snipes (${res.status})`);
  const apiRes = await res.json() as ApiResponse<any[]>;
  // Normalize data so _id is always a string
  const snipes = apiRes.data.map(snipe => ({
    ...snipe,
    _id: typeof snipe._id === "object" && snipe._id.$oid ? snipe._id.$oid : snipe._id
  }));
  return { ...apiRes, data: snipes } as ApiResponse<Snipe[]>;
}

export async function getGlobalStats() {
  const url = `${API_BASE}/stats`;
  const res = await fetch(url);
  if (!res.ok) throw new Error(`Failed to fetch stats (${res.status})`);
  return (await res.json()) as ApiResponse<GlobalStats>;
}

// Batch fetch persons by ID
export async function getPersonsByIds(ids: Array<number | string>) {
  if (!ids.length) return { data: [], success: true };
  const qs = ids.map(encodeURIComponent).join(',');
  const url = `${API_BASE}/persons?ids=${qs}`;
  const res = await fetch(url);
  if (!res.ok) throw new Error(`Failed to fetch persons (${res.status})`);
  return (await res.json()) as ApiResponse<Person[]>;
}
// Add further endpoints here as needed (persons, profiles, per-channel, etc.)
