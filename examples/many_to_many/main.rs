use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy_erm::prelude::*;

pub fn startup(mut registry: ResMut<ErmTypeRegistry>, mut type_registry: ResMut<AppTypeRegistry>) {
    let sql_name = registry.register_type::<SpawnPoint>(type_registry.as_mut());
    assert!(sql_name.is_some());
    let table = registry.get_table_definition(&sql_name.unwrap());

    assert!(table.is_some());
    let table = table.unwrap();
    let rust_name = table.rust_name.clone();
    let sql_name = table.sql_name.clone();
    assert_eq!(sql_name, "SpawnPoints");
    assert_eq!(rust_name, "SpawnPoint");

    info!(
        "Table name '{}' column count {}.",
        sql_name,
        table.no_fields()
    );

    for (name, column) in &table.fields {
        info!(
            "\tColumn: {} ({} - {}) Type: {:?}",
            name,
            if column.is_key() { "Primary Key" } else { " " },
            if column.is_not_null() {
                "Not Null"
            } else {
                " "
            },
            column.sql_type
        );
    }
}

#[derive(Reflect)]
#[reflect(@TableName::new("GameModes"))]
pub struct GameMode {
    #[reflect(@Key, @ColumnName::new("id"))]
    pub key: i32,
    #[reflect(@MaxLength::new(64))]
    pub name: String,
    #[reflect(@ColumnName::new("SpawnPoint"))]
    pub spawn_points: Vec<SpawnPoint>,
}

#[derive(Reflect)]
#[reflect(@TableName::new("SpawnPoints"))]
pub struct SpawnPoint {
    #[reflect(@Key, @ColumnName::new("id"))]
    pub key: i32,
    #[reflect(@MaxLength::new(64))]
    pub name: String,

    pub translation: Option<Vec3>,
    pub rotation: Quat,
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
        .add_plugins(BevyERMPlugin)
        // Register your types. This ensures, that we can reflect over these types.
        .register_type::<SpawnPoint>()
        .register_type::<GameMode>()
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}
