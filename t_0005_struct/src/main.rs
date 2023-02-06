//! # Rust tutorial 0005 - Structure
//! Jay Kim (changjae.kim@gmail.com)
//! 2023 version 0.0.1

fn main() {
    tutorial_structure_init();
    let edward: String = String::from("Edward");
    let people_1 = init_people(edward, 20);
    println!("Name: {}, Age: {}, Male? {}", people_1.name, people_1.age, people_1.male);
    tutorial_tuple_struct();
    tutorial_unit_like_structure();
    tutorial_printout_structure();
    tutorial_method();
}

struct People {
    name: String,
    age: u8,
    male: bool,
}

fn tutorial_structure_init() {
    let mut john = People {
        name: String::from("John"),
        age: 10,
        male: true,
    };
    john.age = 30; // Update Age.

    // create new instance using john's data
    let mary = People {
        name: String::from("Mary"),
        ..john  // ..john means use john's data for the remaing elements. Need to be the last and nd do not use ;
    };
    // In this case, we can use john and mary because mary just copy the data from john and there's no data to be moved (Like String)
    // If any of the copied elements are moved. the original structure is droped.
    println!("Name: {}, Age: {}, Male? {}", john.name, john.age, john.male);
    println!("Name: {}, Age: {}, Male? {}", mary.name, mary.age, mary.male);
}

fn init_people(name: String, age: u8) -> People {
    People { // Do not need to use let People and return it. Just use expression.
        name: name,            // use input parameter, name.
        age, // we can use "age: age" but it can be replaced to name if element name and the parameter name is the same.
        male: true,     // Default is true
    }
}

fn tutorial_tuple_struct() {
    // Tuple structure does not have name field.
    struct Color(u32, u32, u32); // RGB color set. no specific name.
    let color = Color(10, 30, 50);
    println!("Color code R {}, G {} B {}", color.0, color.1, color.2); // use index to access it.
}

fn tutorial_unit_like_structure() {
    // Unit-like structure does not have any fields
    struct NoField;
    let _nofield = NoField;
}

fn tutorial_printout_structure() {
    #[derive(Debug)]
    struct NewStruct {
        name: String,
        number: u32,
    }
    let new = NewStruct {
        name: String::from("Name"),
        number: dbg!(1), // print debug information and it return the expression.
    };
    println!("{:?}", new); // This is how to print structure for debug.
    dbg!(&new); // More information for debug. 
    println!("Structure name:{}, number:{}", new.name, new.number);
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn init(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Rect {
    p1: Point,
    p2: Point,
}

impl Rect {
    // All functions defined within an impl block are called associated functions.
    // Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
    fn init(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }
}

// It's OK to have multiple impl for the one structure.
impl Rect {
    fn area(&self) -> i32 {
        let length: i32 = dbg!(self.p2.x - self.p1.x);
        let height: i32 = dbg!(self.p2.y - self.p1.y);
        let area:i32 = length * height;
        if area > 0 {
            area
        } else {
            -1 * area
        }
    }
}

fn tutorial_method() {
    let p1 = Point {
        x: 0,
        y: 0
    };
    let p2 = Point {
        x: 10,
        y: 10,
    };
    let rect1 = Rect {
        p1: p1,
        p2: p2, 
    };
    let p3 = Point::init(10, 20);
    let p4 = Point::init(30, 40);
    let rect2 = Rect::init(p3, p4);
    let area1 = rect1.area();
    let area2 = rect2.area();
    println!("Area are {area1} and {area2}");
}