# Project ORBIT (working title)

Plataforma para ejecutar múltiples aplicaciones de escritorio Windows dentro de contenedores portables y aislados, manteniendo sus archivos, registro y dependencias en carpetas autocontenidas. El objetivo es permitir instalaciones paralelas de distintas versiones, moverlas entre PCs sin romper rutas, y administrarlas desde un panel web moderno.

## Objetivos
- Virtualizar filesystem, registro y variables por contenedor para que cada app crea que vive en `Program Files/AppData`.
- Automatizar instalaciones guiadas o portables, capturando todos los artefactos dentro del contenedor.
- Ofrecer control centralizado vía API y panel web responsivo para crear, clonar, exportar y monitorear contenedores.
- Facilitar pruebas QA con snapshots, reportes y capacidades de scripting.

> Consulta `docs/sdk.md` para el proceso de empaquetado y publicación de los SDKs (TypeScript + Rust).

## Componentes principales
1. **Agente Core (Rust)**: servicio Windows con privilegios que crea contenedores, aplica virtualización (filesystem + registro) y expone APIs seguras.
2. **Capa de Virtualización**: driver WinFSP/Dokan o minifilter propio + hooking de Win32 para redirigir rutas y claves hacia la carpeta del contenedor.
3. **Backend/API**: servicio REST/gRPC (Rust o Go) con persistencia SQLite/PostgreSQL embebida y cola de tareas.
4. **Panel Web (Next.js)**: dashboard responsivo con control de contenedores, estado en tiempo real y diseño profesional.
5. **CLI/SDK**: herramientas para automatizar tareas y scripts de QA.

## Estado actual
- Documentación inicial en `docs/`.
- Aún no hay código fuente; la prioridad inmediata es detallar arquitectura y roadmap antes de generar scaffolding.

## Próximos pasos
1. Detallar arquitectura en `docs/architecture.md`, incluyendo diagramas, flujos y decisiones tecnológicas.
2. Elaborar roadmap con milestones y dependencias.
3. Preparar PoC de virtualización (Rust + WinFSP/Dokan) antes de construir el stack completo.

## Configuracion de seguridad

El agente expone middleware Bearer y admite tres variables de entorno principales:

- `ORBIT_AUTH_ENABLED`: activa la autenticacion (usa `1` o `true`).
- `ORBIT_ADMIN_TOKEN`: credencial maestra con permisos totales (requerido cuando `auth_enabled = true`).
- `ORBIT_API_TOKENS`: lista separada por comas para tokens estaticos que no se almacenan en la base.

Para recargar cambios sin reiniciar el proceso usa `POST /system/security/reload`, que vuelve a leer las variables anteriores y actualiza el middleware en caliente.

### Tokens de servicio administrados

- `POST /security/tokens`: emite un token de servicio para CLI/automatizaciones, devuelve el valor completo solo una vez.
- `GET /security/tokens`: lista los tokens con su prefijo y fecha de emision/revocacion.
- `DELETE /security/tokens/{id}`: revoca un token activo.

Los tokens emitidos se almacenan en la tabla `api_tokens` con hash SHA-256 y los prefijos permiten auditarlos desde la UI o CLI sin exponer el valor real.

#### Scopes y caducidad

Cada token de servicio puede declararse con un conjunto de scopes (`containers:read`, `containers:write`, `tasks:read`, etc.) y una fecha de expiracion RFC3339. El agente persiste `scopes`, `expires_at` y `last_used_at` para que la UI pueda mostrar reglas de acceso, advertir sobre caducidades y preparar una futura capa de permisos granulares. El endpoint `/system/security/reload` ahora devuelve un resumen (`managed_token_count`, `expiring_token_count`, `scopes_catalog`) para poblar dashboards y alertas.

## Configuracion centralizada

- Define los valores por defecto en `config/orbit.toml` y, opcionalmente, crea un `orbit-data/config.local.toml` ignorado por git para ajustes por entorno.
- Las variables de entorno siguen teniendo prioridad; el endpoint `GET /system/config` (requiere token admin) muestra el snapshot resultante y las fuentes que se aplicaron.
- `POST /system/security/reload` recarga los tokens estaticos/env sin reiniciar el proceso; futuros endpoints `/system/config/reload` reutilizaran la misma base.

### Pruebas end-to-end rapidas

Ejecuta `npm run smoke` desde la raiz para lanzar el agente temporalmente (con `cargo run`), emitir un token via SDK TypeScript, crear un contenedor y validar que la API responde. El script usa el SDK generado en `clients/panel-sdk` y simula el camino panel → API, por lo que es ideal antes de integrar una UI real.

### Inspeccion rapida del entorno
- **Panel SDK**: `ORBIT_BASE_URL=http://127.0.0.1:7443 ORBIT_ADMIN_TOKEN=<token> npm run inspect-config` mostrara el snapshot de configuracion y las fuentes aplicadas.
- **CLI SDK**: `cd clients/cli-rs && ORBIT_BASE_URL=http://127.0.0.1:7443 ORBIT_ADMIN_TOKEN=<token> cargo run --example system_config --features rustls-tls` (o `--features native-tls`) imprime el mismo snapshot usando el SDK Rust.
- **Nueva CLI (`orbit`)**: `cargo run -p cli -- system config --base-url http://127.0.0.1:7443 --admin-token <token>` devolverA el mismo snapshot (usa por defecto las variables `ORBIT_BASE_URL` y `ORBIT_ADMIN_TOKEN`). Usa `--format json` para salida compacta.
- **Panel flow**: `ORBIT_BASE_URL=http://127.0.0.1:7443 ORBIT_ADMIN_TOKEN=<token> npm run panel-flow` ejecuta un ciclo panel-like (listar contenedores, crear uno nuevo y consultar /tasks) usando el SDK TypeScript. Usa este script tras cada cambio del backend para validar la integraci�n sin interfaz.


### Perfilado y logs de memoria para `cargo test`

Ejecuta `powershell -NoProfile -ExecutionPolicy Bypass -File scripts/profile-tests.ps1` para lanzar `cargo test -p agent -- --test-threads=1` mientras se muestrean los procesos `cargo.exe`, `rustc.exe` y `link.exe`. Los datos se guardan (CSV) en `artifacts/mem-log.csv` con columnas `timestamp,process_name,pid,working_set_mb,virtual_mb`. Ajusta los parametros `-Command`, `-IntervalMs` o `-LogPath` segun sea necesario. Este mecanismo genera evidencia del consumo previo al error 1455 y deja trazabilidad directa para reportarlo.

