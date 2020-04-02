// operator overloading
use std::ops::{Add, AddAssign, Neg};
use std::cmp::PartialEq;
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

impl<T> Add for Complex<T>  where T: Add<Output = T> {
    type Output = Complex<T>;

    fn add(self , rhs: Self) -> Self::Output {
        Complex {
            re:  self.re + rhs.re,
            im:  self.im + rhs.im
        }
    }
}

impl<T> AddAssign for Complex<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self){
        self.re  += rhs.re;
        self.im  += rhs.im
    }
}

impl<T> Neg for Complex<T> where T: Neg<Output=T> {
    type Output = Complex<T>;

    fn neg(self)  -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im
        }
    }
}

impl<T> PartialEq for Complex<T> where T:PartialEq{
    fn eq(&self , rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

pub fn operator_overload(){
    let a =  Complex::new(5, 1);
    let b =  Complex::new(7, 3);

    // println!("{:?}", a+b);

    // a += b;
    // println!("{:?}", a);

    // println!("{:?}", -a);
    println!("{:?}", a==a);

}
