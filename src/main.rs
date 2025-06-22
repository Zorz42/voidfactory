mod camera;

extern crate kiss3d;

use kiss3d::event::{Action, Key, WindowEvent};
use kiss3d::nalgebra::{Vector3, Translation, Point3, Translation3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use crate::camera::MyCamera;

fn main() {
    let mut window = Window::new_with_size("VoidFactory", 1000, 800);
    let mut c = window.add_cube(1.0, 1.0, 1.0);
    c.append_translation(&Translation::<f32,3>::new(0.0, 0.0, 3.0));
    c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    window.set_framerate_limit(Some(60));
    let mut camera = MyCamera::new(
        Point3::new(0.0, 0.0, -10.0),
        Point3::new(0.0, 0.0, 1.0),
    );
    let camera_speed = 0.3;

    let mut yaw = std::f32::consts::PI / 2.0; // Initial yaw angle
    let mut pitch = std::f32::consts::PI / 2.0; // Initial pitch angle
    
    window.set_cursor_position(window.size()[0] as f64 / 2.0, window.size()[1] as f64 / 2.0);
    // do not grab the cursor on MacOS, it will cause the cursor to be stationary
    #[cfg(not(target_os = "macos"))]
    window.set_cursor_grab(true);       // Lock cursor to window
    window.hide_cursor(true);      // Hide cursor

    camera.set_yaw_pitch(yaw, pitch);

    let mut prev_cursor_pos = (window.size()[0] as f64 / 2.0, window.size()[1] as f64 / 2.0);
    
    while window.render_with_camera(&mut camera) {
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
        
        let curr_cursor_pos = window.cursor_pos().unwrap_or(prev_cursor_pos);
        if curr_cursor_pos != prev_cursor_pos {

            // Calculate the delta from the center of the window
            let delta_x = curr_cursor_pos.0 - prev_cursor_pos.0;
            let delta_y = curr_cursor_pos.1 - prev_cursor_pos.1;
            
            if delta_x.abs() < 500.0 && delta_y.abs() < 200.0 {
                // sometimes the cursor can jump, so we ignore large deltas

                prev_cursor_pos = curr_cursor_pos;

                // Update camera rotation based on cursor movement
                yaw += delta_x as f32 * 0.001; // Adjust sensitivity as needed
                pitch += delta_y as f32 * 0.001; // Adjust sensitivity as needed

                // Clamp pitch to avoid gimbal lock
                if pitch > std::f32::consts::PI - 0.01 {
                    pitch = std::f32::consts::PI - 0.01;
                } else if pitch < 0.01 {
                    pitch = 0.01;
                }
                camera.set_yaw_pitch(yaw, pitch);
            }

            if curr_cursor_pos.0 < 10.0 || curr_cursor_pos.0 > window.size()[0] as f64 - 10.0 ||
                curr_cursor_pos.1 < 10.0 || curr_cursor_pos.1 > window.size()[1] as f64 - 10.0 {
                // Reset cursor position to the center of the window
                let window_middle_x = window.size()[0] as f64 / 2.0;
                let window_middle_y = window.size()[1] as f64 / 2.0;
                window.set_cursor_position(window_middle_x, window_middle_y);
                prev_cursor_pos = (window_middle_x, window_middle_y);
            }
        }
        
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::Escape, Action::Press, _) => {
                    window.close();
                    event.inhibited = true;
                }
                _ => {}
            }
        }
    }
}