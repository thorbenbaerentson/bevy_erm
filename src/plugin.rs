use crate::prelude::ErmTypesRegistry;
use bevy::prelude::*;

pub struct BevyERMPlugin;

/// All this plugin does is adding the ERM-Registry as a resource to the app.
impl Plugin for BevyERMPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ErmTypesRegistry::default());
    }
}
