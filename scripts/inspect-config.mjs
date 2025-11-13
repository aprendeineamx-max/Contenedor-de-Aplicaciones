#!/usr/bin/env node
import { fileURLToPath, pathToFileURL } from "node:url";
import path from "node:path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "..");
const basePath = process.env.ORBIT_BASE_URL ?? "http://127.0.0.1:7443";
const token = process.env.ORBIT_ADMIN_TOKEN;

if (!token) {
  console.error("ORBIT_ADMIN_TOKEN es obligatorio para consultar /system/config");
  process.exit(1);
}

const sdkPath = path.join(rootDir, "clients", "panel-sdk", "dist", "index.js");
const sdk = await import(pathToFileURL(sdkPath));

const config = new sdk.Configuration({
  basePath,
  accessToken: token,
});

try {
  const systemApi = new sdk.SystemApi(config);
  const snapshot = await systemApi.systemConfigGet();
  console.log(JSON.stringify(snapshot, null, 2));
} catch (err) {
  console.error("No se pudo consultar /system/config:", err);
  process.exitCode = 1;
}
