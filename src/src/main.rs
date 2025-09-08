use std::io;
use rand::Rng;

fn main() {
    println!("Devine mon nombre !\n");
    println!("Saisissez votre proposition.\n");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_int: u32 = input.trim().parse().unwrap();

    println!("Le nombre que vous avez dit est : {}", input_int);

    let variable_aleatoire = rand::thread_rng().gen_range(1..101);

    println!("Le nombre aléatoire est : {}", variable_aleatoire);

    let message = if input_int == variable_aleatoire {
        "Tu as choisi le bon numéro !"
    } else if input_int < variable_aleatoire {
        "Tu as choisi un numéro trop petit !"
    } else {
        "Tu as choisi un numéro trop grand !"
    };

    println!("{}", message);
}
