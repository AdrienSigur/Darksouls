
use crate::stats::Stats;
use crate::entities::player::Personnage;
use crate::Asciart::Art_mobs;
use rand::Rng;
use rand::seq::SliceRandom;
use crate::Color;


pub struct Monstre {
    pub nom: String,
    pub hp: i32,
    pub stats: Stats, // On réutilise la même branche !
    pub degats_base: i32,
    pub status : bool ,
    pub xp_give : i32,
    pub art: String
}

impl  Monstre  {
    pub fn new(nom: &str, force: i32, vig: i32 , luck : i32 , asci : &str) -> Monstre {

        
        let xp_genere = Monstre::randxp(luck);

        Monstre {
             nom: nom.to_string(),
             hp: vig * 15, // Un monstre est peut-être un peu moins costaud ?
             stats: Stats { Vig: vig, Force: force, End: 5, Int: 1, Dex: 5, Luck: luck },
             degats_base: force,
             status : true ,
             xp_give : xp_genere,
             art : asci.to_string()
        }
    }

    pub fn attackplayer(&self, cible: &mut Personnage) ->i32{
        let degats = self.stats.Force * 3 ;

        let damagetaken : i32 = self.stats.Force * 4 ;
        
        damagetaken
        
    }

    pub fn randxp(luck : i32 ) -> i32{

        let mut rng = rand::rng();
        rng.random_range((luck * 4)..(luck * 9))
        
        
    }

    pub fn monstergenerate() -> Monstre {

        let mut rng = rand::thread_rng();

         let mut MonsterList : Vec<Monstre> = vec![ 

            Monstre::new(
                "Slime",
                2,
                3,
                5,
                Art_mobs::SLIME, // Virgule ici même si c'est le dernier
            ),

            Monstre::new(
                "Skeletton",
                3 ,
                4 ,
                6 ,
                Art_mobs::SKELETON
            ),

            Monstre::new(
                "Undead",
                3 ,
                4,
                5,
                Art_mobs::UNDEAD

            )
        ];
         let index = rng.gen_range(0..MonsterList.len());

         MonsterList.remove(index)


    }


    pub fn createmonster() -> Monstre{
        let monster = Monstre::monstergenerate();

        let formatage = format!("THIS MONSTER APPEAR {} IN THIS ROOM" , &monster.nom ); 

        Color::Red.colortext(&formatage);
        
        Color::Blue.colortext(&monster.art);

        monster 
    }

}
