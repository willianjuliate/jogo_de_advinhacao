use rand::Rng;
use std::io;

fn main() {
    println!("Adivinhe o número!");
    let mut numero_secreto = rand::thread_rng().gen_range(1..101);
    println!("Digite seu Palpite.");

    let mut palpite = String::new();
    io::stdin()
        .read_line(&mut palpite)
        .expect("Error! Falha ou ler a entrada");
    println!("Voçê disse: {}", palpite);
}
