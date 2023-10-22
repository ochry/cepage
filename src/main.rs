use rand::prelude::*;
use std::collections::HashSet;
use std::io;

enum Cepage {
    Chardonnay(String),
    CheninBlanc(String),
    Furmint(String),
    Gewurztraminer(String),
    PinotGris(String),
    Riesling(String),
    Semillon(String),
    Viognier(String),
}

struct Vin<'a> {
    regions: Vec<String>,
    aoc: Vec<String>,
    primaires_froid: Vec<String>,
    primaires_chaud: Vec<String>,
    acidite: &'a str,
    noms_alt: String,
    botrytis: bool,
}

impl Vin<'_> {
    fn add_aromes(
        &mut self,
        aromes_froid: &str,
        aromes_chaud: &str,
        primaires_set: &HashSet<&str>,
    ) {
        for arome in aromes_froid.split(", ").collect::<Vec<&str>>() {
            if let Some(fruit) = primaires_set.get(arome) {
                self.primaires_froid.push(fruit.to_string());
            }
        }
        for arome in aromes_chaud.split(", ").collect::<Vec<&str>>() {
            if let Some(fruit) = primaires_set.get(arome) {
                self.primaires_chaud.push(fruit.to_string());
            }
        }
    }

    fn add_regions(
        &mut self,
        vin_regions: &str,
        vin_aoc: &str,
        regions_set: &HashSet<&str>,
        aoc_set: &HashSet<&str>,
    ) {
        for region in vin_regions.split(", ").collect::<Vec<&str>>() {
            if let Some(reg) = regions_set.get(region) {
                self.regions.push(reg.to_string());
            }
        }
        for region in vin_aoc.split(", ").collect::<Vec<&str>>() {
            if let Some(reg) = aoc_set.get(region) {
                self.aoc.push(reg.to_string());
            }
        }
    }

    fn random_three(&mut self, froid: bool) -> Vec<String> {
        let mut three_primaires = Vec::new();
        if self.primaires_froid.len() >= 3 && froid == true {
            for _ in 0..3 {
                let rand = thread_rng().gen_range(0..self.primaires_froid.len());
                three_primaires.push(self.primaires_froid[rand].to_owned());
                self.primaires_froid.remove(rand);
            }
        }
        if self.primaires_chaud.len() >= 3 && froid == false {
            for _ in 0..3 {
                let rand = thread_rng().gen_range(0..self.primaires_chaud.len());
                three_primaires.push(self.primaires_chaud[rand].to_owned());
                self.primaires_chaud.remove(rand);
            }
        }
        three_primaires
    }
}

fn random_cepage() -> Cepage {
    let rand = thread_rng().gen_range(0..5);
    match rand {
        0 => Cepage::Chardonnay("Chardonnay".to_string()),
        1 => Cepage::Riesling("Riesling".to_string()),
        2 => Cepage::Gewurztraminer("Gewurztraminer".to_string()),
        3 => Cepage::Semillon("Semillon".to_string()),
        4 => Cepage::Furmint("Furmint".to_string()),
        _ => panic!("{rand}"),
    }
}

