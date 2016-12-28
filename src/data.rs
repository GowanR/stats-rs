pub struct Array {
    data: Vec<f32>,
    mean: Option<f32>,
    sum: Option<f32>,
    median: Option<f32>,
    stdev: Option<f32>,
    var: Option<f32>
}
impl Array {
    pub fn new( d: Vec<f32> ) -> Array {
        Array {
            data: d,
            mean: None,
            median: None,
            stdev: None,
            sum: None,
            var: None,
        }
    }
    pub fn sum(&mut self) -> f32 {
        match self.sum {
            Some( x ) => return x,
            None => {
                let mut s = 0_f32;
                for i in 0..self.data.len() {
                    s += self.data[i];
                }
                self.sum = Some( s );
                return self.sum.unwrap();
            },
        }
    }
    pub fn mean(&mut self) -> f32 {
        match self.mean {
            Some( x ) => return x,
            None => {
                self.mean = Some( self.sum() / self.data.len() as f32 );
                return self.mean.unwrap();
            }
        }
    }
    pub fn varience(&mut self) -> f32 {
        match self.var {
            Some(x) => return x,
            None => {
                let mut s = 0_f32;
                for i in 0..self.data.len() {
                    s += (self.data[i] - self.mean()).powi(2);
                }
                s /= self.data.len() as f32;
                self.var = Some( s );
                self.stdev = Some( self.var.unwrap().sqrt() );
                return self.var.unwrap();
            }
        }
    }
    pub fn stdev(&mut self) -> f32 {
        match self.stdev {
            Some(x) => return x,
            None => {
                return self.varience().sqrt();
            }
        }
    }
    pub fn median(&mut self) -> f32 {
        unimplemented!();
    }
}

#[test]
fn array_construction() {
    let input_data: Vec<f32> = vec![ 1.0_f32, 2.0, 3.0 ];
    let data = Array::new( input_data );
}
#[test]
fn array_sum() {
    let input_data: Vec<f32> = vec![ 1.0_f32, 2.0, 3.0 ];
    let mut data = Array::new( input_data );
    assert!( data.sum() == 6.0_f32 );
    let input_data2: Vec<f32> = vec![ 1.0_f32, 6.0, 3.0, 78.0, 0.5 ];
    let mut data2 = Array::new( input_data2 );
    assert!( data2.sum() == 88.5_f32 );
}
#[test]
fn array_mean() {
    let input_data: Vec<f32> = vec![ 1.0_f32, 2.0, 3.0 ];
    let mut data = Array::new( input_data );
    assert!( data.mean() == 2.0_f32 );
    let input_data2: Vec<f32> = vec![ 56.0_f32, 2.0, 3.0, 45.0 ];
    let mut data2 = Array::new( input_data2 );
    assert!( data2.mean() == 26.5_f32 );

}
#[test]
#[ignore]
fn array_median() {
    unimplemented!();
}
#[test]
fn array_varience() {
    let input_data: Vec<f32> = vec![ 1.0_f32, 2.0, 3.0, 4.0 ];
    let mut data = Array::new( input_data );
    assert!( data.varience() == 1.25_f32 );

    let input_data2: Vec<f32> = vec![ 1.0_f32,2.0,3.0,1.0,43.0 ];
    let mut data2 = Array::new( input_data2 );
    assert!( data2.varience() == 272.8_f32 );
}
#[test]
fn array_stdev() {
    let input_data: Vec<f32> = vec![ 1.0_f32,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0 ];
    let mut data = Array::new( input_data );
    assert!( data.stdev() == (14_f32).sqrt() );
    let input_data2: Vec<f32> = vec![ 1_f32,2.0,3.0,4.0,5.0,6.0,7.0 ];
    let mut data2 = Array::new( input_data2 );
    assert!( data2.stdev() == 2_f32 );
}
pub fn stdev_test() {
    let input_data: Vec<f32> = vec![ 1.0_f32,2.0,3.0,4.0,5.0,2.0,3.0,5.0,5.0 ];
    let mut data = Array::new( input_data );
    println!("var: {}", data.varience() );
}
