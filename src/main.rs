#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

mod snh;
mod control;
mod data_structs;
mod enums;
mod unions;
mod option;
mod arrays;
mod opover;

const DEATH_BY:i32 = 787;
static mut SOMETHING_STATIC:i8 = 123;

fn data_types() {
    println!("Hello, world!");

    let a:u8 = 12;
    println!("a = {}", a);

    let mut b = 0;
    println!("b = {}", b);

    b = 7;
    println!("b = {}", b);

    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);

    let d:char = 'x';

    let e:f32 = 2.5;

    let g = false;
    let a = 2;
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}",a, a_cubed);

}

fn scope_and_shadowing(){
    let a= 123;
    println!("a = {}", a);
    {
        let b = 009;
        println!("b = {}", b);

    }
}

fn main(){
    opover::operator_overload()
}