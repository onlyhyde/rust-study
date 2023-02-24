// Rust Style Guide
// Function -> snake-case 
// Function 사용을 위해 먼저 노출될 필요가 없음
// parameters are always defined "name:type"
// fn do_stuff(qty: f64)
// return type은 arrow poining 으로 parameter 뒤에 지정
// fn do_stuff(qty: f64) -> f64

fn main() {

}


// tail expression
// 값을 반환하는 약어
// 블럭 마지막 라인에 세미콜론을 빼면 값으로 반환
// {return ture;} == {true}
fn do_stuff(qty: f64, oz: f64) -> f64 {
    qty * oz
}

// macro
// 매크로 이름은 항상 !로 끝난다 
// println!("{}", qty);