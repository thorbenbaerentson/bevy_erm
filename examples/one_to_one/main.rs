use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy_erm::prelude::*;

pub fn startup(mut registry: ResMut<ErmTypeRegistry>, mut type_registry: ResMut<AppTypeRegistry>) {
    let sql_name = registry.register_type::<Zombie>(type_registry.as_mut());
    let sql_name_2 = registry.register_type::<Player>(type_registry.as_mut());
    assert!(sql_name.is_some());
    assert!(sql_name_2.is_some());

    let table = registry.get_table_definition(&sql_name.unwrap());
    assert!(table.is_some());

    let table = table.unwrap();
    let rust_name = table.rust_name.clone();
    let sql_name = table.sql_name.clone();

    assert_eq!(sql_name, "Zombies");
    assert_eq!(rust_name, "Zombie");

    if let Some(col) = table.get("target") {
        let Some(r) = col.get_refence() else {
            return;
        };

        let FieldConstraint::Reference(type_name, column_name) = r else {
            return;
        };

        info!("Type: {} Column {}.", type_name, column_name);
        if let SqlType::One2One(t, b) = col.sql_type {
            // If the type is wrapped in an Option, it is implicitly marked for 'lazy' loading.
            assert!(!b);
            let ty = registry.get_type_from_type_id(&type_registry, t);
            let Some(t) = ty else {
                panic!("Player type not found!");
            };

            assert_eq!(t.rust_name, "Player");
            assert_eq!(t.sql_name, "Players");
        }
    }
}

#[derive(Reflect)]
#[reflect(@TableName::new("Players"))]
pub struct Player {
    #[reflect(@Key, @ColumnName::new("id"))]
    pub key: i32,
    #[reflect(@MaxLength::new(64))]
    pub name: String,
    #[reflect(@MaxLength::new(128))]
    pub comment: Option<String>,
}

#[derive(Reflect)]
#[reflect(@TableName::new("Zombies"))]
pub struct Zombie {
    #[reflect(@Key, @ColumnName::new("id"))]
    pub key: i32,
    #[reflect(@MaxLength::new(64))]
    pub name: String,

    pub translation: Vec3,
    pub rotation: Quat,

    #[reflect(@Reference::new("Player", "id"))]
    pub target: Option<Player>,
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
        .register_type::<Player>()
        .register_type::<Zombie>()
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}
