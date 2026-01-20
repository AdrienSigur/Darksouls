


fn boucle_de_combat(joueur: &mut Personnage, monstre: &mut Monstre) {

    let randomonster = Monstre::monstergenerate();

    let monstercreate = format!("--- UN {} APPARAÎT ! ---", &randomonster.nom.to_uppercase());

    Color::Red::colortext(&monstercreate);

    while joueur.hp > 0 && monstre.hp > 0 {
        // 1. Affichage du statut
        println!("\n{} : {} HP | {} : {} HP", joueur.nom, joueur.hp, monstre.nom, monstre.hp);
        
        // 2. Menu d'actions
        println!("Que voulez-vous faire ?");
        println!("1. Attaquer | 2. Esquiver | 3. Objet | 4. Fuir");

        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).expect("Erreur de lecture");

        // 3. Logique des actions (Match sur le choix)
        match choix.trim() {
            "1" => {
                // Logique Attaque
                let degats = joueur.attack();
                monstre.hp -= degats * 2 ;
                println!("Vous infligez {} dégâts au {} !", degats, monstre.nom);
            },
            "2" => println!("Vous tentez une esquive..."),
            "3" => println!("L'inventaire n'est pas encore prêt !"),
            "4" => {
                println!("Vous fuyez comme un lâche !");
                break; // On sort de la boucle de combat
            },
            _ => println!("Choix invalide, vous perdez votre tour !"),
        }

            if monstre.hp > 0 {
            let degats_m = monstre.attaque;
            joueur.hp -= degats_m;
            println!("Le {} vous inflige {} dégâts !", monstre.nom, degats_m);
        }
    }

    if joueur.hp <= 0 {
       Color::Red.colortext(Asciart::YOUDIED);
    } else if monstre.hp <= 0 {
       Color::Green.colortext(Asciart::VICTORY);
    }
}
