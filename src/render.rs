use crate::objects;
use crate::mymath;
    use mymath::vec3::Vec3;
    use mymath::mathstructs::*;
    use mymath::functions::*;
    use objects::*;
use image::{RgbImage, ImageBuffer, Rgb};

pub struct AnimationObject<'a> {
    pub tick: Option<fn(object: &mut objects::object_traits::Object<'a>)>,
    pub object: objects::object_traits::Object<'a>,
}

pub struct Scene<'a> {
    pub animationobjects: Vec<AnimationObject<'a>>,
    pub background_texture: object_structs::Texture,
    pub max_depth: usize,
    pub frames: usize,
} impl<'a> Scene<'a> {
    pub fn render_frame(&self, image_width: u32, image_height: u32, fov: f32, name: String){
        let mut buffer: RgbImage = ImageBuffer::new(image_width, image_height);

        for (x, y, pixel) in buffer.enumerate_pixels_mut(){
            let x0: f32 = (2.0*((x as f32)+0.5)/(image_width as f32)-1.0) * (fov/2.0).tan() * (image_width as f32)/(image_height as f32);
            let y0: f32 = -(2.0 * ((y as f32) + 0.5)/(image_height as f32) - 1.0) * (fov/2.0).tan();
            let dir = (Vec3 {x:x0, y:y0, z:-1.0}).normalize();
            let color = self.scene_cast(&Vec3{x:0.0,y:0.0,z:0.0}, &dir, 0);
            *pixel = Rgb(color.to_color());
        }

        buffer.save("./output/".to_owned()+&name+".png").unwrap();
    }

    pub fn advance(&mut self) {
        self.frames -=1;
        for animationobject in &mut self.animationobjects {
            match(animationobject.tick){
                Some(tick) => {
                    tick(&mut animationobject.object);
                }None=>{}
            }
        }
    }

    pub fn play(&mut self, image_width: u32, image_height: u32, fov: f32, name: &str) {
        let startframes = self.frames;
        while self.frames > 0 {
            self.render_frame(image_width, image_height, fov, name.to_owned()+&self.frames.to_string());
            self.advance();
            let message = ("frame ".to_owned()+&(startframes-self.frames).to_string()+"/"+&startframes.to_string()+" complete");
            println!("{}",message);
        }
    }

    fn moveme(&mut self, vec: Vec3) {
        for animationobject in &mut self.animationobjects {
            match &mut animationobject.object.object {
                object_traits::Kind::SPHERE(sphere) => sphere.moveme(&vec),
                object_traits::Kind::LIGHT(light) => light.moveme(&vec),
                object_traits::Kind::PLANE(_plane) => panic!("Plane not yet implemented"),
                object_traits::Kind::TRIANGLE(_triangle) => panic!("Triangle not yet implemented"),
            }
        }
    }

    fn spinme(&mut self, center: &Vec3, rotation: &Rotation) {
        for animationobject in &mut self.animationobjects {
            match &mut animationobject.object.object {
                object_traits::Kind::SPHERE(sphere) => sphere.moveme(&spin(sphere.getposition(), center, rotation)),
                object_traits::Kind::LIGHT(light) => light.moveme(&spin(light.getposition(), center, rotation)),
                object_traits::Kind::PLANE(_plane) => panic!("Plane not yet implemented"),
                object_traits::Kind::TRIANGLE(_triangle) => panic!("Triangle not yet implemented"),
            }
        }
    }

    pub fn scene_cast(&self, orig: &Vec3, dirin: &Vec3, depth: usize) -> Vec3 {
        if depth>self.max_depth {
            match self.background_texture.sphereimpl {
                None=>{panic!("No background texture found");},
                Some(sphereimpl)=>{return sphereimpl(dirin).color}
            }
        }
        let mut min: (f32, Option<&object_traits::Object>) = (f32::MAX, None);
        let mut dis: f32;
        for animationobject in &self.animationobjects {
            match &animationobject.object.object {
                object_traits::Kind::SPHERE(sphere) => {
                    match sphere.intersect_point(orig, dirin) {
                        None=>{},
                        Some(points) => {
                            for point in points {
                                dis = Vec3::distance(orig, &point);
                                if dis < min.0 {
                                    min = (dis, Some(&animationobject.object));
                                }
                            }
                        }
                }}
                object_traits::Kind::LIGHT(_light) => {},
                object_traits::Kind::PLANE(_plane) => {panic!("Plane not implemented");},
                object_traits::Kind::TRIANGLE(_triangle) => {panic!("Triangle not implemented");}
            }
        }match min.1 {
            None=>{
                match self.background_texture.sphereimpl {
                    None=>{panic!("No background texture found");},
                    Some(sphereimpl)=>{sphereimpl(dirin).color}
                }
            },
            Some(object) => {
                match &object.object {
                    object_traits::Kind::SPHERE(sphere) => {match sphere.intersect_color(orig, dirin, depth, self) {
                        None => {panic!("Critical error in rendering: expected scene object not found")}
                        Some(point_colors) => {
                            if Vec3::distance(&point_colors[0].point,orig)>Vec3::distance(&point_colors[1].point,orig){
                                point_colors[1].color.copyme()
                            }else{point_colors[0].color.copyme()}
                        }
                    }}
                    object_traits::Kind::LIGHT(_light) => {panic!("Critical error: light in color renderer");},
                    object_traits::Kind::PLANE(_plane) => {panic!("Plane not implemented");},
                    object_traits::Kind::TRIANGLE(_triangle) => {panic!("Triangle not implemented");}
                }
            }
        }
    }

    pub fn does_intersect(&self, orig: &Vec3, dirin: &Vec3) -> bool {
        for animationobject in &self.animationobjects {
            match &animationobject.object.object {
                object_traits::Kind::SPHERE(sphere) => {if sphere.does_intersect(orig, dirin){return true}}
                object_traits::Kind::LIGHT(_light) => {},
                object_traits::Kind::PLANE(_plane) => {panic!("Plane not implemented");},
                object_traits::Kind::TRIANGLE(_triangle) => {panic!("Triangle not implemented");}
            }
        }false
    }
    
    pub fn push(&mut self, animationobject: AnimationObject<'a>) {
        self.animationobjects.push(animationobject);
    }
}