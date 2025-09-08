use std::io;

fn main() {
    println!("Devine mon nombre !\n");
    println!("Saississez votre proposition.");

    let mut input = String::new();
    io::stdin().read_line(&mut input);
    
    println!("Le nombre que vous avez dit est {}", input)
}