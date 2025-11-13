# Arquitectura Técnica

## Resumen
Sistema orientado a contenedores portables para aplicaciones Win32. Cada contenedor encapsula filesystem, registro, variables y configuración de red para ejecutar versiones múltiples de un mismo software sin conflictos y con portabilidad entre equipos.

## Principios
- **Aislamiento determinista**: toda lectura/escritura fuera del contenedor debe redirigirse.
- **Portabilidad total**: carpeta del contenedor contiene binarios, datos, hives de registro y metadatos.
- **Automatización**: procesos largos (instalación, snapshot, export) se manejan como tareas con seguimiento.
- **Observabilidad**: telemetría estructurada para auditar acciones y diagnósticos.
- **Seguridad por diseño**: servicios autenticados, certificados locales, y límites de privilegios en procesos huéspedes.

## Módulos principales
1. **Agente Core (servicio Windows en Rust)**  
   - Gestiona ciclo de vida de contenedores (`create`, `clone`, `snapshot`, `export`).  
   - Orquesta hooking/driver para redirección de IO.  
   - Expone API REST/gRPC y WebSocket seguro.  
   - Persistencia en SQLite (se puede escalar a PostgreSQL embebido).

2. **Capa de Virtualización de IO**  
   - **Filesystem**: Driver WinFSP/Dokan en modo overlay → monta `C:\Containers\<id>\fs`.  
   - **Registro**: hives dedicados (`SYSTEM.reg`, `SOFTWARE.reg`, `NTUSER.dat`) + `RegOverridePredefKey`.  
   - **Proceso Launcher**: ejecuta app dentro de Job Object aislado con variables y rutas modificadas.  
   - **Opcional**: hooking de APIs críticas (File/Reg/Process) mediante Microsoft Detours o EasyHook.

3. **Backend/API Layer**  
   - Framework sugerido: Axum (Rust) o Actix Web.  
   - Autenticación mediante OAuth local + WebAuthn/FIDO opcional.  
   - Motor de colas: NATS JetStream o SQLite job queue para tareas largas.  
   - Exposición de eventos (WebSocket + SSE) para panel web.

4. **Panel Web**  
   - Next.js 14, React Server Components, Tailwind + Radix UI.  
   - Layout tipo dashboard con tarjetas de contenedores, timeline de eventos, asistentes de instalación.  
   - Websocket client para estado en vivo, drag & drop para import/export.

5. **CLI y SDK**  
   - CLI (Rust, `clap`) para scripting: `orbit container create`, `orbit app install`.  
   - SDK Python para integrarse con pipelines QA.

## Flujo de instalación guiada
1. Usuario selecciona instalador desde panel/CLI.  
2. Agente crea snapshot base del contenedor y lanza instalador dentro del runtime.  
3. Hook/driver intercepta rutas `Program Files`, `AppData`, `Temp` y las redirige a la carpeta del contenedor.  
4. Al finalizar, se genera un manifiesto (`manifest.json`) con lista de archivos, claves de registro y dependencias.  
5. Se crea snapshot `post-install` para permitir rollback.

## Estructura de un contenedor
```
Containers/
  <container-id>/
    fs/                # filesystem redirigido
    registry/
      SOFTWARE.reg
      SYSTEM.reg
      NTUSER.dat
    runtime/
      env.json         # variables y rutas virtuales
      manifest.json    # metadatos y checksums
    snapshots/
      000-base/
      001-post-install/
    logs/
```

## Requerimientos técnicos clave
- Soporte mínimo Windows 10 21H2 / Windows Server 2019.  
- Permisos admin para instalar driver WinFSP/Dokan y servicio.  
- Certificado local para firmar driver/servicio (modo producción).  
- Dependencias: Rust 1.80+, Node 20 LTS, SQLite 3.45+, NATS (opcional).

## Seguridad y aislamiento
- Middleware de autenticacion por token opcional (Bearer) configurable via ORBIT_AUTH_ENABLED/ORBIT_ADMIN_TOKEN.

- Servicio se ejecuta como cuenta dedicada con privilegios limitados; elevaciones se controlan por tarea.  
- TLS mutuo entre panel y API cuando se expone remotamente.  
- Sandboxing adicional con Windows Defender Application Control policies para procesos del contenedor.  
- Auditoría: cada acción registra usuario, timestamp, hash de instalador y resultados.

### Gestion de configuracion y secretos
- La configuracion base residira en "config/orbit.toml" y se complementara con overrides locales ("orbit-data/config.local.toml").
- Las variables de entorno siguen teniendo prioridad para credenciales y rutas sensibles ("ORBIT_ADMIN_TOKEN", "ORBIT_DB_PATH", etc.).
- Se expondran endpoints /system/config (solo lectura admin) y /system/security/reload (hot reload existente) para aplicar cambios sin reiniciar.
- Los tokens de servicio incluyen scopes, expiracion y "last_used_at", preparando la futura interfaz para auditoria y control de permisos.
- QA puede inspeccionar rapidamente el entorno ejecutando `npm run inspect-config` (SDK TypeScript) o `cargo run --example system_config` dentro de `clients/cli-rs` (SDK Rust).
## Observabilidad
- Logging estructurado vía OpenTelemetry + `tracing` crate.  
- Métricas expuestas en `/metrics` (Prometheus).  
- Reportes de estado: consumo de disco por contenedor, tasa de fallos de tareas, latencia de operaciones IO.

## Consideraciones de compatibilidad
- Hooking debe cubrir APIs Win32 clásicas y .NET (CreateFile/RegSetValue).  
- Manejar drivers/servicios instalados por la app (advertir limitaciones o usar modo Hyper-V para casos extremos).  
- Controlar accesos a recursos globales (imágenes COM, servicios) mediante plantillas de permisos.



