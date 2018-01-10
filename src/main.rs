use std::io;

fn main() {
    println!("Угадайте число!");

    println!("Пожалуйств, введите предположение");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Не удалось прочитать строку");

    println!("Ваша попытка: {}", guess);
}
