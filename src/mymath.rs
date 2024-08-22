pub mod functions {
    use crate::mymath::vec3::Vec3;
    use crate::mymath::mathstructs;

    pub fn fabs(n: f32) -> f32{
        if n>=0.0 {n}else{-n}
    }

    pub fn cap(n: f32, max: f32) -> f32{
        if n>max{return max;}
        n
    }

    pub fn fmax(a: f32, b: f32) -> f32{
        if a>b {a}else{b}
    }

    pub fn fmin(a: f32, b: f32) -> f32{
        if a>b {b}else{a}
    }

    pub fn spin(position: &Vec3, center: &Vec3, rotation: &mathstructs::Rotation) -> Vec3{
        use crate::mymath::mathstructs::*;
        let centeredvec = position.minus(center);
        let (cx, cy, cz) = (centeredvec.x, centeredvec.y, centeredvec.z);
        let (mut x, mut y, mut z) = (0.0 as f32, 0.0 as f32, 0.0 as f32);
        let axis = &rotation.axis;
        let cos = &rotation.theta.cos();let sin = &rotation.theta.sin();
        match axis {
            RotationAxis::XY => {
                x = cos*cx + sin*cy;
                y = cos*cy - sin*cx;
                z = cz;
            },
            RotationAxis::XZ => {
                x = cos*cx + sin*cz;
                y = cy;
                z = cos*cz - sin*cx;
            },
            RotationAxis::YZ => {
                x = cx;
                y = cos*cy + sin*cz;
                z = cos*cz - sin*cy;
            },
        }
        Vec3::add(
            &Vec3::from_arr([x, y, z]),
            &center
        )
    }
}

pub mod mathstructs {
    pub struct Coords {
        x: f32,
        y: f32,
    }

    pub enum RotationAxis {
        XY,
        XZ,
        YZ,
    }

    pub struct Rotation {
        pub axis: RotationAxis,
        pub theta: f32,
    }
}

pub mod vec3 {
    use crate::mymath::functions as Math;
    pub struct Vec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }impl Vec3{
    //creating vectors, with or without input
    pub fn zero() -> Vec3 {
        Vec3{x:0.0, y:0.0, z:0.0}
    }

    pub fn unit() -> Vec3 {
        Vec3::from_arr([1.0,1.0,1.0])
    }

    pub fn from_arr(arr: [f32;3]) -> Vec3 {
        let x=arr[0];let y=arr[1];let z = arr[2];
        Vec3{x:x,y:y,z:z}
    }

    //return non-parameterized altering of self
    pub fn copyme(&self) -> Vec3 {
        Vec3 {x:self.x, y:self.y, z:self.z}
    }

    pub fn abs(&self) -> Vec3 {
        Vec3 {
            x: Math::fabs(self.x),
            y: Math::fabs(self.y),
            z: Math::fabs(self.z),
        }
    }

    pub fn normalize(&self) -> Vec3 {
        let mag = self.get_magnitude();
        Vec3 {x: self.x/mag, y: self.y/mag, z: self.z/mag}
    }

    //returning non-vector aspect of self
    pub fn get_magnitude(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn max(&self) -> f32 {
        let mut max = self.to_array()[0];
        for i in self.to_array() {
            if i > max {
                max = i;
            }
        }
        max
    }

    pub fn getsubmag(&self) -> f32 {
        ((self.x * self.x) + (self.z * self.z)).sqrt()
    }

    //return self in another form
    pub fn to_array(&self) -> [f32;3] {
        [self.x, self.y, self.z]
    }

    pub fn to_color(&self) -> [u8;3] {
        let mut color: [u8;3] = [0,0,0];
        let selfarr = &self.to_array();
        for i in 0..3 {
            color[i] = (255.0 * Math::fmax(0.0, Math::fmin(1.0, selfarr[i]))) as u8
        }
        color
    }

    pub fn to_string(&self) -> String {
        let (x,y,z) = (self.x.to_string(), self.y.to_string(), self.z.to_string());
        x+" "+&y+" "+&z
    }

    //returned self with parameterized mathematical operations
    pub fn minus(&self, a: &Vec3) -> Vec3 {
        Vec3::add(&self, &a.multiply(&-1.0))
    }

    pub fn addme(&self, a: &Vec3) -> Vec3 {
        Vec3::add(&self, &a)
    }

    pub fn multiply(&self, scal: &f32) -> Vec3 {
        Vec3 {
            x: self.x * scal,
            y: self.y * scal,
            z: self.z * scal,
        }
    }

    //no self, performing vector-related operations
    pub fn add(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
    }

    pub fn distance(a: &Vec3, b: &Vec3) -> f32{
        a.minus(b).get_magnitude()
    }

    pub fn dot_product(a: &Vec3, b: &Vec3) -> f32 {
        let mut sum = 0.0;
        for c in [0,1,2] {
            sum += a.to_array()[c] * b.to_array()[c];
        }
        sum
    }

    pub fn get_ray_point(a: &Vec3, b: &Vec3, t: f32) -> Vec3 {
        Vec3::add(a,&b.multiply(&t))
    }

    pub fn reflect(i: &Vec3, n: &Vec3) -> Vec3{
        i.minus(&n.multiply(&2.0).multiply(&Vec3::dot_product(i, n)))
    }}
}
