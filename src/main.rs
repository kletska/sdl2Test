extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;

use vec3::Vec3;
use ray::Ray;
use hittable::{Hittable, HitRecord};
use sphere::Sphere;
use hittable_list::HittableList;

const SCALE_FACTOR: u32 = 4;
const SCREEN_WIDTH: u32 = SCALE_FACTOR * 200;
const SCREEN_HEIGHT: u32 = SCALE_FACTOR * 100;

fn trace_ray(r: &Ray) -> u32 {
    let h = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if h > 0.0{
        let N = Vec3::unit_vector(r.point_at_parametr(h) - Vec3::new(0.0, 0.0, -1.0));
        (Vec3::new(N.0 + 1.0, N.1 + 1.0, N.2 + 1.0) / 2.0).to_color()
    } else {
        let unit_direction: Vec3 = Vec3::unit_vector(r.direction());
        let t: f32 = 0.5 * (unit_direction.1 + 1.0);
        (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).to_color()
    }
}

fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - center;
    let a = Vec3::dotProd(r.direction(), r.direction());
    let b = 2.0 * Vec3::dotProd(oc, r.direction());
    let c = Vec3::dotProd(oc, oc) - radius * radius;
    let d = b * b - 4.0 * a * c;
    if d > 0.0 {
        (-b - d.sqrt()) / (2.0  * a)
    } else {
        -1.0
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys.window("rust-sdl2_gfx: draw line & FPSManager", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut events = sdl_context.event_pump()?;

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => break 'main,
                _ => {}
            }
        }
        let lower_left_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
        let horisontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
        let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
        let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        for i in 0..SCREEN_WIDTH {
            for j in 0..SCREEN_HEIGHT {
                let u: f32 = i as f32 / SCREEN_WIDTH as f32;
                let v: f32 = (SCREEN_HEIGHT - j - 1) as f32 / SCREEN_HEIGHT as f32;
                let r: Ray = Ray::new(origin, lower_left_corner + horisontal * u + vertical * v);
                canvas.pixel(i as i16, j as i16, 0xFFu32 * 256 * 256 * 256 + trace_ray(&r))?;
            }
        }
        canvas.present();
    }
    Ok(())
}

