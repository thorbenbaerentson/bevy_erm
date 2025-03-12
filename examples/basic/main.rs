use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy_erm::prelude::*;

pub fn startup(
    mut registry: ResMut<ErmTypeRegistry>,
    mut type_registry: ResMut<AppTypeRegistry>,
    mut exit: EventWriter<AppExit>,
) {
    let sql_name = registry.table_definition::<Player>(type_registry.as_mut());
    assert!(sql_name.is_some());
    let table = registry.get_table_definition(&sql_name.unwrap());

    assert!(table.is_some());
    let table = table.unwrap();
    let rust_name = table.rust_name.clone();
    let sql_name = table.sql_name.clone();
    assert_eq!(sql_name, "Players");
    assert_eq!(rust_name, "Player");

    info!(
        "Table name '{}' column count {}.",
        sql_name,
        table.fields.len()
    );

    for (name, column) in &table.fields {
        info!(
            "\tColumn: {} ({}{}) Type: {:?}",
            name,
            if column.is_key() { "Primary Key" } else { "" },
            if column.is_not_null() { "Not Null" } else { "" },
            column.sql_type
        );
    }

    exit.send(AppExit::Success);
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

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyERMPlugin)
        // Register your types. This ensures, that we can reflect over these types.
        .register_type::<Player>()
        .add_systems(Startup, startup)
        .run();
}
