use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::render::camera::Projection;
use bevy_atmosphere::prelude::*;

#[derive(Component)]
pub(crate) struct OrbitalCamera {
    pub focal_point: Vec3,
    pub orbital_radius: f32,
    pub upside_down: bool
}

impl Default for OrbitalCamera {
    fn default() -> Self {
        OrbitalCamera {
            focal_point: Vec3::ZERO,
            orbital_radius: 5.0,
            upside_down: false
        }
    }
}

fn move_camera(
    windows: Res<Windows>,
    orbit_triggered: bool,
    mut pan: &mut Vec2,
    mut rotation: &mut Vec2,
    mut zoom: f32,
    mut query: &mut Query<(&mut OrbitalCamera, &mut Transform, &Projection)>
) {
    for(mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_triggered {
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }
        let mut pos_changed = false;
        if rotation.length_squared() > 0.0 {
            pos_changed = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation.x / window.x * std::f32::consts::PI * 2.0;
                delta
            };
            let delta_y = rotation.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            pos_changed = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
            if let Projection::Perspective(projection) = projection {
                *pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.orbital_radius;
            pan_orbit.focal_point += translation;
        } else if zoom.abs() > 0.0 {
            pos_changed = true;
            pan_orbit.orbital_radius -= zoom * pan_orbit.orbital_radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.orbital_radius = f32::max(pan_orbit.orbital_radius, 0.05);
        }

        if pos_changed {
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = pan_orbit.focal_point + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.orbital_radius));
        }
    }
}

pub(crate) fn orbital_camera(
    windows: Res<Windows>,
    mut motion_events: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut OrbitalCamera, &mut Transform, &Projection)>,
) {
    let orbit_button = MouseButton::Left;
    let pan_button = MouseButton::Middle;

    let mut pan = Vec2::ZERO;
    let mut rotation = Vec2::ZERO;
    let mut zoom = 0.0;
    let mut orbit_triggered = false;

    if mouse_input.pressed(orbit_button) {
        for event in motion_events.iter() {
            rotation += event.delta;
        }
    } else if mouse_input.pressed(pan_button) {
        for event in motion_events.iter() {
            pan += event.delta;
        }
    }
    for event in scroll_events.iter() {
        zoom += event.y;
    }
    if mouse_input.just_pressed(orbit_button) || mouse_input.just_released(orbit_button) {
        orbit_triggered = true;
    }

    move_camera(windows, orbit_triggered, &mut pan, &mut rotation, zoom, &mut query)
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

pub(crate) fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        AtmosphereCamera::default(),
        OrbitalCamera {
            orbital_radius: radius,
            ..Default::default()
        }
    ));
}
