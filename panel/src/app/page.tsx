'use client';

import { useState } from 'react';

export default function Home() {
  const [baseUrl, setBaseUrl] = useState('http://127.0.0.1:7443');
  const [token, setToken] = useState('');
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

      const data = await response.json();
      setSnapshot(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Error inesperado');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-slate-50 text-slate-900">
      <main className="mx-auto flex max-w-3xl flex-col gap-8 px-6 py-12">
        <header className="space-y-2">
          <p className="text-sm uppercase tracking-wide text-slate-500">Panel PoC</p>
          <h1 className="text-3xl font-semibold">Consultar /system/config</h1>
          <p className="text-slate-600">
            Ingresa la URL del agente y el token admin o de servicio con alcance completo para validar la conexión
            antes de montar la UI real.
          </p>
        </header>

        <form onSubmit={handleSubmit} className="space-y-6 rounded-xl border border-slate-200 bg-white p-6 shadow-sm">
          <div className="space-y-2">
            <label className="text-sm font-medium text-slate-700">Base URL</label>
            <input
              type="url"
              value={baseUrl}
              onChange={(event) => setBaseUrl(event.target.value)}
              className="w-full rounded-lg border border-slate-300 px-4 py-2 text-sm focus:border-slate-500 focus:outline-none"
              placeholder="http://127.0.0.1:7443"
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
              placeholder="ORBIT_ADMIN_TOKEN"
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

        <section className="rounded-xl border border-slate-200 bg-white p-6 shadow-sm">
          <h2 className="mb-4 text-lg font-semibold">Resultado</h2>
          {snapshot ? (
            <pre className="max-h-[420px] overflow-auto rounded-lg bg-slate-950/90 p-4 text-sm text-slate-50">
              {JSON.stringify(snapshot, null, 2)}
            </pre>
          ) : (
            <p className="text-sm text-slate-500">Aún no hay datos. Ejecuta la consulta para ver el snapshot.</p>
          )}
        </section>
      </main>
    </div>
  );
}
