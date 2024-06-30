use std::collections::HashMap;

use nalgebra::{Matrix4, Vector3, Vector4};
use std::cmp::min;
use crate::triangle::Triangle;

#[allow(dead_code)]
pub enum Buffer {
    Color,
    Depth,
    Both,
}

#[allow(dead_code)]
pub enum Primitive {
    Line,
    Triangle,
}

#[derive(Default, Clone)]
pub struct Rasterizer {
    model: Matrix4<f64>,
    view: Matrix4<f64>,
    projection: Matrix4<f64>,
    pos_buf: HashMap<usize, Vec<Vector3<f64>>>,
    ind_buf: HashMap<usize, Vec<Vector3<usize>>>,
    col_buf: HashMap<usize, Vec<Vector3<f64>>>,

    frame_buf: Vec<Vector3<f64>>,
    depth_buf: Vec<f64>,
    /*  You may need to uncomment here to implement the MSAA method  */
    frame_sample: Vec<Vector3<f64>>,
    depth_sample: Vec<f64>,
    width: u64,
    height: u64,
    next_id: usize,
    MSAA: u64,
}

#[derive(Clone, Copy)]
pub struct PosBufId(usize);

#[derive(Clone, Copy)]
pub struct IndBufId(usize);

#[derive(Clone, Copy)]
pub struct ColBufId(usize);

impl Rasterizer {
    pub fn new(w: u64, h: u64) -> Self {
        let mut r = Rasterizer::default();
        // in every block, we uniformly sample MSAA * MSAA points
        r.MSAA = 5;
        r.width = w;
        r.height = h;
        r.frame_buf.resize((w * h) as usize, Vector3::zeros());
        r.depth_buf.resize((w * h) as usize, 0.0);
        r.frame_sample.resize((w * h * r.MSAA * r.MSAA) as usize, Vector3::zeros());
        r.depth_sample.resize((w * h * r.MSAA * r.MSAA) as usize, 0.0);
        r
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        ((self.height - 1 - y as u64) * self.width + x as u64) as usize
    }

    fn set_pixel(&mut self, point: &Vector3<f64>, color: &Vector3<f64>) {
        let ind = self.get_index(point.x as usize, point.y as usize);
        self.frame_buf[ind as usize] = *color;
        self.depth_buf[ind as usize] = point.z;
    }

    pub fn clear(&mut self, buff: Buffer) {
        match buff {
            Buffer::Color => {
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                self.frame_sample.fill(Vector3::new(0.0, 0.0, 0.0));
            }
            Buffer::Depth => {
                self.depth_buf.fill(f64::MAX);
                self.depth_sample.fill(f64::MAX);
            }
            Buffer::Both => {
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                self.frame_sample.fill(Vector3::new(0.0, 0.0, 0.0));
                self.depth_buf.fill(f64::MAX);
                self.depth_sample.fill(f64::MAX);
            }
        }
    }

    pub fn set_model(&mut self, model: Matrix4<f64>) {
        self.model = model;
    }

