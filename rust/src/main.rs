use rust_simple_raytracer::Vec3;

fn main() {
    println!("Hello, world!");
    let v = Vec3::new(1.0, 1.0, 1.0);
    println!("{}", v[1] * 2.0);
    println!("{}", v[1]);
    println!("{}", v);
}
