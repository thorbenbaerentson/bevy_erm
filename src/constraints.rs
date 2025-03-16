use bevy::reflect::Reflect;
use std::fmt::Display;

#[derive(Reflect, Debug, Clone, PartialEq, PartialOrd)]
pub enum FieldConstraint {
    Key,
    MaxLength(usize),
    Unique,
    Reference(String, String), // Names the table and the column to use as relation
}

impl Display for FieldConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldConstraint::Key => write!(f, "key"),
            FieldConstraint::MaxLength(max) => write!(f, "length max: {}", max),
            FieldConstraint::Unique => write!(f, "unique"),
            FieldConstraint::Reference(t, c) => write!(f, "reference ({} - {})", t, c),
        }
    }
}
