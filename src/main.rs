use std::io;
use rand::Rng;

fn main() {
    println!("Devine mon nombre !\n");

    let secret = rand::thread_rng().gen_range(1..101);
    println!("[DEBUG] Nombre secret : {}", secret);

    println!("Saisissez votre proposition.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("Votre nombre: {}", input.trim());
}
