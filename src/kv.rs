use sled::Db;
use std::sync::Arc;

pub trait KVStore {
    fn init(file_path: &str) -> Self;
    fn save(&self, k: &str, v: &str) -> bool;
    fn find(&self, k: &str) -> Option<String>;
    fn delete(&self, k: &str) -> bool;
}

#[derive(Clone)]
pub struct SledDb {
    db: Arc<Db>,
}

impl KVStore for SledDb {
    fn init(file_path: &str) -> Self {
        SledDb {
            db: Arc::new(sled::open(file_path).unwrap()),
        }
    }

    fn save(&self, k: &str, v: &str) -> bool {
        self.db.insert(k.as_bytes(), v.as_bytes()).is_ok()
    }

    fn find(&self, k: &str) -> Option<String> {
        match self.db.get(k) {
            Ok(Some(v)) => {
                let result = String::from_utf8(v.to_vec()).unwrap();
                println!("Finding '{}' returns '{}'", k, result);
                Some(result)
            }
            Ok(None) => {
                println!("Finding '{}' returns None", k);
                None
            }
            Err(e) => {
                println!("Error retrieving value for {}: {}", k, e);
                None
            }
        }
    }

    fn delete(&self, k: &str) -> bool {
        self.db.remove(k.as_bytes()).is_ok()
    }
}
