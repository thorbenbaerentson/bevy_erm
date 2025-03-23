use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy_erm::prelude::*;

pub fn startup(mut registry: ResMut<ErmTypesRegistry>, mut type_registry: ResMut<AppTypeRegistry>) {
    let reg = type_registry.as_mut();
    let spawn_point_name = registry.register_type::<SpawnPoint>(reg);
    let game_mode_name = registry.register_type::<GameMode>(reg);
    assert!(spawn_point_name.is_some());
    assert!(game_mode_name.is_some());

    let spawn_point_table_option = registry.get_table_definition(&spawn_point_name.unwrap());
    let game_mode_table_option = registry.get_table_definition(&game_mode_name.unwrap());
    assert!(spawn_point_table_option.is_some());
    assert!(game_mode_table_option.is_some());

    // Spawn point checks.
    let spawn_point_table = spawn_point_table_option.unwrap();
    let mut rust_name = spawn_point_table.rust_name.clone();
    let mut sql_name = spawn_point_table.sql_name.clone();
    assert_eq!(sql_name, "SpawnPoints");
    assert_eq!(rust_name, "SpawnPoint");

    let Some(translation_column) = spawn_point_table.get("translation") else {
        panic!("Expected a column 'translation' to be part of the table.")
    };
    assert!(!translation_column.is_not_null());

    info!("{}", spawn_point_table);

    // Game mode
    let game_mode_table = game_mode_table_option.unwrap();
    rust_name = game_mode_table.rust_name.clone();
    sql_name = game_mode_table.sql_name.clone();
    assert_eq!(sql_name, "GameModes");
    assert_eq!(rust_name, "GameMode");

    let Some(spawn_points_column) = game_mode_table.get("SpawnPoints") else {
        panic!("Expected a column 'SpawnPoints' to be part of the table.")
    };
    assert!(spawn_points_column.is_reference());
    let SqlType::Many2Many(_, _) = spawn_points_column.sql_type else {
        panic!("Expected spawn points to be a many to many relation");
    };

    info!("{}", game_mode_table);
}

#[derive(Reflect, Default)]
#[reflect(Default, @TableName::new("GameModes"))]
pub struct GameMode {
    #[reflect(@Key, @ColumnName::new("id"))]
    pub key: i32,
    #[reflect(@MaxLength::new(64))]
    pub name: String,
    #[reflect(@ColumnName::new("SpawnPoints"), @Reference::new("SpawnPoints", "id"))]
    pub spawn_points: Vec<SpawnPoint>,
}

#[derive(Reflect, Default)]
#[reflect(Default, @TableName::new("SpawnPoints"))]
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
    // This is a bevy issue...
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
