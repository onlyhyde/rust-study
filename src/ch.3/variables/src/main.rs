fn immutable_mutable() {
    let a = 5; // immutable
               // a = 6; => a is immutable. error
    println!("immutable a is {}", a);

    let mut x = 5;
    println!("mutable x is: {x}");
    x = 6;
    println!("mutable x is: {x}");
}

fn constants() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The const value is: {THREE_HOURS_IN_SECONDS}");
}

fn shadowing() {
    // mutable은 동일한 자료형으로만 변경 가능하고,
    // shadowing은 자료형이 달라도 변경 가능하다.
    println!("\n Shadowing");
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2; // inner shadowing
        println!("The value of x in the inner scope is: {x}");
    } // inner shadowing end
    println!("The value of x is : {x}");

    let x = 10;
    println!("{:p}", &x);

    let x = x + 10;
    println!("{:p}", &x);
    let n = &x;
    println!("{:p}", &n);
    println!("{}", x);
    println!("{}", *n);
    println!("{}", n);
}

fn main() {
    // immutable vs mutable
    immutable_mutable();

    // constants
    constants();

    // shadowing
    shadowing();

    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{:p}", &s1);
    println!("{}", s2);
}
