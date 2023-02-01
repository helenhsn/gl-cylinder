use cgmath::{
    Angle, Deg, EuclideanSpace, InnerSpace, Matrix2, Matrix4, Point3, Rad, Vector2, Vector3,
    VectorSpace,
};

pub enum Direction {
    FWD,
    BWD,
    LEFT,
    RIGHT,
}

pub struct Camera {
    ro: Vector3<f32>,
    up: Vector3<f32>,
    fwd: Vector3<f32>,
    rgt: Vector3<f32>,
    up_loc: Vector3<f32>, // world up

    // euler angles
    yaw: Deg<f32>,
    pitch: Deg<f32>,

    camera_speed: f32,
    mouse_speed: f32,
    zoom: Deg<f32>,
}

impl Camera {
    pub fn new(
        cpos: Vector3<f32>,
        y: f32,
        p: f32,
        cspeed: f32,
        mspeed: f32,
        z: f32,
    ) -> Self {

        let mut camera = Self {
            ro: cpos,
            up: Vector3::new(0.0, 1.0, 0.0),
            fwd: Vector3::new(0.0, 0.0, -1.0),
            yaw: Deg(y),
            pitch: Deg(p),
            rgt: Vector3::new(0.0, 0.0, 0.0),
            up_loc: Vector3::new(0.0, 1.0, 0.0),
            camera_speed: cspeed,
            mouse_speed: mspeed,
            zoom: Deg(z),
        };
        camera.update_system();
        camera
    }

    pub fn process_keyboard(&mut self, dir: Direction, d_time: f32) {
        let velocity: f32 = self.camera_speed * d_time;
        println!("fwd = {:?} && rgt = {:?}", self.fwd, self.rgt);
        println!("ro = {:?}", self.ro);
        self.ro += match dir {
            Direction::FWD => self.fwd * velocity,
            Direction::BWD => -self.fwd * velocity,
            Direction::LEFT => -self.rgt * velocity,
            Direction::RIGHT => self.rgt * velocity,
        };
    }

    pub fn process_mouse(&mut self, mut offset: Vector2<f64>) {
        
        offset[0] *= self.mouse_speed as f64;
        offset[1] *= self.mouse_speed as f64;

        self.yaw += Deg(offset[0] as f32);
        self.pitch += Deg(offset[1] as f32);

        // constrain for pitch
        self.pitch = Deg(match self.pitch {
            Deg(angle) if angle < -89. => -89.,
            Deg(angle) if angle > 89. => 89.,
            Deg(angle) => angle,
        });

        self.update_system();
    }

    pub fn process_scroll(&mut self, offset: f64) {
        self.zoom -= Deg(offset as f32);
        if self.zoom < Deg(1.0) {self.zoom = Deg(1.0)};
        if self.zoom > Deg(45.0) {self.zoom = Deg(45.0)}; 

    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(
            Point3::from_vec(self.ro),
            Point3::from_vec(self.ro + self.fwd),
            self.up,
        )
    }

    pub fn get_origin(&self) -> Vector3<f32> {
        self.ro
    }

    pub fn get_zoom(&self) -> Deg<f32> {
        self.zoom
    }

    fn update_system(&mut self) {
        let pitch_cos = self.pitch.cos();
        let rd = Vector3 {
            x: pitch_cos * self.yaw.cos(),
            y: self.pitch.sin(),
            z: -pitch_cos * self.yaw.sin(),
        };
        self.fwd = InnerSpace::normalize(rd);
        self.rgt = InnerSpace::normalize(Vector3::cross(self.fwd, self.up_loc));
        self.up = InnerSpace::normalize(Vector3::cross(self.rgt, self.fwd));
    }
}
