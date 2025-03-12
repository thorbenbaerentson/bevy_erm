use std::any::{Any, TypeId};

use bevy::reflect::Type;
use bevy::{
    prelude::*,
    reflect::{NamedField, TypeInfo},
    utils::HashMap,
};

use crate::prelude::Key;
use crate::{
    column_definition::SqlType,
    prelude::{
        ColumnDefinition, ColumnName, FieldConstraint, MaxLength, NotNull, Reference,
        TableDefinition, TableName,
    },
};

#[derive(Resource, Default)]
pub struct ErmTypeRegistry {
    tables: HashMap<String, TableDefinition>,
}

impl ErmTypeRegistry {
    /// Retrieve
    pub fn get_table_definition(&self, name: &str) -> Option<&TableDefinition> {
        self.tables.get(name)
    }

    /// Map a rust type to a sql type.
    fn rust_to_sql_type(ty: &TypeInfo, app_registry: &AppTypeRegistry) -> SqlType {
        // Integers.
        if *ty.ty() == Type::of::<u8>() {
            return SqlType::Integer(8);
        }
        if *ty.ty() == Type::of::<u16>() {
            return SqlType::Integer(16);
        }
        if *ty.ty() == Type::of::<u32>() {
            return SqlType::Integer(32);
        }
        if *ty.ty() == Type::of::<u64>() {
            return SqlType::Integer(64);
        }
        if *ty.ty() == Type::of::<u128>() {
            return SqlType::Integer(128);
        }

        if *ty.ty() == Type::of::<i8>() {
            return SqlType::Integer(8);
        }
        if *ty.ty() == Type::of::<i16>() {
            return SqlType::Integer(16);
        }
        if *ty.ty() == Type::of::<i32>() {
            return SqlType::Integer(32);
        }
        if *ty.ty() == Type::of::<i64>() {
            return SqlType::Integer(64);
        }
        if *ty.ty() == Type::of::<i128>() {
            return SqlType::Integer(128);
        }

        if *ty.ty() == Type::of::<usize>() {
            return SqlType::Integer(64);
        }

        // Float
        if *ty.ty() == Type::of::<f32>() {
            return SqlType::Float(32);
        }
        if *ty.ty() == Type::of::<f64>() {
            return SqlType::Float(64);
        }

        // Text
        if *ty.ty() == Type::of::<String>() {
            return SqlType::Text;
        }
        if *ty.ty() == Type::of::<str>() {
            return SqlType::Text;
        }

        // Boolean
        if *ty.ty() == Type::of::<bool>() {
            return SqlType::Boolean;
        }

        // Vector
        if *ty.ty() == Type::of::<Vec2>() {
            return SqlType::Blob;
        }
        if *ty.ty() == Type::of::<Vec3>() {
            return SqlType::Blob;
        }
        if *ty.ty() == Type::of::<Vec4>() {
            return SqlType::Blob;
        }

        if *ty.ty() == Type::of::<IVec2>() {
            return SqlType::Blob;
        }
        if *ty.ty() == Type::of::<IVec3>() {
            return SqlType::Blob;
        }
        if *ty.ty() == Type::of::<IVec4>() {
            return SqlType::Blob;
        }

        if *ty.ty() == Type::of::<UVec2>() {
            return SqlType::Blob;
        }
        if *ty.ty() == Type::of::<UVec3>() {
            return SqlType::Blob;
        }
        if *ty.ty() == Type::of::<UVec4>() {
            return SqlType::Blob;
        }

        // Quat
        if *ty.ty() == Type::of::<Quat>() {
            return SqlType::Blob;
        }

        // Check for option:
        if let TypeInfo::Enum(e) = ty {
            if !e.generics().is_empty() && e.generics().len() == 1 {
                let tmp = e.generics()[0].clone();
                let option_type = Self::rust_to_sql_type(ty, app_registry);
                info!("Option type: {}", tmp.ty().ident().unwrap().to_string());

                return SqlType::One2One(option_type.type_id());
            }
        }

        if let TypeInfo::List(v) = ty {
            if !v.generics().is_empty() && v.generics().len() == 1 {
                let tmp = v.generics()[0].clone();
                let list_type = Self::rust_to_sql_type(ty, app_registry);
                info!("Vector type: {}", tmp.ty().ident().unwrap().to_string());

                return SqlType::Many2Many(list_type.type_id(), false);
            }
        }

        let type_name = ty.ty().ident().unwrap().to_string();
        info!("Type name: {}", type_name);
        if let Some(t) = app_registry.read().get_with_short_type_path(&type_name) {
            let type_id = t.type_id();
            return SqlType::One2One(type_id);
        };

        SqlType::Blob
    }

