use std::sync::Mutex;
use crate::{component::ComponentId, database::Database, entity::EntityId};


pub struct World {
    id: u64,
    entities:   Vec<EntityId>,
    components: Vec<ComponentId>,
    database: Database,
}

impl World {
    pub fn new() -> Self {
        static ID: Mutex<u64> = Mutex::new(0);

        World {
            id: {
                let mut id: std::sync::MutexGuard<'_, u64> = ID.lock().unwrap();
                let next: u64 = id.checked_add(1).unwrap();
                *id = next;
                *id
            },
            entities: Vec::new(),
            components: Vec::new(),
            database: Database::new(),
        }
    }
}