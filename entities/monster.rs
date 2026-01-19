
use crate::stats::Stats;
use crate::entities::player::Personnage;
use rand::Rng;


pub struct Monstre {
    pub nom: String,
    pub hp: i32,
    pub stats: Stats, // On réutilise la même branche !
    pub degats_base: i32,
    pub status : bool ,
    pub xp_give : i32,
    art: &'static str
}

impl Monstre {
    pub fn new(nom: &str, force: i32, vig: i32 , luck : i32 , asci : &'static str) -> Monstre {

        
        let xp_genere = Monstre::randxp(luck);

        Monstre {
            nom: nom.to_string(),
            hp: vig * 5, // Un monstre est peut-être un peu moins costaud ?
            stats: Stats { Vig: vig, Force: force, End: 5, Int: 1, Dex: 5, Luck: luck },
            degats_base: force,
            status : true ,
            xp_give : xp_genere,
            art : asci
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
