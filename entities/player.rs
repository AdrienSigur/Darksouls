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
        color::green.colortext("===============================");
        println!("NOM : {}", self.nom);
        println!("CLASSE : {:?}", self.classe);
        println!("SANTÉ : {} HP", self.hp);
        println!("ARME : {:?}", self.arme);
        println!("DÉGÂTS ARME : {}", self.arme.weapondamage());
        println!("NIVEAU : {:?}", self.niveau);
        color::green.colortext("-------------------------------");

        // On appelle la logique de Stats depuis ici
        let noms = Stats::StrStats();
        let valeurs = self.stats.IntStats();

        for (n, v) in noms.iter().zip(valeurs.iter()) {
            print!("{:<12} : ", n);
            color::red.colortext(&v.to_string());
        }
        color::green.colortext("===============================");
    }

    pub fn hp_max(&self) -> i32{
      self.stats.Vig * 10
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
