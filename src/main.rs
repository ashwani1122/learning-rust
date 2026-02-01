use std::io;

fn main() {

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
    main2();
    let a = 5;
    let b = 7;

    println!("a = {a}, b = {b}");
    print!("this is the fn value five {}" , five());
    plus_one(10);

    
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn five() -> i32 {
    5
}

fn main2() {
    println!("Hello, world! main2");
    another_function();
}

fn another_function() {
    println!("Another function.");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}



