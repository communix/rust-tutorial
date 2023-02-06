//! # Rust tutorial 0006 - Enumeration and pattern matching
//! Jay Kim (changjae.kim@gmail.com)
//! 2023 version 0.0.1

/// Enum for Gender and it has two types but it does not have any actual data value.
#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
struct User {
    name: String,
    gender: Gender,
    age: u8,
}

/// Enum can have different data types
#[derive(Debug)]
enum CommonType {
    None, // No data
    Address(String), // String
    AddressWithZip {addr:String, zip:u32}, // Struct
    Points(i32, i32), // Tuple
}

fn main() {
    let user1 = User {
        name: String::from("Edword"),
        gender: Gender::Male, // Assign Male from enum.
        age: 30,
    };
    println!("{:?}", user1);
    let commontype1 = CommonType::None;
    println!("{:?}", commontype1);
    let commontype2 = CommonType::Address(String::from("1234 broadway load, San Jose, CA"));
    println!("{:?}", commontype2);
    let commontype3 = CommonType::AddressWithZip { addr: String::from("1234 broadway load, San Jose, CA"), zip: 95000 };
    println!("{:?}", commontype3);
    let commontype4 = CommonType::Points(10,30);
    println!("{:?}", commontype4);

    tutorial_option();
    tutorial_match();
    tutorial_pattern();
}

fn tutorial_option() {
    // Rust does not have NULL value, instead of the NULL value it has a None value.
    // Option enum is defined in the standard library and <T> is generic type(u32, u8, bool...)
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    let some_value:Option<u32> = Some(10);
    let none_value:Option<i32> = None;
    // Enum value is not the same as generic type. some_value + 10 is invalid expression.
    // We need to convert Enum Option to generic type if it's not None using match.
}

fn tutorial_match() {
    //match can handle all the type and looks like swich-case in C
    let number:u32 = 20;
    match number {
        0 => { println!("The number is zero"); },
        10 => number_10(),
        other => { println!("The number is {}. it's not 0 or 10", other); },
        // _ => (), // _ can't use the value. and () means do not run any code.
        // Match must handle all the cases. if there's any missing value, it trigger the error.
        // other or _ covers all the other values (default in switch-case in C)
    }
    enum Month {
        Jan,
        Feb,
        Mar,
        Apr,
        May,
        Jun,
        Jul,
        Aug,
        Sep,
        Oct,
        Nov,
        Dec,
    }
    fn get_month(mon: &Month) -> u8 { // this function convert enum to u8
        match mon {
            Month::Jan => 1,
            Month::Feb => 2,
            Month::Mar => 3,
            Month::Apr => 4,
            Month::May => 5,
            Month::Jun => 6,
            Month::Jul => 7,
            Month::Aug => 8,
            Month::Sep => 9,
            Month::Oct => 10,
            Month::Nov => 11,
            Month::Dec => { println!("Merry Christmas");
                            12 // expression
                          },
        }
    }
    let january: Month = Month::Jan;
    let december: Month = Month::Dec;
    println!("Jan is {}", get_month(&january));
    println!("Dec is {}", get_month(&december));

    let num_1 = Some(5);
    let num_2 = None;
    println!("Number {}", get_num(num_1));
    println!("Number {}", get_num(num_2));
    fn get_num(num: Option<u32>) -> u32 {
        match num {
            Some(x) => x,
            None => { println!("None");
                        0
                    },
        }
    }
    match num_2 {
        Some(x) => { println!("It's u32 value {}", x); },
        _ => (), // Doing nothing if it's None
    }
    // if let can be used for matching
    // if let is less verbose than match
    // It's good to use when you want to handle only one type of the enum value.
    if let Some(x) = num_1 {
        println!("The number is {}", x);
    }

    // if let can be combined with if, else if let, else if
    let this_month: Month = Month::Sep;
    if let Month::Jan = this_month {
        println!("It's January");
    } else if let Month::Dec = this_month {
        println!("It's December");
    } else if number == 10 {
        println!("Number is not a enum type, it's just a number {}", number);
    }
    else {
        println!("It's not January or December");
    }
}

fn number_10() {
    println!("The number is 10");
}

fn tutorial_pattern() {
    // We can find pattern in the Rust code.

    // 1) match
    // match value {
    //   pattern => expression,
    // }

    // 2) if let

    // 3) while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);

    while let Some(num) = stack.pop() {
        println!("Numer in stack is {}", num);
    }

    // the value that directly follows the 'for' keyword is a pattern.
    let ar:[u32;3] = [1, 2, 3];
    for (ind, data) in ar.iter().enumerate() { //(ind, data) is a pattern.
        println!("{} is {}", ind, data);
    }

    // the value that directly follows the 'let' keyword is a pattern.
    // let pattern = expression;
    // let x = 5 means bind what matches here to the vairable x.
    // this pattern x means "bind everything to the variable x, whatever the value is"
    let (x, y, z) = (0, 1, 3);

    // function parameter is a pattern.

    // Patterns come in two forms: refutable and irrefutable.
    // Patterns that will match for any possible value passed are irrefutable.
    // Function parameters, let statements, and for loops can only accept irrefutable patterns
    // The if let and while let expressions accept refutable and irrefutable(Trigger warning) patterns.
    if let k = 5 { // trigger warning because it's alreays matched.
        println!("k is {}", k);
    }
}