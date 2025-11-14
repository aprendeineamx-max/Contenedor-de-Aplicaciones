import { fetchContainers, fetchTasks } from '@/lib/orbit';
import { SystemConfigForm } from '@/components/SystemConfigForm';

async function loadServerData() {
  const baseUrl = process.env.ORBIT_PANEL_BASE_URL;
  const token = process.env.ORBIT_PANEL_TOKEN;

  if (!baseUrl || !token) {
    return {
      containers: [],
      tasks: [],
      error: 'Define ORBIT_PANEL_BASE_URL y ORBIT_PANEL_TOKEN en panel/.env.local para precargar datos.',
    } as const;
  }

  try {
    const [containers, tasks] = await Promise.all([
      fetchContainers(baseUrl, token),
      fetchTasks(baseUrl, token, 10),
    ]);
    return { containers: containers ?? [], tasks: tasks ?? [], error: undefined } as const;
  } catch (error) {
    console.error('SSR: no se pudo consultar el agente', error);
    return {
      containers: [],
      tasks: [],
      error: 'No se pudo consultar el agente con las credenciales del servidor.',
    } as const;
  }
}

export const dynamic = 'force-dynamic';

export default async function Home() {
  const { containers, tasks, error } = await loadServerData();

  return (
    <div className="min-h-screen bg-slate-50 text-slate-900">
      <main className="mx-auto flex max-w-5xl flex-col gap-8 px-6 py-12">
        <header className="space-y-2">
          <p className="text-sm uppercase tracking-wide text-slate-500">Panel PoC</p>
          <h1 className="text-3xl font-semibold">Estado del agente</h1>
          <p className="text-slate-600">
            La tabla se genera en SSR usando las variables ORBIT_PANEL_BASE_URL / ORBIT_PANEL_TOKEN, mientras que el
            formulario permite probar otras instancias con un token diferente.
          </p>
        </header>

        {error && (
          <p className="rounded-lg bg-yellow-50 px-4 py-3 text-sm text-yellow-900">
            {error}
          </p>
        )}

        <section className="grid gap-6 md:grid-cols-2">
          <div className="rounded-xl border border-slate-200 bg-white p-6 shadow-sm">
            <h2 className="mb-4 text-lg font-semibold">Contenedores recientes</h2>
            {containers.length ? (
              <ul className="space-y-3 text-sm text-slate-700">
                {containers.slice(0, 5).map((container) => (
                  <li key={container.id} className="rounded-lg border border-slate-100 px-4 py-3">
                    <p className="font-medium">{container.name || container.id}</p>
                    <p className="text-xs text-slate-500">Estado: {container.status}</p>
                  </li>
                ))}
              </ul>
            ) : (
              <p className="text-sm text-slate-500">No hay contenedores para mostrar.</p>
            )}
          </div>

          <div className="rounded-xl border border-slate-200 bg-white p-6 shadow-sm">
            <h2 className="mb-4 text-lg font-semibold">Últimas tareas</h2>
            {tasks.length ? (
              <ul className="space-y-3 text-sm text-slate-700">
                {tasks.slice(0, 5).map((task) => (
                  <li key={task.id} className="rounded-lg border border-slate-100 px-4 py-3">
                    <p className="font-medium">{task.type}</p>
                    <p className="text-xs text-slate-500">
                      ID: {task.id} — Estado: <span className="uppercase">{task.status}</span>
                    </p>
                  </li>
                ))}
              </ul>
            ) : (
              <p className="text-sm text-slate-500">Aún no hay tareas registradas.</p>
            )}
          </div>
        </section>

        <SystemConfigForm
          defaultBaseUrl={process.env.ORBIT_PANEL_BASE_URL}
          defaultToken={process.env.ORBIT_PANEL_TOKEN}
        />
      </main>
    </div>
  );
}

