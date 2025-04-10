use std::any::TypeId;

use bevy::reflect::Type;
use bevy::{
    prelude::*,
    reflect::{NamedField, TypeInfo},
    utils::HashMap,
};

use crate::prelude::{Key, Unique};
use crate::{
    prelude::SqlType,
    prelude::{
        ColumnDefinition, ColumnName, FieldConstraint, MaxLength, Reference, TableDefinition,
        TableName,
    },
};

#[derive(Resource, Default)]
pub struct ErmTypesRegistry {
    tables: HashMap<String, TableDefinition>,
}

impl ErmTypesRegistry {
    /// Retrieve table definition by rust or sql name.
    /// The sql name will be quicker, since the definitions are held in a
    /// hashmap using the sql_name as key. However, going through all tables
    /// in a database should not be that expensive, since there usually aren't thounsands
    /// in a database.
    pub fn get_table_definition(&self, name: &str) -> Option<&TableDefinition> {
        if let Some(r) = self.tables.get(name) {
            return Some(r);
        }

        let result: Vec<&TableDefinition> = self
            .tables
            .values()
            .filter(|x| x.rust_name == name || x.sql_name == name)
            .collect();

        if result.is_empty() {
            return None;
        }

        Some(result[0])
    }

    pub fn get_type_from_type_id(
        &mut self,
        type_registry: &AppTypeRegistry,
        type_id: TypeId,
    ) -> Option<&TableDefinition> {
        let reg = type_registry.read();

        if let Some(t) = reg.get(type_id) {
            let TypeInfo::Struct(s) = t.type_info() else {
                info!("Relation is not a structure type.");
                return None;
            };

            let Some(name) = s.ty().ident() else {
                info!("Structure has no identifier");
                return None;
            };

            return self.get_table_definition(name);
        }

        None
    }

    /// Map a rust type to a sql type.
    fn rust_to_sql_type(ty: &TypeInfo, app_registry: &AppTypeRegistry) -> SqlType {
        // Integers.
        if *ty.ty() == Type::of::<u8>() {
            return SqlType::UnsingedInteger(8, true);
        }
        if *ty.ty() == Type::of::<u16>() {
            return SqlType::UnsingedInteger(16, true);
        }
        if *ty.ty() == Type::of::<u32>() {
            return SqlType::UnsingedInteger(32, true);
        }
        if *ty.ty() == Type::of::<u64>() {
            return SqlType::UnsingedInteger(64, true);
        }
        if *ty.ty() == Type::of::<u128>() {
            return SqlType::UnsingedInteger(128, true);
        }

        if *ty.ty() == Type::of::<i8>() {
            return SqlType::Integer(8, true);
        }
        if *ty.ty() == Type::of::<i16>() {
            return SqlType::Integer(16, true);
        }
        if *ty.ty() == Type::of::<i32>() {
            return SqlType::Integer(32, true);
        }
        if *ty.ty() == Type::of::<i64>() {
            return SqlType::Integer(64, true);
        }
        if *ty.ty() == Type::of::<i128>() {
            return SqlType::Integer(128, true);
        }

        if *ty.ty() == Type::of::<usize>() {
            return SqlType::Integer(64, true);
        }

        // Float
        if *ty.ty() == Type::of::<f32>() {
            return SqlType::Float(32, true);
        }
        if *ty.ty() == Type::of::<f64>() {
            return SqlType::Float(64, true);
        }

        // Text
        if *ty.ty() == Type::of::<String>() {
            return SqlType::Text(true);
        }
        if *ty.ty() == Type::of::<str>() {
            return SqlType::Text(true);
        }

        // Boolean
        if *ty.ty() == Type::of::<bool>() {
            return SqlType::Boolean(true);
        }

        // Vector
        if *ty.ty() == Type::of::<Vec2>() {
            return SqlType::Blob(true);
        }
        if *ty.ty() == Type::of::<Vec3>() {
            return SqlType::Blob(true);
        }
        if *ty.ty() == Type::of::<Vec4>() {
            return SqlType::Blob(true);
        }

        if *ty.ty() == Type::of::<IVec2>() {
            return SqlType::Blob(true);
        }
        if *ty.ty() == Type::of::<IVec3>() {
            return SqlType::Blob(true);
        }
        if *ty.ty() == Type::of::<IVec4>() {
            return SqlType::Blob(true);
        }

        if *ty.ty() == Type::of::<UVec2>() {
            return SqlType::Blob(true);
        }
        if *ty.ty() == Type::of::<UVec3>() {
            return SqlType::Blob(true);
        }
        if *ty.ty() == Type::of::<UVec4>() {
            return SqlType::Blob(true);
        }

        // Quat
        if *ty.ty() == Type::of::<Quat>() {
            return SqlType::Blob(true);
        }

        // Check for option:
        if let TypeInfo::Enum(e) = ty {
            
            if !e.generics().is_empty() && e.generics().len() == 1  && e.variant("Some").is_some() {
                let tmp = e.generics()[0].clone();
                if let Some(type_info) = app_registry.read().get(tmp.type_id()) {
                    let option_type = Self::rust_to_sql_type(type_info.type_info(), app_registry);

                    match option_type {
                        SqlType::None => panic!("Invalid sql type!"),

                        SqlType::One2One(t, _) => return SqlType::One2One(t, false),
                        SqlType::Many2Many(t, _) => return SqlType::Many2Many(t, false),
                        SqlType::Integer(s, _) => return SqlType::Integer(s, false),
                        SqlType::UnsingedInteger(s, _) => {
                            return SqlType::UnsingedInteger(s, false)
                        }
                        SqlType::Float(s, _) => return SqlType::Float(s, false),
                        SqlType::Text(_) => return SqlType::Text(false),
                        SqlType::Date(_) => return SqlType::Date(false),
                        SqlType::Time(_) => return SqlType::Time(false),
                        SqlType::DateTime(_) => return SqlType::DateTime(false),
                        SqlType::Blob(_) => return SqlType::Blob(false),
                        SqlType::Boolean(_) => return SqlType::Boolean(false),
                    }
                };

                panic!("Could not map relation type (one_2_one)");
            }
        }

        if let TypeInfo::List(v) = ty {
            if !v.generics().is_empty() && v.generics().len() == 1 {
                let tmp = v.generics()[0].clone();
                if let Some(type_info) = app_registry.read().get(tmp.type_id()) {
                    let option_type = Self::rust_to_sql_type(type_info.type_info(), app_registry);

                    match option_type {
                        SqlType::None => panic!("Invalid sql type!"),
                        SqlType::One2One(t, _) => return SqlType::Many2Many(t, true),
                        SqlType::Many2Many(_, _) => {
                            panic!("Dont know how to handle nested relations!")
                        }
                        SqlType::Integer(s, _) => return SqlType::Integer(s, false),
                        SqlType::UnsingedInteger(s, _) => {
                            return SqlType::UnsingedInteger(s, false)
                        }
                        SqlType::Float(s, _) => return SqlType::Float(s, false),
                        SqlType::Text(_) => return SqlType::Text(false),
                        SqlType::Date(_) => return SqlType::Date(false),
                        SqlType::Time(_) => return SqlType::Time(false),
                        SqlType::DateTime(_) => return SqlType::DateTime(false),
                        SqlType::Blob(_) => return SqlType::Blob(false),
                        SqlType::Boolean(_) => return SqlType::Boolean(false),
                    }
                };

                panic!("Could not map relation type (many_to_many)");
            }
        }

        let type_name = ty.ty().ident().unwrap().to_string();
        if let Some(t) = app_registry.read().get_with_short_type_path(&type_name) {
            let type_id = t.type_id();
            return SqlType::One2One(type_id, true);
        };

        SqlType::Blob(true)
    }

