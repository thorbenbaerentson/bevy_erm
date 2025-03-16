use bevy::prelude::*;
use crate::prelude::ErmTypesRegistry;

/// The plugin needs to be added to the app.
pub struct BevyERMPlugin;

/// All the plugin does is adding the ERM-Registry as a resource to the app.
impl Plugin for BevyERMPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ErmTypesRegistry::default());
    }
}