    fn field_definition(
        f: &NamedField,
        app_registry: &AppTypeRegistry,
    ) -> Option<ColumnDefinition> {
        let Some(type_info) = f.type_info() else {
            info!(
                "Field {} has no type info or it is not a StructInfo.",
                f.name()
            );
            return None;
        };

        let mut def = ColumnDefinition::default();

        // Rust name
        let name = f.name().to_owned();

        def.rust_name = name.to_owned();
        def.sql_name = name.to_owned();

        // Constraints
        if let Some(_) = f.get_attribute::<Key>() {
            def.add(FieldConstraint::Key);
        };

        if let Some(_) = f.get_attribute::<NotNull>() {
            def.add(FieldConstraint::NotNull);
        }

        if let Some(s) = f.get_attribute::<MaxLength>() {
            def.add(FieldConstraint::MaxLength(s.length));
        }

        if let Some(rf) = f.get_attribute::<Reference>() {
            def.add(FieldConstraint::Reference(
                rf.rust_name.clone(),
                rf.key_field.clone(),
            ));
        }

        // Rename column
        if let Some(attrib) = f.get_attribute::<ColumnName>() {
            def.sql_name = attrib.sql_name.clone();
        }

        def.sql_type = Self::rust_to_sql_type(type_info, app_registry);
        if let SqlType::One2One(_) = def.sql_type {
            info!("Found option")
        } else {
            def.add(FieldConstraint::NotNull);
        }

        Some(def)
    }

    /// Reflect over the type T and add a new table definition.
    /// Adds the table definition to the ERM-Registry and returns the sql name.
    pub fn table_definition<T: Reflect>(
        &mut self,
        app_registry: &AppTypeRegistry,
    ) -> Option<String> {
        let type_id = TypeId::of::<T>();
        let registry = app_registry.read();

        // Get type registration from registry.
        let Some(t) = registry.get(type_id) else {
            return None;
        };

        // We expect structs. Tuple structs lack field names,
        // and we need field names to create SQL-Tables.
        let TypeInfo::Struct(strct) = t.type_info() else {
            return None;
        };

        // Rust name
        let Some(name) = strct.ty().ident() else {
            return None;
        };

        // Check for TableName attribute.
        let mut r = TableDefinition::new(name, name);
        let mut sql_name = name.to_string();
        if let Some(table_name) = strct.get_attribute::<TableName>() {
            sql_name = table_name.sql_name.clone();
        };

        // Did we already reflect over this table.
        if let Some(_) = self.tables.get(&sql_name) {
            return None;
        }

        r.sql_name = sql_name.clone();
        for i in 0..strct.field_len() {
            let Some(f) = strct.field_at(i) else {
                info!("Could not retrieve field at position {}", i);
                continue;
            };

            let Some(field) = Self::field_definition(f, app_registry) else {
                continue;
            };

            r.add(field);
        }

        self.tables.insert(sql_name.clone(), r);

        Some(sql_name)
    }
}

/// The plugin needs to be added to the app.
pub struct BevyERMPlugin;

/// All the plugin does is adding the ERM-Registry as a resource to the app.
impl Plugin for BevyERMPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ErmTypeRegistry::default());
    }
}
