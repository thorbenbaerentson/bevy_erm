mod attributes;
mod column_definition;
mod plugin;
mod table_definition;

pub mod prelude {
    pub use crate::plugin::BevyERMPlugin;

    pub use crate::table_definition::TableDefinition;
    pub use crate::table_definition::TableName;

    pub use crate::plugin::ErmTypeRegistry;

    pub use crate::attributes::ColumnName;
    pub use crate::attributes::Key;
    pub use crate::attributes::MaxLength;
    pub use crate::attributes::NotNull;
    pub use crate::attributes::Reference;
    pub use crate::column_definition::ColumnDefinition;

    pub use crate::column_definition::FieldConstraint;
}
