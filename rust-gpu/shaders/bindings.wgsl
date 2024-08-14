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