    pub fn set_view(&mut self, view: Matrix4<f64>) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: Matrix4<f64>) {
        self.projection = projection;
    }

    fn get_next_id(&mut self) -> usize {
        let res = self.next_id;
        self.next_id += 1;
        res
    }

    pub fn load_position(&mut self, positions: &Vec<Vector3<f64>>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions.clone());
        PosBufId(id)
    }

    pub fn load_indices(&mut self, indices: &Vec<Vector3<usize>>) -> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices.clone());
        IndBufId(id)
    }

    pub fn load_colors(&mut self, colors: &Vec<Vector3<f64>>) -> ColBufId {
        let id = self.get_next_id();
        self.col_buf.insert(id, colors.clone());
        ColBufId(id)
    }

    pub fn draw(&mut self, pos_buffer: PosBufId, ind_buffer: IndBufId, col_buffer: ColBufId, _typ: Primitive) {
        let buf = &self.clone().pos_buf[&pos_buffer.0];
        let ind: &Vec<Vector3<usize>> = &self.clone().ind_buf[&ind_buffer.0];
        let col = &self.clone().col_buf[&col_buffer.0];

        let f1 = (50.0 - 0.1) / 2.0;
        let f2 = (50.0 + 0.1) / 2.0;

        let mvp = self.projection * self.view * self.model;

        for i in ind {
            let mut t = Triangle::new();
            let mut v =
                vec![mvp * to_vec4(buf[i[0]], Some(1.0)), // homogeneous coordinates
                     mvp * to_vec4(buf[i[1]], Some(1.0)), 
                     mvp * to_vec4(buf[i[2]], Some(1.0))];
    
            for vec in v.iter_mut() {
                *vec = *vec / vec.w;
            }
            for vert in v.iter_mut() {
                vert.x = 0.5 * self.width as f64 * (vert.x + 1.0);
                vert.y = 0.5 * self.height as f64 * (vert.y + 1.0);
                vert.z = vert.z * f1 + f2;
            }
            for j in 0..3 {
                // t.set_vertex(j, Vector3::new(v[j].x, v[j].y, v[j].z));
                t.set_vertex(j, v[j]);
                t.set_vertex(j, v[j]);
                t.set_vertex(j, v[j]);
            }
            let col_x = col[i[0]];
            let col_y = col[i[1]];
            let col_z = col[i[2]];
            t.set_color(0, col_x[0], col_x[1], col_x[2]);
            t.set_color(1, col_y[0], col_y[1], col_y[2]);
            t.set_color(2, col_z[0], col_z[1], col_z[2]);

            self.rasterize_triangle(&t);
        }
    }

    pub fn rasterize_triangle(&mut self, t: &Triangle) {
        //count the time as requested
        println!("Current time before rasterizing : {:?}", std::time::Instant::now());

        //we sample 3*3 points in this square
        let sample_num = self.MSAA;
        let interval : f64 = 1.0 / sample_num as f64;
        /*  implement your code here  */
        for x in 0..self.width {
            for y in 0..self.height {
                let x : f64 = x as f64;
                let y : f64 = y as f64;
                let org_ind : usize = self.get_index(x as usize, y as usize);
                //convert 4d vector into 3d vector
                let vec3 = [Vector3::new(t.v[0].x, t.v[0].y, t.v[0].z),
                         Vector3::new(t.v[1].x, t.v[1].y, t.v[1].z),
                         Vector3::new(t.v[2].x, t.v[2].y, t.v[2].z)];
                let mut dx : f64 = interval / 2.0;
                let mut ind_offset : usize = 0;
                for i in 0..sample_num {
                    let mut dy : f64 = interval / 2.0;
                    for j in 0..sample_num {
                        let nowx: f64 = x + dx;
                        let nowy: f64 = y + dy;
                        let ind : usize = org_ind * sample_num as usize * sample_num as usize + ind_offset;
                        if (inside_triangle(nowx as f64, nowy as f64, &vec3)) {
                            // let real_z = compute_zvalue(nowx, nowy, &vec3);
                            let real_z = t.v[0].z;
                            if(real_z < self.depth_sample[ind as usize]){
                                self.depth_sample[ind as usize] = real_z;
                                self.depth_buf[org_ind as usize] = real_z.min(self.depth_buf[org_ind as usize]);
                                self.frame_sample[ind as usize] = t.get_color() / (sample_num as f64 * sample_num as f64);
                            }
                        }
                        dy += interval;
                        ind_offset += 1;
                    }
                    dx += interval;
                }
                let mut all_color = Vector3::new(0.0, 0.0, 0.0);
                for i in 0..(sample_num * sample_num) {
                    let ind : usize = org_ind * sample_num as usize * sample_num as usize + i as usize;
                    all_color += self.frame_sample[ind as usize];
                }
                self.set_pixel(&Vector3::new(x as f64, y as f64, 0.0), &all_color);
            }
        }

        //count the time as requested
        println!("Current time after rasterizing : {:?}", std::time::Instant::now());

    }

    pub fn frame_buffer(&self) -> &Vec<Vector3<f64>> {
        &self.frame_buf
    }
}

fn to_vec4(v3: Vector3<f64>, w: Option<f64>) -> Vector4<f64> {
    Vector4::new(v3.x, v3.y, v3.z, w.unwrap_or(1.0))
}

fn inside_triangle(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> bool {
    /*  implement your code here  */
    //calculate the cross product
    let v0 = v[0];
    let v1 = v[1];
    let v2 = v[2];
    let c0 = (v1.x - v0.x) * (y - v0.y) - (v1.y - v0.y) * (x - v0.x);
    let c1 = (v2.x - v1.x) * (y - v1.y) - (v2.y - v1.y) * (x - v1.x);
    let c2 = (v0.x - v2.x) * (y - v2.y) - (v0.y - v2.y) * (x - v2.x);
    if c0 >= 0.0 && c1 >= 0.0 && c2 >= 0.0 {
        return true
    }
    if c0 <= 0.0 && c1 <= 0.0 && c2 <= 0.0 {
        return true
    }
    false
}

fn compute_zvalue(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> f64 {
    //compute the interpolated zvalue of a point in the triangle
    let b: Vector3<f64> = v[1] - v[0];
    let c: Vector3<f64> = v[2] - v[0];
    let det: f64 = b.x * c.y - b.y * c.x;
    let beta: f64 = (x * c.y - c.x * y) / det;
    let gamma: f64 = (b.x * y - x * b.y) / det;
    beta * b.z + gamma * c.z
}

fn compute_barycentric2d(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> (f64, f64, f64) {
    let c1 = (x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * y + v[1].x * v[2].y - v[2].x * v[1].y)
        / (v[0].x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * v[0].y + v[1].x * v[2].y - v[2].x * v[1].y);
    let c2 = (x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * y + v[2].x * v[0].y - v[0].x * v[2].y)
        / (v[1].x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * v[1].y + v[2].x * v[0].y - v[0].x * v[2].y);
    let c3 = (x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * y + v[0].x * v[1].y - v[1].x * v[0].y)
        / (v[2].x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * v[2].y + v[0].x * v[1].y - v[1].x * v[0].y);
    (c1, c2, c3)
}