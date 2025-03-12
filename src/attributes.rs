use bevy::reflect::Reflect;

/// Marker for key coluimns
#[derive(Reflect, Debug, Default)]
pub struct Key;

/// Marker for not null columns
#[derive(Reflect, Debug, Default)]
pub struct NotNull;

/// Rename a column.
#[derive(Reflect, Debug, Default)]
pub struct ColumnName {
    pub sql_name: String,
}

impl ColumnName {
    pub fn new(name: &str) -> Self {
        ColumnName {
            sql_name: name.to_owned(),
        }
    }
}

/// Create a reference to another table
#[derive(Reflect, Debug, Default)]
pub struct Reference {
    pub rust_name: String, // The name of the rust type
    pub sql_name: String,  // The table to reference
    pub key_field: String, // The sql name of the field to use as relation
}

impl Reference {
    pub fn new(rust_name: &str, sql_name: &str, key_field: &str) -> Self {
        Reference {
            rust_name: rust_name.to_owned(),
            sql_name: sql_name.to_owned(),
            key_field: key_field.to_owned(),
        }
    }
}

/// Add max length to a string field.
#[derive(Reflect, Debug, Default)]
pub struct MaxLength {
    pub length: usize,
}

impl MaxLength {
    pub fn new(s: usize) -> Self {
        MaxLength { length: s }
    }
}
