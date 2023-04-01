// 第二章
fn main() {
    // 2.2 関数の実行
    // execute0()
    // execute1()
    // execute2()
    // execute3()
    // execute4()
    // execute5()

    // 2.3 制御構造
    // execute6()
    // execute7()
    execute8()
}

fn execute0() {
    println!("{}", add(1, 2));
}

fn execute1() {
    let _red = Color::Red;
    let _hex = Color::Hex("ffffff".to_string());
}

fn execute2() {
    let some: Result<&str, &str> = Ok("ok");
    println!("{:?}", some); // Ok("ok")

    let err: Result<&str, &str> = Err("err");
    println!("{:?}", err); // Err("err")
}

fn execute3() {
    // 回復可能なエラー
    let message = match might_fail() {
        Ok(_) => "処理に成功しました。".to_string(),
        Err(cause_message) => cause_message,
    };
    println!("{}", message);
}

fn execute4() {
    println!("before panic!");
    // 回復不能なエラー
    panic!("hoge");
    println!("after panic!");
}

fn execute5() {
    let input: Result<&str, &str> = Ok("test");
    // Err or Noneの場合、unwrapメソッドがpanicを引き起こす
    let input = input.unwrap();
    println!("{:?}", input);
}

fn execute6() {
    // // パターンマッチ
    // println!("{}", fizz_buzz(3));
    // println!("{}", fizz_buzz(5));
    // println!("{}", fizz_buzz(15));

    let data = Some("Some text");
    let print_data = match data {
        Some(text) => text, // textで値を取得できる
        None => "None text",
    };

    println!("{}", print_data);
}

fn execute7() {
    if let Some(color) = string_to_color_token("red") {
        println!("red");
    }
}

fn execute8() {
    // ループ
    let result = add_until(1, 10);
    println!("{}", result);
}

fn add_until(start: i32, end: i32) -> i32 {
    let mut sum = 0;
    let mut temp = start;
    // // loop
    // // breakしたときの値を返す
    // sum = loop {
    //     sum += temp;
    //     if temp == end {
    //         break sum;
    //     }
    //     temp += 1;
    // };
    // sum

    // // while
    // // 常に空のタプル()を返す
    // while temp <= end {
    //     sum += temp;
    //     temp += 1;
    // }

    // sum

    // // for
    // // forは値を返さないので注意、イテレーターを実装しているものに対して実行可能
    for temp in start..(end + 1) {
        sum += temp;
    }
    sum

}

fn string_to_color_token(value: &str) -> Option<Color> {
    match value {
        "red" => Some(Color::Red),
        "blue" => Some(Color::Blue),
        "green" => Some(Color::Green),
        "white" => Some(Color::White),
        _ => None,
    }
}

fn fizz_buzz(value: i32) -> String {
    let result = match value {
        v if v % 15 == 0 => "fizz buzz".to_string(),
        v if v % 5 == 0 => "buzz".to_string(),
        v if v % 3 == 0 => "fizz".to_string(),
        _ => value.to_string(),
    };

    return result;
}

/// xとyを足し合わせます
///
/// # Example
///
/// ```
/// use chapter_01::utils::add;
///
/// let r = add(1, 10);
/// assert_eq!(11, r);
/// ```
pub fn add(x: i32, y: i32) -> i32 {
    // 式と文について
    // 式 = 「評価されて値を返すもの」
    // 文 = 「値を返さないもの」
    return x + y;

    // returnを書かなくても、最後に記述されているものが式であれば、暗黙的にその式が返す値を返す。
    // 最後が文であれば、暗黙的に空のタプルを返す
}

fn always_error() -> Result<(), String> {
    Err("常にエラーが発生します。".to_string())
}

fn might_fail() -> Result<(), String> {
    let _result = always_error()?;
    println!("{}", "ここに到達した");
    Ok(())
}

enum Color {
    Red,
    Blue,
    Green,
    White,
    Hex(String),
}
