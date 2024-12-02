use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, (
			camera_follow_focus, 
			confine_camera_movement,
		).chain());
        app.add_systems(Update, camera_zoom);
    }
}

#[derive(Component)]
pub struct CameraFocus {}

#[derive(Component)]
pub struct CameraBounds {
	pub min: Vec2,
	pub max: Vec2,
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
    ));
}

pub fn camera_follow_focus(
    mut camera_query: Query<&mut Transform, With<Camera2d>>, 
    focus_query: Query<&Transform, (With<CameraFocus>, Without<Camera2d>)>
) {
    if let Ok(focus_transform) = focus_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let follow_direction = focus_transform.translation - camera_transform.translation;
            if follow_direction.x > 100. {
                camera_transform.translation.x += (follow_direction.x - 100.) / 10.;
            } else if follow_direction.x < -100. {
                camera_transform.translation.x += (follow_direction.x + 100.) / 10.;
            }
            if follow_direction.y > 100. {
                camera_transform.translation.y += (follow_direction.y - 100.) / 10.;
            } else if follow_direction.y < -100. {
                camera_transform.translation.y += (follow_direction.y + 100.) / 10.;
            }
        }
    }
}

pub fn confine_camera_movement(
    mut camera_query: Query<(&mut Transform, &OrthographicProjection), With<Camera2d>>,
	bounds_query: Query<&CameraBounds>,
) {
	if let Ok(bounds) = bounds_query.get_single() {
		if let Ok((mut camera_transform, camera_projection)) = camera_query.get_single_mut() {
			// TODO: Compensate for camera scaling
			let view_margin = 20.;
			let x_min = bounds.min.x - camera_projection.area.min.x - view_margin;
			let x_max = bounds.max.x - camera_projection.area.max.x + view_margin;
			let y_min = bounds.min.y - camera_projection.area.min.y - view_margin;
			let y_max = bounds.max.y - camera_projection.area.max.y + view_margin;

			if camera_transform.translation.x < x_min {
				camera_transform.translation.x = x_min;
			} else if camera_transform.translation.x > x_max {
				camera_transform.translation.x = x_max;
			}
			if camera_transform.translation.y < y_min {
				camera_transform.translation.y = y_min;
			} else if camera_transform.translation.y > y_max {
				camera_transform.translation.y = y_max;
			}
		}
	}
}

pub fn camera_zoom(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>, 
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::KeyQ) {
            camera_transform.scale *= 1.01;
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            camera_transform.scale *= 0.99;
        }
    }
}