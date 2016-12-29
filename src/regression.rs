use data::Array;
use line::Line;
#[allow(dead_code)]
pub struct Regression {
    pub x: Array,
    pub y: Array,
    lsrl: Option<Line>,
    r_value: Option<f32>,
}
impl Regression {
    fn new( a1: Array, a2: Array ) -> Regression {
        assert!( a1.data.len() == a2.data.len() );
        Regression {
            x: a1,
            y: a2,
            lsrl: None,
            r_value: None,
        }
    }
    fn from( v1: Vec<f32>, v2: Vec<f32> ) -> Regression {
        unimplemented!();
    }
    fn r_value( &mut self ) -> f32 {
        unimplemented!();
        assert!( self.x.data.len() == self.y.data.len() );
        match self.r_value {
            Some( r ) => return r,
            None => {
                let mut sum = 0_f32;
                for i in 0..self.x.len() {
                    sum += ( self.x[i] - self.x.mean() ) * ( self.y[i] - self.y.mean() );
                }
                self.r_value = Some( sum / ( self.x.stdev() * self.y.stdev() ) );
                return self.r_value.unwrap();
            },
        }
    }
}
#[test]
fn regression_construction() {
    let a1 = Array::new( vec![ 1_f32, 2.4, 4.5 ] );
    let cloned_a1 = a1.data.clone();
    let a2 = Array::new( vec![ 2_f32, 2.33, 5.67 ] );
    let reg = Regression::new( a1, a2 );
    assert!( reg.x.data == cloned_a1 );
}
#[test]
fn regression_r_value() {
    let a1 = Array::new( vec![ 1_f32, 2.0, 3.0, 4.0 ] );
    let a2 = Array::new( vec![ 1_f32, 2.0, 3.0, 4.0 ] );
    let mut reg = Regression::new( a1, a2 );
    assert!( reg.r_value() == 1_f32 );
}
pub fn test() {
    let a1 = Array::new( vec![ 1_f32, 2.0, 3.0, 4.0 ] );
    let a2 = Array::new( vec![ 1_f32, 2.0, 3.0, 4.0 ] );
    let mut reg = Regression::new( a1, a2 );
    println!( "r value: {}", reg.r_value() );
}
