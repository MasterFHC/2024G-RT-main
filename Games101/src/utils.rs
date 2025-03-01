#![allow(warnings)]
use std::os::raw::c_void;
use std::ops::AddAssign;
use nalgebra::{Matrix3, Matrix4, Vector3, Vector4};
use opencv::core::{Mat, MatTraitConst};
use opencv::imgproc::{COLOR_RGB2BGR, cvt_color};
use crate::shader::{FragmentShaderPayload, VertexShaderPayload};
use crate::texture::Texture;
use crate::triangle::Triangle;

pub type V3f = Vector3<f64>;
pub type M4f = Matrix4<f64>;

pub(crate) fn get_rotation_matrix(axis: V3f, angle: f64) -> M4f{
    // normalize the axis
    let mut axis = axis.normalize();
    let mut rotation: Matrix4<f64> = Matrix4::identity();
    let mut part1: Matrix3<f64> = Matrix3::identity();
    let mut part2: Matrix3<f64> = Matrix3::identity();
    let mut part3: Matrix3<f64> = Matrix3::identity();
    let rad = angle.to_radians();
    let cos = rad.cos();
    let sin = rad.sin();
    part1 *= cos;
    part2 = (1.0 - cos) * axis * axis.transpose();
    part3 = Matrix3::new(0.0, -axis[2], axis[1],
                         axis[2], 0.0, -axis[0],
                         -axis[1], axis[0], 0.0);
    part3 *= sin;
    rotation.fixed_slice_mut::<3, 3>(0, 0).copy_from(&part1);
    rotation.fixed_slice_mut::<3, 3>(0, 0).add_assign(&part2);
    rotation.fixed_slice_mut::<3, 3>(0, 0).add_assign(&part3);
    rotation
}

pub(crate) fn get_view_matrix(eye_pos: V3f) -> M4f {
    let mut view: Matrix4<f64> = Matrix4::identity();

    view[(0, 3)] = -eye_pos[0];
    view[(1, 3)] = -eye_pos[1];
    view[(2, 3)] = -eye_pos[2];

    view
}

pub(crate) fn get_model_matrix(rotation_angle: f64,scale: f64) -> M4f {
    let mut model: Matrix4<f64> = Matrix4::identity();
    let rad = rotation_angle.to_radians();

    model[(0, 0)] = rad.cos();
    model[(0, 1)] = rad.sin();
    model[(1, 0)] = -rad.sin();
    model[(1, 1)] = rad.cos();
    
    model
}

pub(crate) fn get_model_matrix_lab3(rotation_angle: f64) -> M4f {
    let mut model: M4f = Matrix4::identity();
    let rad = (rotation_angle).to_radians();
    model[(0, 0)] = rad.cos();
    model[(2, 2)] = model[(0, 0)];
    model[(0, 2)] = rad.sin();
    model[(2, 0)] = -model[(0, 2)];
    let mut scale: M4f = Matrix4::identity();
    scale[(0, 0)] = 2.5;
    scale[(1, 1)] = 2.5;
    scale[(2, 2)] = 2.5;
    model * scale
}

