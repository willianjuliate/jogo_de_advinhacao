use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::process::Command;

fn main() {
    println!("### ADIVINHE O NÚMERO! ###");
    let numero_secreto = rand::thread_rng().gen_range(1..=100); //.gen_range(1..101) -> 0.8.3  .gen_range(1, 101) -> 0.5.5
    loop {
        let mut palpite = String::new();

        println!("Digite seu Palpite.: ");

        io::stdin() //Biblioteca padrao do rust de entrada (input)
            .read_line(&mut palpite) // Entrada padrão do teclado
            .expect("Error! Falha ou ler a entrada"); // Enum que retorna uma execao casa uma condicão seja atendida.
        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Voçê disse: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito Baixo!"),
            Ordering::Greater => println!("Muito Alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}
