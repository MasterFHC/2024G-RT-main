#![allow(warnings)]
pub use std::env;
pub use nalgebra::Vector3;
pub use opencv::{
    Result,
};
pub use opencv::core::Vector;
pub use crate::rasterizer3::{Buffer, Rasterizer};
pub use crate::utils::*;
pub use crate::shader::FragmentShaderPayload;
pub use crate::texture::Texture;
use opencv::highgui::{imshow, wait_key};

pub fn t3(filename:String,method:String)-> Result<()>{
    println!("选择任务3");
    let obj_file = "./models/spot/spot_triangulated_good.obj";
    let triangles = load_triangles(&obj_file);
    let mut angle = 140.0;
    let mut r = Rasterizer::new(700, 700);
    let obj_path = "./models/spot/".to_owned();
    let texture_path = "hmap.jpg".to_owned();
    let mut tex = Texture::new(&(obj_path.clone() + &texture_path));
    let mut active_shader: fn(&FragmentShaderPayload) -> Vector3<f64> = normal_fragment_shader; // 默认为<normal shader>
    let ags: Vec<String> = env::args().collect();
    println!("arg len is {}",ags.len());
    let (shader, t) =
        choose_shader_texture(&method, &obj_path);
    active_shader = shader;
    if let Some(tx) = t {
        tex = tx;
    }
    r.set_texture(tex);

    let eye_pos = Vector3::new(0.0, 0.0, 10.0);
    r.set_vertex_shader(vertex_shader);
    r.set_fragment_shader(active_shader);

    let mut k = 0;
    let mut frame_count = 0;
    while k != 27 {
        r.clear(Buffer::Both);
        r.set_model(get_model_matrix_lab3(angle));
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        r.draw(&triangles);
    
        let image = frame_buffer2cv_mat(r.frame_buffer());
        
        imshow("image", &image).unwrap();

        k = wait_key(80).unwrap();
        println!("frame count: {}", frame_count);
        if k == 'a' as i32 {
            angle += 10.0;
        } else if k == 'd' as i32 {
            angle -= 10.0;
        } 
        frame_count += 1;
    }

    Ok(())
}