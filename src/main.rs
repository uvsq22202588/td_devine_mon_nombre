use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Devine mon nombre !\n");

    let secret = rand::thread_rng().gen_range(1..101);
    println!("[DEBUG] Nombre secret : {}", secret);

    loop {
        println!("Saisissez votre proposition.");
        let mut input = String::new();
        if let Err(error) = io::stdin().read_line(&mut input) {
            println!("Erreur d'I/O : {}", error);
            continue;
        }

        match input.trim().parse::<u32>() {
            Ok(input) => {
                println!("Votre nombre: {}", input);

                let message = match input.cmp(&secret) {
                    Ordering::Less => "Trop petit",
                    Ordering::Equal => "Égal",
                    Ordering::Greater => "Trop grand",
                };

                println!("{}", message);

                if input.cmp(&secret) == Ordering::Equal {
                    break;
                }
            }
            Err(error) => {
                println!("Erreur de saisie : {}", error);
            }
        }
    }
}
