import { SystemConfigForm } from '@/components/SystemConfigForm';
import { fetchContainers, fetchTasks } from '@/lib/orbit';
import { createContainerAction, deleteContainerAction } from './actions';

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
      fetchTasks(baseUrl, token, 50),
    ]);
    return { containers: containers ?? [], tasks: tasks ?? [], error: undefined } as const;
  } catch (err) {
    console.error('SSR: no se pudo consultar el agente', err);
    return {
      containers: [],
      tasks: [],
      error: 'No se pudo consultar el agente con las credenciales del servidor.',
    } as const;
  }
}

export const dynamic = 'force-dynamic';

const PER_PAGE = 5;

type PageSearchParams = Promise<Record<string, string | string[] | undefined>>;

export default async function Home({ searchParams }: { searchParams: PageSearchParams }) {
  const resolvedSearchParams = await searchParams;
  const rawPage = resolvedSearchParams?.page;
  const normalizedPage = Array.isArray(rawPage) ? rawPage[0] : rawPage;
  const { containers, tasks, error } = await loadServerData();
  const page = Math.max(1, Number(normalizedPage ?? '1'));
  const totalPages = Math.max(1, Math.ceil(containers.length / PER_PAGE));
  const safePage = Math.min(page, totalPages);
  const start = (safePage - 1) * PER_PAGE;
  const visibleContainers = containers.slice(start, start + PER_PAGE);

  return (
    <div className="min-h-screen bg-slate-50 text-slate-900">
      <main className="mx-auto flex max-w-5xl flex-col gap-8 px-6 py-12">
        <header className="space-y-2">
          <p className="text-sm uppercase tracking-wide text-slate-500">Panel PoC</p>
          <h1 className="text-3xl font-semibold">Estado del agente</h1>
          <p className="text-slate-600">
            La tabla se genera en SSR usando las variables ORBIT_PANEL_BASE_URL / ORBIT_PANEL_TOKEN. El formulario permite
            probar otros entornos manualmente con un token diferente.
          </p>
        </header>

        <section className="rounded-xl border border-slate-200 bg-white p-6 text-sm text-slate-600 shadow-sm">
          <h2 className="mb-3 text-lg font-semibold text-slate-900">Guia rapida del panel</h2>
          <ul className="list-disc space-y-2 pl-5">
            <li>
              <span className="font-medium text-slate-900">Contenedores:</span> muestra el estado, nombre y plataforma de
              cada sandbox. Usa el paginador inferior para recorrer bloques de cinco elementos.
            </li>
            <li>
              <span className="font-medium text-slate-900">Formulario &quot;Nombre&quot; y selector &quot;Plataforma&quot;:</span> definen
              como se creara el contenedor. El nombre identifica la instancia y la plataforma invoca la plantilla de
              recursos (windows-x64 o windows-arm64).
            </li>
            <li>
              <span className="font-medium text-slate-900">Botones Crear / Eliminar:</span> crean el contenedor con las
              credenciales del panel o borran la seleccion actual registrando una tarea de auditoria.
            </li>
            <li>
              <span className="font-medium text-slate-900">Ultimas tareas:</span> valida en segundos si la operacion termino
              bien. Si ves estados distintos a &quot;succeeded&quot; revisa logs del agente.
            </li>
            <li>
              <span className="font-medium text-slate-900">Consultar manualmente /system/config:</span> permite probar otros
              tokens/urls desde la misma pagina y guardar el snapshot para QA.
            </li>
          </ul>
        </section>

        {error && <p className="rounded-lg bg-yellow-50 px-4 py-3 text-sm text-yellow-900">{error}</p>}

        <section className="rounded-xl border border-slate-200 bg-white p-6 shadow-sm">
          <div className="flex flex-wrap items-center justify-between gap-4">
            <div>
              <h2 className="text-lg font-semibold">Contenedores</h2>
              <p className="text-sm text-slate-500">Pagina {safePage} de {totalPages}</p>
            </div>
            <form action={createContainerAction} className="flex flex-wrap gap-2 text-sm">
              <input
                type="text"
                name="name"
                placeholder="Nombre"
                className="rounded-lg border border-slate-300 px-3 py-1"
                required
              />
              <select
                name="platform"
                className="rounded-lg border border-slate-300 px-3 py-1"
                defaultValue="windows-x64"
              >
                <option value="windows-x64">windows-x64</option>
                <option value="windows-arm64">windows-arm64</option>
              </select>
              <button
                type="submit"
                className="rounded-lg bg-slate-900 px-3 py-1 font-medium text-white hover:bg-slate-700"
                disabled={!process.env.ORBIT_PANEL_BASE_URL}
              >
                Crear
              </button>
            </form>
          </div>

          {visibleContainers.length ? (
            <ul className="mt-6 divide-y divide-slate-100 text-sm text-slate-700">
              {visibleContainers.map((container) => (
                <li key={container.id} className="flex flex-wrap items-center justify-between gap-3 py-3">
                  <div>
                    <p className="font-medium">{container.name || container.id}</p>
                    <p className="text-xs text-slate-500">Estado: {container.status}</p>
                  </div>
                  <form action={deleteContainerAction} className="text-xs">
                    <input type="hidden" name="containerId" value={container.id} />
                    <button
                      type="submit"
                      className="rounded border border-red-200 px-3 py-1 text-red-600 hover:bg-red-50"
                      disabled={!process.env.ORBIT_PANEL_BASE_URL}
                    >
                      Eliminar
                    </button>
                  </form>
                </li>
              ))}
            </ul>
          ) : (
            <p className="mt-4 text-sm text-slate-500">No hay contenedores para mostrar.</p>
          )}

          <div className="mt-4 flex items-center justify-between text-sm text-slate-500">
            <a
              className={`rounded px-3 py-1 ${safePage <= 1 ? 'cursor-not-allowed opacity-50' : 'hover:bg-slate-100'}`}
              aria-disabled={safePage <= 1}
              href={safePage <= 1 ? '#' : `/?page=${safePage - 1}`}
            >
              Anterior
            </a>
            <a
              className={`rounded px-3 py-1 ${safePage >= totalPages ? 'cursor-not-allowed opacity-50' : 'hover:bg-slate-100'}`}
              aria-disabled={safePage >= totalPages}
              href={safePage >= totalPages ? '#' : `/?page=${safePage + 1}`}
            >
              Siguiente
            </a>
          </div>
        </section>

        <section className="rounded-xl border border-slate-200 bg-white p-6 shadow-sm">
          <h2 className="mb-4 text-lg font-semibold">Ultimas tareas</h2>
          {tasks.length ? (
            <ul className="space-y-3 text-sm text-slate-700">
              {tasks.slice(0, 5).map((task) => (
                <li key={task.id} className="rounded-lg border border-slate-100 px-4 py-3">
                  <p className="font-medium">{task.type}</p>
                  <p className="text-xs text-slate-500">
                    ID: {task.id} - Estado: <span className="uppercase">{task.status}</span>
                  </p>
                </li>
              ))}
            </ul>
          ) : (
            <p className="text-sm text-slate-500">Aun no hay tareas registradas.</p>
          )}
        </section>

        <SystemConfigForm
          defaultBaseUrl={process.env.ORBIT_PANEL_BASE_URL}
          defaultToken={process.env.ORBIT_PANEL_TOKEN}
        />
      </main>
    </div>
  );
}
