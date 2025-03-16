mod attributes;
mod column_definition;
mod plugin;
mod table_definition;
mod erm_types_registry;
mod constraints;
mod sql_types;

pub mod prelude {
    pub use crate::plugin::BevyERMPlugin;

    pub use crate::erm_types_registry::ErmTypesRegistry;

    pub use crate::table_definition::TableDefinition;
    pub use crate::table_definition::TableName;

    pub use crate::attributes::ColumnName;
    pub use crate::attributes::Key;
    pub use crate::attributes::MaxLength;
    pub use crate::attributes::Reference;
    pub use crate::attributes::Unique;

    pub use crate::column_definition::ColumnDefinition;
    pub use crate::constraints::FieldConstraint;
    pub use crate::sql_types::SqlType;
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::prelude::*;

    #[derive(Reflect)]
    #[reflect(@TableName::new("Players"))]
    struct Player {
        #[reflect(@Key)]
        pub id : i64,
        #[reflect(@MaxLength::new(64))]
        pub name : String,
        pub comments : Option<String>,
    }

    #[derive(Reflect)]
    #[reflect(@ColumnName::new("Zombies"))]
    struct Zombie {
        #[reflect(@Key)]
        pub id : i64,
        #[reflect(@Reference::new("Player", "id"))]
        pub target : Option<Player>,
    }

    fn prepare_app() -> App {
        let mut app = App::new();
        app.insert_resource(AppTypeRegistry::default());
        app.add_plugins(BevyERMPlugin);

        app.register_type::<Player>();
        app.register_type::<Zombie>();

        app
    }

    fn basic_setup(
        mut bevy_types_registry : ResMut<AppTypeRegistry>,
        mut erm_types_registry : ResMut<ErmTypesRegistry>,
    ) {
        let player_name = erm_types_registry.register_type::<Player>(bevy_types_registry.as_mut());

        assert!(player_name.is_some());

        // It should be possible to retrieve tables via sql or rust name.
        assert!(erm_types_registry.get_table_definition("Player").is_some());
        assert!(erm_types_registry.get_table_definition("Players").is_some());

        let table_def = erm_types_registry.get_table_definition("Player").unwrap();

        assert_eq!(table_def.no_fields(), 3);
        assert!(table_def.get("id").is_some());
        assert!(table_def.get("name").is_some());
        assert!(table_def.get("comments").is_some());

        assert!(table_def.get("id").unwrap().is_key());
        assert!(table_def.get("id").unwrap().is_not_null());
        assert!(!table_def.get("id").unwrap().is_reference());
        assert!(!table_def.get("id").unwrap().is_unique());
        assert!(!table_def.get("id").unwrap().has_max_length());
        assert_eq!(table_def.get("id").unwrap().sql_type, SqlType::Integer(64, true));

        assert!(!table_def.get("name").unwrap().is_key());
        assert!(table_def.get("name").unwrap().is_not_null());
        assert!(!table_def.get("name").unwrap().is_reference());
        assert!(!table_def.get("name").unwrap().is_unique());
        assert!(table_def.get("name").unwrap().has_max_length());
        assert_eq!(table_def.get("name").unwrap().sql_type, SqlType::Text(true));

        assert!(!table_def.get("comments").unwrap().is_key());
        assert!(!table_def.get("comments").unwrap().is_not_null());
        assert!(!table_def.get("comments").unwrap().is_reference());
        assert!(!table_def.get("comments").unwrap().is_unique());
        assert!(!table_def.get("comments").unwrap().has_max_length());
        assert_eq!(table_def.get("comments").unwrap().sql_type, SqlType::Text(false));
    }

    #[test]
    fn basic_table_mapping() {
        let mut app = prepare_app();
        app.add_systems(Startup, basic_setup);
        app.update();

    }
}
