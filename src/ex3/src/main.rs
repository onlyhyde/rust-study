fn mutable_test() {
    // mutalable vs immutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn constants_test() {
    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");
}

fn shadowing_test() {
    // Shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces : {spaces}");
}

fn data_types_test() {
    // Data Types
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess : {guess}");

    // Scalar Types
    // integer Types
    // singed : i8, i16, i32, i64, i128, isize
    // unsigned : u8, u16, u32, u64, u128, usize
    //let mut overflowtest: u8 = 255;
    let overflowtest: u8 = 255;
    //let overflowtest = overflowtest + 1;
    println!("{overflowtest}");

    // Floating-Point Types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{:.2}, {:10.2}", x, y);

    // Numeric Operations
    // addition
    let sum = 5 + 10;
    println!("{sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{:.2}", difference);

    // multiplication
    let product = 4 * 30;
    println!("{product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("{:.10}", quotient);
    println!("{:.10}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("{remainder}");

    // Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation
    if t {
        println!("true");
    } else if f {
        println!("false");
    }

    // Character types
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{}, {}, {}", c, z, heart_eyed_cat);

    // compound types
    // tutple type
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);

    // Array Types
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    let a = [3; 5];
    println!("{:?}", a);
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{:?}", months);
}

fn function_test() {
    another_functions(5);
    statements_expressions_test();
    functions_return_test();
}

fn another_functions(x: i32) {
    println!("The value of x is: {x}");
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value}{unit_label}");
}

fn statements_expressions_test() {
    let y = {
        let x = 3;
        x + 1 // expression. so return is possible
              //x + 1; // statement. so return is impossible
    };
    println!("The value of y is: {y}");

    let x = {
        let y = 6;
        y
    };
    println!("The value of x is: {x}");
}

fn functions_return_test() {
    let x = five();
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn comments_test() {
    // blah blah blah
}

fn control_flow_test() {
    if_expression_test();
    loop_test();
}

fn if_expression_test() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn loop_test() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // return value
        }
    };
    println!("The result is: {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // conditional loops with while
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping through a collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("{:?}", a[index]);
        index += 1;
    }

    for element in a {
        println!("{:?}", element);
    }

    for element in a.iter().rev() {
        println!("{:?}", element);
    }

    for element in (1..4).rev() {
        println!("{:?}", element);
    }
}

fn main() {
    mutable_test();
    constants_test();
    shadowing_test();
    data_types_test();
    function_test();
    comments_test();
    control_flow_test();
}
