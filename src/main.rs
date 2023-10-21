// Given a list of coordinates containing
// circles, and a start and end point, generate the shortest 
// path

struct Point {
    x: f32,
    y: f32
}
struct Circle {
    coordinates: Point,
    radius: f32
}

fn magnitude(p1: Point, p2: Point) -> f32{
    return ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).powf(0.5);
}

fn internal_bitangent_angle(a: Circle, b: Circle) -> f32 {

   let distance = magnitude(a.coordinates, b.coordinates);
   return ((a.radius + b.radius) / distance).acos();

}


fn main() {
    println!("Hello, world!");
}
