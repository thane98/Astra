use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;

use anyhow::Result;
use astra_formats::TextBundle;
use parking_lot::Mutex;

use crate::LocalizedFileSystem;

pub struct ScriptSystem {
    file_system: Arc<LocalizedFileSystem>,
    opened_scripts: HashMap<String, OpenScript>,
}

impl ScriptSystem {
    pub fn new(file_system: Arc<LocalizedFileSystem>) -> Self {
        Self {
            file_system,
            opened_scripts: HashMap::new(),
        }
    }

    pub fn open(
        &mut self,
        script_name: &str,
        editor_program: &str,
        editor_args: &str,
    ) -> Result<()> {
        let root = self.file_system.root();
        let script_path = if let Some(script) = self.opened_scripts.get(script_name) {
            root.join(&script.script_path)
        } else {
            let script = OpenScript::load(&self.file_system, script_name)?;
            let path = root.join(&script.script_path);
            self.opened_scripts.insert(script_name.to_string(), script);
            path
        };

        let args_with_file_path =
            editor_args.replace("$FILE", script_path.to_string_lossy().as_ref());
        let full_args: Vec<&str> = args_with_file_path.split_ascii_whitespace().collect();

        Command::new(editor_program).args(&full_args).spawn()?;
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

    pub fn list(&self) -> impl Iterator<Item = &String> {
        self.opened_scripts.keys()
    }
}

struct OpenScript {
    bundle_path: PathBuf,
    script_path: PathBuf,
    bundle: Mutex<TextBundle>,
}

impl OpenScript {
    pub fn load(file_system: &LocalizedFileSystem, script_name: &str) -> Result<Self> {
        let base_path = Path::new(r"StreamingAssets\aa\Switch\fe_assets_scripts").join(script_name);

        // Load the bundle.
        let bundle_path = base_path.with_extension("txt.bundle");
        let raw_bundle = file_system.read(&bundle_path, false)?;
        let mut bundle = TextBundle::from_slice(&raw_bundle)?;

        // Extract the script. If it doesn't exist on disk, save it.
        // Otherwise, dropping it saves some memory since we have to hold the bundle.
        let script_contents = bundle.take_raw()?;
        let script_path = base_path.with_extension("lua");
        if !file_system.exists(&script_path, false)? {
            file_system.write(&script_path, &script_contents, false)?;
        }

        Ok(Self {
            bundle_path,
            script_path,
            bundle: Mutex::new(bundle),
        })
    }

    pub fn save(&self, file_system: &LocalizedFileSystem, backup_root: &Path) -> Result<()> {
        file_system.backup(&self.script_path, backup_root, false)?;
        let script = file_system.read(&self.script_path, false)?;
        let mut bundle = self.bundle.lock();
        bundle.replace_raw(script)?;
        file_system.write(&self.bundle_path, &bundle.serialize()?, false)
    }
}
