use crate::consts::TICK_RATE;
use crate::interpolate::InterpolatePlugin;
use crate::pawns::fly::FlyPawnPlugin;
use crate::pawns::fps::FirstPersonPawnPlugin;
use crate::scenes;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct SharedPlugins;

impl PluginGroup for SharedPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule())
            .add(RapierDebugRenderPlugin::default())
            .add(FirstPersonPawnPlugin)
            .add(FlyPawnPlugin)
            .add(InterpolatePlugin)
            .add(SharedPlugin)
    }
}

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, scenes::example::setup)
            .insert_resource(Time::<Fixed>::from_hz(TICK_RATE as f64))
            .insert_resource(TimestepMode::Fixed {
                dt: 1.0 / TICK_RATE as f32,
                substeps: 1,
            });
    }
}