    fn field_definition(
        f: &NamedField,
        app_registry: &AppTypeRegistry,
        order : usize,
    ) -> Option<ColumnDefinition> {
        let Some(type_info) = f.type_info() else {
            info!(
                "Field {} has no type info or it is not a StructInfo.",
                f.name()
            );
            return None;
        };

        // Rust name
        let name = f.name().to_owned();
        let mut def = ColumnDefinition {
            rust_name: name.to_owned(),
            sql_name: name.to_owned(),
            ty: *f.ty(),
            order,

            ..ColumnDefinition::default()
        };

        // Constraints
        if f.get_attribute::<Key>().is_some() {
            def.add(FieldConstraint::Key);
        };

        if let Some(s) = f.get_attribute::<MaxLength>() {
            def.add(FieldConstraint::MaxLength(s.length));
        }

        if f.get_attribute::<Unique>().is_some() {
            def.add(FieldConstraint::Unique);
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

        Some(def)
    }

    /// Reflect over the type T and add a new table definition.
    /// Adds the table definition to the ERM-Registry and returns the sql name.
    /// Remember to use the reflect marco and reflect over Default, like so: #[reflect(Default)]
    pub fn register_type<T>(&mut self, app_registry: &AppTypeRegistry) -> Option<String>
    // We expect T to be a struct! Unnamed tuples cannot be mapped to a typical relational datamodel. 
    // All SQL implementation i've encountered so far required a table to explicitly name its fields.
    where
        T: Reflect + Default + TypePath + bevy::prelude::Struct
    {
        let type_id = TypeId::of::<T>();
        let registry = app_registry.read();

        // Get type registration from registry.
        let t = registry.get(type_id)?;

        // We expect structs. Tuple structs lack field names,
        // and we need field names to create SQL-Tables.
        let TypeInfo::Struct(strct) = t.type_info() else {
            return None;
        };

        let Some(ref_default) = t.data::<ReflectDefault>() else {
            panic!(
                "Type {} has no reflect default",
                strct.ty().ident().unwrap()
            );
        };

        // Rust name
        let rust_name = strct.ty().ident()?;

        // Check for TableName attribute.
        let mut r = TableDefinition::new(rust_name, rust_name, strct.ty(), ref_default);
        let mut sql_name = rust_name.to_string();
        if let Some(table_name) = strct.get_attribute::<TableName>() {
            sql_name = table_name.sql_name.clone();
        };

        // Did we already reflect over this table.
        if self.tables.get(&sql_name).is_some() {
            return None;
        }

        r.sql_name = sql_name.clone();
        for i in 0..strct.field_len() {
            let Some(f) = strct.field_at(i) else {
                info!("Could not retrieve field at position {}", i);
                continue;
            };

            let Some(field) = Self::field_definition(f, app_registry, i) else {
                continue;
            };

            r.add(field);
        }

        self.tables.insert(rust_name.to_owned(), r);

        Some(sql_name)
    }
}
