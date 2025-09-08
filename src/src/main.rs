use std::io;
use rand::Rng;

fn main() {
    let variable_aleatoire = rand::thread_rng().gen_range(1..101);
    loop {
    println!("Le nombre aléatoire est : {}", variable_aleatoire);
    println!("Devine mon nombre !\n");
    println!("Saisissez votre proposition.\n");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_int: u32 = input.trim().parse().unwrap();

    println!("Le nombre que vous avez dit est : {}", input_int);


    
    let message = match input_int.cmp(&variable_aleatoire) {
        std::cmp::Ordering::Equal => "Tu as choisi le bon numéro !",
        std::cmp::Ordering::Less => "Tu as choisi un numéro trop bas !",
        std::cmp::Ordering::Greater => "Tu as choisi un numéro trop haut !",
    };

    if input_int == variable_aleatoire {
        break;
    }

    println!("{}", message);
    }
}
