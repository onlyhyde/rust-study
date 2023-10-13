fn main() {

    // immutable vs mutable
	//let x = 5; // immutable
    let mut x = 5;
	println!("The value of x is: {x}");
	x = 6;
	println!("The value of x is: {x}");

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The const value is: {THREE_HOURS_IN_SECONDS}");

    test();

    // shadowing
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2; // inner shadowing
        println!("The value of x in the inner scope is: {x}");
    } // inner shadowing end
    println!("The value of x is : {x}");
}

fn test() {
    const THREE_HOURS_IN_SECONDS: u32 = 10 * 10 * 3;
    println!("The const value is: {THREE_HOURS_IN_SECONDS}");
}

