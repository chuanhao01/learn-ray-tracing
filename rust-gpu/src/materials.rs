use crate::{gpu_buffer, Vec3f};

pub struct Lambertain {
    albedo: Vec3f,
}

pub enum ScatterMaterial {
    Lambertain(Lambertain),
}
impl ScatterMaterial {
    pub fn get_t(&self) -> u32 {
        match self {
            Self::Lambertain(_) => 0,
        }
    }
    pub fn to_gpu_material(&self) -> gpu_buffer::ScatterMaterial {
        let t = self.get_t();
        match self {
            Self::Lambertain(lambertain) => {
                gpu_buffer::ScatterMaterial::new(lambertain.albedo, t, 0f32, 0f32)
            }
        }
    }

    // Generators
    pub fn new_lambertain(albedo: Vec3f) -> Self {
        Self::Lambertain(Lambertain { albedo })
    }
}

pub struct Diffuse {
    power: f32,
}
pub enum EmitMaterial {
    Diffuse(Diffuse),
}
impl EmitMaterial {
    pub fn get_t(&self) -> u32 {
        match self {
            Self::Diffuse(_) => 0,
        }
    }
    pub fn to_gpu_material(&self) -> gpu_buffer::EmitMaterial {
        let t = self.get_t();
        match self {
            Self::Diffuse(diffuse) => gpu_buffer::EmitMaterial::new(t, diffuse.power),
        }
    }

    // Generators
    pub fn new_diffuse(power: f32) -> Self {
        Self::Diffuse(Diffuse { power })
    }
}

pub enum Material {
    ScatterMaterial(ScatterMaterial),
    EmitMaterial(EmitMaterial),
}
impl Material {
    pub fn get_t(&self) -> u32 {
        match self {
            Self::ScatterMaterial(_) => 0,
            Self::EmitMaterial(_) => 1,
        }
    }
}
