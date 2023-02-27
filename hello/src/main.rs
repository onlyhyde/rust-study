// Strings
// Rust 표준 라이브러리에는 6가지 문자열 타입이 있다.
// 2가지를 주로 사용
// string slice는 &str 타입으로 표현한다.
// 1. str (string slice) 
// 2. String
// 두가지의 가장 큰 차이점은 string slice는 수정할 수 없음.
// String은 수정할 수 있음.
// string slice에서 to_string() 메서드를 호출하면 String을 얻을 수 있다.
// or String::from() 메서드를 호출하면 String을 얻을 수 있다.
// string slice는 내부적으로 포인터로 구성됨
// String은 내부적으로 포인터와 길이, 용량으로 구성됨. 현재 사용중인 것보다 클 수 있음.

// let msg = "abc".to_string();
// let msg = Strings::from("abc");
fn main() {
    
}