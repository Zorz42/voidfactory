mod camera;

extern crate kiss3d;

use kiss3d::event::{Action, Key};
use kiss3d::nalgebra::{Vector3, UnitQuaternion, Translation, Point3, Translation3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use crate::camera::MyCamera;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    let mut c = window.add_cube(1.0, 1.0, 1.0);
    c.append_translation(&Translation::<f32,3>::new(0.0, 0.0, 3.0));
    c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    window.set_framerate_limit(Some(60));
    let mut camera = MyCamera::new(
        Point3::new(0.0, 0.0, -10.0),
        Point3::new(0.0, 0.0, 1.0),
    );
    let camera_speed = 0.3;

    while window.render_with_camera(&mut camera) {
        c.prepend_to_local_rotation(&rot);
        // check for user input to move camera
        let mut movement: Vector3<f32> = Vector3::zeros();

        if window.get_key(Key::W) == Action::Press {
            movement[0] += 1.0;
        }
        if window.get_key(Key::S) == Action::Press {
            movement[0] -= 1.0;
        }
        if window.get_key(Key::A) == Action::Press {
            movement[2] -= 1.0;
        }
        if window.get_key(Key::D) == Action::Press {
            movement[2] += 1.0;
        }
        if window.get_key(Key::Space) == Action::Press {
            movement[1] += 1.0;
        }
        if window.get_key(Key::LShift) == Action::Press {
            movement[1] -= 1.0;
        }

        // Move relative to camera's orientation
        if movement != Vector3::zeros() {
            let mut forward = camera.eye_dir();
            forward[1] = 0.0; // Ignore vertical movement
            forward = forward.normalize();
            let right = forward.cross(&Vector3::y_axis()).normalize();
            let up = *Vector3::y_axis();

            let delta = (forward * movement[0] + right * movement[2] + up * movement[1]) * camera_speed;
            camera = camera.translate(&Translation3::from(delta));
        }
    }
}