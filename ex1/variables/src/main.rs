const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // let mut missiles = 8;
    // let ready = 2;
    // println!("Firing {} of my {} missiles...", ready, missiles);

    // missiles = missiles - ready;
    // println!("Firing {} of my {} missiles...", ready, missiles);



    // let missiles = STARTING_MISSILES;
    // let ready = READY_AMOUNT;
    // println!("Firing {} of my {} missiles...", ready, missiles);



    // let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    // println!("Firing {} of my {} missiles...", ready, missiles);
    // missiles = missiles - ready;
    // println!("{} missiles left", missiles);



    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
