//! # Rust tutorial 0004 - String
//! Jay Kim (changjae.kim@gmail.com)
//! 2023 version 0.0.1

fn main() {
    tutorial_string_type();
    tutorial_slice();
}

fn tutorial_string_type() {
    let s_lit = "Hello";                       // String literal, immutable. data is stored in the stack.
    let s_str = String::from("String");        // String type, variable is stored in the stack but string data is stored in the heap.
    let mut s_mut_str = String::from("");
    s_mut_str.push_str("New string");
    println!("{}, {}, {}", s_lit, s_str, s_mut_str);
    // memory allocation.
    // s_str : stack, { str(porinter of the heap data), len(current length), capacity(total amount of memory)}, str is the heap address of the str
    //         heap {index and value}
    // heap memory is feeded (droped) when the s_str is droped.

    // copy and move
    // copy - stack only data. integer type, boolena type, floating pint type, char type and tuples if it only contain types of copy.
    // move - tuples which contain types of move. other types(use heap memory)
    // function parameter and return - the same method. copy or move. (see the t_0002_function_modules)

    // example of copy
    let cp1:u32 = 100;
    let cp2:u32 = cp1; // copy cp1 value to cp2. both cp1 and cp2 are not droped.
    println!("cp1 {cp1} and cp2{cp2} are still alive");
    let str1 = String::from("String 1");
    let str2 = str1; // move str1 to str2, str1 is droped now.
    println!("str2 {str2} but str1 is droped");
}

fn tutorial_slice() {
    // range syntax '..', it can be used for iteration 1..10 (from 1 to 10 -1)
    // slice use range syntax in the []
    // slice save the {ptr len} in the stack.
    let str1 = String::from("This is the first string.");
    let str_this = &str1[0..4];
    let str_this_2 = &str1[..4]; // 0 can be removed
    println!("str_this is {str_this} and str_this_2 is {str_this_2}");
    let str_first = &str1[12..17];
    println!("str_first is {str_first}");
    let str_end = &str1[18..]; // the last can be removed.
    println!("str_end is {str_end}");
    let str_slice = &str1[..]; // start 0 to then end
    println!("str_slice is {str_slice}");

    // String literal is String slice type (&str) &str is immutable reference.
    let str_literteral = "Nice to meet you";
    // Generally, use &str instead of the String in the furnction parameter and return.
}