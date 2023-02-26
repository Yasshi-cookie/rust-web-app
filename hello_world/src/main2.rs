// 第二章
fn main() {
    println!("{}", add(1, 2));
}

fn add(x: i32, y: i32) -> i32 {
    // 式と文について
    // 式 = 「評価されて値を返すもの」
    // 文 = 「値を返さないもの」
    return x + y;

    // returnを書かなくても、最後に記述されているものが式であれば、暗黙的にその式が返す値を返す。
    // 最後が文であれば、暗黙的に空のタプルを返す
}
