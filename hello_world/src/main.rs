fn main() {
    // 変数の中身を変更することは基本的に不可
    // しかし、mut をつけると変更可能になる
    // let mut message = "hello, foo";
    let message;
    if false {
        message = "hello, foo";
    } else {
        message = "hello, bar";
    }

    println!("{message}");

    // 再宣言はできる
    let url = "https://rustup.rs/";
    let url = "https://www.yahoo.co.jp/";

    // データ型
    // 文字型（1文字）と文字列の型の違いに注意
    // 'で指定すると文字型になる。
    let moji = 'a';
    // "で指定すると文字列になる。
    let mojiretsu = "mojiretsu";

    // タプル型
    let touple = ("hoge", 123);
    println!("{}", touple.0);
}
