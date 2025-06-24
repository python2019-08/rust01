
// main.rs

use rust01::test01::{
    test_closure, 
    test_ref_ownship,
    test_vec ,
    test_slice,
    test_struct,
    test_result
};

fn main() {
    test_result::test_result();
    test_struct::test_struct();
    test_slice::test_slice();

    test_vec::test_vec();
    test_closure::test_closure();
    test_ref_ownship::test_ref_1();

    println!("Hello, world!");
}