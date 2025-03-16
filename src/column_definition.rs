use crate::prelude::{FieldConstraint, SqlType};
use bevy::log::info;
use std::fmt::Display;

#[derive(Debug, Default)]
pub struct ColumnDefinition {
    pub rust_name: String,
    pub sql_name: String,

    pub sql_type: SqlType,

    pub constraints: Vec<FieldConstraint>,
}

impl ColumnDefinition {
    pub fn new(name: &str, sql_name: &str) -> Self {
        ColumnDefinition {
            rust_name: name.to_owned(),
            sql_name: sql_name.to_owned(),
            constraints: Vec::new(),

            sql_type: SqlType::Blob(true),
        }
    }

    /// Add another constrain to the list of constraints.
    /// If an constraint of the same type already exists,
    /// the existing will be replaced with the new one.
    pub fn add(&mut self, constraint: FieldConstraint) {
        // Replace any existing constraint of the same type.
        match constraint {
            FieldConstraint::Key => {
                if self.is_key() {
                    info!("Column {} is already marked as key column!", self.sql_name);
                    return;
                }
            }

            FieldConstraint::MaxLength(_) => {
                if self.has_max_length() {
                    if let Some(x) = self
                        .constraints
                        .iter()
                        .position(|e| matches!(e, FieldConstraint::MaxLength(_)))
                    {
                        self.constraints.remove(x);
                    };
                }
            }

            FieldConstraint::Reference(_, _) => {
                if self.is_reference() {
                    if let Some(x) = self
                        .constraints
                        .iter()
                        .position(|e| matches!(e, FieldConstraint::Reference(_, _)))
                    {
                        self.constraints.remove(x);
                    };
                }
            }

            FieldConstraint::Unique => {
                if self.is_unique() {
                    return;
                }
            }
        }

        self.constraints.push(constraint);
    }

    /// Returns true, if this column has been marked with key.
    pub fn is_key(&self) -> bool {
        self.constraints
            .iter()
            .filter(|e| matches!(e, FieldConstraint::Key))
            .count()
            > 0
    }

    /// Returns true, if this column requires a value.
    pub fn is_not_null(&self) -> bool {
        match self.sql_type {
            SqlType::None => false,
            SqlType::Integer(_, b) => b,
            SqlType::Float(_, b) => b,
            SqlType::Text(b) => b,
            SqlType::Date(b) => b,
            SqlType::Time(b) => b,
            SqlType::DateTime(b) => b,
            SqlType::Blob(b) => b,
            SqlType::Boolean(b) => b,
            SqlType::One2One(_, _) => false,
            SqlType::Many2Many(_, _) => false,
        }
    }

    /// Returns true, if this column has a relation to another table.
    pub fn is_reference(&self) -> bool {
        self.constraints
            .iter()
            .filter(|e| matches!(e, FieldConstraint::Reference(_, _)))
            .count()
            > 0
    }

    /// Returns true, if this column has a max length attribute.
    pub fn has_max_length(&self) -> bool {
        self.constraints
            .iter()
            .filter(|e| matches!(e, FieldConstraint::MaxLength(_)))
            .count()
            > 0
    }

    /// Returns true, if this column has been marked as unique.
    pub fn is_unique(&self) -> bool {
        self.constraints
            .iter()
            .filter(|e| matches!(e, FieldConstraint::Unique))
            .count()
            > 0
    }

    /// Return the value for the max length attribute.
    pub fn get_max_length(&self) -> usize {
        // Check, when debugging but not in release.
        #[cfg(debug_assertions)]
        if !self.has_max_length() {
            panic!("The column {} has no max length attribute!", self.sql_name);
        }

        let tmp: Vec<&FieldConstraint> = self
            .constraints
            .iter()
            .filter(|e| matches!(e, FieldConstraint::MaxLength(_)))
            .collect();

        let FieldConstraint::MaxLength(result) = tmp[0] else {
            panic!("Expected a max length attribute!");
        };

        *result
    }

    /// Get the reference constraint if one exists.qlType::Many2Many(_, _) => false,
    pub fn get_refence(&self) -> Option<FieldConstraint> {
        // Check, when debugging but not in release.
        #[cfg(debug_assertions)]
        if !self.is_reference() {
            panic!("The column {} is not a reference column!", self.sql_name);
        }

        let col: Vec<&FieldConstraint> = self
            .constraints
            .iter()
            .filter(|e| matches!(e, FieldConstraint::Reference(_, _)))
            .collect();

        let result = col.first()?;

        Some((**result).clone())
    }
}

impl Display for ColumnDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let constraints = self
            .constraints
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" - ");

        write!(
            f,
            "{} ({}) - {} {}",
            self.rust_name, self.sql_name, self.sql_type, constraints
        )
    }
}
