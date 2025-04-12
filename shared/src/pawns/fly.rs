use crate::interpolate::{InterpolateRotation, InterpolateTranslation};
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

pub struct FlyPawnPlugin;

impl Plugin for FlyPawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, simulate_system);
    }
}

#[derive(Component, Default)]
pub struct FlyPawnCommand {
    angle: Vec2,
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl FlyPawnCommand {
    pub fn apply(&mut self, command: &crate::Command) {
        self.angle = command.angle;
        self.forward = command.forward;
        self.backward = command.backward;
        self.left = command.left;
        self.right = command.right;
        self.up = command.up;
        self.down = command.down;
    }
    fn direction(&self) -> Vec3 {
        let mut direction = Vec3::ZERO;
        if self.forward {
            direction.z -= 1.0;
        }
        if self.backward {
            direction.z += 1.0;
        }
        if self.right {
            direction.x += 1.0;
        }
        if self.left {
            direction.x -= 1.0;
        }
        if self.up {
            direction.y += 1.0;
        }
        if self.down {
            direction.y -= 1.0;
        }
        direction.normalize_or_zero()
    }
}

#[derive(Component)]
#[require(FlyPawnCommand, Transform, InterpolateTranslation, InterpolateRotation)]
pub struct FlyPawn {
    pub speed: f32,
}

const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;

pub fn simulate_system(mut q: Query<(&FlyPawn, &FlyPawnCommand, &mut Transform)>, time: Res<Time>) {
    for (pawn, command, mut transform) in q.iter_mut() {
        // translation
        let wish_direction = transform.rotation.mul_vec3(command.direction());
        transform.translation += wish_direction * pawn.speed * time.delta_secs();

        // rotation
        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw - command.angle.x;
        let pitch = (pitch - command.angle.y).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}
