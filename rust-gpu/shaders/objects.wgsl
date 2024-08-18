struct HitRecord {
    t: f32,
    p: vec3f,
    against_normal_unit: vec3f,
    front_face: bool,
    hit: bool,
    material: Material,
}
fn no_hit_record() -> HitRecord {
    return HitRecord(0.0, vec3f(0.0), vec3f(0.0), false, false, Material(0u, 0u, 0u));
}
fn new_hit_record(ray: Ray, outward_normal_unit: vec3f, t: f32, p: vec3f, material: Material) -> HitRecord{
    let front_face = dot(ray.direction, outward_normal_unit) < 0f;
    var against_normal_unit = outward_normal_unit;
    if !front_face{
        against_normal_unit = -outward_normal_unit;
    }
    return HitRecord(t, p, against_normal_unit, front_face, true, material);
}

struct Sphere {
    center: vec3f,
    radius: f32,
    material_idx: u32
}
fn intersect_sphere(sphere: Sphere, ray: Ray, t_max: f32) -> HitRecord {
    let v = ray.origin - sphere.center;
    let a = dot(ray.direction, ray.direction);
    let h = dot(v, ray.direction);
    let c = dot(v, v) - sphere.radius * sphere.radius;

    let d = h * h - a * c;
    if d < 0.0 {
        return no_hit_record();
    }

    let sqrt_d = sqrt(d);
    let recip_a = 1.0 / a;
    let t1 = (-h - sqrt_d) * recip_a;
    let t2 = (-h + sqrt_d) * recip_a;
    var t = select(t2, t1, t1 > T_MIN);
    if !within(T_MIN, t, t_max) {
        return no_hit_record();
    }

    let p = at(ray, t);
    let outward_normal_unit = (p - sphere.center) / sphere.radius;
    return new_hit_record(ray, outward_normal_unit, t, p, materials[sphere.material_idx]);
}
