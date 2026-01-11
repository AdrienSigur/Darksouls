mod entities;
mod types;
mod stats ;



use entities::{Personnage, Monstre};
use types::Role;




fn main() {

    let noms_monstres = vec!["Slime", "Squelette", "Chevalier Noir"];
    let nom_choisi = noms_monstres.choose(&mut rng).unwrap();
    let mut ennemi = Monstre::new(nom_choisi, force_aleatoire, vig_aleatoire, luck_aleatoire);

    let mut solaire : Personnage = Personnage::new("solaire" , Role::Chevalier );
    let mut slime : Monstre = Monstre::new("slime" , 5 , 5 , 5 );
 
   

    solaire.fiche();

    slime.attackplayer(&mut solaire);

    solaire.attack(&mut slime);

    solaire.estus();

    solaire.attack(&mut slime);

    solaire.fiche();


    loop {
    println!("--- Une nouvelle salle s'offre à vous ---");


    // 1. Générer un monstre au hasard
    // 2. Lancer le combat
    // 3. Demander : "Continuer ou Quitter ?"
    
    if choix == "quitter" || solaire.hp <= 0 {
        break; 
    }
}
