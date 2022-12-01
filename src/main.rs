use std::io;
use rand::Rng;

fn main() {
    println!("Devine mon nombre !\n");

    let secret = rand::thread_rng().gen_range(1..101);
    println!("[DEBUG] Nombre secret : {}", secret);

    println!("Saisissez votre proposition.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: u32 = input.trim().parse().unwrap();
    println!("Votre nombre: {}", input);
    
    if input == secret {
        println!("Égal");
    } else if input > secret {
        println!("Trop grand");
    } else {
        println!("Trop petit");
    }

}
