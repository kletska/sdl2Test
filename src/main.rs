extern crate sdl2;
extern crate rand;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;

use rand::prelude::*;

mod vec3;
mod ray;
mod material;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;

use vec3::{Vec3, random_in_unit_sphere};
use ray::Ray;
use material::Material;
use hittable::{Hittable, HitRecord};
use sphere::Sphere;
use hittable_list::HittableList;
use camera::Camera;

const SCALE_FACTOR: u32 = 2;
const SCREEN_WIDTH: u32 = SCALE_FACTOR * 200;
const SCREEN_HEIGHT: u32 = SCALE_FACTOR * 100;
const SAMPLES: u32 = 100;

fn trace_ray(r: &Ray, world: &HittableList, depth: u32) -> Vec3 {
    if depth > 50 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Some(res) =  world.hit(r, 0.001, std::f32::MAX) {
        let mut scattered = Ray::new(Vec3::default(), Vec3::default());
        let mut attenuation = Vec3::default(); 
        if depth < 50 && res.material.scatter(r, &res, &mut attenuation, &mut scattered) {
           attenuation * trace_ray(&scattered, world, depth + 1)
           
        } else {
            Vec3::default()
        }
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.1 + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys.window("rust-sdl2_gfx: draw line & FPSManager", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut events = sdl_context.event_pump()?;
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();   
    list.push(Box::new(Sphere::new(
                Vec3::new(0.0, -1000.0, 0.0), 
                1000.0,
                Material::Lambertian { 
                    albedo: Vec3::new(0.5, 0.5, 0.5) 
                }
            )
        )
    );

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3::new((a as f32) + 0.9 * rng.gen::<f32>(), 0.2, (b as f32) + 0.9 * rng.gen::<f32>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian { 
                            albedo: Vec3::new(
                                        rng.gen::<f32>() * rng.gen::<f32>(), 
                                        rng.gen::<f32>() * rng.gen::<f32>(), 
                                        rng.gen::<f32>() * rng.gen::<f32>()
                                        ) 
                        }
                    )));
                } else if choose_mat < 0.95 {
                     list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Metal { 
                            albedo: Vec3::new(
                                        0.5 * (1.0 + rng.gen::<f32>()), 
                                        0.5 * (1.0 + rng.gen::<f32>()), 
                                        0.5 * (1.0 + rng.gen::<f32>()), 
                                        ),
                            fuzz: 0.5 * rng.gen::<f32>()
                        }
                    )));
                } else {
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric { 
                            ref_idx: 1.5
                        }
                    )));
                }
            }
        }
    }

    list.push(Box::new(Sphere::new(
                Vec3::new(0.0, 1.0, 0.0), 
                1.0,
                Material::Dielectric { 
                    ref_idx: 1.5
                }
            )
        )
    );
    list.push(Box::new(Sphere::new(
                Vec3::new(-4.0, 1.0, 0.0), 
                1.0,
                Material::Lambertian { 
                    albedo: Vec3::new(0.4, 0.2, 0.1) 
                }
            )
        )
    );
    list.push(Box::new(Sphere::new(
                Vec3::new(4.0, 1.0, 0.0), 
                1.0,
                Material::Metal { 
                    albedo: Vec3::new(0.7, 0.6, 0.5),
                    fuzz: 0.0
                }
            )
        )
    );
    let world = HittableList::new(list);
        
    
    let look_from = Vec3::new(3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture: f32 = 2.0;

    let cam = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        std::f32::consts::PI / 9.0,
        SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32,
        aperture,
        dist_to_focus,
    );
    
    for i in 0..SCREEN_WIDTH {
        for j in 0..SCREEN_HEIGHT {
            let mut col = Vec3::default();
            for _ in 0..SAMPLES {
                let u = (i as f32 + rng.gen::<f32>()) / SCREEN_WIDTH as f32;
                let v = ((SCREEN_HEIGHT - j - 1) as f32 +rng.gen::<f32>()) / SCREEN_HEIGHT as f32;
                //println!("1");
                let r = &cam.get_ray(u, v);
                //println!("2");
                let tmp = trace_ray(&r, &world, 0);
                //println!("3");
                col = col + tmp;
            }
            col = col / (SAMPLES as f32);
            canvas.pixel(i as i16, j as i16, 0xFFu32 * 256 * 256 * 256 + col.to_color())?;
        }
    }
    canvas.present();

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => break 'main,
                _ => {}
            }
        }
        /*
        let mut rng = rand::thread_rng();
        for i in 0..SCREEN_WIDTH {
            for j in 0..SCREEN_HEIGHT {
                let mut col = Vec3::default();
                for _ in 0..SAMPLES {
                    let u = (i as f32 + rng.gen::<f32>()) / SCREEN_WIDTH as f32;
                    let v = ((SCREEN_HEIGHT - j - 1) as f32 +rng.gen::<f32>()) / SCREEN_HEIGHT as f32;
                    let r = &cam.get_ray(u, v);
                    let tmp = trace_ray(&r, &world, 0);
                    col = col + tmp;
                }
                col = col / (SAMPLES as f32);
                canvas.pixel(i as i16, j as i16, 0xFFu32 * 256 * 256 * 256 + col.to_color())?;
            }
        }
        canvas.present();
        */
    }
    Ok(())
}

