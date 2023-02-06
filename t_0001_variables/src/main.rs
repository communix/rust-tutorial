//! # Rust tutorial 0001 - variables
//! Jay Kim (changjae.kim@gmail.com)
//! 2023 version 0.0.1

/// main function
fn main() {
    // println! is macro
    // @todo: need to understand the difference between macro and function.
    let x:u32 = 3334452;
    println!("first variable x is {}", x);
    let x:u8 = 255;
    // This is called shadowing. reuse the same variable name.
    // The first variable is shadowed by the second.
    // difference between mut and shadowing, shadowing can change the type.
    // The first variable x is not valid from here.
    // if x value is greater than 255, it trigger overflow error
    println!("second variable x is {}", x);

    // Scalar types
    // integers: i8, u8, i16, u16, i32, u32, i64, u64, isize, usize
    // number: desimal 100_000 (_ is comma), hex 0xFF, Octal 0o74, binary 0b0101_1100, byte b'A'(8bit)
    // Default number is i32
    // Flating-point type
    // f32, f64, f64 is default.
    let float_number1:f32 = 2.543;
    let float_number2:f64 = 2.443;
    // boolean type
    let is_enabled:bool = true; // or false
    // chracter type
    let c_type:char = 'a'; // single quotes
    println!("{} {} {} {}", float_number1, float_number2, is_enabled, c_type);
    // Compound type - Tuple and arrays.
    // tuple is immutable and save in the stack because the length is fixed.
    // mutable tuple : with mut keyward.
    let tup:(u8, u32, char) = (1, 353456, 'b');
    let (t1, t2, t3) = tup;
    println!("t1 {}, t2 {}, t3 {}", t1, t2, t3);
    // for i in 1..3 {
    //     println!("Tuple tup {} is {}", i, tup.i); // This is not valied.
    //                                                  there's no function for the length of the tuple
    //                                                  and can't access with variable
    // }
    println!("Tuple tup.0 is {}", tup.0);

    let u32_data = 12_1234u32; // You can put data type at the end of the data.
    println!("u32_data is {}", u32_data);
    // Array, array has a fixed length (always). Can't add or remove element.
    // Array is saved in the stack because the length is fixed.
    // Array element can be mutable with mut keyward.
    let arr:[i32; 5] = [1, 2, 3, 4, 5];
    for i in 0..arr.len() {
        println!("array {} element is {}", i, arr[i]);
    }
    let mut arr:[i8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    arr[3] = 100;
    for i in 0..arr.len() {
        println!("array {} element is {}", i, arr[i]);
    }
    // array elements is iterable in the loop
    for value in arr.iter() {
        println!("array element {}", value);
    }
    // const is different from let. it's immutable and must have type.
    const MAX_VALUE:u32 = 23_000 * 34;
    println!("Maximum value is {}", MAX_VALUE);
    tutorial_slice();
}

fn tutorial_slice() {
    // Slice is a part of a array. String slice is part of the String.
    let arr_in:[u32;5] = [1, 2, 3, 4, 5];
    let arr_slice = &arr_in[3..5]; // from index 3 to 4 (5 - 1), slice type is &[u32]
    println!("arr_slice [0] {} and arr_slice[1]:{}", arr_slice[0], arr_slice[1]);
}