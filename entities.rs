use crate::stats::Stats;
use crate::Role;
use crate::types::Weapon;  
use rand::prelude::*;

pub struct Monstre {
    nom: String,
    hp: i32,
    stats: Stats, // On réutilise la même branche !
    degats_base: i32,
    status : bool ,
    xp_give : i32,
}

impl Monstre {
    pub fn new(nom: &str, force: i32, vig: i32 , luck : i32) -> Monstre {

        
        let xp_genere = Monstre::randxp(luck);

        Monstre {
            nom: nom.to_string(),
            hp: vig * 5, // Un monstre est peut-être un peu moins costaud ?
            stats: Stats { Vig: vig, Force: force, End: 5, Int: 1, Dex: 5, Luck: luck },
            degats_base: force,
            status : true ,
            xp_give : xp_genere 
        }
    }

    pub fn attackplayer(&self, cible: &mut Personnage) {
        let degats = self.stats.Force * 3 ; 
        cible.hp -= degats * 2 ;
        println!("{} attaque {} et inflige {} dégâts !", self.nom, cible.nom, degats);
    }

    pub fn randxp(luck : i32 ) -> i32{

        let mut rng = rand::rng();
        rng.random_range((luck * 4)..(luck * 9))
        
        
    }
}

pub struct Personnage {
    hp : i32 , 
    nom : String ,
    classe : Role ,
    arme : Weapon ,
    stats : Stats ,
    xp : i32 ,
    niveau : i32
    
}

impl Personnage {

    pub fn new(nom : &str , classe : Role) -> Personnage {

        let arme = classe.Weaponclass();
        let stats = classe.basicStats();
        let hpmax = classe.stats.hp_max();    

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
        println!("===============================");
        println!("NOM : {}", self.nom);
        println!("CLASSE : {:?}", self.classe);
        println!("SANTÉ : {} HP", self.hp);
        println!("ARME : {:?}", self.arme);
        println!("DÉGÂTS ARME : {}", self.arme.weapondamage());
        println!("NIVEAU : {:?}", self.niveau);
        println!("-------------------------------");

        // On appelle la logique de Stats depuis ici
        let noms = Stats::StrStats();
        let valeurs = Stats::IntStats();

        for (n, v) in noms.iter().zip(valeurs.iter()) {
            println!("{:<12} : {}", n, v);
        }
        println!("===============================");
    }
    

   pub fn attack(&mut self, cible: &mut Monstre) {

        let hpmax = cible.stats.Vig * 5 ;
        let degats = self.stats.Force + self.arme.weapondamage();
        cible.hp -= degats ;

        if cible.hp < 0 {
            cible.hp = 0;

            self.xp += cible.xp_give;
            
            println!("Le monstre {} est mort" , cible.nom);
            println!("{} récupère {} xp de {} " , self.nom , cible.xp_give , cible.nom);

            self.checklevelup();
        }else{
            println!("{} attaque {} et inflige {} dégâts !", self.nom, cible.nom, degats);
            println!("Vie restante de {} est de {}/{}", cible.nom , cible.hp , hpmax);

        }
    
    
    }
    
    pub fn estus(&mut self){

        let estus = 20 ;
        let hpmax = self.stats.Vig * 10 ;

        if self.hp + estus > hpmax {
            self.hp = hpmax
        }else{
            self.hp += estus
        }
        println!("{} boit fiole d'estus et régénérer Vie actuelle {}/{}" , self.nom , self.hp , hpmax );
    }

    

    pub fn hp_max(&self) -> i32 {
        self.stats.Vig * 10
    }

    pub fn checklevelup(&mut self){
        
       if self.xp >= self.niveau * 10 {
            self.niveau += 1;
            self.xp -= self.niveau * 10;
            println!("✨ LEVEL UP ! {} est maintenant niveau {} !", self.nom, self.niveau);

            self.hp = self.hp_max() ;

            self.stats.Force += 2;
            self.stats.Vig += 1;
            self.hp += 30  ;

            println!("Tes stats augmentent ! Force: {}, Vie: {}", self.stats.Force, self.hp);
       }
    }


}


