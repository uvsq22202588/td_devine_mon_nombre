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
        io::stdin().read_line(&mut input).unwrap();
        let input: u32 = input.trim().parse().unwrap();
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
}
