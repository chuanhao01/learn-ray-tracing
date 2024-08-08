
struct Uniforms {
    look_at: vec3f,
    theta: f32, // In Degrees
    look_from: vec3f,
    focal_distance: f32,
    v_up: vec3f,
    vp_width: u32,
    vp_height: u32,
    frame_count: u32
}
@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read> spheres: array<Sphere>;
@group(0) @binding(2) var<storage, read> materials: array<Material>;
@group(0) @binding(3) var<storage, read> scatter_materials: array<ScatterMaterial>;
@group(0) @binding(4) var<storage, read> emit_materials: array<EmitMaterial>;

@group(1) @binding(0) var radiance_samples_old: texture_2d<f32>;
@group(1) @binding(1) var radiance_samples_new: texture_storage_2d<rgba32float, write>;

const T_MIN: f32 = 0.001;
const FLT_MAX: f32 = 3.40282346638528859812e+38;
const PI = 3.1415927f;
const FRAC_1_PI = 0.31830987f;
const FRAC_PI_2 = 1.5707964f;
const NEAR_ZERO = 0.000000001f;
const DEPTH = 50u;

fn within(_min: f32, a: f32, _max: f32) -> bool {
    return _min <= a && a <= _max;
}
fn near_zero(v: vec3f) -> bool{
    return v.x < NEAR_ZERO && v.y < NEAR_ZERO && v.z < NEAR_ZERO;
}


struct Rng{
    state: u32
}
var<private> rng: Rng;

// Taken from https://raytracing.github.io/gpu-tracing/book/MovingToTheGPU.htm
fn init_rng(pixel: vec2u) {
    // seed PRNG using scalar index of the pixel and current frame count
    let seed = (pixel.x + pixel.y * uniforms.vp_width) ^ jenkins_hash(uniforms.frame_count);
    rng.state = jenkins_hash(seed);
}
// A slightly modified version of the "One-at-a-Time Hash" function by Bob Jenkins.
// See https://www.burtleburtle.net/bob/hash/doobs.html
fn jenkins_hash(i: u32) -> u32{
    var x = i;
    x += x << 10u;
    x ^= x >> 6u;
    x += x << 3u;
    x ^= x >> 11u;
    x += x << 15u;
    return x;
}
// The 32-bit "xor" function from Marsaglia G., "Xorshift RNGs", Section 3.
fn xorshift32() -> u32 {
    var x = rng.state;
    x ^= x << 13u;
    x ^= x >> 17u;
    x ^= x << 5u;
    // So that the next rand call is another rand 32u
    rng.state = x;
    return x;
}
// Returns a random float in the range [0...1]. This sets the floating point exponent to zero and
// sets the most significant 23 bits of a random 32-bit unsigned integer as the mantissa. That
// generates a number in the range [1, 1.9999999], which is then mapped to [0, 0.9999999] by
// subtraction. See Ray Tracing Gems II, Section 14.3.4.
fn rand_f32() -> f32 {
    return bitcast<f32>(0x3f800000u | (xorshift32() >> 9u)) - 1.;
}
fn rand_in_hemisphere() -> vec3f{
    let r1 = rand_f32();
    let r2 = rand_f32();

    let phi = 2f * PI * r1;
    let sinTheta = sqrt(1f - r2 * r2);

    let x = cos(phi) * sinTheta;
    let y = sin(phi) * sinTheta;
    let z = r2;

    return vec3f(x, y, z);
}

struct Ray {
    origin: vec3f,
    direction: vec3f
}
fn no_ray() -> Ray{
    Ray(vec3f(0f), vec3f(0f));
}

fn at(ray: Ray, t: f32) -> vec3f {
    return ray.origin + t * ray.direction;
}

struct HitRecord {
    t: f32,
    p: vec3f,
    against_normal_unit: vec3f,
    hit: bool,
    material: Material,
}
fn no_hit_record() -> HitRecord {
    return HitRecord(0.0, vec3f(0.0), vec3f(0.0), false, Material(0u, 0u, 0u));
}

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
    let t = select(t2, t1, t1 > 0.0);
    if !within(T_MIN, t, t_max) {
        return no_hit_record();
    }

    let p = at(ray, t);
    return HitRecord(t, p, (p - sphere.center) / sphere.radius, true, materials[sphere.material_idx]);
}

