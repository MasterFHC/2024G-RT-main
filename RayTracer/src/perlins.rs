use crate::util;
use crate::vec3::Vec3;

pub struct perlin {
    randvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl perlin {
    pub fn new() -> Self {
        let mut randvec: Vec<Vec3> = Vec::new();
        for i in 0..256 {
            randvec.push(util::random_vec3().unit_vector());
        }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();        

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.randvec[(self.perm_x[((i + di) & 255) as usize] 
                                                                ^ self.perm_y[((j + dj) & 255) as usize] 
                                                                ^ self.perm_z[((k + dk) & 255) as usize]) as usize];
                }
            }
        }
        Self::perlin_interp(c, u, v, w)
    }
    pub fn turb(&self, p: &Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
        util::fabs(accum)
    }
    fn permute(p: &mut Vec<i32>, n: i32) {
        for i in (1..n).rev() {
            let target = util::random_range(0.0, i as f64) as i32;
            let tmp = p[i as usize];
            p[i as usize] = p[target as usize];
            p[target as usize] = tmp;
        }
    }
    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = Vec::new();
        for i in 0..256 {
            p.push(i);
        }
        Self::permute(&mut p, 256);
        p
    }
    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        //Hermitian smoothing
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu)) * 
                            (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv)) * 
                            (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww)) * 
                            (c[i][j][k] * weight_v);
                }
            }
        }
        accum
    }
}