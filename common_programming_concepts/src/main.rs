use std::io;

fn main() {
    // mutable variables
    let mut x = 5;                      // without 'mut', we couldn't reassign [x]
    println!("x: {x}");
    x = 6;
    println!("x: {x}");

    // chars
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{c} {z} {heart_eyed_cat}");

    // tuples (unfortunately they don't have a default formatter)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let _tup2: (bool, char, (i32, f64, u8)) = (true, '\\', tup);

    let (mut i, mut f, mut u) = tup;
    // let mut (i, f, u) = tup;     would be illegal
    println!("({i}, {f}, {u})");
    i = tup.0;
    f = tup.1;
    u = tup.2;
    println!("({i}, {f}, {u})");

    // arrays
    let mut _arr: [i32; 5] = [1, 2, 3, 4, 5];
    _arr = [3; 5];   // 3 is the initial value, while 5 is the size

    // std::io
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

    // functions
    functions(element);

    let mut sendHelp = five();
    println!("{sendHelp}");
    sendHelp = loop_de_loop();
    println!("{sendHelp}");
}

fn functions(x: i32)
{
    println!("\n\nfunctions entered with value {x}");

    let y = {
        let x = 3;
        x + 1;
        'c'   // it is important that here is no semicolon; i.e. it evaluates to the last expression in this block-statement
    };
    
    println!("The value of y is: {y}");
}

// this is valid, it returns 6, obviously
fn five() -> i32
{
    5;
    // return 5; return statements are still perfectly fine

    if (true) {6} else {7}
}

// returns the loop evaluated to an expression, which is 20
fn loop_de_loop() -> i32
{
    let mut counter = 0;

    let mut number = 3;

    while number != 0
    {
        println!("{number}!");

        number -= 1;
    } // while-loops don't have a semicolon at the end (same for for-loops)

    for number in (1..4).rev() // 4 is not included here, i.e. 1 <= number < 4
    {
        println!("{number}!");
    }

    loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }
}
