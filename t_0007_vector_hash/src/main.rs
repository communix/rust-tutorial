//! # Rust tutorial 0007 - vector and hash - common collectors
//! Jay Kim (changjae.kim@gmail.com)
//! 2023 version 0.0.1

fn main() {
    tutorial_vactor();
    tutorial_hashmap();
}

fn tutorial_vactor() {
    // create new empty vector
    let mut vec_u32:Vec<u32> = Vec::new(); // type annotation is needed to create empty vector
    // let vec_u16 = Vec::new(); // this is not allowed because there is no type annotation. 
    // create new vector with init value
    let vec_i32 = vec![1, 2, 3, 4]; // use vec! macro the default is i32 for number if it does not have type annotation
    // Adding element
    vec_u32.push(100);
    vec_u32.push(200);
    // Access element with index - the retuend value is the same type
    println!("This is the first element: {} and {}", vec_u32[0], vec_i32[0]);
    let u32_second_no: &u32 = &vec_u32[1];
    let i32_second_no: &i32 = &vec_i32[1];
    println!("This is the second element: {} and {}", u32_second_no, i32_second_no);
    // Accessing element with get method - The retuned value is Option<&T>
    // we can handle out of index case with get method.
    // vec_u32[2] triggers panic because it does not have element of the index 2.
    let u32_third_get: Option<&u32> = vec_u32.get(2);
    match u32_third_get {
        Some(n) => println!{"Third u32 element is {}", n},
        None => println!{"There is no third element in the u32_vector"},
    }
    // iterating
    for i in &mut vec_u32 {
        *i += 10; // we need to have * (dereference) to change the value of the reference.
        print!("{}, ", i);
    }
    println!("");

    // use enum to store different type in a vector
    enum EType {
        EInt(i32),
        EStr(String),
    }
    let mut evector = vec![EType::EInt(100), EType::EStr("Hello World".to_string())];
    match evector.pop() { // Pop return the last index (for stack) and it's removed in the vector.
        Some(n) => {
            match n {
            EType::EInt(n) => println!("This is number type {}", n),
            EType::EStr(n) => println!("This is String type {}", n),
            }
        },
        None => println!("It's empty"),
    }
}

use std::collections::HashMap; // Need it to use HashMap

fn tutorial_hashmap() {
    let mut hash_map:HashMap<String, u32> = HashMap::new(); // Type annotation is needed for empty hashmap
    let key_1 = String::from("Key1");
    hash_map.insert(key_1, 1); // Ownership of the key_1 moves to hash_map. So key_1 is dropped at this point.
    hash_map.insert(String::from("Key2"), 2);
    let key_2:String = String::from("Key2");
    let value_key2 = hash_map.get(&key_2).copied().unwrap_or(0); // get returns Option<&u32> and copied convert it to Option<32>
                                                                 // unwrap_or(0) convert Option<u32> to u32, or 0 if it's None.
    println!("The value in the key{} is {}", key_2, value_key2);
    hash_map.insert(String::from("Key2"), 3); // insert methos overwrite the value if the key is already existed.
    hash_map.entry(String::from("key3")).or_insert(4); // entry return Entry enum and or_insert method insert new value if key is not existed.
    let hash_value = hash_map.entry(String::from("key3")).or_insert(0); // hash_value is the address of the hash_value.
    *hash_value += 100; // * is needed to change the value.
    for (key, value) in &hash_map {
        println!("Key {} has {}", key, value);
    }
}