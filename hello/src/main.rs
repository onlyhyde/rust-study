// memory safety
// Rust는 컴파일 타임에 Memory safety를 보장함 
fn main() {
    let enigma: i32;        // enigma 선언은 했으나, 초기화 x
    println!("{}", enigma); // 컴파일 타임에 값을 알 수 없으므로, 에러
                            // c언어에서는 에러 X -> 쓰레기값 출력

    let enigma2: i32;
    if true {               // 컴파일러는 컴파일 타임에 조건 값에 대해 추론X
                            // 조건이 어떻든, 조건부 평가는 런타임에 처리되므로
                            // 컴파일러는 enigma2가 사용되기 전에 초기화되도록 보장
        enigma2 = 42;
    }
    println!("{}", enigma2); // 에러

    let enigma3: i32;
    if true {
        enigma3 = 42;
    } else {
        enigma3 = 1;
    }
    println!("{}", enigma3); // 정상동작, 위 조건문을 통해, enigma3이 사용되기전,
                             // 초기화 된다는 것을 컴파일러가 알 수 있음
}
