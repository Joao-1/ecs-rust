use std::collections::HashMap;
use crate::{component::ComponentId, entity::EntityId};

// A table is a collection of rows and columns. Each row represents an entity and each column represents a component.
// The table data is stored in a 2D array where the first dimension represents the columns and the second dimension represents the rows.
// The columns are stored in a HashMap where the key is the component id and the value is the vector index of the column in the data array.
// The rows are stored in a HashMap where the key is the entity id and the value is the vector index of the row in the data array.

// Example:

// columns indexes       ->        0,               1,              2,               3
// columns(components)   -> {     86:0,           123:1,          126:2,           685:3}
// data                  -> [ [A-0, A-1, A-2], [B-0, B-1, B-2], [C-0, C-1, C-2], [D-0, D-1, D-2] ]
// rows indexes          ->    0,   1,   2      0,   1,   2      0,  1,  2        0,   1,   2
// rows(entities)        -> { 76:0, 89:1, 102:2 }
// In that way, in all components, the data of the entity with id 76 is stored in the first row (index 0), 
// the data of the entity with id 89 is stored in the second row (index 1), 
// and the data of the entity with id 102 is stored in the third row (index 2).


//                          index 0           index 1           index 2           index 3
//         +----------+-----------------+-----------------+-----------------+-----------------+
//         | EntityId | Component 86    | Component 123   | Component 126   | Component 685   |
//         +----------+-----------------+-----------------+-----------------+-----------------+
// index 0 | 76       | A-0             | B-0             | C-0             | D-0             |
//         +----------+-----------------+-----------------+-----------------+-----------------+
// index 1 | 89       | A-1             | B-1             | C-1             | D-1             |
//         +----------+-----------------+-----------------+-----------------+-----------------+
// index 2 | 102      | A-2             | B-2             | C-2             | D-2             |
//         +----------+-----------------+-----------------+-----------------+-----------------+
//
pub struct TableId(u64);


pub struct Column {
    data: *mut u8,      // Pointer to a memory location where an vector of some type is stored
    data_length: usize, // The size of a single element in the vector
    size: usize,       // The number of elements in the vector
}

pub struct Table {
    rows: HashMap<EntityId, usize>,
    columns: HashMap<ComponentId, usize>,
    data: Vec<Column>
}

pub struct Database {
    tables: HashMap<TableId, Table>,
    indexes: Indexes
}

impl Database {
    pub fn new() -> Self {
        Database {
            tables: HashMap::new(),
            indexes: Indexes {
                table_by_entity: HashMap::new(),
                table_by_components: HashMap::new(),
            }
        }
    }
}

struct Indexes {
    table_by_entity: HashMap<EntityId, Table>,
    table_by_components: HashMap<Vec<ComponentId>, Table>,
}
