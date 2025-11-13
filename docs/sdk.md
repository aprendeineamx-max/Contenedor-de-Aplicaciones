# SDKs y Publicación

## SDK TypeScript (`clients/panel-sdk`)

1. Instalar dependencias y compilar:
   ```bash
   cd clients/panel-sdk
   npm install
   npm run build
   ```
2. Empaquetar:
   ```bash
   npm pack
   ```
   El archivo `.tgz` se copia a `artifacts/` para facilitar la publicación manual.
3. Publicar en un registry privado/npm (requiere `npm login` configurado):
   ```bash
   npm publish artifacts/orbit-panel-sdk-0.1.0.tgz --tag beta
   ```
4. Consumir desde un proyecto React/Next:
   ```bash
   npm install @orbit/panel-sdk@beta
   ```
   o apuntar al tarball directamente:
   ```bash
   npm install file:../artifacts/orbit-panel-sdk-0.1.0.tgz
   ```

## SDK Rust (`clients/cli-rs`)

Este repo ahora es un workspace (`Cargo.toml` en la raíz). Para usar el cliente directamente desde Git mientras se publica en crates.io:

```toml
[dependencies]
orbit_cli_sdk = { git = "https://github.com/aprendeineamx-max/Contenedor-de-Aplicaciones.git", package = "orbit_cli_sdk", branch = "master" }
```

Se recomienda fijar `rev = "1884d14"` (o el commit deseado) para builds reproducibles.

### Publicación futura
- Cuando se quiera publicar en crates.io, ejecutar `cargo publish -p orbit_cli_sdk` desde la raíz.
- Antes, actualizar `clients/cli-rs/Cargo.toml` con la metadata final (description, repository, license).

## Checklist antes de liberar una versión
- [ ] Actualizar `docs/api/openapi.yaml`.
- [ ] Regenerar clientes (`npm run regenerate-sdks` cuando exista script).
- [ ] Ejecutar `npm pack` y subir el tarball al registry/npm.
- [ ] Asegurar que el commit de referencia está etiquetado (`git tag sdk-v0.1.0`).
- [ ] Actualizar este documento con la nueva versión del paquete.
