// control flow
// loop
// while
// rust에는 do while이 없음.
// for
fn main() {
    // loop {
    //     break;
    // }

    // 루프를 빠져나가는 방법
    // 'bob: loop {
    //     loop {
    //         loop {
    //             break 'bob;
    //         }
    //     }
    // }

    // 루프를 계속하는 방법
    // 'bob: loop {
    //     loop {
    //         continue 'bob;
    //     }
    // }

    // while dizzy() {
    //     // do stuff
    // }
    // loop {
    //     if !dizzy() {break}
    //     // do stuff
    // }

    // iter()는 collection의 iterator를 반환
    // collection의 순서가 있는 경우, 순서대로,
    // collection의 순서가 없는 경우, 임의의 순서로
    // for num in [7, 8, 9].iter() {
    //     // do stuff with num
    // }

    // for 루프는 패턴을 사용하여 수신한 항목을 분해하고 내부 부분을 변수에 바인딩할 수 있음
    // let array = [(1,2),(3,4)];
    // for (x,y) in array.iter() {
    //     // do stuff with x and y
    // }

    // for loop에 range를 사용하고 싶을 때
    // for num in 0..50 {  // 0부터 49까지 반복
    //     // do stuff with num
    // }
    // for num in 0..=50 {  // 0부터 50까지 반복
    //     // do stuff with num
    // }
}