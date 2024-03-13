struct Uniforms{
    vp_width: u32,
    vp_height: u32,
    focal_distance: f32
}
@group(0) @binding(0) var<uniform> uniforms: Uniforms;


alias TriangleVertices = array<vec2f, 6>;
var<private> vertices: TriangleVertices = TriangleVertices(
    vec2f(-1.0, -1.0),
    vec2f(1.0, 1.0),
    vec2f(-1.0, 1.0),
    vec2f(-1.0, -1.0),
    vec2f(1.0, -1.0),
    vec2f(1.0, 1.0),
);
@vertex fn display_vs(@builtin(vertex_index) vid: u32) -> @builtin(position) vec4f{
    return vec4f(vertices[vid], 0.0, 1.0);
}


struct Ray{
    origin: vec3f,
    direction: vec3f,
}
struct Sphere{
    center: vec3f,
    radius: f32
}

fn intersect_sphere(sphere: Sphere, ray: Ray) -> f32{
    let v = ray.origin - sphere.center;
    let a = dot(ray.direction, ray.direction);
    let h = dot(v, ray.direction);
    let c = dot(v, v) - sphere.radius * sphere.radius;

    let d = h * h - a * c;
    if d < 0.0{
        return -1.0;
    }

    let sqrt_d = sqrt(d);
    let recip_a = 1.0 / a;
    let t = (-h - sqrt_d) * recip_a;
    if t > 0.0 {
        return t;
    }
    return (-h + sqrt_d) * recip_a;
}


fn sky_color(ray: Ray) -> vec3f{
    let a = 0.5 * (normalize(ray.direction).y + 1.0);
    return (1.0 - a) * vec3f(1.0) + a * vec3f(0.0, 0.0, 1.0);
}

@fragment fn display_fs(@builtin(position) pos: vec4f) -> @location(0) vec4f{
    let origin = vec3f(0.0);
    let aspect_raio = f32(uniforms.vp_width) / f32(uniforms.vp_height);

    // Convert gpu viewport cords to world viewport cords
    // Using height as [-1, 1] then scaling the width to aspect ratio
    var uv = pos.xy / vec2f(f32(uniforms.vp_width - 1u), f32(uniforms.vp_height - 1u));
    uv = (2.0 * uv - vec2f(1.0)) * vec2f(aspect_raio, -1.0);

    let direction = vec3f(uv, -uniforms.focal_distance);
    let ray = Ray(origin, direction);


    let sphere = Sphere(vec3f(0.0, 0.0, -1.0), 0.5);
    if intersect_sphere(sphere, ray) > 0.0{
        return vec4f(1.0, 0.0, 0.0, 1.0);
    }

    return vec4f(sky_color(ray), 1.0);
}
