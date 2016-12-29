#![allow(dead_code)]
use point::Point;

pub struct Line {
    slope: f32,
    y_int: f32,
}
impl Line {
    fn get( &self, x: f32 ) -> f32 {
        (self.slope * x) + self.y_int
    }
    fn new( s: f32, y: f32 ) -> Line {
        Line {
            slope: s,
            y_int: y,
        }
    }
    fn from( p1: Point, p2: Point ) -> Option<Line> {
        if p1.x == p2.x {
            return None;
        }
        let m = ( p2.y - p1.y ) / (p2.x - p1.x );
        let b = p1.y - (m * p1.x);
        return Some( Line {
            slope: m,
            y_int: b,
        } );
    }
    fn intersects( &self, p1: Point ) -> bool {
        self.get( p1.x ) == p1.y
    }
    fn intersection( &self, p: Line ) -> Option<Point> {
        if self.slope == p.slope {
            return None;
        }
        let x = (self.y_int - p.y_int) / (p.slope - self.slope);
        let y = self.get(x);
        return Some( Point{ x: x, y: y } );
    }
}
#[test]
fn line_construction() {
    let line = Line::new( 2f32, 3f32 );
    assert!( line.get(0f32) == 3f32 );
    assert!( line.get(5f32) == 13f32 );
    assert!( line.slope == 2f32 );
    assert!( line.y_int == 3f32 );
}
#[test]
fn line_intersection() {
    let line = Line::new( 3f32, 6f32 );
    let point = Point::new( 1f32, 9f32 );
    assert!( line.intersects( point ) );
    assert!( !line.intersects( Point { x: 0f32, y: 0f32 } ) );
}
#[test]
fn line_from() {
    let p1 = Point::new( 3f32 ,2f32 );
    let p2 = Point::new( 7f32, -4f32 );
    let line = Line::from( p1, p2 );
    match line {
        Some(l) => {
            assert!( l.slope == (-3f32/2f32) );
            assert!( l.y_int == (13f32/2f32) );
        },
        None => {
            assert!( false );
        }
    }
}
#[test]
fn line_from_undefined_slope() {
    let p1 = Point::new( 3f32, 2f32 );
    let p2 = Point::new( 3f32, 1f32 );
    let line = Line::from( p1, p2 );
    match line {
        Some( _ ) => assert!( false ),
        None => assert!( true ),
    }
}
