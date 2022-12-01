use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Devine mon nombre !\n");

    let secret = rand::thread_rng().gen_range(1..101);
    println!("[DEBUG] Nombre secret : {}", secret);

    loop {
        let input = read_int_from_stdin();

        if let Some(input) = input { // if_let
            let comparison = get_ordering(secret, input);
            display_result(comparison);
    
            if has_found(comparison) {
                break;
            }
        } else {
            println!("Saisie incorrecte.");
        }
    }
}

fn read_int_from_stdin() -> Option<u32> {
    println!("Saisissez votre proposition.");
    let mut input = String::new();
    if let Err(error) = io::stdin().read_line(&mut input) { // ref_mut
        println!("Erreur d'I/O : {}", error);
        return None;
    }

    println!("Vous avez saisi : {}", input.trim());

    input.trim().parse::<u32>().ok() // turbofish
}

fn get_ordering(secret: u32, input: u32) -> Ordering {
    input.cmp(&secret)
}

fn display_result(comparison: Ordering) {
    let message = match comparison {
        Ordering::Less => "Trop petit",
        Ordering::Equal => "Égal, bravo !",
        Ordering::Greater => "Trop grand",
    };
    println!("{}", message);
}

fn has_found(comparison: Ordering) -> bool {
    comparison == Ordering::Equal
}

// Documentation complémentaire
// if_let : construction if let (https://doc.rust-lang.org/book/ch06-03-if-let.html)
// ref_mut : référence mutable (https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references)
// turbofish : notation ::<> (https://doc.rust-lang.org/std/primitive.str.html#method.parse)
