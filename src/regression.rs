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
    pub fn new( a1: Array, a2: Array ) -> Regression {
        assert!( a1.data.len() == a2.data.len() );
        Regression {
            x: a1,
            y: a2,
            lsrl: None,
            r_value: None,
        }
    }
    pub fn from( v1: Vec<f32>, v2: Vec<f32> ) -> Regression {
        Regression::new( Array::new(v1), Array::new(v2) )
    }
    pub fn r_value( &mut self ) -> f32 {
        assert!( self.x.len() == self.y.len() );
        match self.r_value {
            Some( r ) => return r,
            None => {
                let mut sum = 0_f32;
                for i in 0..self.x.len() {
                    sum += ( self.x[i] - self.x.mean() ) * ( self.y[i] - self.y.mean() );
                }
                let coef_n = 1f32 / self.x.len() as f32;
                self.r_value = Some( coef_n * (sum / ( self.x.stdev() * self.y.stdev() ) ) );
                return self.r_value.unwrap();
            },
        }
    }
    pub fn least_squares(&mut self) -> Line {
        match self.lsrl {
            Some( line ) => return line,
            None => {
                let b = self.r_value() * ( self.y.stdev() / self.x.stdev() );
                let m = self.y.mean() - ( b * self.x.mean() );
                self.lsrl = Some( Line::new( m, b ) );
                return self.lsrl.unwrap();
            }
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
    let mut reg2 = Regression::from( vec![ 1.0f32,2.0,3.0,4.0,65.0,23.0 ], vec![ 23.3f32,12.2,13.4,12.0,23.44,14.3 ] );
    assert!( reg2.r_value() == 0.5504537f32 );
}
#[test]
fn regression_least_squares() {
    let mut reg = Regression::from( vec![1.0f32,2.0,3.0,4.0], vec![2.3, 3.55, 4.56, 3.45] );
    let line = reg.least_squares();
    assert!( line.slope ==  2.35f32 );
    assert!( line.y_int == 0.44600004f32 );
}
