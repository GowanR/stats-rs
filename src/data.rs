use std::ops::Index;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone)]
pub struct Array {
    pub data: Vec<f32>,
    mean: Option<f32>,
    sum: Option<f32>,
    median: Option<f32>,
    stdev: Option<f32>,
    var: Option<f32>
}
impl Index<usize> for Array {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        &self.data[i]
    }
}
impl Debug for Array {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.data )
    }
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
    pub fn len(&self) -> usize {
        return self.data.len();
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
        match self.median {
            Some(x) => return x,
            None => {
                let mut sorted_data = self.data.clone();
                sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
                self.median = Some( sorted_data[sorted_data.len()/2] );
                return self.median.unwrap();
            },
        }
    }
    pub fn reset(&mut self) {
        self.data = vec![];
        self.mean = None;
        self.sum = None;
        self.median = None;
        self.stdev = None;
        self.var = None;
    }
}

#[test]
fn array_construction() {
    let input_data: Vec<f32> = vec![ 1.0_f32, 2.0, 3.0 ];
    let cloned_data = input_data.clone();
    let data = Array::new( input_data );
    assert!( data.data == cloned_data );
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
fn array_median() {
    let input_data: Vec<f32> = vec![ 1.1231_f32,2.123,3.14,4.333,5.4545 ];
    let mut data = Array::new( input_data );
    assert!( data.median() == 3.14 );
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
#[test]
fn array_indexing() {
    let array = Array::new( vec![ 1f32, 2.0, 3.0 ] );
    assert!( array[0] == 1f32 );
    assert!( array[1] == 2f32 );
    assert!( array[2] == 3f32 );
}
#[test]
fn array_len() {
    let array = Array::new( vec![ 1f32, 2.0, 3.0 ] );
    assert!( array.len() == 3 );
}
