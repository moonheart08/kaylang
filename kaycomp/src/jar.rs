use std::{path::PathBuf, error::Error, sync::Mutex};
use salsa::DebugWithDb;
use dashmap::{DashMap, mapref::entry::Entry};

use crate::parsing::{File, LexedFile, lex};

#[salsa::jar(db = Db)]
pub struct Jar(File, LexedFile, lex);

#[salsa::db(Jar)]
pub struct Database {
    storage: salsa::Storage<Self>,
    logs: Mutex<Vec<String>>,
    files: DashMap<PathBuf, File>,
}

impl Database {
    pub fn new() -> Self {
        let storage = Default::default();
        Self {
            storage,
            logs: Default::default(),
            files: DashMap::new(),
        }
    }
}

pub trait Db: salsa::DbWithJar<Jar> {
    fn input(&self, path: PathBuf) -> Result<crate::parsing::File, Box<dyn Error>>;
}

impl Db for Database
{
    fn input(&self, path: PathBuf) -> Result<File, Box<dyn Error>> {
        let path = path
            .canonicalize()?;
        Ok(match self.files.entry(path.clone()) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let contents = std::fs::read_to_string(&path)?;
                *entry.insert(File::new(self, path, contents))
            }
        })
    }
}

impl salsa::Database for Database {
    fn salsa_event(&self, event: salsa::Event) {
        if let salsa::EventKind::WillExecute { .. } = event.kind {
            self.logs
                .lock()
                .unwrap()
                .push(format!("{:?}", event.debug(self)));
        }
    }
}