'use client';

import { useState } from 'react';

type Props = {
  defaultBaseUrl?: string;
  defaultToken?: string;
};

export function SystemConfigForm({ defaultBaseUrl = 'http://127.0.0.1:7443', defaultToken = '' }: Props) {
  const [baseUrl, setBaseUrl] = useState(defaultBaseUrl);
  const [token, setToken] = useState(defaultToken);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [snapshot, setSnapshot] = useState<unknown>(null);

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setLoading(true);
    setError(null);
    setSnapshot(null);

    try {
      const response = await fetch('/api/system-config', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ baseUrl, token }),
      });

      if (!response.ok) {
        const message = (await response.json().catch(() => null))?.error ?? 'Error desconocido';
        throw new Error(message);
      }

      setSnapshot(await response.json());
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Error inesperado');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="space-y-6 rounded-xl border border-slate-200 bg-white p-6 shadow-sm">
      <h2 className="text-lg font-semibold">Consultar manualmente /system/config</h2>
      <form onSubmit={handleSubmit} className="space-y-4">
        <div className="space-y-2">
          <label className="text-sm font-medium text-slate-700">Base URL</label>
          <input
            type="url"
            value={baseUrl}
            onChange={(event) => setBaseUrl(event.target.value)}
            className="w-full rounded-lg border border-slate-300 px-4 py-2 text-sm focus:border-slate-500 focus:outline-none"
            required
          />
        </div>
        <div className="space-y-2">
          <label className="text-sm font-medium text-slate-700">Token</label>
          <input
            type="password"
            value={token}
            onChange={(event) => setToken(event.target.value)}
            className="w-full rounded-lg border border-slate-300 px-4 py-2 text-sm focus:border-slate-500 focus:outline-none"
            required
          />
        </div>
        <button
          type="submit"
          className="inline-flex items-center justify-center rounded-lg bg-slate-900 px-4 py-2 text-sm font-medium text-white hover:bg-slate-700 disabled:cursor-not-allowed disabled:opacity-60"
          disabled={loading}
        >
          {loading ? 'Consultando…' : 'Consultar snapshot'}
        </button>
        {error && <p className="text-sm text-red-600">{error}</p>}
      </form>

      <section className="rounded-lg border border-slate-100 bg-slate-950/90 p-4 text-slate-50">
        {snapshot ? (
          <pre className="max-h-[320px] overflow-auto text-sm">{JSON.stringify(snapshot, null, 2)}</pre>
        ) : (
          <p className="text-sm text-slate-400">Aún no hay datos. Ejecuta la consulta para ver el snapshot.</p>
        )}
      </section>
    </div>
  );
}
