use crate::command::CommandPlugin;
use crate::net::NetPlugin;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow, WindowMode};
use shared::interpolate::InterpolateRotation;
use shared::pawns::fly::{FlyPawn, FlyPawnCommand};
use shared::pawns::fps::{FirstPersonPawn, FirstPersonPawnCommand};
use shared::plugins::SharedPlugins;

mod command;
mod net;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                focused: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(SharedPlugins)
        .add_plugins(CommandPlugin)
        .add_plugins(NetPlugin)
        .add_systems(
            Startup,
            (shared::scenes::example::setup, startup, cursor_grab),
        )
        .add_systems(Update, (handle_mouse_motion, handle_button_inputs))
        .add_systems(
            FixedPreUpdate,
            (control_fly_pawn, control_first_person_pawn),
        )
        .add_systems(FixedPostUpdate, clear_command)
        .init_resource::<PlayerCommand>()
        .run();
}

#[derive(Debug, Resource, Default, Deref, DerefMut)]
struct PlayerCommand(shared::Command);

fn startup(mut commands: Commands) {
    // commands.spawn((
    //     FlyPawn { speed: 10.0 },
    //     // Camera3d::default(),
    //     // Camera {
    //     //     is_active: true,
    //     //     ..Default::default()
    //     // },
    //     // PerspectiveProjection {
    //     //     fov: 90.0,
    //     //     ..Default::default()
    //     // },
    //     Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    // ));

    commands
        .spawn(FirstPersonPawn::default())
        .with_children(|spawner| {
            spawner.spawn((
                Camera3d::default(),
                Camera {
                    is_active: true,
                    ..Default::default()
                },
                Projection::Perspective(PerspectiveProjection {
                    fov: 90.0,
                    ..Default::default()
                }),
                InterpolateRotation::default(),
                Transform::from_xyz(0.0, 0.5, 0.0),
            ));
        });
}

fn control_first_person_pawn(
    command: Res<PlayerCommand>,
    mut q: Query<&mut FirstPersonPawnCommand, With<FirstPersonPawn>>,
) {
    q.iter_mut().for_each(|mut c| c.apply(&command));
}

fn control_fly_pawn(command: Res<PlayerCommand>, mut q: Query<&mut FlyPawnCommand, With<FlyPawn>>) {
    q.iter_mut().for_each(|mut c| c.apply(&command));
}

const SENSITIVITY: Vec2 = Vec2::new(0.05, 0.05);

fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut().unwrap();
    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor_options.visible = false;
}

fn handle_mouse_motion(
    mut evr_mouse: EventReader<MouseMotion>,
    mut command: ResMut<PlayerCommand>,
) {
    for MouseMotion { delta } in evr_mouse.read() {
        command.angle.x += (delta.x * SENSITIVITY.x).to_radians();
        command.angle.y += (delta.y * SENSITIVITY.y).to_radians();
    }
}

fn clear_command(mut command: ResMut<PlayerCommand>) {
    command.angle = Vec2::ZERO;
    command.jump = false;
    command.fire = false;
}

fn handle_button_inputs(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut command: ResMut<PlayerCommand>,
) {
    command.forward = keyboard.pressed(KeyCode::KeyW);
    command.backward = keyboard.pressed(KeyCode::KeyS);
    command.left = keyboard.pressed(KeyCode::KeyA);
    command.right = keyboard.pressed(KeyCode::KeyD);
    command.up = keyboard.pressed(KeyCode::KeyE);
    command.down = keyboard.pressed(KeyCode::KeyC);
    command.crouch = keyboard.pressed(KeyCode::ControlLeft);
    command.sneak = keyboard.pressed(KeyCode::ShiftLeft);

    if !command.jump {
        command.jump = keyboard.just_pressed(KeyCode::Space);
    }

    if !command.fire {
        command.fire = mouse.pressed(MouseButton::Left);
    }
}
