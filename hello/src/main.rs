// Module
// package name의 greet 를 사용하겠다는 의미
// package name은 cargo.toml 에 정의
use hello::greet;
use rand::thread_rng;
use rand::Rng;

fn main() {
    greet();
    let random = thread_rng().gen_range(0, 100);
    println!("{}", random);
}
s