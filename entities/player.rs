
use crate::stats::Stats;
use crate::Role;
use crate::types::Weapon;  
use rand::prelude::*;
use rand::Rng;
use crate::entities::monster::Monstre;
use crate::Color;

#[derive(Clone)]
pub struct Personnage {
    pub hp : i32 , 
    pub nom : String ,
    pub classe : Role ,
    pub arme : Weapon ,
    pub stats : Stats ,
    pub xp : i32 ,
    pub niveau : i32
    
}


impl Personnage {

    pub fn new(nom : String , classe : Role) -> Personnage {

        let arme = classe.Weaponclass();
        let stats = classe.basicStats();

        let hpmax = stats.Vig * 10 ;
        
        Personnage {
            hp : hpmax ,
            nom : nom.to_string() , 
            classe : classe,
            arme : arme ,
            stats : stats,
            xp : 0 , 
            niveau : 1
        }

    }
    

  pub  fn fiche(&self) {
        Color::Green.colortext("===============================");
        println!("NOM : {}", self.nom);
        println!("CLASSE : {:?}", self.classe);
        println!("SANTÉ : {} HP", self.hp);
        println!("ARME : {:?}", self.arme);
        println!("DÉGÂTS ARME : {}", self.arme.weapondamage());
        println!("NIVEAU : {:?}", self.niveau);
        println!("XP : {:?} / {:?}", self.xp , self.niveau * 50);
        Color::Green.colortext("-------------------------------");

        // On appelle la logique de Stats depuis ici
        let noms = Stats::StrStats();
        let valeurs = self.stats.IntStats();

        for (n, v) in noms.iter().zip(valeurs.iter()) {
            print!("{:<12} : ", n);
            Color::Red.colortext(&v.to_string());
        }
        Color::Green.colortext("===============================");
    }

    pub fn hp_max(&self) -> i32{
      self.stats.Vig * 10
    }
    

    pub fn attack(&mut self, cible: &mut Monstre) -> i32 {

        let degats = self.stats.Force + self.arme.weapondamage()  ;
        
        if cible.hp < 0 {

            self.xp += cible.xp_give;

            println!("{:?} récupère {} xp de {}", self.nom , cible.xp_give , cible.nom);
            
            //println!("Le monstre {} est mort" , cible.nom);
            //println!("{} récupère {} xp de {} " , self.nom , cible.xp_give , cible.nom);

           // self.checklevelup();
        }

        degats

    
    }

       pub fn estus(&mut self){

        let estus = 20 ;
        let hpmax = self.hp_max() ;

        if self.hp + estus > hpmax {
 
            self.hp = hpmax;
            
            self.afficher_hud();    
        }else{
            self.hp += estus;
                
            self.afficher_hud();

        }
         
       
    }


    pub fn afficher_hud(&self){
        
        let taille_barre = 20;
        let ratio = self.hp as f32 / self.hp_max() as f32;
        let blocs_pleins = (ratio * taille_barre as f32) as usize;

        let couleur_barre = if ratio > 0.6 {
            Color::Green
        } else if ratio > 0.2 {
            Color::Yellow
        } else {
            Color::Red
        };

    // On construit la chaîne de blocs
        let mut barre_visuelle = String::new();
        for i in 0..taille_barre {
            if i < blocs_pleins {
                barre_visuelle.push('█');
            } else {
            barre_visuelle.push('░');
            }
        }
        

        print!("HP :  ");
        println!("{:?} {}/{} HP", couleur_barre.colortext(&barre_visuelle), self.hp, self.hp_max());
    }


     
    pub fn checklevelup(&mut self){
        
        if self.xp >= self.niveau * 50 {
            println!("Vous avez gagné {} xp  en tuant ce monstre " , self.xp);
            self.niveau += 1;
            self.xp -= self.niveau * 50;
            println!("✨ LEVEL UP ! {} est maintenant niveau {} !", self.nom, self.niveau);

            self.hp = self.hp_max() ;

            self.stats.Force += 2;
            self.stats.Vig += 1;
            self.hp += 30  ;

            println!("Tes stats augmentent ! Force: {}, Vie: {}", self.stats.Force, self.hp);
       }else{
           println!("Vous avez gagné {} xp en tuant ce mob" , self.xp); 
       }


    }


}  
