use anyhow::Result;
use serde::Serialize;
use std::path::{Path, PathBuf};
use tokio::fs;

const REQUIRED_DIRS: &[&str] = &["fs", "registry", "runtime", "logs"];

#[derive(Clone, Debug)]
pub struct FsLayer {
    root: PathBuf,
}

impl FsLayer {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    pub async fn prepare(&self) -> Result<()> {
        for dir in REQUIRED_DIRS {
            fs::create_dir_all(self.root.join(dir)).await?;
        }

        // Pre-crear directorios típicos para overlays.
        for rel in [
            "ProgramFiles",
            "ProgramData",
            "Users\\Default\\AppData\\Local",
        ] {
            fs::create_dir_all(self.root.join("fs").join(rel)).await?;
        }

        Ok(())
    }

    pub fn snapshot(&self) -> FsSnapshot {
        let mount_root = self.root.join("fs");
        let overlays = vec![
            VirtualMount::new("C:\\\\Program Files", mount_root.join("ProgramFiles")),
            VirtualMount::new("C:\\\\ProgramData", mount_root.join("ProgramData")),
            VirtualMount::new(
                "%LOCALAPPDATA%",
                mount_root.join("Users/Default/AppData/Local"),
            ),
        ];

        FsSnapshot {
            mount_root: mount_root.display().to_string(),
            overlays,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FsSnapshot {
    pub mount_root: String,
    pub overlays: Vec<VirtualMount>,
}

#[derive(Debug, Serialize)]
pub struct VirtualMount {
    pub virtual_path: String,
    pub physical_path: String,
}

impl VirtualMount {
    pub fn new(path: impl Into<String>, physical: impl AsRef<Path>) -> Self {
        Self {
            virtual_path: path.into(),
            physical_path: physical.as_ref().display().to_string(),
        }
    }
}
