#!/usr/bin/env node
import { spawn } from "node:child_process";
import { setTimeout as delay } from "node:timers/promises";
import { fileURLToPath, pathToFileURL } from "node:url";
import path from "node:path";
import { existsSync } from "node:fs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "..");

const fallbackCargo = process.platform === "win32" ? "cargo.exe" : "cargo";
const cargoHome =
  process.env.CARGO_HOME ||
  (process.platform === "win32"
    ? path.join(process.env.USERPROFILE ?? "", ".cargo")
    : path.join(process.env.HOME ?? "", ".cargo"));
const defaultCargo = path.join(
  cargoHome,
  "bin",
  process.platform === "win32" ? "cargo.exe" : "cargo",
);
const cargoCandidates = [
  process.env.CARGO_BIN,
  defaultCargo,
  fallbackCargo,
].filter(Boolean);
const cargoBin =
  cargoCandidates.find((candidate) =>
    path.isAbsolute(candidate)
      ? existsSync(candidate)
      : true,
  ) ?? fallbackCargo;
const adminToken = process.env.ORBIT_SMOKE_ADMIN || "smoke-admin";
const bind = process.env.ORBIT_SMOKE_BIND || "127.0.0.1:7845";
const baseUrl = `http://${bind}`;
const dbPath =
  process.env.ORBIT_SMOKE_DB ||
  path.join(rootDir, "orbit-data", "smoke-agent.db");
const containersRoot =
  process.env.ORBIT_SMOKE_CONTAINERS ||
  path.join(rootDir, "sandboxes", "smoke");

const agentEnv = {
  ...process.env,
  ORBIT_AUTH_ENABLED: "1",
  ORBIT_ADMIN_TOKEN: adminToken,
  ORBIT_API_BIND: bind,
  ORBIT_DB_PATH: dbPath,
  ORBIT_CONTAINERS_ROOT: containersRoot,
};

const agent = spawn(
  cargoBin,
  ["run", "--quiet", "--bin", "agent"],
  {
    cwd: rootDir,
    env: agentEnv,
    stdio: ["ignore", "pipe", "pipe"],
  },
);

agent.stdout.on("data", (chunk) => process.stdout.write(chunk));
agent.stderr.on("data", (chunk) => process.stderr.write(chunk));

const agentReady = new Promise((resolve, reject) => {
  agent.once("spawn", resolve);
  agent.once("error", reject);
});

let shuttingDown = false;
const agentExit = new Promise((resolve, reject) => {
  agent.once("exit", (code, signal) => {
    if (shuttingDown) {
      resolve();
    } else {
      reject(
        new Error(
          `El agente se detuvo antes de completar el smoke (code=${code} signal=${signal})`,
        ),
      );
    }
  });
});

const cleanup = () => {
  shuttingDown = true;
  if (!agent.killed) {
    agent.kill();
  }
};

process.on("SIGINT", () => {
  cleanup();
  process.exit(1);
});
process.on("SIGTERM", () => {
  cleanup();
  process.exit(1);
});

async function waitForServer() {
  const retries = 240;
  for (let attempt = 0; attempt < retries; attempt += 1) {
    try {
      const res = await fetch(`${baseUrl}/system/info`, {
        headers: { Authorization: `Bearer ${adminToken}` },
      });
      if (res.ok) {
        return;
      }
    } catch {
      // ignore until timeout
    }
    await delay(500);
  }
  throw new Error("El agente no respondió a tiempo");
}

async function runSmoke() {
  const sdkPath = path.join(rootDir, "clients", "panel-sdk", "dist", "index.js");
  const sdk = await import(pathToFileURL(sdkPath));
  await agentReady;
  await Promise.race([waitForServer(), agentExit]);

  const adminConfig = new sdk.Configuration({
    basePath: baseUrl,
    accessToken: adminToken,
  });
  const securityApi = new sdk.SecurityApi(adminConfig);
  const expiresAt = new Date(Date.now() + 60 * 60 * 1000).toISOString();
  const issued = await securityApi.securityTokensPost({
    securityTokensPostRequest: {
      name: "smoke-cli",
      scopes: ["containers:write", "tasks:read"],
      expires_at: expiresAt,
    },
  });

  const userConfig = new sdk.Configuration({
    basePath: baseUrl,
    accessToken: issued.token,
  });
  const containersApi = new sdk.ContainersApi(userConfig);
  const tasksApi = new sdk.TasksApi(adminConfig);

  const createResponse = await containersApi.containersPost({
    containersPostRequest: {
      name: `smoke-${Date.now()}`,
      platform: "windows-x64",
    },
  });
  if (!createResponse || !createResponse.id) {
    throw new Error("El contenedor no devolvió un ID de tarea");
  }

  const tasks = await tasksApi.tasksGet();
  if (!Array.isArray(tasks) || tasks.length === 0) {
    throw new Error("No se listaron tareas tras la creación del contenedor");
  }

  console.log("✅ Smoke test completado usando el SDK del panel.");
}

runSmoke()
  .catch((err) => {
    console.error("❌ Smoke test falló:", err);
    process.exitCode = 1;
  })
  .finally(async () => {
    cleanup();
    await agentExit.catch(() => {});
    await delay(200);
  });
