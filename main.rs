mod entities;
mod types;
mod stats ;
mod Utils ;
mod Asciart;

use std::io::Write;
use Asciart::Art_ui;
use Asciart::Art_mobs;
use entities::*;
use types::Role;
use crate::player::Personnage;
use crate::monster::Monstre;
use crate::entities::monster::*;
use std::io;
use Utils::Color::*;


fn main() {

    Color::Blue.colortext(Art_ui::DARKSOULS);

    Color::Blue.colortext("Choose your undead name : \n");

    // color::red.colortext(Art_ui::YOUDIED);
    Color::Blue.colortext(Art_mobs::SLIME);
    
    let mut nom_input = String::new();
    std::io::stdin().read_line(&mut nom_input).expect("Error this name is not avaible");

    Color::Blue.colortext("\n Choose your class between this list \n");

    let mut class = String::new();
    std::io::stdin().read_line(&mut class).expect("Error this name is not available");
    println!("");

    let roll = match class.trim().to_lowercase().as_str() {
        "chevalier" => Role::Chevalier,
        "mendiant"  => Role::Mendiant,
        "clerc" =>  Role::Clerc, 
        "prisonnier" => Role::Prisonnier ,
        "pyromancien" =>   Role::Pyromancien ,
        "necromancien" => Role::Necromancien ,
        _ => Role::Mendiant
    };

    Color::Blue.colortext(&format!("Vous avez choose the class {:?} . You gonna enter in the world of lordran  \n" , &roll) );

    let mut user : Personnage = Personnage::new(nom_input , roll);

    Color::Green.colortext("--- Welcome to lordran realm ---");

    loop {

    print!(" > ");   

    io::stdout().flush().expect("Failed to flush stdout");
    
    
    let mut choix = String::new();

    
    io::stdin()
        .read_line(&mut choix) 
        .expect("Échec de la lecture"); 

    let choix = choix.trim();

    println!("Type help for commands");
    
    match choix {
        "Fiche" => user.fiche(),
        "continuer" => println!("hello world"),
        "quitter"  => break ,
        "estus" => user.estus(),
        "help" => println!("hello worldu"),
        "random" => ennemi = Some(monster::monstergenerate()), 
        _ => println!("choose something else")
    }
    

    }
}
