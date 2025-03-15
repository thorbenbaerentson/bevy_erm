use bevy::{log::info, reflect::Reflect};
use std::collections::HashMap;

use crate::prelude::ColumnDefinition;

#[derive(Reflect, Debug, Default)]
pub struct TableName {
    pub sql_name: String,
}

impl TableName {
    pub fn new(name: &str) -> Self {
        TableName {
            sql_name: name.to_owned(),
        }
    }
}

pub struct TableDefinition {
    pub rust_name: String,
    pub sql_name: String,

    pub fields: HashMap<String, ColumnDefinition>,
}

impl TableDefinition {
    /// Create a new table definition with the given rust and sql name.
    pub fn new(rst_name: &str, sql_name: &str) -> TableDefinition {
        TableDefinition {
            rust_name: rst_name.to_owned(),
            sql_name: sql_name.to_owned(),

            fields: HashMap::new(),
        }
    }

    /// Add a new column to this table
    pub fn add(&mut self, column: ColumnDefinition) {
        if self.fields.contains_key(&column.sql_name.to_string()) {
            info!(
                "A field with the name {} already exists for table {}",
                column.sql_name, self.sql_name
            );
            return;
        }

        self.fields.insert(column.sql_name.clone(), column);
    }

    /// Return the number of fields.
    pub fn no_fields(&self) -> usize {
        self.fields.len()
    }

    /// Return the column defintion with the given name.
    pub fn get(&self, column: &str) -> Option<&ColumnDefinition> {
        self.fields.get(column)
    }
}