fn quizz(mut vin: Vin, mut cepage_string: String) {
    let mut questions: Vec<String> = vec![

        format!("-> Quel cépage donne un vin aux arômes de {} lorsqu'il est cultivé dans un climat froid ?",
        vin.random_three(true).join(", ")),

        format!("-> Des arômes de {} embellisent cet elixir quand il provient d'un pays tempéré... Qui suis-je ?",
        vin.random_three(true).join(", ")),

        format!("-> En contrées chaudes, ce cépage donnera un vin aux notes plus exotiques de {}. Je suis le...",
        vin.random_three(false).join(", ")),

        format!("-> {}... autant de régions et de pays qui ont fait sa renommée ! Je suis le...",
        vin.regions.join(", ")),

        format!("-> Des quilles renommées proviennent entre autres de: {}.",
        vin.aoc.join(", ")),

        format!("-> Ce cépage a une acidité {}.",
        vin.acidite),

    ];

    if vin.botrytis == true {
        questions.push("Parfois on me laisse pourrir... mais de façon noble, avec mon acolyte le botrytis, qui me confère un goût exquis !".to_string());
    }

    let mut iter_track = 0;
    let mut essai = true;
    let empty_string = "".to_string();
    let mut user_input = String::new();

    loop {
        match &user_input {
            a if a == &cepage_string => {
                let mut score = 11 - iter_track;
                if score < 0 {
                    score = 0
                };
                println!("\n * * * B R A V O !\n Tu es le sommelier du jour, le gredin du vin, le roi des cépages !");
                println!(
                    " Pour info, ton score est de {}/10. Chapeau. \n * * * * * * * * * *\n",
                    score
                );
                break;
            }
            a if a == &empty_string && questions.len() > 0 => {
                let rand = thread_rng().gen_range(0..questions.len());
                if iter_track > 0 {
                    println!("Avine ton verre sommelier, voici un autre indice :")
                };
                println!("{}", questions[rand]);
                questions.remove(rand);
                essai = true;
                iter_track += 1;
            }
            _ if essai == true => {
                if questions.is_empty() {
                    println!("L'happy hour tourne au vinaigre ! ");
                    println!("Dernière chance: que boit-on ?");
                } else {
                    println!("Faux mon coco! In vino veritas, il te reste 1 essai...");
                    println!("(ENTER pour un nouvel indice, sinon tapote ta réponse)");
                }
                essai = false;
            }
            _ => {
                println!("\n * * * * * * * * * * \n Quand on préfère la picole aux colles, c'est le nez droit dans le ra...vin !");
                println!(
                    " La réponse correcte était le {}.\n * * * P E R D U :( \n ",
                    cepage_string
                );
                break;
            }
        };

        user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Erreur");
        user_input = user_input.trim().to_lowercase();

        cepage_string = cepage_string.trim().to_lowercase();
        // let empty_string = "".to_string();
    }
}

fn intro() {
    fn wait_for_enter() {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Erreur");
    }

    println!("* * * * * * * * * * \nBienvenue à CÉPAGE !");
    println!("Le but du jeu est de découvrir le mystérieux cépage décrit...");
    println!("Appuie sur ENTER pour afficher les règles du jeu.");
    wait_for_enter();
    println!("1. Pour un indice supplémentaire, appuie simplement sur ENTER sans rien écrire.");
    println!("2. Quand tu connais la réponse, tapote-la. Mais attention, tu n'as que deux essais.");
    println!("Seras-tu le sommelier du jour ? Bonne chance !\n* * * * * * * * * * ");
    wait_for_enter();
}

