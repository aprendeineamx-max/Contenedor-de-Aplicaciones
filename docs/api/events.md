# Esquema de Eventos en Tiempo Real

## Transporte
- **WebSocket seguro** en `wss://localhost:7443/ws` (mismo host del API).  
- **Server-Sent Events** opcional para clientes ligeros (`/events/stream`).  
- Mensajes JSON codificados en UTF-8.  
- Autenticación reutiliza token Bearer (en `Sec-WebSocket-Protocol` o header `Authorization` para SSE).

## Formato general
```json
{
  "id": "uuid",
  "type": "task.updated",
  "timestamp": "2025-11-13T09:21:05Z",
  "payload": { ... }
}
```

### Campos
- `id`: UUID del evento (permite deduplicar).  
- `type`: nombre jerárquico (`entity.action`).  
- `timestamp`: ISO8601 UTC.  
- `payload`: objeto dependiente del tipo.  
- `context` (opcional): `{ "container_id": "...", "task_id": "...", "user_id": "..." }`

## Tipos principales

### `task.created`
Payload:
```json
{
  "task": {
    "id": "uuid",
    "type": "app.install",
    "status": "queued",
    "progress": 0
  }
}
```

### `task.updated`
Payload incluye campos cambiados:
```json
{
  "task_id": "uuid",
  "status": "running",
  "progress": 45,
  "message": "Descargando instalador"
}
```

### `task.completed`
Se emite con `status` `succeeded | failed | cancelled` y `result`.

### `container.status`
```json
{
  "container_id": "uuid",
  "status": "ready"
}
```

### `container.metrics`
Datos periódicos:
```json
{
  "container_id": "uuid",
  "cpu_percent": 12.5,
  "memory_bytes": 2147483648,
  "disk_bytes": 734003200
}
```

### `app.install.log`
Streaming de logs de instalación:
```json
{
  "app_id": "uuid",
  "task_id": "uuid",
  "level": "info",
  "line": "Copying files..."
}
```

### `snapshot.created`
```json
{
  "snapshot": {
    "id": "uuid",
    "container_id": "uuid",
    "label": "post-install",
    "type": "full"
  }
}
```

### `export.ready`
Notifica exportación disponible:
```json
{
  "package_id": "uuid",
  "container_id": "uuid",
  "location": "file:///C:/Containers/exports/chrome.orbit"
}
```

## Reglas de cliente
- Clientes pueden enviar `ping` → agente responde `pong`.  
- Reintentos SSE con backoff exponencial y `Last-Event-ID`.  
- Para filtrar, clientes envían query `?topics=task.*,container.status`.  
- Eventos se versionan mediante `payload.schema_version`. Cambios incompatibles generan nuevo tipo (`task.updated.v2`).

## Integración
- Panel web se subscribe tras login, muestra toast basado en `task.completed`.  
- CLI puede usar SSE para esperar finalización de tareas sin polling.  
- SDK Python expone `@client.on("task.completed")`.