struct LightRay{
    ray: Ray,
    attenuation: vec3f,
    done: bool,
}
fn scatter_lambertain(hit: HitRecord, lambertain: ScatterMaterial) -> LightRay{
    var scattered_direction = hit.against_normal_unit + rand_in_hemisphere();
    if near_zero(scattered_direction) {
        scattered_direction = hit.against_normal_unit;
    }
    return LightRay(Ray(hit.p, scattered_direction), lambertain.albedo, false);
}

fn color_ray(ray: Ray, t_min: f32, t_max: f32) -> LightRay {
    var t_max_so_far = t_max;
    var closest_hit = no_hit_record();
    for (var i = 0u; i < arrayLength(&spheres); i += 1u) {
        let sphere = spheres[i];
        // sphere.radius += sin(f32(uniforms.frame_count) * 0.02) * 0.2;
        let hit = intersect_sphere(sphere, ray, t_max_so_far);
        if within(t_min, hit.t, t_max_so_far) && hit.hit {
            closest_hit = hit;
            t_max_so_far = closest_hit.t;
        }
    }

    if closest_hit.hit{
        if closest_hit.material.t == 0u {
            let scatter_material = scatter_materials[closest_hit.material.sactter_idx];
            if scatter_material.t == 0u{
                return scatter_lambertain(closest_hit, scatter_material);
            }
        }
    }
    // return LightRay(no_ray(), vec3f(0f), true);
    return sky_color(ray);
}


fn sky_color(ray: Ray) -> LightRay {
    let a = 0.5 * (normalize(ray.direction).y + 1.0);
    return LightRay(Ray(vec3f(0f), vec3f(0f)), (1.0 - a) * vec3f(1.0) + a * vec3f(0.5, 0.7, 1.0), true);
}
@fragment
fn display_fs(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    init_rng(vec2u(pos.xy));
    let camera_origin = uniforms.look_from;
    let aspect_raio = f32(uniforms.vp_width) / f32(uniforms.vp_height);

    // let offset = vec2f(0f, 0f);
    let offset = vec2f(
        rand_f32() - 0.5,
        rand_f32() - 0.5
    );

    let _w = uniforms.look_at - uniforms.look_from;
    // let _w = uniforms.look_from - uniforms.look_at;
    let focal_distance = length(_w);

    let w = normalize(_w);
    let u = normalize(cross(uniforms.v_up, w));
    let v = cross(w, u);


    let height = focal_distance * tan(radians(uniforms.theta / 2f));
    let width = aspect_raio * height;
    let delta_x = vec3f(width / f32(uniforms.vp_width)) * u;
    let delta_y = vec3f(-height / f32(uniforms.vp_height)) * v;
    let top_left = uniforms.look_at + vec3f(-width / 2f) * u + vec3f(height / 2f) * v + vec3f(0.5) * delta_x + vec3f(0.5) * delta_y;
    // Add a offset from unit_square to the x, y * delta
    var uv = top_left + vec3f((f32(pos.x) + offset.x)) * delta_x + vec3f((f32(pos.y) + offset.y)) * delta_y;

    let camera_pixel_direction = uv - camera_origin;
    let camera_pixel_ray = Ray(camera_origin, camera_pixel_direction);

    var throughput = vec3f(1f);
    var ray = camera_pixel_ray;
    var current_depth = 0u;
    while current_depth < DEPTH{
        let radiance_sample = color_ray(ray, T_MIN, FLT_MAX);
        throughput *= radiance_sample.attenuation;
        ray = radiance_sample.ray;
        current_depth += 1u;
        if radiance_sample.done{
            break;
        }
    }

    var old_sum: vec3f;
    if uniforms.frame_count > 1u {
        old_sum = textureLoad(radiance_samples_old, vec2u(pos.xy), 0).xyz;
    } else {
        old_sum = vec3f(0f);
    }

    let new_sum = old_sum + throughput;
    textureStore(radiance_samples_new, vec2u(pos.xy), vec4f(new_sum, 0f));

    // return vec4f(throughput, 1f);
    // Not sure if gamma correction is needed
    // return vec4f(sqrt(new_sum / f32(uniforms.frame_count)), 1f);
    return vec4f(new_sum / f32(uniforms.frame_count), 1f);
    // return vec4f(spheres[1].center, 1f);
}
