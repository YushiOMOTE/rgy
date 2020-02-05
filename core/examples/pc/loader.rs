use log::*;
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn load_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut f = File::open(path).expect("Couldn't open file");
    let mut buf = Vec::new();

    f.read_to_end(&mut buf).expect("Couldn't read file");

    buf
}

pub struct Loader {
    roms: HashMap<String, PathBuf>,
}

impl Loader {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let paths = read_dir(path).unwrap();

        Self {
            roms: paths
                .filter_map(|p| {
                    let path = p.ok()?.path();
                    let key = path.file_stem()?.to_str()?.to_string();
                    let ext = path.extension()?;
                    if ext == "gb" || ext == "gbc" || ext == "rom" {
                        Some((key, path))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}

impl utils::Loader for Loader {
    fn roms(&mut self) -> Vec<String> {
        self.roms
            .iter()
            .map(|(key, _)| {
                info!("ROM: {}", key);
                key.clone()
            })
            .collect()
    }

    fn load(&mut self, rom: &str) -> Vec<u8> {
        load_rom(self.roms.get(rom).unwrap())
    }
}
