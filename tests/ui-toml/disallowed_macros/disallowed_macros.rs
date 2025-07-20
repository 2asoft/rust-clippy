//@aux-build:macros.rs
//@aux-build:proc_macros.rs

#![allow(unused)]

extern crate macros;
extern crate proc_macros;

use proc_macros::Derive;
use serde::Serialize;

fn main() {
    println!("one");
    println!("two");
    cfg!(unix);
    vec![1, 2, 3];

    #[derive(Serialize)]
    struct Derive;

    let _ = macros::expr!();
    macros::stmt!();
    let macros::pat!() = 1;
    let _: macros::ty!() = "";
    macros::item!();
    let _ = macros::binop!(1);

    eprintln!("allowed");
}

macros::attr! {
    struct S;
}

impl S {
    macros::item!();
}

trait Y {
    macros::item!();
}

impl Y for S {
    macros::item!();
}

#[derive(Derive)]
struct Foo;

pub fn print_1_arg() {
    print!("foo");
    //~^ disallowed_macros
}
pub fn print_2_args() {
    print!("{}", 123);
    //~^ disallowed_macros
}
pub fn print_3_args() {
    print!("{} {}", 123, 456);
    //~^ disallowed_macros
}

pub fn panic_1_arg() {
    panic!("foo");
    //~^ disallowed_macros
}
pub fn panic_2_args() {
    panic!("{}", 123);
    //~^ disallowed_macros
}
pub fn panic_3_args() {
    panic!("{} {}", 123, 456);
    //~^ disallowed_macros
}