fn main() {
    intro();

    let primaires_set = HashSet::from([
        "abricot",
        "acacia",
        "ananas",
        "banane",
        "citron",
        "citron confit",
        "citron vert",
        "coquille d'huître",
        "craie",
        "eucalyptus",
        "fleurs blanches",
        "fleur d'oranger",
        "fruit de la passion",
        "groseille",
        "jasmin",
        "litchi",
        "gingembre",
        "mandarine",
        "mangue",
        "melon",
        "menthe",
        "miel",
        "muscade",
        "nectarine",
        "noix de coco",
        "noix de muscade",
        "noix fraîche",
        "orange",
        "orange confite",
        "pamplemousse jaune",
        "pamplemousse rose",
        "papaye",
        "paraffine",
        "pêche blanche",
        "pêche jaune",
        "pétrole",
        "pierre à fusil",
        "poire",
        "poivre blanc",
        "pomme verte",
        "rose",
        "sirop d'érable",
        "shiste",
        "tilleul",
    ]);

    let secondaires_set = HashSet::from([
        "amande",
        "beurre",
        "brioche",
        "noisette",
        "noix",
        "pain grillé",
        "vanille",
        "yoghurt",
    ]);

    let tertiaire_set = HashSet::from([
        "champignon",
        "cuir",
        "fruits secs",
        "sauce soja",
        "sous-bois",
        "tabac",
    ]);

    let regions_set = HashSet::from([
        "Afrique du Sud",
        "Alsace",
        "Australie",
        "Autriche",
        "Bordeaux",
        "Bourgogne",
        "Californie",
        "Rhin",
        "Provence",
    ]);

    let aoc_set = HashSet::from([
        "Chablis (Bourgogne)",
        "Meursault (Bourgogne)",
        "Montrachet (Bourgogne)",
        "Monbazillac (Dordogne)",
        "Mosel (Rhin)",
        "Pfalz (Rhin)",
        "Napa Valley (Californie)",
        "Puligny-Montrachet (Bourgogne)",
        "Rheingau (Rhin)",
        "Rheinessen (Rhin)",
        "Sauternes (Bordeaux)",
        "Sonoma County(Californie)",
    ]);

    let acidite: [&str; 4] = ["peu élevée", "modérée", "élevée", "très élevée"];

    // Wines are instantiated as structs
    // Method are used to add values after verification

    let mut chardonnay = Vin {
        regions: Vec::new(),
        aoc: Vec::new(),
        primaires_froid: Vec::new(),
        primaires_chaud: Vec::new(),
        acidite: acidite[1],
        noms_alt: String::from(""),
        botrytis: false,
    };

    chardonnay.add_aromes(
        "pomme verte, poire, citron, citron vert, nectarine, pêche blanche, fleurs blanches, acacia, craie",
        "ananas, mangue, melon, abricot",
        &primaires_set,
    );
    chardonnay.add_regions(
        "Bourgogne, Bordeaux, Champagne, Californie, Australie",
        "Chablis (Bourgogne), Puligny-Montrachet (Bourgogne), Meursault (Bourgogne), Montrachet (Bourgogne),
        Napa Valley (Californie), Sonoma County(Californie)",
        &regions_set,
        &aoc_set,
    );

    let mut riesling = Vin {
        regions: Vec::new(),
        aoc: Vec::new(),
        primaires_froid: Vec::new(),
        primaires_chaud: Vec::new(),
        acidite: acidite[3],
        noms_alt: String::from(""),
        botrytis: false,
    };

    riesling.add_aromes("pomme verte, poire, pêche blanche, citron, mandarine, miel, pamplemousse jaune, parafine, pétrole, pierre à fusil, shiste, fleur d'oranger", "ananas, pêche, abricot, fleurs blanches", &primaires_set);
    riesling.add_regions(
        "Rhin, Alsace, Autriche",
        "Mosel (Rhin), Rheingau (Rhin), Pfalz (Rhin)",
        &regions_set,
        &aoc_set,
    );

    let mut gewurztraminer = Vin {
        regions: Vec::new(),
        aoc: Vec::new(),
        primaires_froid: Vec::new(),
        primaires_chaud: Vec::new(),
        acidite: acidite[0],
        noms_alt: String::from(""),
        botrytis: true,
    };

    gewurztraminer.add_aromes("litchi, rose, pamplemousse rose, jasmin, fleurs blanches, menthe, poivre blanc, noix de muscade, gingembre, citron vert, pomme verte, poire",
    " litchi, ananas, passion, papaye, mangue, fleur d'oranger, gingembre, miel, orange confite",
    &primaires_set);
    riesling.add_regions(
        "Rhin, Alsace, Autriche",
        "Mosel (Rhin)",
        &regions_set,
        &aoc_set,
    );

    let mut semillon = Vin {
        regions: Vec::new(),
        aoc: Vec::new(),
        primaires_froid: Vec::new(),
        primaires_chaud: Vec::new(),
        acidite: acidite[1],
        noms_alt: String::from("Chevrier, Semilao, Hunter River Riesling"),
        botrytis: true,
    };

    semillon.add_aromes(
        "pomme verte, poire, citron vert, tilleul, fleurs blanches, craie, groseille, miel, noix fraîche, acacia",
        "ananas, mangue, citron confit, orange, pêche, abricot, fleur d'oranger, jasmin",
        &primaires_set);
    semillon.add_regions(
        "Bordeaux, Provence",
        "Sauternes (Bordeaux), Monbazillac (Dordogne)",
        &regions_set,
        &aoc_set,
    );

    // Random wine is selected

    let cepage_enum = random_cepage();

    match cepage_enum {
        Cepage::Chardonnay(cep) => {
            //println!("{}", cep);
            quizz(chardonnay, cep);
        }
        Cepage::Gewurztraminer(cep) => {
            //println!("{}", cep);
            quizz(gewurztraminer, cep);
        }
        Cepage::Riesling(cep) => {
            //println!("{}", cep);
            quizz(riesling, cep);
        }
        Cepage::Semillon(cep) => {
            //println!("{}", cep);
            quizz(semillon, cep);
        }
        _ => println!("Autre"),
    };
}
