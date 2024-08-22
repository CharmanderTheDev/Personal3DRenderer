pub mod object_structs {
    use crate::mymath;
    use mymath::vec3::Vec3;

    pub static mut ONID: isize = 0;

    pub struct Material {
        pub color: Vec3,
        pub albedo: (f32, f32, f32),
        pub specular_exponet: f32
    }

    pub struct Texture {
        pub sphereimpl: Option<fn(&Vec3)->Material>,
        pub planeimpl: Option<fn(&mymath::mathstructs::Coords)->Material>,
    }

    pub struct PointColor {
        pub point: Vec3,
        pub color: Vec3,
    }
}

pub mod object_traits {
    use crate::objects::objects::*;

    pub enum Kind<'a> {
        SPHERE(&'a mut Sphere),
        PLANE(&'a mut Plane),
        TRIANGLE(&'a mut Triangle),
        LIGHT(&'a mut Light),
    }pub struct Object<'a> {
        pub object: Kind<'a>,
    }
}

pub mod my_textures {
    use crate::objects;
        use objects::object_structs::*;

    use crate::mymath;
        use mymath::vec3::Vec3;
        use mymath::mathstructs::Coords;

    pub const RAINBOW_MIRROR: Texture = Texture {
        sphereimpl: Some(|dir: &Vec3|->Material {        
            Material {
                color: dir.copyme().abs(),
                albedo: (0.25, 10.0, 0.8),
                specular_exponet: 1425.0,
        }
    }),
        planeimpl: None
    };

    pub const BORING_TEXTURE: Texture = Texture {
        sphereimpl: Some(|_dir: &Vec3|->Material {        
            Material {
                color: Vec3::unit(),
                albedo: (1.0, 1.0, 1.0),
                specular_exponet: 1.0,
        }
    }),
        planeimpl: Some(|_Coords: &mymath::mathstructs::Coords| -> Material {
            Material {
                color: Vec3::unit(),
                albedo: (1.0, 1.0, 1.0),
                specular_exponet: 1.0,
        }
        })
    };

    pub const BLACK_MIRROR: Texture = Texture {
        sphereimpl: Some(|_dir: &Vec3|->Material {        
            Material {
                color: Vec3::zero(),
                albedo: (0.25, 5.0, 0.8),
                specular_exponet: 1425.0,
        }
    }),
        planeimpl: Some(|_Coords: &mymath::mathstructs::Coords| -> Material {
            Material {
                color: Vec3::zero(),
                albedo: (0.25, 10.0, 0.8),
                specular_exponet: 1425.0,
        }
        })
    };

    pub const PURPLE: Texture = Texture {
        sphereimpl: Some(|_dir: &Vec3|->Material {        
            Material {
                color: Vec3::from_arr([1.0, 0.0, 1.0]),
                albedo: (0.1, 0.1, 0.1),
                specular_exponet: 1.0,
        }
    }),
        planeimpl: Some(|_Coords: &mymath::mathstructs::Coords| -> Material {
            Material {
                color: Vec3::from_arr([1.0, 0.0, 1.0]),
                albedo: (0.1, 0.1, 0.1),
                specular_exponet: 1.0,
        }
        })
    };

    pub const RED: Texture = Texture {
        sphereimpl: Some(|_dir: &Vec3|->Material {        
            Material {
                color: Vec3::from_arr([1.0, 0.0, 0.0]),
                albedo: (0.1, 0.0, 0.1),
                specular_exponet: 0.0,
        }
    }),
        planeimpl: Some(|_Coords: &mymath::mathstructs::Coords| -> Material {
            Material {
                color: Vec3::from_arr([1.0, 0.0, 0.0]),
                albedo: (0.1, 0.0, 0.1),
                specular_exponet:0.0,
        }
        })
    };

    pub const BLUE: Texture = Texture {
        sphereimpl: Some(|_dir: &Vec3|->Material {        
            Material {
                color: Vec3::from_arr([0.0, 0.0, 1.0]),
                albedo: (0.1, 0.1, 0.1),
                specular_exponet: 1.0,
        }
    }),
        planeimpl: Some(|_Coords: &mymath::mathstructs::Coords| -> Material {
            Material {
                color: Vec3::from_arr([0.0, 0.0, 1.0]),
                albedo: (0.1, 0.1, 0.1),
                specular_exponet: 1.0,
        }
        })
    };
}


pub mod objects {
    use crate::objects;
        use objects::object_structs::*;

    use crate::mymath;
        use mymath::vec3::Vec3;
        use mymath::functions::*;
    
    use crate::render;
        use render::Scene;
    pub struct Light {
        pub position: Vec3,
        pub intensity: f32,
    }impl Light {
        pub fn getposition(&self)->&Vec3{&self.position}
        pub fn moveme(&mut self, position: &Vec3){self.position = position.copyme();}
        pub fn pushme(&mut self, vector: &Vec3){self.position = self.position.addme(vector);}
    }

    pub struct Sphere {
        pub position: Vec3,
        pub radius: f32,
        pub texture: Texture,
    }impl Sphere {
        pub fn getposition(&self)->&Vec3{&self.position}

        pub fn moveme(&mut self, position: &Vec3){self.position = position.copyme();}

        pub fn pushme(&mut self, vector: &Vec3){self.position = self.position.addme(vector);}

        fn gettexture(&self)->&Texture{&self.texture}

        pub fn does_intersect(&self, orig: &Vec3, dirin: &Vec3) -> bool {
            let dir = &dirin.normalize();
            let l = &self.position.minus(&orig);
            let tc = Vec3::dot_product(&l, &dir);
    
            if tc<0.0{false}else{
            let lmag = l.get_magnitude();
            let d = ((tc * tc) - (lmag * lmag)).abs().sqrt();
    
            if d > self.radius{return false}true
        }}

        pub fn intersect_point(&self, orig: &Vec3, dirin: &Vec3) -> Option<[Vec3;2]> {
            let dir = &dirin.normalize();
            let l = &self.position.minus(&orig);
            let tc = Vec3::dot_product(&l, &dir);
    
            if tc<0.0{return None;}else{
            let lmag = l.get_magnitude();
            let d = ((tc * tc) - (lmag * lmag)).abs().sqrt();
    
            if d > self.radius{return None;}else{
            let t1c = ((self.radius * self.radius) - (d * d)).sqrt();
            
            let t1 = tc - t1c;
            let t2 = tc + t1c;
    
            let p1 = Vec3::get_ray_point(orig, dir, t1);
            let p2 = Vec3::get_ray_point(orig, dir, t2);
            Some([p1,p2])
        }}}

        pub fn intersect_color(&self, orig: &Vec3, dirin: &Vec3, depth: usize, scene: &Scene) -> Option<[PointColor;2]> {
            let dir = &dirin.normalize();
            let l = &self.position.minus(&orig);
            let tc = Vec3::dot_product(&l, &dir);
    
            if tc<0.0{return None}else{
            let lmag = l.get_magnitude();
            let d = ((tc * tc) - (lmag * lmag)).abs().sqrt();
    
            if d > self.radius{return None}else{
            let t1c = ((self.radius * self.radius) - (d * d)).sqrt();
            
            let t1 = tc - t1c;
            let t2 = tc + t1c;
    
            let p1 = Vec3::get_ray_point(orig, dir, t1);
            let p2 = Vec3::get_ray_point(orig, dir, t2);
            let points = [p1,p2];
    
    
            let mut diffuses = [0.0,0.0];
            let mut speculars = [0.0,0.0];
            let mut reflectcolors = [Vec3::unit(), Vec3::unit()];
            let texture = match self.texture.sphereimpl {
                Some(texturefunc) => texturefunc,
                None => panic!("Sphere was missing texture"),
            };
            let mut lights = Vec::new();
            for object in &scene.animationobjects{match &object.object.object{objects::object_traits::Kind::LIGHT(light)=>{lights.push(light);}_=>{}}}
    
            let mut mats = [texture(&points[0]),texture(&points[1])];
            
    
            for p in 0..2 {'lights: for i in 0..lights.len() {
    
                //preliminary calculations
                let light_dir = (lights[i].position.minus(&points[p])).normalize();
                let n = &points[p].minus(&self.position).normalize();
                mats[p] = (texture)(n);
                //shadow checker
                let shadow_orig = if Vec3::dot_product(&light_dir, n)<0.0{&points[p].minus(&n.multiply(&0.001))}else{&Vec3::add(&points[p], &n.multiply(&0.001))};
                if scene.does_intersect(&shadow_orig, &lights[i].position.minus(&shadow_orig).normalize()){continue 'lights;}
    
                //diffuse math
                diffuses[p] += lights[i].intensity * fmax(0.0, Vec3::dot_product(&light_dir, n));
    
                //specular math
                speculars[p] += fmax(0.0, Vec3::dot_product(&Vec3::reflect(&light_dir, n),dir)).powf(mats[p].specular_exponet) * lights[i].intensity;
    
                //reflection math
                let reflect_dir = Vec3::reflect(dir, n);
                let reflect_orig = if Vec3::dot_product(&light_dir, n)<0.0{&points[p].minus(&n.multiply(&0.001))}else{&points[p].addme(&n.multiply(&0.001))};
                reflectcolors[p] = scene.scene_cast(reflect_orig, &reflect_dir, depth + 1);
            }}
    
            let mut color1 = mats[0].color.copyme();
            color1 = color1.multiply(&diffuses[0]).multiply(&mats[0].albedo.0);
            color1 = color1.addme(&Vec3::unit().multiply(&speculars[0]).multiply(&mats[0].albedo.1));
            color1 = color1.addme(&reflectcolors[0].multiply(&mats[0].albedo.2));
    
            let pointcolor1 = PointColor {
                point: points[0].copyme(),
                color: color1
            };
    
            let mut color2 = mats[0].color.copyme();
            color2 = color2.multiply(&diffuses[1]).multiply(&mats[1].albedo.0);
            color2 = color2.addme(&Vec3::unit().multiply(&speculars[1]).multiply(&mats[1].albedo.1));
            color2 = color2.addme(&reflectcolors[1].multiply(&mats[1].albedo.2));
    
            let pointcolor2 = PointColor {
                point: points[1].copyme(),
                color: color2
            };
            return Some([pointcolor1, pointcolor2])
            }
            }
        }
    }

    pub struct Plane {

    }impl Plane{

    }

    pub struct Triangle {

    }impl Triangle{

    }
}

