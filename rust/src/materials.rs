use std::sync::Arc;

use rand::{thread_rng, Rng};

use crate::{ray::Ray, ColorTexture, HitRecord};

use super::Vec3;

pub struct Scattered {
    pub attenuation: Vec3,
    pub ray: Ray,
}

pub trait Scatterable: Sync + Send {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Scattered>;
}

pub struct Lambertain {
    pub albedo: Arc<dyn ColorTexture>,
}
impl Scatterable for Lambertain {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Scattered> {
        let scattered_direction =
            hit_record.against_normal_unit.clone() + Vec3::random_vector_in_unit_sphere();
        let scattered_direction = if scattered_direction.near_zero() {
            hit_record.against_normal_unit.clone()
        } else {
            scattered_direction
        };
        Some(Scattered {
            attenuation: self
                .albedo
                .color(hit_record.u, hit_record.v, hit_record.p.clone()),
            ray: Ray {
                origin: hit_record.p.clone(),
                direction: scattered_direction,
            },
        })
    }
}

pub struct Metal {
    albedo: Vec3,
    /// Ratio to scale the sampled unit circle, for the reflected ray + fuzziness
    fuzzy_factor: f64,
}
impl Metal {
    pub fn new(albedo: Vec3, fuzzy_factor: f64) -> Metal {
        Metal {
            albedo,
            fuzzy_factor: if fuzzy_factor < 1_f64 {
                fuzzy_factor
            } else {
                1_f64
            },
        }
    }
}
impl Scatterable for Metal {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Scattered> {
        let scattered_direction = Vec3::reflect(
            &_ray.direction.unit_vector(),
            &hit_record.against_normal_unit,
        ) + self.fuzzy_factor * Vec3::random_vector_in_unit_sphere();
        // Check if the scattered rays are cancelled out or scattered below the surface, in that case, ray is absorbed
        if Vec3::dot(&scattered_direction, &hit_record.against_normal_unit) > 0_f64 {
            Some(Scattered {
                attenuation: self.albedo.clone(),
                ray: Ray {
                    origin: hit_record.p.clone(),
                    direction: scattered_direction,
                },
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub index_of_reflectance: f64,
}
impl Dielectric {
    /// Calculate the reflectance given the refraction_ratio (in relation to air index of refraction being 1.0)
    fn reflectance(cos_theta: f64, refraction_ratio: f64) -> f64 {
        let r0 = (1_f64 - refraction_ratio) / (1_f64 + refraction_ratio);
        let r0 = r0 * r0;
        r0 + (1_f64 - r0) * (1_f64 - cos_theta).powi(5)
    }
}
impl Scatterable for Dielectric {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Scattered> {
        let mut rng = thread_rng();

        // Diaelectric passes the color along
        let albedo = Vec3::new_int(1, 1, 1);

        let refraction_ratio = if hit_record.front_face {
            1_f64 / self.index_of_reflectance
        } else {
            self.index_of_reflectance
        };
        let unit_direction = _ray.direction.unit_vector();
        let cos_theta =
            Vec3::dot(&(-unit_direction.clone()), &hit_record.against_normal_unit).min(1_f64);
        let sin_theta = (1_f64 - cos_theta * cos_theta).sqrt();
        if refraction_ratio * sin_theta > 1_f64
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>()
        {
            Some(Scattered {
                attenuation: albedo,
                ray: Ray {
                    origin: hit_record.p.clone(),
                    direction: Vec3::reflect(&unit_direction, &hit_record.against_normal_unit),
                },
            })
        } else {
            Some(Scattered {
                attenuation: albedo,
                ray: Ray {
                    origin: hit_record.p.clone(),
                    direction: Vec3::refract(
                        &unit_direction,
                        &hit_record.against_normal_unit,
                        refraction_ratio,
                    ),
                },
            })
        }
    }
}

pub trait Emittable: Sync + Send {
    /// Return the light value of the material
    fn emit(&self) -> Vec3;
}

pub struct Diffuse {
    pub power: f64,
}
impl Emittable for Diffuse {
    fn emit(&self) -> Vec3 {
        Vec3::new(self.power, self.power, self.power)
    }
}

pub enum Materials {
    ScatterMaterial(Arc<dyn Scatterable>),
    LightMaterial(Arc<dyn Emittable>),
}
impl Clone for Materials {
    fn clone(&self) -> Self {
        match self {
            Self::ScatterMaterial(scatter_material) => {
                Self::ScatterMaterial(scatter_material.clone())
            }
            Self::LightMaterial(light_material) => Self::LightMaterial(light_material.clone()),
        }
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::Scatterable;
    use super::*;

    pub struct TestScatterable {}
    impl Scatterable for TestScatterable {
        fn scatter(&self, _ray: &crate::Ray, hit_record: &crate::HitRecord) -> Option<Scattered> {
            None
        }
    }
}
