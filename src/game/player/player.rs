use avian3d::{math::*, prelude::*};
use bevy::prelude::*;

use super::{camera_controller, camera_controller::*, character_controller::*};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CharacterControllerPlugin, CameraControllerPlugin))
            //.add_systems(Update, (camera_controller::update_camera_controller,))
            //physics timestep
            .add_systems(Startup, init_player);
    }
}

#[derive(Debug, Component)]
struct Player;

fn init_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let player = commands
        .spawn((
            Player,
            Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            Transform::from_xyz(0.0, 1.5, 0.0),
            CharacterControllerBundle::new(Collider::capsule(0.4, 1.0), Vector::NEG_Y * 9.81 * 2.0)
                .with_movement(30.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
        ))
        .id();

    let fov = 90.0_f32.to_radians(); //106.0_f32.to_radians();
                                     // Camera
    let camera = commands
        .spawn((
            Camera3d::default(),
            Camera {
                // Bump the order to render on top of the world model.
                order: 1,
                ..default()
            },
            Projection::from(PerspectiveProjection {
                fov: fov,
                ..default()
            }),
            camera_controller::CameraController {
                //sensitivity: 0.035,
                sensitivity: 0.05,
                rotation: Vec2::ZERO,
                rotation_lock: 88.0,
            },
        ))
        .id();

    commands.entity(camera);
    commands.entity(player).add_child(camera);
}
