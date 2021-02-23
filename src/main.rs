extern crate rand; // Essa linha já não e mais necessária!

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o número!");

    let numero_secreto: i32 = rand::thread_rng().gen_range(1..=100); //.gen_range(1..101) -> 0.8.3  .gen_range(1, 101) -> 0.5.5
    let mut palpite = String::new();

    println!("O número secreto é: {}", numero_secreto);
    println!("Digite seu Palpite.");

    io::stdin() //Biblioteca padrao do rust de entrada (input)
        .read_line(&mut palpite) // Entrada padrão do teclado
        .expect("Error! Falha ou ler a entrada"); // Enum que retorna uma execao casa uma condicão seja atendida.
    let palpite: i32 = palpite
        .trim()
        .parse()
        .expect("Por favor, digite um número valido!");

    println!("Voçê disse: {}", palpite);

    match palpite.cmp(&numero_secreto) {
        Ordering::Less => println!("Muito Baixo!"),
        Ordering::Greater => println!("Muito Alto!"),
        Ordering::Equal => println!("Você acertou!"),
    }
}
