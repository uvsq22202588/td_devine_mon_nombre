use std::{cmp::Ordering, io};
use rand::Rng;

/// Fonction comprennant toutes les autres, et initialise le nombre secret.
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Le nombre aléatoire est : {}", secret_number);

    /// Retourne l’entier saisi en ignorant les erreurs d’I/O ou de conversion
    fn read_int_from_stdin() -> Option<u32> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input.parse::<u32>() {
            Ok(num) => Some(num),
            Err(_) => None,
        }
    }

    /// Encapsule la comparaison entre le nombre secret et la saisie
    fn get_ordering(secret_number: u32, input: u32) -> Ordering {
        input.cmp(&secret_number)
    }

    /// Affiche le message approprié en fonction du résultat de la comparaison
    fn display_result(comparison: Ordering) {
        match comparison {
            Ordering::Less => println!("Trop petit !"),
            Ordering::Greater => println!("Trop grand !"),
            Ordering::Equal => println!("Bravo, vous avez trouvé !"),
        }
    }

    /// Retourne true si le nombre a été trouvé
    fn has_found(comparison: Ordering) -> bool {
        comparison == Ordering::Equal
    }

    loop {
        let input = read_int_from_stdin();

        if let Some(input) = input {
            let comparison = get_ordering(secret_number, input);
            display_result(comparison);

            if has_found(comparison) {
                break;
            }
        } else {
            println!("Saisie incorrecte, veuillez entrer un nombre.");
        }
    }
}
