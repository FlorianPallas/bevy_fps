use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct SharedPlugins;

impl PluginGroup for SharedPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RapierPhysicsPlugin::<NoUserData>::default())
            .add(RapierDebugRenderPlugin::default())
    }
}
