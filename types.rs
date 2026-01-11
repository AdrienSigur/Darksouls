
use crate::stats::Stats;

#[derive(Debug)]

pub enum Role {
    Chevalier ,
    Clerc ,
    Pyromancien,
    Mendiant , 
    Necromancien ,
    Prisonnier
}

impl Role{

    pub fn basicStats(&self) -> Stats {

    match self {
    Role::Chevalier => Stats { Vig: 12, Force: 14, End: 12, Int: 6, Dex: 10, Luck: 7 },
    Role::Clerc => Stats { Vig: 10, Force: 12, End: 9, Int: 14, Dex: 8, Luck: 11 },
    Role::Pyromancien => Stats { Vig: 11, Force: 11, End: 10, Int: 12, Dex: 12, Luck: 7 },
    Role::Prisonnier => Stats { Vig: 8, Force: 8, End: 11, Int: 14, Dex: 14, Luck: 6 },
    Role::Necromancien => Stats { Vig: 7, Force: 6, End: 8, Int: 16, Dex: 9, Luck: 12 },
    Role::Mendiant => Stats { Vig: 10, Force: 10, End: 10, Int: 10, Dex: 10, Luck: 10 },
    }

    }

    pub fn Weaponclass(&self) -> Weapon {
        match self {
            Role::Chevalier => Weapon::Epee , 
            Role::Clerc => Weapon::Baton,
            Role::Mendiant => Weapon::MainNue,
            Role::Prisonnier => Weapon::Morgeinsteirn,
            Role::Pyromancien => Weapon::Arc,
            Role::Necromancien => Weapon::Baton

        }
    }

    
}

#[derive(Debug)]
pub enum Weapon {
    Epee ,
    Arc ,
    Baton , 
    MainNue,
    Morgeinsteirn

}

impl Weapon {


   pub fn weapondamage(&self) -> i32 {

        match self {
            Weapon::Epee => 10,
            Weapon::Arc => 5 ,
            Weapon::Morgeinsteirn => 12 ,
            Weapon::MainNue => 2 ,
            Weapon::Baton => 5
        }
    }
}