pub(crate) fn get_projection_matrix(eye_fov: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> M4f {
    let mut projection: Matrix4<f64> = Matrix4::identity();
    let mut scale: M4f = Matrix4::identity();
    /*  implement your code here  */
    let mut persp_ortho: Matrix4<f64> = Matrix4::identity();
    let mut ortho_left: Matrix4<f64> = Matrix4::identity();
    let mut ortho_right: Matrix4<f64> = Matrix4::identity();
    /*  implement your code here  */
    let top = z_near.abs() * ((eye_fov / 2.0).to_radians().tan());
    let right = top * aspect_ratio;
    let left = -right;
    let bottom = -top;
    // perspective projection
    persp_ortho[(0, 0)] = -z_near;
    persp_ortho[(1, 1)] = -z_near;
    persp_ortho[(2, 2)] = z_near + z_far;
    persp_ortho[(2, 3)] = -z_near * z_far;
    persp_ortho[(3, 2)] = 1.0;
    persp_ortho[(3, 3)] = 0.0;
    // orthographic projection
    ortho_right[(0, 3)] = -(right + left) / 2.0;
    ortho_right[(1, 3)] = -(top + bottom) / 2.0;
    ortho_right[(2, 3)] = -(z_near + z_far) / 2.0;
    ortho_left[(0, 0)] = 2.0 / (right - left);
    ortho_left[(1, 1)] = 2.0 / (top - bottom);
    ortho_left[(2, 2)] = 2.0 / (z_near - z_far);
    projection = ortho_left * ortho_right * persp_ortho;

    projection * scale
}

pub(crate) fn frame_buffer2cv_mat(frame_buffer: &Vec<V3f>) -> Mat {
    let mut image = unsafe {
        Mat::new_rows_cols_with_data(
            700, 700,
            opencv::core::CV_64FC3,
            frame_buffer.as_ptr() as *mut c_void,
            opencv::core::Mat_AUTO_STEP,
        ).unwrap()
    };
    let mut img = Mat::copy(&image).unwrap();
    image.convert_to(&mut img, opencv::core::CV_8UC3, 1.0, 1.0).expect("panic message");
    cvt_color(&img, &mut image, COLOR_RGB2BGR, 0).unwrap();
    image
}

pub fn load_triangles(obj_file: &str) -> Vec<Triangle> {
    let (models, _) = tobj::load_obj(&obj_file, &tobj::LoadOptions::default()).unwrap();
    let mesh = &models[0].mesh;
    let n = mesh.indices.len() / 3;
    let mut triangles = vec![Triangle::default(); n];

    // 遍历模型的每个面
    for vtx in 0..n {
        let rg = vtx * 3..vtx * 3 + 3;
        let idx: Vec<_> = mesh.indices[rg.clone()].iter().map(|i| *i as usize).collect();

        // 记录图形每个面中连续三个顶点（小三角形）
        for j in 0..3 {
            let v = &mesh.positions[3 * idx[j]..3 * idx[j] + 3];
            triangles[vtx].set_vertex(j, Vector4::new(v[0] as f64, v[1] as f64, v[2] as f64, 1.0));
            let ns = &mesh.normals[3 * idx[j]..3 * idx[j] + 3];
            triangles[vtx].set_normal(j, Vector3::new(ns[0] as f64, ns[1] as f64, ns[2] as f64));
            let tex = &mesh.texcoords[2 * idx[j]..2 * idx[j] + 2];
            triangles[vtx].set_tex_coord(j, tex[0] as f64, tex[1] as f64);
        }
    }
    triangles
}

// 选择对应的Shader
pub fn choose_shader_texture(method: &str,
                             obj_path: &str) -> (fn(&FragmentShaderPayload) -> Vector3<f64>, Option<Texture>) {
    let mut active_shader: fn(&FragmentShaderPayload) -> Vector3<f64> = phong_fragment_shader;
    let mut tex = None;
    if method == "normal" {
        println!("Rasterizing using the normal shader");
        active_shader = normal_fragment_shader;
    } else if method == "texture" {
        println!("Rasterizing using the texture shader");
        active_shader = texture_fragment_shader;
        tex = Some(Texture::new(&(obj_path.to_owned() + "spot_texture.png")));
    } else if method == "phong" {
        println!("Rasterizing using the phong shader");
        active_shader = phong_fragment_shader;
    } else if method == "bump" {
        println!("Rasterizing using the bump shader");
        active_shader = bump_fragment_shader;
    } else if method == "displacement" {
        println!("Rasterizing using the displacement shader");
        active_shader = displacement_fragment_shader;
    }
    (active_shader, tex)
}

pub fn vertex_shader(payload: &VertexShaderPayload) -> V3f {
    payload.position
}

#[derive(Default)]
struct Light {
    pub position: V3f,
    pub intensity: V3f,
}

pub fn normal_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    let result_color =
        (payload.normal.xyz().normalize() + Vector3::new(1.0, 1.0, 1.0)) / 2.0;
    result_color * 255.0
}

