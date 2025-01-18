use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;

use anyhow::Result;
use parking_lot::Mutex;
use tracing::{error, info};

use crate::{BundlePersistFormat, CobaltFileSystemProxy};

pub struct ScriptSystem {
    file_system: Arc<CobaltFileSystemProxy>,
    opened_scripts: HashMap<String, OpenScript>,
}

impl ScriptSystem {
    pub fn new(file_system: Arc<CobaltFileSystemProxy>) -> Self {
        Self {
            file_system,
            opened_scripts: Default::default(),
        }
    }

    pub fn open(
        &mut self,
        script_name: &str,
        editor_program: &str,
        editor_args: &str,
    ) -> Result<()> {
        let script_path = if let Some(script) = self.opened_scripts.get(script_name) {
            script.absolute_script_path.clone()
        } else {
            let script = OpenScript::load(&self.file_system, script_name)?;
            let path = script.absolute_script_path.clone();
            self.opened_scripts.insert(script_name.to_string(), script);
            path
        };

        let args_with_file_path =
            editor_args.replace("$FILE", script_path.to_string_lossy().as_ref());
        let full_args: Vec<&str> = args_with_file_path.split_ascii_whitespace().collect();

        info!("Opening script with command '{} {}'", editor_program, args_with_file_path);
        Command::new(editor_program).args(&full_args).spawn()?;
        info!("Successfully ran command to open script '{}'", script_name);

        Ok(())
    }

    pub fn save(&self, backup_root: &Path) -> Result<()> {
        for script in self.opened_scripts.values() {
            script.save(&self.file_system, backup_root)?;
        }
        Ok(())
    }

    pub fn forget(&mut self, script_name: &str) {
        self.opened_scripts.remove(script_name);
    }

    pub fn list_open(&self) -> HashSet<String> {
        self.opened_scripts.keys().map(|k| k.to_string()).collect()
    }

    pub fn list_all(&self) -> BTreeSet<String> {
        match self.file_system.list_scripts() {
            Ok(scripts) => scripts,
            Err(err) => {
                error!("Failed to list scripts: {:?}", err);
                BTreeSet::new()
            }
        }
    }
}

pub struct OpenScript {
    absolute_script_path: PathBuf,
    persist_format: Mutex<BundlePersistFormat>,
}

impl OpenScript {
    pub fn load(file_system: &CobaltFileSystemProxy, script_file_name: &str) -> Result<Self> {
        info!("Loading script {}", script_file_name);

        let (absolute_script_path, persist_format) = file_system.read_script(script_file_name)?;

        info!("Loaded script {:?}", persist_format);

        Ok(Self {
            absolute_script_path,
            persist_format: Mutex::new(persist_format),
        })
    }

    pub fn save(&self, file_system: &CobaltFileSystemProxy, backup_root: &Path) -> Result<()> {
        info!("Saving script {:?}", self.persist_format);
        file_system.save_script(
            &self.absolute_script_path,
            &mut self.persist_format.lock(),
            backup_root,
        )
    }
}
