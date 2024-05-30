alias TriangleVertices = array<vec2f, 6>;
var<private> vertices: TriangleVertices = TriangleVertices(
    vec2f(-1.0, -1.0),
    vec2f(1.0, 1.0),
    vec2f(-1.0, 1.0),
    vec2f(-1.0, -1.0),
    vec2f(1.0, -1.0),
    vec2f(1.0, 1.0),
);
@vertex
fn display_vs(@builtin(vertex_index) vid: u32) -> @builtin(position) vec4f {
    return vec4f(vertices[vid], 0.0, 1.0);
}

struct Uniforms {
    vp_width: u32,
    vp_height: u32,
    focal_distance: f32,
    frame_count: u32
}
@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var radiance_samples_old: texture_2d<f32>;
@group(0) @binding(2) var radiance_samples_new: texture_storage_2d<rgba32float, write>;

const T_MIN: f32 = 0.001;
const FLT_MAX: f32 = 3.40282346638528859812e+38;
fn within(_min: f32, a: f32, _max: f32) -> bool {
    return _min <= a && a <= _max;
}


struct Ray {
    origin: vec3f,
    direction: vec3f
}
fn at(ray: Ray, t: f32) -> vec3f {
    return ray.origin + t * ray.direction;
}

struct HitRecord {
    t: f32,
    p: vec3f,
    against_normal_unit: vec3f,
    hit: bool,
}
fn no_hit_record() -> HitRecord {
    return HitRecord(0.0, vec3f(0.0), vec3f(0.0), false);
}

struct Sphere {
    center: vec3f,
    radius: f32
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
    return HitRecord(t, p, (p - sphere.center) / sphere.radius, true);
}

const SPHERE_COUNT: u32 = 2u;
var<private> scene: Scene = Scene(array<Sphere, SPHERE_COUNT>(
    Sphere(vec3f(0.0, 0.0, -1.0), 0.5),
    Sphere(vec3f(0.0, -100.5, -1.0), 100.0),
));
struct Scene {
    sphere_objects: array<Sphere, SPHERE_COUNT>,
}
fn color_ray(ray: Ray, scene: Scene, depth: u32, t_min: f32, t_max: f32) -> vec3f {
    var t_max_so_far = t_max;
    var sphere_objects = scene.sphere_objects;
    var closest_hit = no_hit_record();
    for (var i = 0u; i < SPHERE_COUNT; i += 1u) {
        var sphere = sphere_objects[i];
        // sphere.radius += sin(f32(uniforms.frame_count) * 0.02) * 0.2;
        let hit = intersect_sphere(sphere, ray, t_max_so_far);
        if within(t_min, hit.t, t_max_so_far) && hit.hit {
            closest_hit = hit;
            t_max_so_far = closest_hit.t;
        }
    }

    if !closest_hit.hit {
        return sky_color(ray);
    }
    return vec3f(0.5 * closest_hit.against_normal_unit + vec3(0.5));
}


fn sky_color(ray: Ray) -> vec3f {
    let a = 0.5 * (normalize(ray.direction).y + 1.0);
    return (1.0 - a) * vec3f(1.0) + a * vec3f(0.0, 0.0, 1.0);
}
@fragment
fn display_fs(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    let camera_origin = vec3f(0.0);
    let aspect_raio = f32(uniforms.vp_width) / f32(uniforms.vp_height);

    let offset = vec2f(
        f32(uniforms.frame_count % 4u) * 0.25 - 0.5,
        f32((uniforms.frame_count % 16u) / 4u) * 0.25 - 0.5
    );

    // Convert gpu viewport cords to world viewport cords
    // Using height as [-1, 1] then scaling the width to aspect ratio
    var uv = (pos.xy + offset) / vec2f(f32(uniforms.vp_width - 1u), f32(uniforms.vp_height - 1u));
    uv = (2.0 * uv - vec2f(1.0)) * vec2f(aspect_raio, -1.0);

    let camera_pixel_direction = vec3f(uv, -uniforms.focal_distance);
    let camera_pixel_ray = Ray(camera_origin, camera_pixel_direction);

    var radiance_sample = color_ray(camera_pixel_ray, scene, 10u, T_MIN, FLT_MAX);

    var old_sum: vec3f;
    if uniforms.frame_count > 1u {
        old_sum = textureLoad(radiance_samples_old, vec2u(pos.xy), 0).xyz;
    } else {
        old_sum = vec3f(0f);
    }

    let new_sum = old_sum + radiance_sample;
    textureStore(radiance_samples_new, vec2u(pos.xy), vec4f(new_sum, 0f));

    return vec4f(new_sum / f32(uniforms.frame_count), 1f);
}
