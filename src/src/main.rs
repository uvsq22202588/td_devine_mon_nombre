use std::io;
use rand;

fn main() {
    println!("Devine mon nombre !\n");
    println!("Saississez votre proposition.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    println!("Le nombre que vous avez dit est : {}", input);

    let variable_aleatoire = rand::thread_rng().gen_range(1..101);

    println!("Le nombre aléatoire est : {}", variable_aleatoire)
}