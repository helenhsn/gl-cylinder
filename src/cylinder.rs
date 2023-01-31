use gl::VertexP2ui;

type vtx = [f32; 3];

#[derive(Debug)]
pub struct Vertex {
    vertex: vtx,
    normal: vtx,
}

impl Vertex {
    pub fn new(v: vtx, n: vtx) -> Self {
        Self {
            vertex:v,
            normal:n,
        }
    }
}

pub struct Cylinder {
    vertices: Vec<Vertex>,
    indices: Vec<[i32; 3]>,
}

impl Cylinder {

    pub fn get_indices(&self) -> &Vec<[i32; 3]> {
        &self.indices
    }

    pub fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }


    fn get_unit_circle_vertices(nb_slices: usize) -> (Vec<f32>, Vec<f32>) {
        let PI = 3.14159265359;
        let step_th = 2.*PI/ (nb_slices as f32);
        let mut current_angle;

        let mut cosines = Vec::new();
        let mut sines = Vec::new();

        for i in 0..=nb_slices {
            current_angle = (i as f32) * step_th;
            cosines.push(current_angle.cos());
            sines.push(current_angle.sin());
        }
        (cosines, sines)
    }

    pub fn new(nb_slices: usize, height:f32, radius: f32) -> Self {

        let mut vertices = Vec::new();
        let mut indices:Vec<[i32;3]> = Vec::new();
        
        let (cosines, sines) = Self::get_unit_circle_vertices(nb_slices);
        let mut current_point;
        let mut current_normal;
        let mut index;
        let mut h;

        for j in 0..nb_slices {

            index = 2*j;
            indices.push([index.try_into().unwrap(), (index+1).try_into().unwrap(), ((index+2)%(2*nb_slices)).try_into().unwrap()]);
            indices.push([((index+2)%(2*nb_slices)).try_into().unwrap(), (index+1).try_into().unwrap(), ((index+3)%(2*nb_slices)).try_into().unwrap()] as [i32;3]);

            for i in 0..2 {
                // adding current vertex to the vect of vertices
                h = height/2.0 - (i as f32)*height;
                current_point = [cosines[j]*radius, h, sines[j]*radius];
                current_normal = [cosines[j], 0., sines[j]];
                vertices.push(Vertex::new(current_point, current_normal));
            }
        }

        let nb_side_vertices = vertices.len();

        // adding top & bottom center vertices and update of indices
        let top_index = nb_side_vertices + nb_slices*2;
        let bottom_index = top_index + 1;

        for j in 0..nb_slices {

            if j < nb_slices {
                // handling indices
            index = j*2 + nb_side_vertices;
            indices.push([(index+2).try_into().unwrap(), index.try_into().unwrap(), top_index.try_into().unwrap()]);
            indices.push([(index+3).try_into().unwrap(), (index+1).try_into().unwrap(), bottom_index.try_into().unwrap()]);
            }
            
            for i in 0..2 {
                h = height/2.0 - (i as f32)*height;
                current_point = [cosines[j]*radius, h, sines[j]*radius];
                current_normal = [0., 1.0 - 2.0 * (i as f32), 0.];
                vertices.push(Vertex::new(current_point, current_normal));
            }
        }

        // last triangles which need to be treated separately
        let last_index = 2*(nb_slices-1)+nb_side_vertices;
        indices.push([(nb_side_vertices).try_into().unwrap(), (last_index).try_into().unwrap(), top_index.try_into().unwrap()]);
        indices.push([(nb_side_vertices+1).try_into().unwrap(), (last_index+1).try_into().unwrap(), bottom_index.try_into().unwrap()]);

        vertices.push(Vertex::new([0., height/2., 0.], [0., 1., 0.]));
        vertices.push(Vertex::new([0., -height/2., 0.], [0., -1., 0.]));

        Self {
            vertices: vertices,
            indices: indices,
        }

    }
}