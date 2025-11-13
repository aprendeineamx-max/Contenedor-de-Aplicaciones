use anyhow::Result;
use serde::Serialize;
use std::path::{Path, PathBuf};
use tokio::fs;

#[derive(Clone, Debug)]
pub struct RegistryLayer {
    root: PathBuf,
}

impl RegistryLayer {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    pub async fn prepare(&self) -> Result<()> {
        let hive_dir = self.root.join("registry");
        fs::create_dir_all(&hive_dir).await?;
        for hive in ["SOFTWARE.reg", "SYSTEM.reg", "NTUSER.dat"] {
            let file_path = hive_dir.join(hive);
            if !file_path.exists() {
                fs::write(&file_path, b"; orbit placeholder hive\n").await?;
            }
        }
        Ok(())
    }

    pub fn snapshot(&self) -> RegistrySnapshot {
        let base = self.root.join("registry");
        RegistrySnapshot {
            software: base.join("SOFTWARE.reg").display().to_string(),
            system: base.join("SYSTEM.reg").display().to_string(),
            ntuser: base.join("NTUSER.dat").display().to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RegistrySnapshot {
    pub software: String,
    pub system: String,
    pub ntuser: String,
}
