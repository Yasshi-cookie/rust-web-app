fn main() {
    // 変数の中身を変更することは基本的に不可
    // しかし、mut をつけると変更可能になる
    // let mut message = "hello, foo";
    let message = if false { "hello, foo" } else { "hello, bar" };

    println!("{message}");

    // 再宣言はできる
    let url = "https://rustup.rs/";
    println!("{url}");
    let url = "https://www.yahoo.co.jp/";
    println!("{url}");

    // データ型
    // 文字型（1文字）と文字列の型の違いに注意
    // 'で指定すると文字型になる。
    let moji = 'a';
    // "で指定すると文字列になる。
    let mojiretsu = "mojiretsu";
    println!("{moji}");
    println!("{mojiretsu}");

    // タプル型
    let touple = ("hoge", 123);
    println!("{}", touple.1);

    // 配列型
    let array = ["hoge", "fuga", "bar"];
    println!("{}", array[0]);

    // ベクター型
    let v = vec![99, 100];
    println!("{}", v[1]);

    // 文字列
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s:?}");

    // ハッシュマップ
    let mut scores = std::collections::HashMap::new();
    scores.insert("Sato", 100);
    scores.insert("Tanaka", 90);
    // "Tanaka"がなかったら100をinsert
    scores.entry("Tanaka").or_insert(100);

    // HashMap::fromからハッシュマップを生成
    let solar_distance = std::collections::HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);
}
