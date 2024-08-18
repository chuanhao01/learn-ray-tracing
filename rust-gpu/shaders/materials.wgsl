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

fn scatter_metal(ray: Ray, hit: HitRecord, metal: ScatterMaterial) -> LightRay {
    var scattered_direction = reflect(ray.direction, hit.against_normal_unit) + metal.fuzzy_factor * rand_in_hemisphere();
    if dot(scattered_direction, hit.against_normal_unit) > 0f{
        return LightRay(Ray(hit.p, scattered_direction), metal.albedo, false);
    } else {
        return LightRay(no_ray(), vec3f(0f), true);
    }
}

fn reflectance(cos_theta: f32, refraction_ratio: f32) -> f32 {
    var r0 = (1f - refraction_ratio) / (1f + refraction_ratio);
    r0 = r0 * r0;
    return r0 + (1f - r0) * pow(1f - cos_theta, 5f);
}
fn scatter_dielectric(ray: Ray, hit: HitRecord, dielectric: ScatterMaterial) -> LightRay {
    let albedo = vec3f(1f);

    var refraction_ratio = dielectric.index_of_reflectance;
    if hit.front_face {
        refraction_ratio = 1f / dielectric.index_of_reflectance;
    }
    let unit_direction = normalize(ray.direction);
    let cos_theta = min(dot(-unit_direction, hit.against_normal_unit), 1f);
    let sin_theta = sqrt(1f - cos_theta * cos_theta);
    if refraction_ratio * sin_theta > 1f || reflectance(cos_theta, refraction_ratio) > rand_f32() {
        // Reflecting
        return LightRay(Ray(hit.p, reflect(unit_direction, hit.against_normal_unit)), albedo, false);
    } else {
        // Refracting
        return LightRay(Ray(hit.p, refract(unit_direction, hit.against_normal_unit, refraction_ratio)), albedo, false);
    }
}
