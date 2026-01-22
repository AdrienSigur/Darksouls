
use crate::Asciart::Art_ui::*;
use crate::Personnage;
use crate::Monstre;
use crate::Color::*;

pub struct FightingRoom {
    pub adversaire : Monstre 
}

impl FightingRoom {

    pub fn combat(&mut self , joueur: &mut Personnage) {

    
    while joueur.hp > 0 && self.adversaire.hp > 0 {
        
        
        println!("{:<10} : {:>3} HP  |  {:<10} : {:>3} HP", 
                &joueur.nom.trim(),     &joueur.hp, 
                self.adversaire.nom, self.adversaire.hp
        );

        println!("1. Attaquer | 2. Esquiver | 3. Objet | 4. Fuir");

        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).expect("Erreur de lecture");

        match choix.trim() {
            "1" => {
                // Logique Attaque
                let degats = joueur.attack(&mut self.adversaire);
                self.adversaire.hp -= degats  ;
                println!("Vous infligez {:?} dégâts au {} !", degats, self.adversaire.nom);
            },
            "2" => println!("Vous tentez une esquive..."),
            "3" => println!("L'inventaire n'est pas encore prêt !"),
            "4" => {
                println!("Vous fuyez comme un lâche !");
                break; // On sort de la boucle de combat
            },
            _ => println!("Choix invalide, vous perdez votre tour !"),
        }

            if self.adversaire.hp > 0 {
            let degats_m = self.adversaire.attackplayer(joueur);
            joueur.hp -= degats_m;
            println!("Le {} vous inflige {} dégâts !", self.adversaire.nom, degats_m);
        }
    }

        if joueur.hp <= 0 {
            Red.colortext(YOUDIED);
        } else if self.adversaire.hp <= 0 {
            Green.colortext(VICTORY);
            
            let mut xp = Monstre::randxp(self.adversaire.stats.Luck);

            joueur.xp += xp;
            
           joueur.checklevelup();
            
        }

    }

}
