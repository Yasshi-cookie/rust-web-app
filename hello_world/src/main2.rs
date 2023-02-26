// 第二章
fn main() {
    // execute0()
    // execute1()
    // execute2()
    // execute3()
    // execute4()
    // execute5()
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
    Hex(String)
}

enum Option<T> {
    Some(T),
    None,
}
