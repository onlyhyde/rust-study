// control flow
// 조건 주위 괄호 필요 없음
// 조건을 만족하거나, 만족하지 않을 때 구문은 반드시 중괄호로 감싸야 한다.
// c 에서처럼 if (a == 5) a = 6; 이런 식으로 사용할 수 없음
fn main() {
    // let mut msg: &str = "";
    // let num: i32 = 0;

    // if num == 5 {
    //     msg = "five";
    // } else if num == 6 {
    //     msg = "six";
    // } else {
    //     msg = "other";
    // }

    // => 위 식은 다음과 같이 표현 가능
    // msg = if num == 5 {
    //     "five"  // return keyword와 세미콜론을 사용하지 않음
    // } else if num == 6 {
    //     "six"
    // } else {
    //     "other"
    // }; // 세미콜론 사용

    // num = if a { b } else { c };
    // num = if a { if x { y } else { y }
    // } else {
    //     c
    // };
}