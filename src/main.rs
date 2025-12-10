fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", add(5, 3));

    println!("Hello {:20}!", "world");  // 幅20（固定値）で"world"を表示
    println!("Hello {:1$}!", "Rust", 20);  // 幅を第2引数(20)で指定して"Rust"を表示
    println!("Hello {1:0$}!", 20, "Rust"); // "Rust"を第1引数の幅(20)で表示
    println!("Hello {2:0$} {1}!", 20, "ignored", "Rust"); // "Rust"を第1引数の幅(20)で表示
    println!("Hello {2:1$} {0}!", "ignored", 20, "Rust"); // "Rust"を第2引数の幅(20)で表示

    /* 
    使わない引数はダメ 
    println!("Hello {2:0$}!", 20, "ignored", "Rust"); // エラー
    */

    println!("Hello {:<20}!", "left");   // 左寄せ
    println!("Hello {:^20}!", "center"); // 中央寄せ
    println!("Hello {:>20}!", "right");  // 右寄せ

    println!("Hello {:0>20}!", "left");   // '0'で埋める左寄せ
    println!("Hello {:*^20}!", "center"); // '*'で埋める中央寄せ
    println!("Hello {:_>20}!", "right");  // '_'で埋める右寄せ

}