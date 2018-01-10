extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Угадайте число!");

    let secred_number = rand::thread_rng().gen_range(1, 101);

    println!("Загаданное число: {}", secred_number);

    println!("Пожалуйста, введите предположение");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Не удалось прочитать строку");

    let guess: u32 = guess.trim().parse().expect("Пожалуйста, введите число!");

    println!("Ваша попытка: {}", guess);

    match guess.cmp(&secred_number) {
        Ordering::Less => println!("Слишком маленькое!"),
        Ordering::Greater => println!("Слишком большое!"),
        Ordering::Equal => println!("Вы выиграли!")        
    }
}
