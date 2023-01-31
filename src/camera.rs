use cgmath::{Vector3, Vector2, Matrix2, Matrix4, Point3, InnerSpace, EuclideanSpace, VectorSpace};
use libm::{cos, sin};

pub enum Direction {
    FWD,
    BWD,
    LEFT,
    RIGHT,
}

pub struct Camera {
    ro: Vector3<f32>,
    rd: Vector3<f32>,
    up: Vector3<f32>,
    fwd: Vector3<f32>,
    rgt: Vector3<f32>,
    up_loc: Vector3<f32>, // world up
    camera_speed: f32,
    mouse_speed: f32,
    zoom: f32
}

fn rotate(th: f64) -> Matrix2<f32> {
    Matrix2::new(cos(th) as f32, 
    sin(th) as f32, 
    -sin(th) as f32, 
    cos(th) as f32)
}


impl Camera {

    pub fn new(
        cpos: Vector3<f32>, 
        cdir: Vector3<f32>, 
        cspeed: f32, 
        mspeed: f32, 
        z: f32) -> Self {

        let u: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
        let f: Vector3<f32> = InnerSpace::normalize(cdir - cpos);
        let r: Vector3<f32> = InnerSpace::normalize(Vector3::cross(f, u));
        let uploc: Vector3<f32> = Vector3::cross(f, r);
        
        
        Self{
            up: u,
            ro: cpos, 
            rd: cdir,
            fwd: f,
            rgt: r,
            up_loc: uploc,
            camera_speed: cspeed,
            mouse_speed: mspeed,
            zoom: z,
        }
    }

    pub fn process_keyboard(&mut self, dir: Direction, d_time: f32) {
        let velocity: f32 = self.camera_speed * d_time;
        self.ro += match dir {
            Direction::FWD => self.fwd*velocity,
            Direction::BWD => -self.fwd*velocity,
            Direction::LEFT => -self.rgt*velocity,
            Direction::RIGHT => self.rgt*velocity, 
        };
        self.update_system();
    }

    pub fn process_mouse(&mut self, x_offset: f32, y_offset: f32) {
        let PI = 3.14159265359;

        let new_yz = rotate((-y_offset*PI).into()) * Vector2::new(self.ro.y, self.ro.z);
        self.ro.y = new_yz.x;
        self.ro.z = new_yz.y;

        let new_xz = rotate((-x_offset*PI*2.0).into()) * Vector2::new(self.ro.x, self.ro.z);
        self.ro.x = new_xz.x;
        self.ro.z = new_xz.y;
    }   

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(Point3::from_vec(self.ro), Point3::from_vec(self.rd - self.ro), self.up)
    }

    pub fn get_origin(&self) -> Vector3<f32> {
        self.ro
    }

    fn update_system(&mut self) {
        self.fwd = InnerSpace::normalize(self.rd - self.ro);
        self.rgt = InnerSpace::normalize(Vector3::cross(self.fwd, self.up));
        self.up_loc = Vector3::cross(self.fwd, self.rgt);
    }

}
