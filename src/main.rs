use std::io;

fn main() {
    println!("Adivinhe o número!");
    println!("Digite seu Palpite.");

    let mut palpite = String::new();
    io::stdin()
        .read_line(&mut palpite)
        .expect("Error! Falha ou ler a entrada");
    println!("Voçê disse: {}", palpite);
}
