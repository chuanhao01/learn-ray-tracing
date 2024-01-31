// Computes the direction vector of a primary ray
// \param x, y Coordinates between 0 and 1 for the pixel that the ray
//      passes through (x, y)
// \param camera_position Position vector for the camera (o)
// \param left_bottom Position vector for the left bottom corner of
//      the image plane (c)
// \param right, up Direction vectors towards the right and top end
//      of the image plane (r, u)
// \return A direction vector of length 1 for a ray through the
//      specified pixel (d)
vec3 get_primary_ray_direction(float x, float y, vec3 camera_position, vec3 left_bottom, vec3 right, vec3 up) {
    vec3 p = left_bottom + x * right + y * up;
    vec3 d = p - camera_position;
    return normalize(d);
}


// A triangle along with some shading parameters
struct triangle_t {
    // The positions of the three vertices (v_0, v_1, v_2)
    vec3 positions[3];
    // A vector of length 1, orthogonal to the triangle (n)
    vec3 normal;
    // The albedo of the triangle (i.e. the fraction of
    // red/green/blue light that gets reflected) (a)
    vec3 color;
    // The radiance emitted by the triangle (for light sources) (L_e)
    vec3 emission;
};


// Checks whether a ray intersects a triangle
// \param out_t The ray parameter at the intersection (if any) (t)
// \param origin The position at which the ray starts (o)
// \param direction The direction vector of the ray (d)
// \param tri The triangle for which to check an intersection
// \return true if there is an intersection, false otherwise
bool ray_triangle_intersection(out float out_t, vec3 origin, vec3 direction, triangle_t tri) {
    // TODO
    vec3 v0 = tri.positions[0];
    mat3 matrix = mat3(-direction, tri.positions[1] - v0, tri.positions[2] - v0);
    vec3 solution = inverse(matrix) * (origin - v0);
    out_t = solution.x;
    vec2 bcords = solution.yz;
    return out_t >= 0.0 && bcords.x >=0.0 && bcords.y >= 0.0 && bcords.x + bcords.y <= 1.0;
}


void mainImage(out vec4 out_color, in vec2 pixel_coord) {
    // Define the camera position and the view plane
    vec3 camera_position = vec3(0.278, 0.8, 0.2744);
    vec3 middle = camera_position - vec3(0.0, 0.8, 0.0);
    vec3 up = vec3(0.0, 0.0, 0.56);
    float aspect = float(iResolution.x) / float(iResolution.y);
    vec3 right = aspect * vec3(-0.56, 0.0, 0.0);
    vec3 left_bottom = middle - 0.5 * right - 0.5 * up;
    // Compute the camera ray
    vec2 tex_coord = pixel_coord / iResolution.xy;
    vec3 ray_direction = get_primary_ray_direction(
        tex_coord.x, tex_coord.y, camera_position, left_bottom, right, up);
    // Trace a ray against a single triangle and display t
    triangle_t tri;
    tri.positions[0] = vec3(0.555999935, -0.000000119, 0.548799932);
    tri.positions[1] = vec3(0.555999935, -0.000000119, 0.000000040);
    tri.positions[2] = vec3(0.555999935, -0.559199989, 0.000000040);
    tri.normal = vec3(-1.0, 0.0, -0.0);
    tri.color = vec3(0.730000019, 0.246000007, 0.250999987);
    tri.emission = vec3(0.0, 0.0, 0.0);
    float t;
    out_color.rgb = vec3(0.0);
    if (ray_triangle_intersection(t, camera_position, ray_direction, tri))
        out_color.rgb = vec3(0.7 * t);
    out_color.a = 1.0;
}
