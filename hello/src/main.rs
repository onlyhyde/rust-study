// scope
fn main() {
    let x = 5;
    {
        let y = 99;
        println!("{}, {}", x, y);
    }
    //println!("{}, {}", x, y); // error, y undefined

    {
        let x = 99;
        println!("{}", x);
    }
    println!("{}", x);

    // shadow 
    let mut x = 5; // x is mutable
    let x = x; // x is now immutable

    // let meme = "Meme";
    // let meme = make_image(meme); // meme은 "MeMe"로 시작하여 이미가 됨.
}
