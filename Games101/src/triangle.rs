// from 2023 Lab 2 可以兼容覆盖所有Lab使用的函数

#![allow(dead_code)]

use std::process::exit;
use nalgebra::{Vector2, Vector3, Vector4};

#[derive(Default, Clone, Debug)]
pub struct Triangle {
    pub v: [Vector4<f64>; 3],
    pub color: [Vector3<f64>; 3],
    pub tex_coords: [Vector2<f64>; 3],
    pub normal: [Vector3<f64>; 3],
}

impl Triangle {
    pub(crate) fn new() -> Self {
        let v4: Vector4<f64> = Vector4::new(0.0, 0.0, 0.0,0.0);
        let v3: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
        Triangle {
            v: [v4; 3],
            color: [v3; 3],
            tex_coords: [Vector2::new(0.0, 0.0); 3],
            normal: [v3; 3],
        }
    }
    pub fn set_vertex(&mut self, ind: usize, ver: Vector4<f64>) {
        self.v[ind] = ver;
    }
    pub fn set_normal(&mut self, ind: usize, n: Vector3<f64>) {
        self.normal[ind] = n;
    }
    pub fn set_color(&mut self, ind: usize, r: f64, g: f64, b: f64) {
        if r < 0.0 || r > 255.0 || g < 0.0 || g > 255.0 || b < 0.0 || b > 255.0 {
            eprintln!("ERROR! Invalid color values");
            exit(-1);
        }
        self.color[ind] = Vector3::new(r / 255.0, g / 255.0, b / 255.0);
    }
    pub fn set_tex_coord(&mut self, ind: usize, s: f64, t: f64) {
        self.tex_coords[ind] = Vector2::new(s, t);
    }
    pub fn to_vector4(&self) -> [Vector4<f64>; 3] {
        let v: Vec<Vector4<f64>> = self.v.iter().map(|vec| Vector4::new(vec[0], vec[1], vec[2], 1.0)).collect();
        [v[0], v[1], v[2]]
    }
    pub fn get_color(&self) -> Vector3<f64> {
        self.color[0] * 255.0 // only one color per triangle.
    }
}