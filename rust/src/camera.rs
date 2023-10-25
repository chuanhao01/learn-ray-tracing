use super::helper::from_fdegree_to_fradian;

use super::vec3::Vec3;

/// Camera Parameters defined by the user, which is then used to generate a [Camera].
/// Should be used in conjuection with [Camera::new]
pub struct CameraParams {
    /// Generated Image Width (resolution) in pixels
    pub image_width: i64,
    /// Aspect ratio of the image (Width / Height)
    pub aspect_ratio: f64,
    /// Number of simulated rays per pixel
    pub samples_per_pixel: i64,
    // Maximum Depth of ray simulation
    pub max_depth: i64,

    /// Field of View Angle (in degrees)
    /// (Angle from Camera center to top of viewport)
    pub fov: f64,
    /// Point the Camera is looking from (Camera Center)
    pub look_from: Vec3,
    /// Point the Camera is looking at
    pub look_at: Vec3,
    /// Vector specifying where up for the camera is
    /// Used to calculate the vertical plane of the Camera
    pub v_up: Vec3,
    /// Angle (in degrees) from the Camera center to top of defocus blur disk
    /// Set to 0 for no defocus blur
    pub focus_angle: f64,
    /// Distance from Camera center to focus plane
    pub focus_distance: f64,
}

pub struct Camera {
    image_width: i64,
    image_height: i64,
    aspect_ratio: f64,

    samples_per_pixel: i64,
    max_depth: i64,

    /// Point of the Camera center (Same as [CameraParams.look_from])
    center: Vec3,

    /// Unit vector in relation to the direction the camera is facing, u, x-axis
    u: Vec3,
    /// Unit vector in relation to the direction the camera is facing, v, y-axis
    v: Vec3,
    /// Unit vector in relation to the direction the camera is facing, w, z-axis
    w: Vec3,

    /// Unit vector u (x-axis) per pixel on the viewport
    pixel_delta_u: Vec3,
    /// Unit vector v (y-axis) per pixel on the viewport
    pixel_delta_v: Vec3,

    /// Point of the center of the top left 00 pixel
    pixel_00_loc: Vec3,

    /// Vector u of the defocus disk (x-axis length)
    defocus_disk_u: Vec3,
    /// Vector v of the defocus disk (y-axis length)
    defocus_disk_v: Vec3,
}

impl Default for CameraParams {
    fn default() -> Self {
        CameraParams {
            image_width: 400,
            aspect_ratio: 16_f64 / 9_f64,
            samples_per_pixel: 50,
            max_depth: 20,
            fov: 90_f64,
            look_from: Vec3::new_int(0, 0, 0),
            look_at: Vec3::new_int(0, 0, -1),
            v_up: Vec3::new_int(0, 1, 0),
            focus_angle: 0_f64,
            focus_distance: 1_f64,
        }
    }
}

impl Camera {
    pub fn new(camera_params: CameraParams) -> Self {
        let image_height = camera_params.image_width / camera_params.aspect_ratio as i64;

        let w = (camera_params.look_from.clone() - camera_params.look_at).unit_vector();
        let u = Vec3::cross(&camera_params.v_up, &w).unit_vector();
        let v = Vec3::cross(&w, &u).unit_vector();

        // Ratio of 1/2 viewport height to focus_distance
        let h = from_fdegree_to_fradian(camera_params.fov).tan();
        let viewport_v = 2_f64 * h * camera_params.focus_distance;
        let viewport_u = viewport_v * (camera_params.image_width as f64) / (image_height as f64);

        let delta_u = u.clone() * viewport_u;
        let delta_v = -v.clone() * viewport_v; // Since the vertical pixels goes down
        let pixel_delta_u = delta_u.clone() / (camera_params.image_width as f64);
        let pixel_delta_v = delta_v.clone() / (image_height as f64);

        let pixel_00_loc = camera_params.look_from.clone()
            - w.clone() * camera_params.focus_distance
            - 0.5_f64 * (delta_u.clone() + delta_v.clone())
            + 0.5_f64 * (pixel_delta_u.clone() + pixel_delta_v.clone());

        let defocus_radius =
            camera_params.focus_distance * from_fdegree_to_fradian(camera_params.focus_angle).tan();
        let defocus_disk_u = u.clone() * defocus_radius;
        let defocus_disk_v = v.clone() * defocus_radius;

        Camera {
            image_width: camera_params.image_width,
            image_height,
            aspect_ratio: camera_params.aspect_ratio,
            samples_per_pixel: camera_params.samples_per_pixel,
            max_depth: camera_params.max_depth,
            center: camera_params.look_from.clone(),
            u: u.clone(),
            v: v.clone(),
            w: w.clone(),
            pixel_delta_u: pixel_delta_u.clone(),
            pixel_delta_v: pixel_delta_v.clone(),
            pixel_00_loc: pixel_00_loc.clone(),
            defocus_disk_u: defocus_disk_u.clone(),
            defocus_disk_v: defocus_disk_v.clone(),
        }
    }
    // pub fn render() {}
}
