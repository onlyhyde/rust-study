// 변수에 저장할 수 있는 함수와 유사한 구조인 클로저
// 요소들의 연속을 처리하는 방법인 iterator
// 클로저와 iterator를 사용하는 방법
// 클로저와 iterator의 성능

fn main() {
    closure_test();
}

fn closure_test() {
    // Rust의 클로저는 변수에 저장하거나 다른 함수에 인수로 전달할 수 있는 익명 함수이다.
    // 함수와 달리 클로저는 정의된 범위에서 값을 포착할 수 있다
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // unwrap_or_else는 Rust의 Option 및 Result 열거형에 대한 메서드입니다. 이 메서드는 Option 또는 Result 값이 None 또는 Err일 경우 제공된 클로저를 호출하고, 그렇지 않으면 Some 또는 Ok에 포함된 값을 반환합니다.

    // 동작 방식:
    // Option<T>에 대한 unwrap_or_else:

    // Some(value)일 경우: value를 반환합니다.
    // None일 경우: 제공된 클로저를 호출하고 그 결과를 반환합니다.
    // Result<T, E>에 대한 unwrap_or_else:

    // Ok(value)일 경우: value를 반환합니다.
    // Err(err)일 경우: 제공된 클로저를 err와 함께 호출하고 그 결과를 반환합니다.

    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
        // user_preference에 Some(value)가 있으면 value를 반환하고, None이면 self.most_stocked()를 반환한다.
        // unwarp_or_else에 Inventory의 self.most_stocked()함수를 호출하는 closure를 전달한것.
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
