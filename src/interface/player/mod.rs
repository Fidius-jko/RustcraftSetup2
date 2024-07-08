use bevy::{
    render::view::RenderLayers,
    window::{CursorGrabMode, PrimaryWindow},
};
use leafwing_input_manager::{
    action_state::ActionState,
    axislike::{DualAxis, VirtualDPad},
    input_map::InputMap,
    plugin::InputManagerPlugin,
    user_input::{InputKind, UserInput},
    Actionlike,
};

use crate::{
    prelude::*,
    voxel::{blocks::Block, chunks::chunk::Chunk},
};

use super::{
    constants::{CAMERA_SENTIVITY, CAMERA_SPEED, VOXEL_SIZE},
    render::{
        camera::MainCamera,
        voxel::{blocks::storage::BlockStorage, chunk::RenderOfChunk},
    },
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_camera,
                grab_cursor,
                change_view,
                player_action,
                select_block,
            )
                .run_if(in_state(GameState::Play)),
        )
        .insert_resource(SelectedBlock("grass".to_string()))
        .init_gizmo_group::<PointerGizmo>()
        .add_systems(OnEnter(GameState::Play), set_cam_pos)
        .add_plugins(InputManagerPlugin::<PlayerActions>::default())
        .init_resource::<ActionState<PlayerActions>>()
        .insert_resource(PlayerActions::mkb_input_map());
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerActions {
    Move,
    Up,
    Down,
    GrabCursor,
    UnGrabCursor,
    ViewMotion,
    PlaceBlock,
    HurtBlock,
}

impl PlayerActions {
    fn mkb_input_map() -> InputMap<PlayerActions> {
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
                UserInput::Single(InputKind::PhysicalKey(KeyCode::Tab)),
            ),
            (
                Self::UnGrabCursor,
                UserInput::Single(InputKind::PhysicalKey(KeyCode::Escape)),
            ),
            (
                Self::ViewMotion,
                UserInput::Single(InputKind::DualAxis(DualAxis::mouse_motion())),
            ),
            (
                Self::HurtBlock,
                UserInput::Single(InputKind::Mouse(MouseButton::Right)),
            ),
            (
                Self::PlaceBlock,
                UserInput::Single(InputKind::Mouse(MouseButton::Left)),
            ),
        ])
    }
}
fn set_cam_pos(
    mut cam: Query<&mut Transform, With<MainCamera>>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    let mut cam = cam.single_mut();
    cam.translation = Vec3::new(0., VOXEL_SIZE * 18., 0.);
    let (my_config, _) = config_store.config_mut::<PointerGizmo>();
    my_config.render_layers = RenderLayers::none().with(1);
}
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct PointerGizmo;

#[derive(Resource)]
pub struct SelectedBlock(String);

fn select_block(keys: Res<ButtonInput<KeyCode>>, mut selected: ResMut<SelectedBlock>) {
    if keys.pressed(KeyCode::Digit1) {
        selected.0 = "grass".to_string();
    } else if keys.pressed(KeyCode::Digit2) {
        selected.0 = "dirt".to_string();
    } else if keys.pressed(KeyCode::Digit3) {
        selected.0 = "cobblestone".to_string();
    }
}

fn player_action(
    mut chunk: Query<(&mut Chunk, &mut RenderOfChunk)>,
    cam: Query<&GlobalTransform, With<MainCamera>>,
    action_state: Res<ActionState<PlayerActions>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut pointer_gizmos: Gizmos<PointerGizmo>,
    mut gizmos: Gizmos,
    storage: Res<BlockStorage>,
    selected: Res<SelectedBlock>,
) {
    if let Ok(window) = primary_window.get_single() {
        match window.cursor.grab_mode {
            CursorGrabMode::None => {
                return;
            }
            _ => {}
        }
    }
    let camera_transform = cam.single();
    pointer_gizmos.line_2d(Vec2::new(-10., 0.), Vec2::new(10., 0.), Color::WHITE);
    pointer_gizmos.line_2d(Vec2::new(0., -10.), Vec2::new(0., 10.), Color::WHITE);
    let (mut chunk, mut render) = chunk.single_mut();

    let max_dist = 20.;
    let mut ray_pos = camera_transform.translation();
    let mut iray_pos = (camera_transform.translation() / VOXEL_SIZE)
        .floor()
        .as_ivec3();
    let mut prev_iray_pos = iray_pos.clone();
    let dir = camera_transform.forward() / 100.;
    while camera_transform.translation().distance(ray_pos) <= max_dist {
        if let Some(Block::Solid(_)) = chunk.get_i32(iray_pos.x, iray_pos.y, iray_pos.z) {
            break;
        }
        prev_iray_pos = iray_pos.clone();
        ray_pos += dir;
        iray_pos = ((ray_pos) / VOXEL_SIZE).floor().as_ivec3();
    }
    if camera_transform.translation().distance(ray_pos) > max_dist {
        return;
    }

    gizmos.cuboid(
        Transform {
            translation: iray_pos.as_vec3()
                + Vec3::new(VOXEL_SIZE / 2., VOXEL_SIZE / 2., VOXEL_SIZE / 2.),
            ..Default::default()
        },
        Color::BLACK,
    );
    if action_state.just_pressed(&PlayerActions::PlaceBlock) && prev_iray_pos != iray_pos {
        chunk.set_i32(
            prev_iray_pos.x,
            prev_iray_pos.y,
            prev_iray_pos.z,
            crate::voxel::blocks::Block::Solid(
                storage.get_id_by_name(selected.0.clone()).unwrap().clone(),
            ),
        );
        render.is_generated_mesh = false;
    }
    if action_state.just_pressed(&PlayerActions::HurtBlock) {
        chunk.set_i32(
            iray_pos.x,
            iray_pos.y,
            iray_pos.z,
            crate::voxel::blocks::Block::Air,
        );
        render.is_generated_mesh = false;
    }
}
fn move_camera(
    mut cam: Query<&mut Transform, With<MainCamera>>,
    action_state: Res<ActionState<PlayerActions>>,
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

                if action_state.pressed(&PlayerActions::Up) {
                    velocity += Vec3::Y;
                }
                if action_state.pressed(&PlayerActions::Down) {
                    velocity -= Vec3::Y;
                }
                if action_state.pressed(&PlayerActions::Move) {
                    let axis_pair = action_state
                        .clamped_axis_pair(&PlayerActions::Move)
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
    action_state: Res<ActionState<PlayerActions>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = primary_window.get_single() {
        if action_state.pressed(&PlayerActions::ViewMotion) {
            let mut transform = cam.single_mut();
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            match window.cursor.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    let axis_pair = action_state.axis_pair(&PlayerActions::ViewMotion).unwrap();
                    let window_scale = window.height().min(window.width());
                    pitch -= (CAMERA_SENTIVITY * axis_pair.y() * window_scale).to_radians();
                    yaw -= (CAMERA_SENTIVITY * axis_pair.x() * window_scale).to_radians();
                }
            }

            pitch = pitch.clamp(-1.54, 1.54);

            transform.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    } else {
        warn!("Can't found primary window!");
    }
}

fn grab_cursor(
    action_state: Res<ActionState<PlayerActions>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if action_state.just_pressed(&PlayerActions::GrabCursor) {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        }
        if action_state.just_pressed(&PlayerActions::UnGrabCursor) {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    } else {
        warn!("Can't found primary window!");
    }
}
