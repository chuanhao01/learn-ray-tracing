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
