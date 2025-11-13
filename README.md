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
