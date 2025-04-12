use crate::interpolate::InterpolateTranslation;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::FRAC_PI_2;

pub struct FirstPersonPawnPlugin;

impl Plugin for FirstPersonPawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (simulate_system, write_transform_system).chain(),
        );
    }
}

#[derive(Component, Default)]
pub struct FirstPersonPawnCommand {
    pub angle: Vec2,
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
}

impl FirstPersonPawnCommand {
    pub fn apply(&mut self, command: &crate::Command) {
        self.angle = command.angle;
        self.forward = command.forward;
        self.backward = command.backward;
        self.left = command.left;
        self.right = command.right;
        self.jump = command.jump;
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
        direction.normalize_or_zero()
    }
}

#[derive(Debug, Component)]
#[require(
    FirstPersonPawnCommand,
    KinematicCharacterController(default_controller),
    Transform(default_transform),
    Velocity,
    Collider(default_collider),
    InterpolateTranslation
)]
pub struct FirstPersonPawn {
    pub grounded: bool,
    pub acceleration: f32,
    pub jump_force: f32,
    pub damping: f32,
}

impl Default for FirstPersonPawn {
    fn default() -> Self {
        Self {
            grounded: false,
            damping: 20.0,
            jump_force: 5.0,
            acceleration: 20.0,
        }
    }
}

fn default_transform() -> Transform {
    Transform::from_xyz(0.0, 1.0, 0.0)
}

fn default_collider() -> Collider {
    Collider::capsule_y(0.5, 0.5)
}

fn default_controller() -> KinematicCharacterController {
    KinematicCharacterController {
        custom_mass: Some(60.0),
        ..Default::default()
    }
}

const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;

fn simulate_system(
    mut q_pawn: Query<(
        &FirstPersonPawn,
        &FirstPersonPawnCommand,
        &mut Velocity,
        &mut KinematicCharacterController,
        &Children,
    )>,
    mut q_camera: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let delta_seconds = time.delta_secs();
    for (pawn, command, mut velocity, mut controller, children) in q_pawn.iter_mut() {
        for &child in children.iter() {
            let Some(mut camera) = q_camera.get_mut(child).ok() else {
                continue;
            };

            // rotation
            let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);
            let yaw = yaw - command.angle.x;
            let pitch = (pitch - command.angle.y).clamp(-PITCH_LIMIT, PITCH_LIMIT);
            camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

            // gravity
            if !pawn.grounded {
                velocity.linvel.y -= 9.81 * delta_seconds;
            } else {
                velocity.linvel.y = velocity.linvel.y.max(0.0);
            }

            // jumping
            if pawn.grounded && command.jump {
                velocity.linvel.y = pawn.jump_force;
            }

            // friction
            velocity.linvel.x *= 1.0 - pawn.damping * delta_seconds;
            velocity.linvel.z *= 1.0 - pawn.damping * delta_seconds;

            // movement
            let wish_direction =
                Quat::from_euler(EulerRot::YXZ, yaw, 0.0, 0.0).mul_vec3(command.direction());
            velocity.linvel += wish_direction * pawn.acceleration * delta_seconds;

            // physics command
            controller.translation = Some(velocity.linvel * delta_seconds);
        }
    }
}

fn write_transform_system(
    mut q: Query<(
        &mut FirstPersonPawn,
        &mut Transform,
        &KinematicCharacterControllerOutput,
    )>,
) {
    for (mut pawn, mut transform, output) in q.iter_mut() {
        pawn.grounded = output.grounded;
        transform.translation += output.effective_translation;
    }
}
