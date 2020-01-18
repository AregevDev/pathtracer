use crate::camera::Camera;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::random_float;
use crate::sphere::Sphere;
use crate::vector::Vector3;
use crate::world::World;
use std::rc::Rc;

pub fn basic_scene(width: usize, height: usize) -> World {
    let eye = Vector3::new(4.0, 4.0, 4.0);
    let center = Vector3::new(0.0, 0.0, 0.0);
    let up = Vector3::unit_y();

    let focus = (eye - center).length();
    let aperture = 1.5;

    let camera = Camera::new(
        eye,
        center,
        up,
        30.0,
        width as f32 / height as f32,
        aperture,
        focus,
    );

    let mut world = World::new(camera);
    world.add(Sphere::new(
        Vector3::new(0.0, -100.5, 0.0),
        100.0,
        Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0))),
    ));
    world.add(Sphere::new(
        Vector3::new(0.0, 0.0, 0.0),
        0.5,
        Rc::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.5))),
    ));
    world.add(Sphere::new(
        Vector3::new(1.0, 0.0, 0.0),
        0.5,
        Rc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.2)),
    ));
    world.add(Sphere::new(
        Vector3::new(-1.0, 0.0, 0.0),
        0.5,
        Rc::new(Dielectric::new(1.5)),
    ));
    world.add(Sphere::new(
        Vector3::new(-1.0, 0.0, 0.0),
        -0.45,
        Rc::new(Dielectric::new(1.5)),
    ));

    world
}

pub fn random_scene(width: usize, height: usize) -> World {
    let eye = Vector3::new(0.0, 3.0, 3.0);
    let center = Vector3::new(0.0, 0.0, 0.0);
    let up = Vector3::unit_y();

    let focus = (eye - center).length();
    let aperture = 0.1;

    let camera = Camera::new(
        eye,
        center,
        up,
        60.0,
        width as f32 / height as f32,
        aperture,
        focus,
    );

    let mut world = World::new(camera);
    world.add(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.5))),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let random_mat = random_float();
            let sp = Vector3::new(
                a as f32 + 0.9 * random_float(),
                0.2,
                b as f32 + 0.9 * random_float(),
            );

            if (sp - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if random_mat < 0.8 {
                    world.add(Sphere::new(
                        sp,
                        0.2,
                        Rc::new(Lambertian::new(Vector3::new(
                            random_float() * random_float(),
                            random_float() * random_float(),
                            random_float() * random_float(),
                        ))),
                    ));
                } else if random_mat < 0.95 {
                    world.add(Sphere::new(
                        sp,
                        0.2,
                        Rc::new(Metal::new(
                            Vector3::new(
                                0.5 * (random_float() + 1.0),
                                0.5 * (random_float() + 1.0),
                                0.5 * (random_float() + 1.0),
                            ),
                            0.5 * random_float(),
                        )),
                    ));
                } else {
                    world.add(Sphere::new(sp, 0.2, Rc::new(Dielectric::new(1.5))));
                }
            }
        }
    }

    world.add(Sphere::new(
        Vector3::new(0.0, 0.9, 0.0),
        1.0,
        Rc::new(Metal::new(Vector3::new(0.0, 0.5, 0.9), 0.0)),
    ));

    world
}

pub fn colored_sphere_scene(width: usize, height: usize) -> World {
    let eye = Vector3::new(-5.5, 5.5, 5.5);
    let center = Vector3::new(0.0, 0.0, 0.0);
    let up = Vector3::unit_y();

    let focus = (eye - center).length();
    let aperture = 0.1;

    let camera = Camera::new(
        eye,
        center,
        up,
        60.0,
        width as f32 / height as f32,
        aperture,
        focus,
    );

    let mut world = World::new(camera);

    world.add(Sphere::new(
        Vector3::new(0.0, -1003.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.8))),
    ));

    for a in -5..=5 {
        for b in -5..=5 {
            for c in -5..=5 {
                let color = Vector3::new(
                    (a as f32 + 5.0) / 11.0,
                    (b as f32 + 5.0) / 11.0,
                    (c as f32 + 5.0) / 11.0,
                );
                let sp = Vector3::new(a as f32 * 0.5, b as f32 * 0.5, c as f32 * 0.5);

                world.add(Sphere::new(sp, 0.2, Rc::new(Lambertian::new(color))));
            }
        }
    }

    world
}
