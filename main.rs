mod entities;
mod types;
mod stats ;
mod utils ;

use entities::{Personnage, Monstre};
use types::Role;
use std::io;
use utils::Color::color;


fn main() {

    color::blue.colortext("Choose your undead name : \n");

    
    let mut nom_input = String::new();
    std::io::stdin().read_line(&mut nom_input).expect("Error this name is not avaible");

    color::blue.colortext("\n Choose your class between this list \n");

    let mut class = String::new();
    std::io::stdin().read_line(&mut class).expect("Error this name is not available");
    println!("");

    let roll = match class.trim().to_lowercase().as_str() {
        "chevalier" => Role::Chevalier,
        _ => Role::Mendiant
    };

    color::blue.colortext(&format!("Vous avez choose the class {:?} . You gonna enter in the world of lordran  \n" , &roll) );

    let mut user : Personnage = Personnage::new(nom_input , roll);

    color::green.colortext("--- Welcome to lordran realm ---");

    loop {
    
    
    let mut choix = String::new();

    
    io::stdin()
        .read_line(&mut choix) 
        .expect("Échec de la lecture"); 

    let choix = choix.trim();
    
    match choix {
        "Fiche" => user.fiche(),
        "continuer" => println!("hello world"),
        "quitter"  => break ,
        "estus" => user.estus(),
        _ => println!("choose something else")
    }
    

    }
}