pub fn phong_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    // 泛光、漫反射、高光系数
    let ka = Vector3::new(0.005, 0.005, 0.005);
    let kd = payload.color;
    let ks = Vector3::new(0.7937, 0.7937, 0.7937);

    // 灯光位置和强度
    let l1 = Light {
        position: Vector3::new(20.0, 20.0, 20.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let l2 = Light {
        position: Vector3::new(-20.0, 20.0, 0.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let lights = vec![l1, l2];
    let amb_light_intensity = Vector3::new(10.0, 10.0, 10.0);
    let eye_pos = Vector3::new(0.0, 0.0, 10.0);

    let p = 150.0;

    // ping point的信息
    let normal = payload.normal;
    let point = payload.view_pos;
    let color = payload.color;

    let mut result_color = Vector3::zeros(); // 保存光照结果
    
    // <遍历每一束光>
    for light in lights {
        // LAB3 TODO: For each light source in the code, calculate what the *ambient*, *diffuse*, and *specular* 
        // components are. Then, accumulate that result on the *result_color* object.
        let l = light.position - point;
        let ln = l.normalize();
        let v = (eye_pos - point).normalize();
        let amb = amb_light_intensity.component_mul(&ka);
        let dif = light.intensity.component_mul(&kd) * f64::max(0.0, normal.dot(&ln)) / l.dot(&l);
        let spe = light.intensity.component_mul(&ks) * f64::max(0.0, normal.dot(&(ln + v).normalize())).powf(p) / l.dot(&l);
        result_color += amb + dif + spe;
    }
    result_color * 255.0
}

fn get_payload_color(payload: &FragmentShaderPayload, u: f64, v: f64) -> Vector3<f64> {
    let texture_color: Vector3<f64> = match &payload.texture {
        None => Vector3::new(0.0, 0.0, 0.0),
        Some(texture) => payload.texture.as_ref().unwrap().get_color(u, v),
    };
    texture_color
}
fn h(payload: &FragmentShaderPayload, u: f64, v: f64) -> f64 {
    let texture_color: Vector3<f64> = get_payload_color(payload, u, v);
    return texture_color.norm();
}

pub fn texture_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    let ka = Vector3::new(0.005, 0.005, 0.005);
    // let texture_color: Vector3<f64> = match &payload.texture {
    //     // LAB3 TODO: Get the texture value at the texture coordinates of the current fragment
    //     // <获取材质颜色信息>
    //     None => Vector3::new(0.0, 0.0, 0.0),
    //     Some(texture) => payload.texture.as_ref().unwrap().get_color(payload.tex_coords.x, payload.tex_coords.y),
    // };
    let texture_color: Vector3<f64> = get_payload_color(payload, payload.tex_coords.x, payload.tex_coords.y);
    let kd = texture_color / 255.0; // 材质颜色影响漫反射系数
    let ks = Vector3::new(0.7937, 0.7937, 0.7937);

    let l1 = Light {
        position: Vector3::new(20.0, 20.0, 20.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let l2 = Light {
        position: Vector3::new(-20.0, 20.0, 0.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let lights = vec![l1, l2];
    let amb_light_intensity = Vector3::new(10.0, 10.0, 10.0);
    let eye_pos = Vector3::new(0.0, 0.0, 10.0);

    let p = 150.0;

    let color = texture_color;
    let point = payload.view_pos;
    let normal = payload.normal;

    let mut result_color = Vector3::zeros();

    for light in lights {
        // LAB3 TODO: For each light source in the code, calculate what the *ambient*, *diffuse*, and *specular* 
        // components are. Then, accumulate that result on the *result_color* object.
        let l = light.position - point;
        let ln = l.normalize();
        let v = (eye_pos - point).normalize();
        let amb = amb_light_intensity.component_mul(&ka);
        let dif = light.intensity.component_mul(&kd) * f64::max(0.0, normal.dot(&ln)) / l.dot(&l);
        let spe = light.intensity.component_mul(&ks) * f64::max(0.0, normal.dot(&(ln + v).normalize())).powf(p) / l.dot(&l);
        result_color += amb + dif + spe;
    }

    result_color * 255.0
}

pub fn bump_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    let ka = Vector3::new(0.005, 0.005, 0.005);
    let kd = payload.color;
    let ks = Vector3::new(0.7937, 0.7937, 0.7937);

    let l1 = Light {
        position: Vector3::new(20.0, 20.0, 20.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let l2 = Light {
        position: Vector3::new(-20.0, 20.0, 0.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let lights = vec![l1, l2];
    let amb_light_intensity = Vector3::new(10.0, 10.0, 10.0);
    let eye_pos = Vector3::new(0.0, 0.0, 10.0);

    let p = 150.0;

    let normal = payload.normal;
    let point = payload.view_pos;
    let color = payload.color;

    let (kh, kn) = (0.2, 0.1);

    // LAB3 TODO: Implement bump mapping here 
    let n = normal;
    let (x, y, z) = (n.x, n.y, n.z);
    let t = Vector3::new(x*y/((x*x+z*z).sqrt()),(x*x+z*z).sqrt(),z*y/((x*x+z*z).sqrt()));
    //comput b by n cross product t
    let b = Vector3::new(n[1]*t[2]-n[2]*t[1],n[2]*t[0]-n[0]*t[2],n[0]*t[1]-n[1]*t[0]).normalize();
    let TBN = Matrix3::new(t.x, b.x, n.x,
                           t.y, b.y, n.y,
                           t.z, b.z, n.z);

    let u = payload.tex_coords.x;
    let v = payload.tex_coords.y;
    let wid = payload.texture.as_ref().unwrap().width as f64;
    let hei = payload.texture.as_ref().unwrap().height as f64;

    let dU = kh * kn * (h(payload,u+1.0/wid,v)-h(payload,u,v));
    let dV = kh * kn * (h(payload,u,v+1.0/hei)-h(payload,u,v));
    let ln = Vector3::new(-dU, -dV, 1.0);

    let mut result_color = (TBN * ln).normalize();

    result_color * 255.0
}

pub fn displacement_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    let ka = Vector3::new(0.005, 0.005, 0.005);
    let kd = payload.color;
    let ks = Vector3::new(0.7937, 0.7937, 0.7937);

    let l1 = Light {
        position: Vector3::new(20.0, 20.0, 20.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let l2 = Light {
        position: Vector3::new(-20.0, 20.0, 0.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let lights = vec![l1, l2];
    let amb_light_intensity = Vector3::new(10.0, 10.0, 10.0);
    let eye_pos = Vector3::new(0.0, 0.0, 10.0);

    let p = 150.0;

    let normal = payload.normal;
    let point = payload.view_pos;
    let color = payload.color;

    let (kh, kn) = (0.2, 0.1);

    // LAB3 TODO: Implement displacement mapping here
    let n = normal;
    let (x, y, z) = (n.x, n.y, n.z);
    let t = Vector3::new(x*y/((x*x+z*z).sqrt()),(x*x+z*z).sqrt(),z*y/((x*x+z*z).sqrt()));
    //comput b by n cross product t
    let b = Vector3::new(n[1]*t[2]-n[2]*t[1],n[2]*t[0]-n[0]*t[2],n[0]*t[1]-n[1]*t[0]).normalize();
    let TBN = Matrix3::new(t.x, b.x, n.x,
                           t.y, b.y, n.y,
                           t.z, b.z, n.z);

    let u = payload.tex_coords.x;
    let v = payload.tex_coords.y;
    let wid = payload.texture.as_ref().unwrap().width as f64;
    let hei = payload.texture.as_ref().unwrap().height as f64;

    let dU = kh * kn * (h(payload,u+1.0/wid,v)-h(payload,u,v));
    let dV = kh * kn * (h(payload,u,v+1.0/hei)-h(payload,u,v));
    let ln = Vector3::new(-dU, -dV, 1.0);
    // Position p = p + kn * n * h(u,v)
    // Normal n = normalize(TBN * ln)
    let mut point = point + kn * n * h(payload,u,v);
    let mut normal = (TBN * ln).normalize();


    let mut result_color = Vector3::zeros();
    for light in lights {
        // LAB3 TODO: For each light source in the code, calculate what the *ambient*, *diffuse*, and *specular* 
        // components are. Then, accumulate that result on the *result_color* object.
        let l = light.position - point;
        let ln = l.normalize();
        let v = (eye_pos - point).normalize();
        let amb = amb_light_intensity.component_mul(&ka);
        let dif = light.intensity.component_mul(&kd) * f64::max(0.0, normal.dot(&ln)) / l.dot(&l);
        let spe = light.intensity.component_mul(&ks) * f64::max(0.0, normal.dot(&(ln + v).normalize())).powf(p) / l.dot(&l);
        result_color += amb + dif + spe;
    }

    result_color * 255.0
}
