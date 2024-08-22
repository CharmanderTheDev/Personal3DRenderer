use objects::my_textures;

mod mymath;
    use mymath::vec3::Vec3;
mod objects;
    use objects::objects::*;
mod render;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    //DEFINING SCENE
    let mut scene = render::Scene {
        animationobjects: Vec::new(),
        background_texture: objects::my_textures::BORING_TEXTURE,
        max_depth: 4,
        frames: 1
    };


    //DEFINING MOTION FUNCTIONS
    fn spinmeten(object: &mut objects::object_traits::Object){
        match &mut object.object {
            objects::object_traits::Kind::SPHERE(sphere)=>{sphere.moveme(&mymath::functions::spin(&sphere.position, &mymath::vec3::Vec3::zero(), &mymath::mathstructs::Rotation{axis: mymath::mathstructs::RotationAxis::XY, theta: std::f32::consts::PI/18.0}));sphere.moveme(&Vec3::get_ray_point(&sphere.position,&Vec3::minus(&Vec3::from_arr([0.0,0.0,-5.0]),&sphere.position),0.05));}
            objects::object_traits::Kind::LIGHT(light)=>{light.moveme(&mymath::functions::spin(&light.position, &mymath::vec3::Vec3::zero(), &mymath::mathstructs::Rotation{axis: mymath::mathstructs::RotationAxis::XY, theta: std::f32::consts::PI/18.0}))}
            objects::object_traits::Kind::PLANE(_plane) => panic!("Plane not yet implemented"),
            objects::object_traits::Kind::TRIANGLE(_triangle) => panic!("Triangle not yet implemented"),
        }
    }

    //DEFINING OBJECTS

        //SPHERE DEFINITIONS
        let mut spheres = [
            &mut Sphere {
                position: Vec3::from_arr([5.0, -5.0, -5.0]),
                radius: 2.0,
                texture: my_textures::RED,                   
            },

            &mut Sphere {
                position: Vec3::from_arr([-5.0, 5.0, -5.0]),
                radius: 2.0,
                texture: my_textures::BLUE,
            },
        ];

        //LIGHT DEFINTIONS
        let mut lights = [
            &mut objects::objects::Light {
                position: Vec3::from_arr([0.1, 100.0, 0.1]),
                intensity: 2.0,
            },

            &mut objects::objects::Light {
                position: Vec3::from_arr([100.0, -100.0, 0.1]),
                intensity: 2.0,
            },

            &mut objects::objects::Light {
                position: Vec3::from_arr([100.0, -100.0, -100.0]),
                intensity: 2.0,
            },

            &mut objects::objects::Light {
                position: Vec3::from_arr([-100.0, -100.0, -100.0]),
                intensity: 2.0,
            },

            &mut objects::objects::Light {
                position: Vec3::from_arr([0.1, 0.1, -5.0]),
                intensity: 2.0,
            }
        ];

   

    //PUSHING OBJECTS

        //PUSHING SPHERES
        spheres.iter_mut().for_each(|sphere| scene.push(
            render::AnimationObject {
                object: objects::object_traits::Object {
                object: objects::object_traits::Kind::SPHERE(
                    sphere
                )
            },
            tick: Some(spinmeten),
        }
        ));

        //PUSHING LIGHTS
        lights.iter_mut().for_each(|light| scene.push(
            render::AnimationObject {
                object: objects::object_traits::Object {
                object: objects::object_traits::Kind::LIGHT(
                    light
                )
            },
            tick: None,
        }
        ));

    //RENDERING SCENE
    let width = 1024;
    let height = 768;

    let img_factor = 1;

    scene.play(width*img_factor, height*img_factor, std::f32::consts::FRAC_PI_2, "circling");
    
    let elapsed = now.elapsed();
    println!("Time to render: {:.2?}", elapsed);
}