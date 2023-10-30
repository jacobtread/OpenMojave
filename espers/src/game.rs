use crate::common::FormID;
use crate::error::Error;
use crate::plugin::Plugin;
use crate::records::Record;
use crate::string_table::StringTables;
use glob::glob;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

pub struct Game {
    plugins: HashMap<String, Plugin>,
    _string_tables: StringTables,
}

impl Game {
    pub fn load(paths: &[&str], language: &str) -> Result<Self, Error> {
        let mut string_tables = StringTables::new();
        let mut plugins = HashMap::new();

        for path in paths {
            let g = PathBuf::from(path)
                .join("*.es[mp]")
                .to_string_lossy()
                .to_string();
            for plugin_path in glob(&g).unwrap() {
                let pp = plugin_path.unwrap();
                println!("Loading {}", pp.display());
                let plugin = Plugin::parse(&mut File::open(&pp)?)?;
                string_tables.load_plugin_path(pp.display().to_string().as_ref(), language)?;

                plugins.insert(
                    pp.file_name().unwrap().to_string_lossy().to_string(),
                    plugin,
                );
            }
        }

        for plugin in plugins.values_mut() {
            plugin.localize(&string_tables);
        }
        Ok(Self {
            plugins,
            _string_tables: string_tables,
        })
    }

    pub fn plugins(&self) -> &HashMap<String, Plugin> {
        &self.plugins
    }

    pub fn get_record_by_form_id(&self, fid: &FormID) -> Option<&Record> {
        for plugin in self.plugins.values() {
            let rec = plugin
                .form_ids
                .get(&fid.0)
                .and_then(|fid| plugin.get_record_by_key(&fid));

            if rec.is_some() {
                return rec;
            }
        }

        None
    }
}
