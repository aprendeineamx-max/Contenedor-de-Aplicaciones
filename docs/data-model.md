# Modelo de Datos

## Entidades principales

### Container
- `id` (UUID)  
- `name` (string único por usuario)  
- `description` (string)  
- `status` (`creating`, `ready`, `running`, `error`, `archived`)  
- `platform` (`windows-x64`, `windows-arm64`, futuro `linux-x64`)  
- `created_at`, `updated_at` (timestamps)  
- `storage_path` (ruta absoluta del contenedor)  
- `size_bytes` (actualizado por telemetría)  
- `tags` (array string)  
- `settings` (JSON: límites CPU/RAM, reglas red, flags de compatibilidad)

### AppInstance
- `id` (UUID)  
- `container_id` (FK → Container)  
- `name` (ej. “Chrome 122 Beta”)  
- `version` (string)  
- `vendor` (string)  
- `install_source` (ruta/URL del instalador original)  
- `entry_points` (array de comandos con iconos/opciones)  
- `status` (`installing`, `ready`, `failed`, `disabled`)  
- `created_at`, `updated_at`

### Snapshot
- `id` (UUID)  
- `container_id` (FK)  
- `label` (string)  
- `type` (`full`, `delta`)  
- `base_snapshot_id` (nullable, para diferenciales)  
- `size_bytes`  
- `checksum_manifest` (hash del contenido)  
- `created_at`  
- `trigger` (`system`, `manual`, `pre-install`, `post-install`)

### Task
- `id` (UUID)  
- `type` (`container.create`, `app.install`, `snapshot.create`, `export`, etc.)  
- `status` (`queued`, `running`, `succeeded`, `failed`, `cancelled`)  
- `progress` (0-100)  
- `created_at`, `updated_at`, `started_at`, `finished_at`  
- `payload` (JSON con parámetros)  
- `result` (JSON con salidas, logs resumidos, paths)

### EventLog
- `id` (UUID)  
- `timestamp`  
- `source` (`agent`, `driver`, `user`, `app`)  
- `level` (`info`, `warning`, `error`, `audit`)  
- `message` (string)  
- `context` (JSON: container_id, task_id, app_instance_id)  
- `tags` (array)

### User / AuthSession (para panel multiusuario)
- `user.id`, `email`, `display_name`, `role` (`owner`, `operator`, `viewer`)  
- `session.id`, `user_id`, `created_at`, `expires_at`, `mfa_state`, `client_info`.

### ExportPackage
- `id`  
- `container_id`  
- `format` (`zip`, `7z`)  
- `location` (ruta/URL)  
- `status`  
- `integrity_hash`  
- `created_at`

## Relaciones clave
- `Container` 1—N `AppInstance`  
- `Container` 1—N `Snapshot` (con jerarquía via `base_snapshot_id`)  
- `Container` 1—N `Task` (algunas tareas globales sin contenedor)  
- `Task` 1—N `EventLog` (contextual)  
- `AppInstance` se vincula con tareas de instalación/actualización.  
- `User` 1—N `Task` (quién disparó) y `Session`.

## Reglas y constraints
- No se puede eliminar `Container` con `Task` en progreso.  
- Al clonar contenedor se crea `Task` `container.clone` + snapshot inicial.  
- `Snapshot.delta` requiere `base_snapshot_id` válido y mismo `container_id`.  
- `AppInstance` `status=ready` implica al menos un `entry_point`.  
- `ExportPackage` sólo puede crearse desde `Task` `export`; `status` sigue el de la tarea.

## Metadatos dentro del contenedor
Cada contenedor guarda un `manifest.json` con:
- `container_id`, `schema_version`.  
- Lista de `app_instances` con rutas locales.  
- `snapshots` disponibles y hashes.  
- `runtime.env` (variables) y `runtime.mounts`.  
- `compat_flags` (ej. “force_32bit_path”, “disable_hw_accel”).

## Persistencia
- Base principal SQLite con WAL activado; tablas normalizadas según entidades arriba.  
- Archivos grandes (snapshots, export) quedan en filesystem y se referencian desde la DB con `storage_path`.  
- Versionado de esquema con `refinery` o `sqlx migrate`.

