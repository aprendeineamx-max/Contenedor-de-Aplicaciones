# Roadmap e Iteraciones

## Fase 0 – Investigación (Semanas 1-3)
- Validar mecanismos de redirección filesystem: comparar WinFSP/Dokan vs minifilter propio en Rust/C++.  
- PoC de `RegOverridePredefKey` + hooking para registro.  
- Pruebas con instaladores comunes (MSI, EXE, instaladores silenciosos) midiendo cobertura.

## Fase 1 – Diseño y Fundamentos (Semanas 4-6)
- Especificación funcional detallada (casos de uso QA, multiusuario).  
- Definición de contratos API (OpenAPI + esquema eventos).  
- Modelo de datos `Container`, `AppInstance`, `Task`, `Snapshot`.  
- Diseñar estrategia de autenticación y permisos.

## Fase 2 – MVP del Agente (Semanas 7-12)
- Servicio Windows en Rust (Tokio) con endpoints CRUD de contenedores.  
- Integración inicial con WinFSP/Dokan (modo user).  
- Ejecución de procesos aislados con job objects y variables redirigidas.  
- Registro básico de tareas y logs.  
- CLI mínima (`orbit container create`, `orbit app install`).  
- Tests unitarios + integración (Pester para escenarios Windows).

## Fase 3 – Panel Web y API Pública (Semanas 13-18)
- Backend REST/gRPC consolidado, WebSocket de eventos.  
- Panel Next.js con dashboards, wizard de instalación, monitor de tareas.  
- Theming moderno (Tailwind + Radix).  
- Autenticación local (OAuth password + WebAuthn opcional).  
- Empaquetado inicial (MSIX/Inno) para distribución interna.

## Fase 4 – Funcionalidades Avanzadas (Semanas 19-26)
- Snapshots diferenciales y rollback con copy-on-write.  
- Export/import (`.orbit` zip firmado).  
- Integración de cola NATS/SQLite para tareas concurrentes.  
- APIs para scripts QA (SDK Python).  
- Reglas de firewall y límites de red por contenedor.  
- Sistema de reportes (runbooks de pruebas, generador PDF/HTML).

## Fase 5 – Hardening y Publicación (Semanas 27-32)
- Certificación de driver/controladores.  
- Seguridad avanzada: TLS mutuo, auditoría, RBAC.  
- Escalabilidad multiusuario (self-hosted).  
- Documentación final + tutoriales video.  
- Preparar lanzamiento beta cerrado.

## Backlog Futuro
- Integración con Hyper-V/WSL2 para apps que requieran kernel drivers.  
- Marketplace de contenedores preconfigurados.  
- Telemetría opcional hacia vendors (con consentimiento).  
- Plugins para pipelines CI/CD (GitHub Actions, Azure DevOps).  
- Compatibilidad macOS/Linux vía VM ligera controlada por el agente.

