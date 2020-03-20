use std::ops::{Add};
use std::fmt::Debug;

#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T
}

impl <T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> {re, im}
    }
}

pub fn operator_overload(){
    let mut a =  Complex::new(5, 1);
    let mut a =  Complex::new(7, 3);

    println!("{:?}", a);
}