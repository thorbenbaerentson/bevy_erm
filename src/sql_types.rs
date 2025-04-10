use bevy::reflect::Reflect;
use std::{any::TypeId, fmt::Display};

// The bool used for atomic values indicates, whether a value is not null.
#[derive(Reflect, Debug, Default, Clone, PartialEq, PartialOrd)]
pub enum SqlType {
    // Dummy to satisfy the default trait.
    #[default]
    None,

    /// The value provides the number of bits.
    Integer(usize, bool),
    UnsingedInteger(usize, bool),

    /// Value can be 32 or 64.
    Float(usize, bool),

    Text(bool),

    Date(bool),
    Time(bool),
    DateTime(bool),

    Blob(bool),
    Boolean(bool),

    One2One(TypeId, bool),      // The bool marks, whether this relation is marked for eager or lazy loading.
    Many2Many(TypeId, bool),    // The bool marks, whether this relation is marked for eager or lazy loading.
}

impl Display for SqlType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlType::None => write!(f, "None"),
            SqlType::Integer(bits, not_null) => write!(
                f,
                "i-{} ({})",
                bits,
                if *not_null { "not null" } else { "nullable" }
            ),
            SqlType::UnsingedInteger(bits, not_null) => write!(
                f,
                "u-{} ({})",
                bits,
                if *not_null { "not null" } else { "nullable" }
            ),
            SqlType::Float(bits, not_null) => write!(
                f,
                "f-{} ({})",
                bits,
                if *not_null { "not null" } else { "nullable" }
            ),
            SqlType::Text(not_null) => write!(
                f,
                "Text ({})",
                if *not_null { "not null" } else { "nullable" }
            ),
            SqlType::Date(not_null) => write!(
                f,
                "Date ({})",
                if *not_null { "not null" } else { "nullable" }
            ),
            SqlType::Time(not_null) => write!(
                f,
                "Time ({})",
                if *not_null { "not null" } else { "nullable" }
            ),
            SqlType::DateTime(not_null) => write!(
                f,
                "DateTime ({})",
                if *not_null { "not null" } else { "nullable" }
            ),
            SqlType::Blob(not_null) => write!(
                f,
                "Blob ({})",
                if *not_null { "not null" } else { "nullable" }
            ),
            SqlType::Boolean(not_null) => write!(
                f,
                "Boolean ({})",
                if *not_null { "not null" } else { "nullable" }
            ),
            SqlType::One2One(_, eager) => write!(
                f,
                "One2One (Eager: {})",
                if *eager {
                    "eager loading"
                } else {
                    "lazy loading"
                }
            ),
            SqlType::Many2Many(_, eager) => write!(
                f,
                "Many2Many (Eager: {})",
                if *eager {
                    "eager loading"
                } else {
                    "lazy loading"
                }
            ),
        }
    }
}
