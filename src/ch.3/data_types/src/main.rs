fn integer_types() {
    let mut _x: u8 = 255;
    let mut _y: i8 = -128;
    println!("x is {}", _x);
    //println!("overflow test {}", (_x + 1));

    println!("y is {}", _y);
    //println!("underflow test {}", (_y - 1));
}

fn float_types() {
    let _x: f64 = 2.1; // f64
    let _y: f32 = 3.0; // f32
    println!("x is {}", _x);
}

fn bool_types() {
    let t = true;

    let f: bool = false; // with explicit type annotation

    if t == true {
        println!("t is {}", t);
    }

    if f == false {
        println!("f is {}", f);
    }
}

fn char_types() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("c is {}", c);
    println!("z is {}", z);
    println!("heart_eyed_cat is {}", heart_eyed_cat);
}

fn number_test() {
    // addition
    let sum = 5 + 10;
    println!("sum is {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference is {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product is {}", product);

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("quotient is {}", quotient);
    println!("truncated is {}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("remainder is {}", remainder);
}

fn tuple_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    println!("x is {}", tup.0);
    println!("y is {}", tup.1);
    println!("z is {}", tup.2);

    println!("The value of x,y,z is: {x}, {y}, {z}");
    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
}

fn array_type() {
    let a = [1, 2, 3, 4, 5];

    println!("a is {:#?}", a);
}

fn main() {
    integer_types();
    float_types();
    bool_types();
    char_types();
    number_test();
    tuple_type();
    array_type();
}
