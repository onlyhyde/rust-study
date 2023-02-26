// Scalar Types
// Integers, floats, booleans, characters
fn main() {
    // Unsigned types
    // u8 _a;
    // u16 _b;
    // u32 _c;
    // u64 _d;
    // u128 _e;

    // Signed tyeps
    // i8 _ia;
    // i16 _ib;
    // i32 _ic;
    // i64 _id;
    // i128 _ie;

    // Decimal 100
    // Hex 0x100
    // Octal 0o100
    // Binary 0b100
    // '_'를 숫자 사이에 넣으면 '_'는 무시된다. -> 가독성을 위해 지원
    // ex) 1_000_000

    // floating point types
    // f32 _fa;
    // f64 _fb; // default

    let _x: u16 = 5; 
    let _x1 = 5u16;
    let _x2 = 5_u16;
    let _y: f32 = 3.14;
    let _y1 = 3.14f32;
    let _y2 = 3.14_f32;

    // Boolean types
    // true | false
    // 정수가 아니므로, 필요시 정수유형으로 캐스팅해야함
    // true as u8
    // false as u8

    // character
    // 4byte
    // 유니코드 문자 인코딩 방식 사용 UCS-4 | UTF-32
    // 문자 배열을 UCS-4 or UTF-32 문자열로..
}