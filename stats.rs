#[derive(Debug)]
#[derive(Clone)]
pub struct Stats {
    pub Vig : i32 ,
    pub Force : i32 ,
    pub End : i32 ,
    pub Int : i32 ,
    pub Dex : i32 ,
    pub Luck : i32 
}

impl Stats {

    pub fn IntStats(&self) -> [i32 ; 6]{
        [self.Vig , self.Force , self.End , self.Int , self.Dex , self.Luck]
    }

    pub fn StrStats() -> [String ; 6] {
       ["Vigueur".to_string() , "Force".to_string() , "Endurance".to_string(), "Intelligence".to_string(), "Dexterity".to_string() ,"Luck".to_string() ]
    }


   
}




