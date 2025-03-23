mod attributes;
mod column_definition;
mod constraints;
mod erm_types_registry;
mod from_blob;
mod plugin;
mod sql_types;
mod table_definition;

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

    pub use crate::from_blob::*;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use bevy::prelude::*;

    #[derive(Reflect, Default)]
    #[reflect(Default, @TableName::new("Players"))]
    struct Player {
        #[reflect(@Key)]
        pub id: i64,
        #[reflect(@MaxLength::new(64))]
        pub name: String,
        pub comments: Option<String>,
    }

    #[derive(Reflect, Default)]
    #[reflect(Default, @TableName::new("Zombies"))]
    struct Zombie {
        #[reflect(@Key)]
        pub id: i64,
        #[reflect(@Reference::new("Player", "id"))]
        pub target: Option<Player>,
    }

    #[derive(Reflect, Default)]
    #[reflect(Default, @TableName::new("GameModes"))]
    struct GameMode {
        #[reflect(@Key)]
        pub id: i64,
        #[reflect(@Reference::new("SpawnPoints", "id"))]
        pub spawn_points: Vec<SpawnPoint>,
    }

    #[derive(Reflect, Default)]
    #[reflect(Default, @TableName::new("SpawnPoints"))]
    struct SpawnPoint {
        #[reflect(@Key)]
        pub id: i64,
        pub location: Vec3,
    }

    fn prepare_app() -> App {
        let mut app = App::new();
        app.insert_resource(AppTypeRegistry::default());
        app.add_plugins(BevyERMPlugin);

        app.register_type::<Player>();
        app.register_type::<Zombie>();
        app.register_type::<GameMode>();
        app.register_type::<SpawnPoint>();

        app
    }

    fn startup(
        mut bevy_types_registry: ResMut<AppTypeRegistry>,
        mut erm_types_registry: ResMut<ErmTypesRegistry>,
    ) {
        let player_name = erm_types_registry.register_type::<Player>(bevy_types_registry.as_mut());
        let zombie_name = erm_types_registry.register_type::<Zombie>(bevy_types_registry.as_mut());
        let game_mode_name =
            erm_types_registry.register_type::<GameMode>(bevy_types_registry.as_mut());
        let spawn_point_name =
            erm_types_registry.register_type::<SpawnPoint>(bevy_types_registry.as_mut());

        assert!(player_name.is_some());
        assert!(zombie_name.is_some());
        assert!(game_mode_name.is_some());
        assert!(spawn_point_name.is_some());
    }

    fn basic(
        // mut bevy_types_registry : ResMut<AppTypeRegistry>,
        erm_types_registry: ResMut<ErmTypesRegistry>,
    ) {
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
        assert_eq!(
            table_def.get("id").unwrap().sql_type,
            SqlType::Integer(64, true)
        );

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
        assert_eq!(
            table_def.get("comments").unwrap().sql_type,
            SqlType::Text(false)
        );
    }

    #[test]
    fn basic_table_mapping() {
        let mut app = prepare_app();
        app.add_systems(Startup, startup);
        app.add_systems(PostStartup, basic);
        app.update();
    }

    fn basic_with_relation(
        bevy_types_registry: ResMut<AppTypeRegistry>,
        erm_types_registry: ResMut<ErmTypesRegistry>,
    ) {
        // It should be possible to retrieve tables via sql or rust name.
        assert!(erm_types_registry.get_table_definition("Zombie").is_some());
        assert!(erm_types_registry.get_table_definition("Zombies").is_some());

        let table_def = erm_types_registry.get_table_definition("Zombie").unwrap();

        assert_eq!(table_def.no_fields(), 2);
        assert!(table_def.get("id").is_some());
        assert!(table_def.get("target").is_some());

        assert!(table_def.get("id").unwrap().is_key());
        assert!(table_def.get("id").unwrap().is_not_null());
        assert!(!table_def.get("id").unwrap().is_reference());
        assert!(!table_def.get("id").unwrap().is_unique());
        assert!(!table_def.get("id").unwrap().has_max_length());
        assert_eq!(
            table_def.get("id").unwrap().sql_type,
            SqlType::Integer(64, true)
        );

        assert!(!table_def.get("target").unwrap().is_key());
        assert!(!table_def.get("target").unwrap().is_not_null());
        assert!(table_def.get("target").unwrap().is_reference());
        assert!(!table_def.get("target").unwrap().is_unique());
        assert!(!table_def.get("target").unwrap().has_max_length());

        let target_column = table_def.get("target").unwrap();
        assert!(target_column.is_reference());
        assert_eq!(
            target_column.get_refence().unwrap(),
            FieldConstraint::Reference("Player".to_owned(), "id".to_owned())
        );

        let binding = bevy_types_registry.read();
        let type_id = binding.get_with_short_type_path("Player");
        assert!(type_id.is_some());

        // If the relation is behind an option it is implicitly marked for 'lazy loading'.
        // Hence the bool in the type must be false.
        let player_registration = type_id.unwrap();
        assert_eq!(
            table_def.get("target").unwrap().sql_type,
            SqlType::One2One(player_registration.type_id(), false)
        );
    }

    #[test]
    fn basic_table_mapping_with_relation() {
        let mut app = prepare_app();
        app.add_systems(Startup, startup);
        app.add_systems(PostStartup, basic_with_relation);
        app.update();
    }

    fn many_to_many_relation(
        bevy_types_registry: ResMut<AppTypeRegistry>,
        erm_types_registry: ResMut<ErmTypesRegistry>,
    ) {
        // It should be possible to retrieve tables via sql or rust name.
        assert!(erm_types_registry
            .get_table_definition("GameMode")
            .is_some());
        assert!(erm_types_registry
            .get_table_definition("GameModes")
            .is_some());

        let table_def = erm_types_registry.get_table_definition("GameMode").unwrap();

        assert_eq!(table_def.no_fields(), 2);
        assert!(table_def.get("id").is_some());
        assert!(table_def.get("spawn_points").is_some());

        assert!(table_def.get("id").unwrap().is_key());
        assert!(table_def.get("id").unwrap().is_not_null());
        assert!(!table_def.get("id").unwrap().is_reference());
        assert!(!table_def.get("id").unwrap().is_unique());
        assert!(!table_def.get("id").unwrap().has_max_length());
        assert_eq!(
            table_def.get("id").unwrap().sql_type,
            SqlType::Integer(64, true)
        );

        assert!(!table_def.get("spawn_points").unwrap().is_key());
        assert!(!table_def.get("spawn_points").unwrap().is_not_null());
        assert!(table_def.get("spawn_points").unwrap().is_reference());
        assert!(!table_def.get("spawn_points").unwrap().is_unique());
        assert!(!table_def.get("spawn_points").unwrap().has_max_length());

        let target_column = table_def.get("spawn_points").unwrap();
        assert!(target_column.is_reference());
        assert_eq!(
            target_column.get_refence().unwrap(),
            FieldConstraint::Reference("SpawnPoints".to_owned(), "id".to_owned())
        );

        let binding = bevy_types_registry.read();
        let type_id = binding.get_with_short_type_path("SpawnPoint");
        assert!(type_id.is_some());

        // The relation is not behind an Option, this means the relation is implicitly marked for
        // eager loading. Hence the bool in the type must be true.
        let spawn_point_registration = type_id.unwrap();
        assert!(table_def.get("spawn_points").is_some());
        assert_eq!(
            table_def.get("spawn_points").unwrap().sql_type,
            SqlType::Many2Many(spawn_point_registration.type_id(), true)
        );
    }

    #[test]
    fn many_to_many() {
        let mut app = prepare_app();
        app.add_systems(Startup, startup);
        app.add_systems(PostStartup, many_to_many_relation);
        app.update();
    }
}
