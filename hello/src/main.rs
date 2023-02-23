// variable
fn main() {
    let _a = 10; // immutable variable
    //_a = 5;    // error, 변경 불가

    let mut _b = 10; // mutable variable
    _b = 5;          // 변경 가능

    // 상수 선언
    // 1. let -> const
    // 2. use screaming-snake-case 
    // 3. type annotation is required
    // 4. value must be a constant expression 
    // that can be determined at compile tiem

    // ex.
    // let _a = random();
    // 1. const _a = random();
    // 2. const A_VALUE = random();
    // 3. const A_VALUE: i32 = random();
    // 4. const A_VALUE: i32 = 10;

    // why use const variable 
    // 1. reusalbe, global variable
    // 2. const 값은 컴파일 타임에 인라인 되기 때문에 빠름
}
