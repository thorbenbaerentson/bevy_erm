use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy_erm::prelude::*;

// Retrieve all meta data about table using ErmTypeRegistry and AppTypeRegisrty.
// Both registries must be mutable.
pub fn startup(mut registry: ResMut<ErmTypesRegistry>, mut type_registry: ResMut<AppTypeRegistry>) {
    // Retrieve the sql name from the registry using AppTypeRegistry and the type in question.
    let sql_name = registry.register_type::<Player>(type_registry.as_mut());
    assert!(sql_name.is_some());

    // Then retrieve the table definition using the name the registry provided.
    let table = registry.get_table_definition(&sql_name.unwrap());
    assert!(table.is_some());

    let table = table.unwrap();
    let rust_name = table.rust_name.clone();
    let sql_name = table.sql_name.clone();
    assert_eq!(sql_name, "Players");
    assert_eq!(rust_name, "Player");

    assert!(table.get("id").is_some());
    assert!(table.get("name").is_some());
    assert!(table.get("comment").is_some());

    if let Some(col) = table.get("id") {
        assert!(col.is_key());
        assert!(col.is_not_null());
        if let SqlType::Integer(s, b) = col.sql_type {
            assert_eq!(s, 32);
            assert!(b);
        };
    }

    if let Some(col) = table.get("name") {
        if let SqlType::Text(b) = col.sql_type {
            assert!(b);
        };
        assert!(col.is_not_null());
        assert!(col.has_max_length());
        assert_eq!(col.get_max_length(), 64);
        assert!(col.is_unique());
    }

    if let Some(col) = table.get("comment") {
        if let SqlType::Text(b) = col.sql_type {
            assert!(!b);
        };
        assert!(col.has_max_length());
        assert_eq!(col.get_max_length(), 128);
    }
}

#[derive(Reflect)]
#[reflect(@TableName::new("Players"))]
pub struct Player {
    #[reflect(@Key, @ColumnName::new("id"))]
    pub key: i32,
    #[reflect(@MaxLength::new(64), @Unique)]
    pub name: String,
    #[reflect(@MaxLength::new(128))]
    pub comment: Option<String>,
}

pub fn update(mut exit: EventWriter<AppExit>, frame_count: Res<FrameCount>) {
    // Delay app exit to avoid segmentation faults.
    if frame_count.0 >= 20 {
        exit.send(AppExit::Success);
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add your Plugin
        .add_plugins(BevyERMPlugin)
        // Register your types. This ensures, that we can reflect over these types
        // regardless whether a type has been used as a component or not.
        .register_type::<Player>()
        // A system make some assertions
        .add_systems(Startup, startup)
        // Update system quits bevy. Calling AppExit::Success before the first frame causes a crash.
        .add_systems(Update, update)
        .run();
}
