struct Material{
    t: u32,
    sactter_idx: u32,
    emit_idx: u32
}
// Align 16
struct ScatterMaterial{
    albedo: vec3f,
    t: u32,
    fuzzy_factor: f32,
    index_of_reflectance: f32
}
struct EmitMaterial {
    t: u32,
    power: f32
}

fn scatter_lambertain(hit: HitRecord, lambertain: ScatterMaterial) -> LightRay{
    var scattered_direction = hit.against_normal_unit + rand_in_hemisphere();
    if near_zero(scattered_direction) {
        scattered_direction = hit.against_normal_unit;
    }
    return LightRay(Ray(hit.p, scattered_direction), lambertain.albedo, false);
}
