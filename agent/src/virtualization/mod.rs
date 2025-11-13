mod filesystem;
mod registry;
mod runtime;

pub use filesystem::{FsLayer, FsSnapshot};
pub use registry::{RegistryLayer, RegistrySnapshot};
pub use runtime::{ProcessLauncher, RuntimeEnv};

use anyhow::Result;
use serde::Serialize;
use std::path::{Path, PathBuf};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Platform {
    WindowsX64,
    WindowsArm64,
}

#[derive(Debug, Clone)]
pub struct SandboxDescriptor {
    pub container_id: Uuid,
    pub name: String,
    pub platform: Platform,
    pub root: PathBuf,
}

impl SandboxDescriptor {
    pub fn new(name: impl Into<String>, platform: Platform, root: impl AsRef<Path>) -> Self {
        Self {
            container_id: Uuid::new_v4(),
            name: name.into(),
            platform,
            root: root.as_ref().to_path_buf(),
        }
    }
}

pub struct SandboxRuntime {
    descriptor: SandboxDescriptor,
    fs: FsLayer,
    registry: RegistryLayer,
    launcher: ProcessLauncher,
}

impl SandboxRuntime {
    pub fn new(descriptor: SandboxDescriptor) -> Self {
        let root = descriptor.root.clone();
        Self {
            descriptor,
            fs: FsLayer::new(root.clone()),
            registry: RegistryLayer::new(root.clone()),
            launcher: ProcessLauncher::new(root.join("runtime")),
        }
    }

    pub async fn prepare(&self) -> Result<()> {
        self.fs.prepare().await?;
        self.registry.prepare().await?;
        Ok(())
    }

    pub async fn persist_manifest(&self) -> Result<()> {
        let manifest = SandboxManifest {
            container_id: self.descriptor.container_id,
            name: self.descriptor.name.clone(),
            platform: self.descriptor.platform.clone(),
            created_at: OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_default(),
            filesystem: self.fs.snapshot(),
            registry: self.registry.snapshot(),
            runtime: self.launcher.runtime_env(self.descriptor.container_id),
        };

        let manifest_path = self.descriptor.root.join("runtime").join("manifest.json");
        let json = serde_json::to_vec_pretty(&manifest)?;
        tokio::fs::write(&manifest_path, json).await?;
        Ok(())
    }

    pub fn descriptor(&self) -> &SandboxDescriptor {
        &self.descriptor
    }
}

#[derive(Debug, Serialize)]
struct SandboxManifest {
    container_id: Uuid,
    name: String,
    platform: Platform,
    created_at: String,
    filesystem: FsSnapshot,
    registry: registry::RegistrySnapshot,
    runtime: RuntimeEnv,
}
