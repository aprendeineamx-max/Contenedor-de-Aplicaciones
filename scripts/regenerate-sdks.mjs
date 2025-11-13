#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { createRequire } from "node:module";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "..");

const specArg = path.posix.join("docs", "api", "openapi.yaml");
const tsOutRelFs = path.join("clients", "panel-sdk");
const tsOutArg = path.posix.join("clients", "panel-sdk");
const rsOutRelFs = path.join("clients", "cli-rs");
const rsOutArg = path.posix.join("clients", "cli-rs");
const artifactsRelFs = path.join("artifacts");
const tsOutDir = path.join(rootDir, tsOutRelFs);
const rsOutDir = path.join(rootDir, rsOutRelFs);
const artifactsDir = path.join(rootDir, artifactsRelFs);
const npmCmd = process.platform === "win32" ? "npm.cmd" : "npm";
const require = createRequire(import.meta.url);
const openapiCli = require.resolve("@openapitools/openapi-generator-cli/main.js");

const defaultJavaBin = path.join(
  "C:\\Program Files\\Eclipse Adoptium",
  "jdk-17.0.16.8-hotspot",
  "bin",
);

if (process.platform === "win32" && existsSync(defaultJavaBin)) {
  if (!process.env.JAVA_HOME) {
    process.env.JAVA_HOME = path.dirname(defaultJavaBin);
  }
  if (!process.env.PATH.includes(defaultJavaBin)) {
    process.env.PATH = `${defaultJavaBin}${path.delimiter}${process.env.PATH}`;
  }
}

if (!existsSync(artifactsDir)) {
  mkdirSync(artifactsDir, { recursive: true });
}

const quoteArg = (value) =>
  /\s/.test(value) ? `"${value.replace(/"/g, '\\"')}"` : value;

function run(command, args, options = {}) {
  const cwd = options.cwd ?? rootDir;
  const useShell = process.platform === "win32" && command.endsWith(".cmd");
  const result = useShell
    ? spawnSync([command, ...args.map(quoteArg)].join(" "), {
        cwd,
        stdio: "inherit",
        shell: true,
        ...options,
      })
    : spawnSync(command, args, {
        cwd,
        stdio: "inherit",
        shell: false,
        ...options,
      });
  if (result.status !== 0) {
    if (result.error) {
      console.error(result.error);
    }
    throw new Error(`Command failed: ${command} ${args.join(" ")}`);
  }
}

console.log("→ Generando SDK TypeScript…");
run(process.execPath, [
  openapiCli,
  "generate",
  "-i",
  specArg,
  "-g",
  "typescript-fetch",
  "-o",
  tsOutArg,
  "--additional-properties=supportsES6=true,npmName=@orbit/panel-sdk,npmVersion=0.1.0",
]);
console.log("✓ SDK TypeScript generado.");

console.log("→ Generando SDK Rust…");
run(process.execPath, [
  openapiCli,
  "generate",
  "-i",
  specArg,
  "-g",
  "rust",
  "-o",
  rsOutArg,
  "--additional-properties=packageName=orbit_cli_sdk,packageVersion=0.1.0",
]);
console.log("✓ SDK Rust generado.");

console.log("→ Instalando dependencias del SDK TS…");
run(npmCmd, ["install"], { cwd: tsOutDir });
console.log("✓ Dependencias instaladas.");

console.log("→ Empaquetando SDK TS…");
run(
  npmCmd,
  ["pack", "--pack-destination", artifactsDir],
  { cwd: tsOutDir },
);
console.log("✓ Paquete listo en artifacts/.");

console.log("SDKs generados y empaquetados correctamente.");
