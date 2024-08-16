use nalgebra_glm::{Vec3, normalize};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod ray_intersect;
mod sphere;
mod color;
mod camera;

use framebuffer::Framebuffer;
use sphere::Sphere;
use color::Color;
use ray_intersect::{Intersect, RayIntersect, Material};
use camera::Camera;

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            intersect = tmp;
        }
    }

    if !intersect.is_intersecting {
        return Color::new(120, 180, 130); // Green background
    }
    
    let diffuse = intersect.material.diffuse;

    diffuse
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere], camera: &Camera) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI/3.0;
    let perspective_scale = (fov * 0.5).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
            let rotated_direction = camera.basis_change(&ray_direction);

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects);

            framebuffer.set_current_color(pixel_color.to_hex());
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Osito Teddy",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    window.set_position(500, 500);
    window.update();

    // Define materials
    let fur = Material {
        diffuse: Color::new(139, 69, 19), // Brown color for fur
    };
    let eye = Material {
        diffuse: Color::new(0, 0, 0), // Black color for eyes
    };
    let nose = Material {
        diffuse: Color::new(0, 0, 0), // Black color for nose
    };
    let inner_ear = Material {
        diffuse: Color::new(255, 255, 255), // White color for inner ear
    };
    let mouth = Material {
        diffuse: Color::new(255, 255, 255), // White color for inner ear
    };

    // Define spheres for the bear face
    let objects = [
        // Head
        Sphere {
            center: Vec3::new(0.0, 0.0, -5.0),
            radius: 1.0,
            material: fur,
        },
        // Left Ear
        Sphere {
            center: Vec3::new(-0.75, 0.75, -5.0),
            radius: 0.5,
            material: fur,
        },
        Sphere {
            center: Vec3::new(-0.75, 0.75, -4.75),
            radius: 0.3,
            material: inner_ear,
        },
        // Right Ear
        Sphere {
            center: Vec3::new(0.75, 0.75, -5.0),
            radius: 0.5,
            material: fur,
        },
        Sphere {
            center: Vec3::new(0.75, 0.75, -4.75),
            radius: 0.3,
            material: inner_ear,
        },
        // Left Eye
        Sphere {
            center: Vec3::new(-0.45, 0.1, -4.2), 
            radius: 0.15, 
            material: eye,
        },
        // Right Eye
        Sphere {
            center: Vec3::new(0.45, 0.1, -4.2), 
            radius: 0.15, 
            material: eye,
        },
        // Nose
        Sphere {
            center: Vec3::new(0.0, -0.3, -4.2), 
            radius: 0.25, 
            material: nose,
        },
        // Mouth
        Sphere {
            center: Vec3::new(0.0, -0.4, -4.5), 
            radius: 0.5, 
            material: mouth,
        },
    ];

    // Initialize camera
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),  // Camera at origin
        Vec3::new(0.0, 0.0, -5.0),  // Looking directly at the bear face
        Vec3::new(0.0, 1.0, 0.0)   // up: World up vector
    );

    let rotation_speed = PI / 10.0;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Camera controls
        if window.is_key_down(Key::Left) {
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, rotation_speed);
        }

        // Render the bear face
        render(&mut framebuffer, &objects, &camera);

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
