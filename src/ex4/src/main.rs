fn variable_scope_test() {
    let s = "hello"; // s is valid from this point forward
                     // do stuff with s
    println!("{}", s);
} // this scope is now over, and s is no longer valid

fn string_type_test() {
    let mut s = String::from("hello"); // s is valid from this point forward
                                       // do stuff with s
    s.push_str(", world !"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world !`
}

fn memory_and_allocation_test() {
    // The memory must be requested from the memory allocator at runtime.
    // We need a way of returning this memory to the allocator when we’re done with our String.
    // that first part is done by us: when we call String::from, its implementation requests the memory it needs. This is pretty much universal in programming languages.
    // However, the second part is different. In languages with a garbage collector (GC), the GC keeps track of and cleans up memory that isn’t being used anymore, and we don’t need to think about it. In most languages without a GC, it’s our responsibility to identify when memory is no longer being used and to call code to explicitly free it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early,
    // we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.
    //
    // Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.
    {
        let s = String::from("hello");
        println!("{}", s);
        // let s2 = s;
        // println!("{}", s); // error: value borrowed here after move
        let s2 = s.clone();
        println!("s = {}, s2 = {}", s, s2);
    }

    {
        // stack-only Data: copy
        let x = 5;
        let y = x;
        println!("x = {}, y = {}", x, y);
    }
}

fn main() {
    // Ownership
    // Ownership is a set of rules that govern how a Rust program manages memory.
    // These rules are checked at compile time.
    // Ownership is Rust's most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector.
    // memory is managed through a system of ownership with a set of rules that the compiler checks.
    // None of the ownership features slow down your program while it's running.
    //
    // stack : fixed size
    // heap : dynamic size (unknown size at compile time)
    // Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.
    // When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.
    // Once you understand ownership, you won’t need to think about the stack and the heap very often, but knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.
    //
    // Ownership Rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // Variable Scope
    variable_scope_test();
    string_type_test();
    memory_and_allocation_test();
}
