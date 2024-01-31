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
    // Display the ray direction as color (mapping [-1,1] to [0, 1])
    out_color.rgb = ray_direction * 0.5 + vec3(0.5);
    out_color.a = 1.0;
}
