use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!"); // 数を当ててごらん

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // 行の読み込みに失敗しました

    println!("You guessed: {}", guess); // 次のように予想しました: {}
}
