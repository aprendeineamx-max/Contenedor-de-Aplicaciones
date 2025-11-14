#!/usr/bin/env node
import { fileURLToPath, pathToFileURL } from 'node:url';
import path from 'node:path';
import { randomUUID } from 'node:crypto';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..');
const basePath = process.env.ORBIT_BASE_URL ?? 'http://127.0.0.1:7443';
const adminToken = process.env.ORBIT_ADMIN_TOKEN;

if (!adminToken) {
  console.error('ORBIT_ADMIN_TOKEN es obligatorio para ejecutar el flujo del panel.');
  process.exit(1);
}

const sdkPath = path.join(rootDir, 'clients', 'panel-sdk', 'dist', 'index.js');
const sdk = await import(pathToFileURL(sdkPath));

const config = new sdk.Configuration({
  basePath,
  accessToken: adminToken,
});

const containersApi = new sdk.ContainersApi(config);
const tasksApi = new sdk.TasksApi(config);
const systemApi = new sdk.SystemApi(config);

async function wait(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

try {
  const info = await systemApi.systemInfoGet();
  console.log('Agente listo:', info);

  const existing = await containersApi.containersGet();
  console.log(`Contenedores existentes: ${existing.length}`);

  const name = `panel-flow-${randomUUID().slice(0, 8)}`;
  console.log('Creando contenedor', name);
  const task = await containersApi.containersPost({
    containersPostRequest: {
      name,
      platform: 'windows-x64',
    },
  });
  console.log('Tarea creada:', task.id);

  console.log('Esperando a que aparezcan tareas...');
  await wait(2000);
  const tasks = await tasksApi.tasksGet({ limit: 10 });
  console.log('Ultimas tareas:', tasks.map((t) => ({ id: t.id, status: t.status, type: t.type })));

  console.log('Flujo completado');
} catch (err) {
  console.error('flujo panel fallido:', err);
  process.exitCode = 1;
}

