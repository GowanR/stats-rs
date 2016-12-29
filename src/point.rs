#![allow(dead_code)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
impl Point {
    pub fn new( x: f32, y: f32 ) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
}
#[test]
fn point_construction() {
    let p = Point::new( 1.2, 3.4 );
    assert!( p.x == 1.2 );
    assert!( p.y == 3.4 );
}
