use avian3d::{math::*, prelude::*};
use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct CameraControllerPlugin;

/*impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CameraAction>().add_systems(
            Update,
            (mouse_input, gamepad_input, update_camera_controller).chain(),
        );
    }
}*/
impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CameraAction>();
        app.add_systems(Update, mouse_input);
        app.add_systems(Update, gamepad_input);
        app.add_systems(Update, update_camera_controller);
    }
}

/// An event sent for a movement input action.
#[derive(Event)]
pub enum CameraAction {
    Move(Vector2),
}
#[derive(Component)]
pub struct CameraController {
    pub rotation: Vec2,
    pub rotation_lock: f32,
    pub sensitivity: f32,
}

pub fn update_camera_controller(
    mut camera_action_reader: EventReader<CameraAction>,
    mut camera_query: Query<(&mut CameraController, &mut Transform)>,
) {
    if let Ok((mut camera_controller, mut transform)) = camera_query.get_single_mut() {
        for ev in camera_action_reader.read() {
            match ev {
                CameraAction::Move(direction) => {
                    camera_controller.rotation.y -= direction.x; // * camera_controller.sensitivity; // may need to edit this in mouse_input
                    camera_controller.rotation.x -= direction.y; // * camera_controller.sensitivity;

                    camera_controller.rotation.x = f32::clamp(
                        camera_controller.rotation.x,
                        -camera_controller.rotation_lock,
                        camera_controller.rotation_lock,
                    );
                }
            }
        }
        let y_quat = Quat::from_axis_angle(Vec3::Y, camera_controller.rotation.y.to_radians());
        let x_quat = Quat::from_axis_angle(Vec3::X, camera_controller.rotation.x.to_radians());
        transform.rotation = y_quat * x_quat;
    }
}

fn gamepad_input(mut movement_event_writer: EventWriter<CameraAction>, gamepads: Query<&Gamepad>) {
    for gamepad in gamepads.iter() {
        if let (Some(x), Some(y)) = (
            gamepad.get(GamepadAxis::RightStickX),
            gamepad.get(GamepadAxis::RightStickY),
        ) {
            movement_event_writer.send(CameraAction::Move(
                Vector2::new(x * 4.0 as Scalar, y * 4.0 as Scalar).clamp_length_max(1.0),
            ));
        }
    }
}

fn mouse_input(
    mut mouse_motion: EventReader<MouseMotion>,
    mut movement_event_writer: EventWriter<CameraAction>,
) {
    for ev in mouse_motion.read() {
        let x = ev.delta.x * 0.05;
        let y = ev.delta.y * 0.05;
        // Only send the event if either axis is above 0.1 in absolute value
        if x.abs() > 0.1 || y.abs() > 0.1 {
            movement_event_writer.send(CameraAction::Move(
                Vector2::new(x as Scalar, y as Scalar).clamp_length_max(1.0),
            ));
        }
    }
}
