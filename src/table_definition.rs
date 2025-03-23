use bevy::{
    log::info,
    reflect::{prelude::ReflectDefault, Reflect, Type},
};
use std::{collections::HashMap, fmt::Display};

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

    pub ty: Type,
    pub reflect_default: ReflectDefault,
}

impl TableDefinition {
    /// Create a new table definition with the given rust and sql name.
    pub fn new(
        rst_name: &str,
        sql_name: &str,
        ty: &Type,
        ref_default: &ReflectDefault,
    ) -> TableDefinition {
        TableDefinition {
            rust_name: rst_name.to_owned(),
            sql_name: sql_name.to_owned(),

            fields: HashMap::new(),

            ty: *ty,
            reflect_default: ref_default.to_owned(),
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
        if let Some(table) = self.fields.get(column) {
            return Some(table);
        }

        if let Some(r) = self
            .fields
            .iter()
            .map(|y| y.1)
            .filter(|x| x.rust_name == column || x.sql_name == column)
            .collect::<Vec<&ColumnDefinition>>()
            .first()
        {
            return Some(*r);
        }

        None
    }
}

impl Display for TableDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let columns = self
            .fields
            .iter()
            .map(|x| format!("\t{}", x.1))
            .collect::<Vec<String>>()
            .join("\n");

        write!(
            f,
            "Table: {} ({})\n{}",
            self.rust_name, self.sql_name, columns
        )
    }
}
