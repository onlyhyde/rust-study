// Compound Types
// 다른 Type의 여러 값을 하나의 Type으로 수집
fn main() {
    // Tuple
    // 모든 Type의 여러값을 저장
    // 괄호로 묶고, 쉼표로 구분
    // 현재 Maximum Arity는 12개 이상
    // Arity는 Tuple에 있는 항목의 수를 의미
    let _info: (u8, f64, i32) = (1, 3.3, 999);
    //let info = (1, 3.3, 999);
    // Tuple 멤버 접근 방식 
    // 1. field access expression (하나씩 접근)
    // let jets = info.0;
    // let fuel = info.1;
    // 2. 한번에 모두 접근
    // let (jetx, fuel, ammo) = info;
    let _arity_test = (1,2,3,4,5,6,7,8,9,10,11,12,13,14,15);
    println!("{}", _arity_test.14);

    // Array 
    // 동일한 Type의 여러 값을 저장
    // 다른 언어의 array와 다른점. Rust는 고정된 길이를 갖고 있음.
    // Heap 보다 Stack에 allocated 되길 원할때 사용.
    // 이미 알고 있는 크기의 값들을 변경없이 사용한다면, Array가 효율적
    // 변동성이 큰것은 vector사용.
    // Array, Vector중 뭘 사용할지 모르겠다 싶으면 Vector 써라.
    // 대괄호로 묶고, 쉼표로 구분
    let _buf = [1,2,3];
    // 대괄호 내에서 세미콜론으로 구분시,
    // 세미콜론 앞=값, 세미콜론 뒤=값의 갯수 
    let _buf2 = [0; 3]; // [0,0,0]
    println!("{}, {}, {}", _buf2[0], _buf2[1], _buf2[2]);
    // array에 대한 type 지정은 항상 세미콜론을 사용
    let _buf3: [u8; 3] = [1, 2, 3];
    println!("{}, {}, {}", _buf3[0], _buf3[1], _buf3[2]);
    // array의 크기 = 32로 제한, 그 이상의 경우 손실이 발생
    let _buf4: [u128; 33] = [2; 33];
    println!("{}", _buf4[32]);
}