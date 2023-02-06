//! # Rust tutorial 0002 - function and modules
//! Jay Kim (changjae.kim@gmail.com)
//! 2023 version 0.0.1

// Packages: A Cargo feature that lets you build, test, and share crates
// Crates: A tree of modules that produces a library or executable
// Modules and use: Let you control the organization, scope, and privacy of paths
// Paths: A way of naming an item, such as a struct, function, or module

// A package is a bundle of one or more crates that provides a set of functionality.
// A package contains a Cargo.toml file that describes how to build those crates.
// A package can contain as many binary crates as you like, but at most only one library crate.
// crate - binary crate and library crate.
// crate root is the source file that the Rust compiler start from.
// usually crate root is src/lib.rs for a library crate or src/main.rs for a binary crate

// mod module1 can be found in the below files
//    /src/module1.rs
// or /src/module1/mod.rs

// submodule sub_moule1 can be found in the below files
//    /src/module1/sub_module1.rs
// or /src/module1/sub_module1/mod.rs

// module is private by default. need "pub" to make it public.

// path: crate::module1::sub_module1::pub_function
// "use" to make a path as a short cut.

use crate::module1::sub_module1::pub_function;
pub mod module1;
/// main function
fn main() {
    // statement: instruction and do not return a value
    // expressions: retun value
    let mut x:u8 = 1; // statement, let x = let y = 1 is not acceptable.
    x = x + 1; // x + 1 is expression so it can be assigned to mutable variable x.
    println!("x is {}", x);
    let result = add(1, 2);
    println!("result of add(1, 2) is {}", result);
    // expression has return value.
    let val_01 = {
        let b = 3;
        let c = 4;
        b + c  // this is expression (do not have ;)
    }; // ; is needed here because this is end of statement.
    println!("value 01 is {}", val_01);
    let ret = condition_function(-1);
    println!("ret is {}", ret);
    tutorial_function_ownership();
    pub_function();
    crate::module2::pub_function2(); // absolute path, which is start from crate::
    tutorial_as();
    tutorial_rand();
}

// function return is the expression at the end of the function without ;
fn add(x:i32, y:i32) -> i32 {
    x + y // expression, without ;
}

fn condition_function(in_value:i32) -> i32 {
    // we can use return keyward when we return in the middle of the function.
    if in_value < 0 {
        return -1; // Error return
    }
    0 // Non-error return
}

fn tutorial_function_ownership() {
    let mut cnt:u32 = 10;
    cnt = inc_number(cnt); // cnt is parameter and cnt value is copied to the function parameter in_value
    println!("cnt is {cnt}");
    let mut str_1 = String::from("str_1");
    str_1 = add_new_str(str_1, " added str".to_string()); // string literal need to be changed to string type
    // str_1 is moved to the in_str of the add_new_str function. and returned string is moved to str_1.
    println!("Added str is {}", str_1);
    let mut str_2 = add_new_str(str_1, " end of the string".to_string());
    // println!("Added str is {}", str_1); // this code trigger error because str_1 is moved to the in_str of the add_new_str and droped.
    println!("Added str is {}", str_2);
    let str_3: &String = bypass_str_ref(&str_2); // str_3 is reference String.
    println!("Added str is {}", str_3);
    add_str_mut_str(&mut str_2); // mutable reference of the str_2
    println!("Added str is {}", str_2);
    let str_4 = &mut str_2; // reference of the str_2
    println!("str_4 is {}", str_4); // this code is added to avoid warning (unused variable)
    let str_5 = &mut str_2; // second reference is not allowed. so str_4 is droped here and str_5 is the only reference of str_2.
    println!("str_5 is {}", str_5);
    // basically, the value has only one owner which can change the value (mutable).
    let mut srt_mut: String = String::from("Mutable string");
    let str_6: &String = &srt_mut; // multiple immutable reference is allowed.
    let str_7: &String = &srt_mut;
    let str_8: &String = &srt_mut;
    //let str_9: &mut String = &mut srt_mut; // this is not allowed because str_2 already have a multiple immutable reference. only one mutable reference is allowed.
    println!("str_6 is {}", str_6);
    println!("str_7 is {}", str_7);
    println!("str_8 is {}", str_8);
    let str_9: &mut String = &mut srt_mut;
    println!("str_9 is {}", str_9); // this is OK because str_6, str_7, and str_8 are used and droped from here.
    // println!("str_6 is {}", str_6); // This is not allowed because the str_6 is droped at the line 69
}

fn inc_number(in_value: u32) -> u32 {
    in_value + 1 // return this value this value is copyed to the return. and in_value is droped.
}

fn add_new_str(mut in_str: String, add_str: String) -> String {
    let address_str: &String = &add_str; // &add_str is addres of the add_str, it's the reference of the add_str.
                                         // address_str has 32 or 64 bits address in the stack and it points the addres of the add_str in the stack.
                                         // add_str {add, len, capacity} is in the stack and string data is in the heap.
    in_str.push_str(address_str); // reference does not have the ownership of the data so even the reference droped, the real data is not droped.
    println!("address_str is {address_str}"); // address_str is not droped becuase it's in the stack.
    in_str
}

fn bypass_str_ref(in_str: &String) -> &String {
    // reference is borrowing the data and it's immutable. So in_str can't be modified. new need to change it mutable.
    in_str
}

fn add_str_mut_str(in_str: &mut String)  { // this is the mutable reference. func(&mut s), fn func(in_data: &mut String)
    in_str.push_str(" this is real end");
    // do not need to return the modified string because real data is modified and bollowed reference will return it to the original owner.
}

// Module creation in the one file.
pub mod module2 {
    mod sub_module2 {
        fn print_sub_module2() {
            println!("print_sub_module2() private function");
        }
        pub fn pub_function3() {
            println!("Call private function, print_sub_module2()");
            print_sub_module2();
        super::pub_function3(); // super means ../
        }
    }
    pub fn pub_function2() {
        sub_module2::pub_function3(); // relative path.
    }
    pub fn pub_function3() {
        println!("pub_function3()");
    }
}

// as to rename the path
// when you add 'pub' before use, we can use this new name (as) or short name in the outer of the module
pub use crate::module2::pub_function3 as new_pub_function3;

fn tutorial_as() {
    new_pub_function3();
}

// to use external package we need to change Cargo.toml file
// find out the external package 'https://crates.io/'
// 'https://crates.io/crates/rand' this is the rand package

// "*" is glop operator which is all the public items defined in the scope.
use rand::prelude::*;
// Nested path
// use rand::{rngs, seq}
// This is the same as below.
// use rand::rngs;
// use rand::seq;
// We can use self in the nested path
// use rand::{self, rngs, seq}
// ==> use rand;
    // use rand::rngs;
    // use rand::seq;

fn tutorial_rand() {
    let rand_num:u32 = rand::random::<u32>();
    println!("Random u32 number is {}", rand_num);
}


