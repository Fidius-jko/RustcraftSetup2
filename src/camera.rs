use crate::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use leafwing_input_manager::prelude::*;

pub const CAMERA_SPEED: f32 = 8.;
pub const CAMERA_SENTIVITY: f32 = 0.00012;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    #[autodefault]
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera)
            .add_systems(
                Update,
                (move_camera, grab_cursor, change_view).run_if(in_state(GameState::Play)),
            )
            .add_plugins(InputManagerPlugin::<CameraActions>::default())
            .init_resource::<ActionState<CameraActions>>()
            .insert_resource(CameraActions::mkb_input_map());
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CameraActions {
    Move,
    Up,
    Down,
    GrabCursor,
    UnGrabCursor,
    ViewMotion,
}

impl CameraActions {
    fn mkb_input_map() -> InputMap<CameraActions> {
        use KeyCode::*;
        InputMap::new([
            (Self::Up, UserInput::Single(InputKind::PhysicalKey(Space))),
            (
                Self::Down,
                UserInput::Single(InputKind::PhysicalKey(ShiftLeft)),
            ),
            (Self::Move, UserInput::VirtualDPad(VirtualDPad::wasd())),
            (
                Self::Move,
                UserInput::VirtualDPad(VirtualDPad::arrow_keys()),
            ),
            (
                Self::GrabCursor,
                UserInput::Single(InputKind::Mouse(MouseButton::Right)),
            ),
            (
                Self::UnGrabCursor,
                UserInput::Single(InputKind::PhysicalKey(KeyCode::Escape)),
            ),
            (
                Self::ViewMotion,
                UserInput::Single(InputKind::DualAxis(DualAxis::mouse_motion())),
            ),
        ])
    }
}

#[autodefault]
fn add_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {}).insert(MainCamera);
}

fn move_camera(
    mut cam: Query<&mut Transform, With<MainCamera>>,
    action_state: Res<ActionState<CameraActions>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = primary_window.get_single() {
        match window.cursor.grab_mode {
            CursorGrabMode::None => {}
            _ => {
                let mut transform = cam.single_mut();
                let mut velocity = Vec3::ZERO;
                let local_z = transform.local_z();
                let forward = -Vec3::new(local_z.x, 0., local_z.z);
                let right = Vec3::new(local_z.z, 0., -local_z.x);

                if action_state.pressed(&CameraActions::Up) {
                    velocity += Vec3::Y;
                }
                if action_state.pressed(&CameraActions::Down) {
                    velocity -= Vec3::Y;
                }
                if action_state.pressed(&CameraActions::Move) {
                    let axis_pair = action_state
                        .clamped_axis_pair(&CameraActions::Move)
                        .unwrap();
                    velocity += axis_pair.y() * forward;
                    velocity += axis_pair.x() * right;
                }
                velocity = velocity.normalize_or_zero();

                transform.translation += velocity * time.delta_seconds() * CAMERA_SPEED;
            }
        }
    } else {
        warn!("Can't found primary window!");
    }
}

fn change_view(
    mut cam: Query<&mut Transform, With<MainCamera>>,
    action_state: Res<ActionState<CameraActions>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = primary_window.get_single() {
        if action_state.pressed(&CameraActions::ViewMotion) {
            let mut transform = cam.single_mut();
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            match window.cursor.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    let axis_pair = action_state.axis_pair(&CameraActions::ViewMotion).unwrap();
                    // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                    let window_scale = window.height().min(window.width());
                    pitch -= (CAMERA_SENTIVITY * axis_pair.y() * window_scale).to_radians();
                    yaw -= (CAMERA_SENTIVITY * axis_pair.x() * window_scale).to_radians();
                }
            }

            pitch = pitch.clamp(-1.54, 1.54);

            // Order is important to prevent unintended roll
            transform.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    } else {
        warn!("Can't found primary window!");
    }
}

fn grab_cursor(
    action_state: Res<ActionState<CameraActions>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if action_state.just_pressed(&CameraActions::GrabCursor) {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        }
        if action_state.just_pressed(&CameraActions::UnGrabCursor) {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    } else {
        warn!("Can't found primary window!");
    }
}
