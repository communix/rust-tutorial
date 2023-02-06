//! # Rust tutorial 0003 - control flow
//! Jay Kim (changjae.kim@gmail.com)
//! 2023 version 0.0.1

fn main() {
    tutorial_if();
    tutorial_loop();
    tutorial_for();
}

fn tutorial_if() {
    //if state must have bool condition, number is not acceptable.
    let number = -10;
    if number < 0 {
        println!("Number {} is minus", number);
    } else if number == 0 {
        println!("Number {} is zero", number);
    } else {
        println!("Number {} is plus", number);
    }

    // example of let with if
    let abs_number = if number < 0 {
        -1 * number // expression, no ;
    } else {
        number // must be the number type because the first retrun(-1 * number) is number type, string or boolen type are not accepted.
    };
    println!("Absolute value of {number} is {abs_number}");
}

fn tutorial_loop() {
    let mut cnt = 0;
    loop {
        if cnt >= 10 {
            break; // Exit loop
        }
        cnt += 1;
    }
    println!("looping {} times", cnt);

    // return value from loops
    let cnt2 = loop {
        if cnt >= 10 {
            break cnt; // break return the cnt.
        }
        cnt += 1;
    };
    println!("looping {} times", cnt2);

    // break outer loop with lables
    'loop_1: loop { // lable loop_1
        let mut looping_cnt = 0;
        loop {
            looping_cnt += 1;
            if looping_cnt >= 10 {
                break 'loop_1; // exit loop_1
            }
        }
        // println!("Exit loop_2"); // Trigger warning message because it's not reached.
    }
    println!("Exit loop_1");
    // multiple looping with 
    cnt = 0;
    while cnt < 5 { // while loop with condition.
        cnt += 1;
    }
    println!("looping in while {} times", cnt);
}

fn tutorial_for() {
    // for loop
    let array_01:[u32; 5] = [1, 2, 3, 4, 5];
    let mut sum = 0;
    // for value in array_01.iter() { // .iter() is not used anymore.
    for value in array_01 {
        sum += value;
    }
    println!("Sum in the array is {}", sum);

    for i in 1..5 { // iterator 1..5 means start from 1 to 5 -1 (4)
        print!("{i} ");
        sum += i;
    }
    println!("");
    println!("Sum in the array is {}", sum);

    for i in (1..=10).rev() { // iterator 1..=10 means start from 1 to 10. 1..10 means from 1 to 10 -1 (9)
        print!("{i} ");
        // this is reverse, from 10 to 1
        sum += i;
    }
    println!("");
    println!("Sum in the array is {}", sum);

}