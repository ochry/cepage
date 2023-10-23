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

    CabernetFranc(String),
    CabernetSauvignon(String),
    Gamay(String),
    Grenache(String),
    Malbec(String),
    Merlot(String),
    Nebiolo(String),
    Pinotage(String),
    PinotNoir(String),
    SanGiovese(String),
    Syrah(String),
    Tempranillo(String),
}

struct Vin<'a> {
    regions: Vec<String>,
    aoc: Vec<String>,
    primaires_froid: Vec<String>,
    primaires_chaud: Vec<String>,
    acidite: &'a str,
    rouge: bool,
    noms_alt: String,
    botrytis: bool,
    elevage: bool,
    malo: bool,
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
        for aoc in vin_aoc.split(", ").collect::<Vec<&str>>() {
            if let Some(reg) = aoc_set.get(aoc) {
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
    let rand = thread_rng().gen_range(0..6);
    match rand {
        0 => Cepage::Chardonnay("Chardonnay".to_string()),
        1 => Cepage::Riesling("Riesling".to_string()),
        2 => Cepage::Gewurztraminer("Gewurztraminer".to_string()),
        3 => Cepage::Semillon("Semillon".to_string()),
        4 => Cepage::CheninBlanc("CheninBlanc".to_string()),
        5 => Cepage::Furmint("Furmint".to_string()),
        _ => panic!("{rand}"),
    }
}

fn quizz(mut vin: Vin, mut cepage_string: String) {
    let mut questions: Vec<String> = vec![

        format!("-> Quel cépage donne un vin aux arômes de {} lorsqu'il est cultivé dans un climat froid ?",
        vin.random_three(true).join(", ")),

        format!("-> Des arômes de {} embellissent cet elixir quand il provient d'un pays tempéré...",
        vin.random_three(true).join(", ")),

        format!("-> En contrées chaudes, ce cépage donnera un vin aux notes plus exotiques de {}.",
        vin.random_three(false).join(", ")),

        format!("-> {}... autant de régions et de pays qui apprécient mes qualités gustatives ! Je suis le...",
        vin.regions.join(", ")),

        format!("-> Les quilles qui font ma renommée proviennent entre autres de: {}.",
        vin.aoc.join(", ")),

        format!("-> On qualifie mon acidité de {}... Une petit idée ?",
        vin.acidite),

    ];

    if vin.botrytis == true {
        questions.push("-> Parfois on me laisse pourrir... mais de façon noble, ce qui me confère un goût exquis !".to_string());
    }
    if vin.rouge == false {
        questions.push("-> On me qualifie de raisin blanc, mais perso je me trouve plutôt vert clair avec des reflets ambrés.".to_string());
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
        "abricot sec",
        "acacia",
        "ananas",
        "banane",
        "canelle",
        "citron",
        "citron confit",
        "citron vert",
        "coing",
        "coquille d'huître",
        "craie",
        "datte",
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
        "noisette",
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
        "raisin sec",
        "rose",
        "sirop d'érable",
        "shiste",
        "tilleul",
        "verveine",
    ]);

    let secondaires_set = HashSet::from([
        "amande",
        "beurre",
        "brioche",
        "noisette",
        "noix",
        "pain grillé",
        "torrefaction",
        "vanille",
        "yoghurt",
    ]);

    let tertiaire_set = HashSet::from([
        "champignon",
        "chocolat",
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
        "Croatie",
        "France",
        "Hongrie",
        "Rhin",
        "Slovaquie",
        "Slovénie",
        "Provence",
    ]);

    let aoc_set = HashSet::from([
        "Bergerac (Dordogne)",
        "Chablis (Bourgogne)",
        "Coteaux du Layon (Loire)",
        "Meursault (Bourgogne)",
        "Montrachet (Bourgogne)",
        "Monbazillac (Dordogne)",
        "Moselle (Rhin)",
        "Pfalz (Rhin)",
        "Napa Valley (Californie)",
        "Pessac-Léognan (Bordeaux)",
        "Puligny-Montrachet (Bourgogne)",
        "Rheingau (Rhin)",
        "Rheinessen (Rhin)",
        "Sauternes (Bordeaux)",
        "Stellenbosch (Afrique du Sud)",
        "Sonoma County(Californie)",
        "Vouvray (Loire)",
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
        rouge: false,
        elevage: true,
        malo: true,
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
        rouge: false,
        elevage: false,
        malo: false,
    };

    riesling.add_aromes("pomme verte, poire, pêche blanche, citron, mandarine, miel, pamplemousse jaune, parafine, pétrole, pierre à fusil, shiste, fleur d'oranger", "ananas, pêche, abricot, fleurs blanches", &primaires_set);
    riesling.add_regions(
        "Rhin, Alsace, Autriche",
        "Moselle (Rhin), Rheingau (Rhin), Pfalz (Rhin), Rheinessen (Rhin)",
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
        rouge: false,
        elevage: false,
        malo: false,
    };

    gewurztraminer.add_aromes("litchi, rose, pamplemousse rose, jasmin, fleurs blanches, menthe, poivre blanc, noix de muscade, gingembre, citron vert, pomme verte, poire",
    " litchi, ananas, passion, papaye, mangue, fleur d'oranger, gingembre, miel, orange confite",
    &primaires_set);
    gewurztraminer.add_regions(
        "Rhin, Alsace, Autriche",
        "Moselle (Rhin), Rheingau (Rhin), Pfalz (Rhin), Rheinessen (Rhin)",
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
        rouge: false,
        elevage: true,
        malo: true,
    };

    semillon.add_aromes(
        "pomme verte, poire, citron vert, tilleul, fleurs blanches, craie, groseille, miel, noix fraîche, acacia",
        "ananas, mangue, citron confit, orange, pêche, abricot, fleur d'oranger, jasmin",
        &primaires_set);
    semillon.add_regions(
        "Bordeaux, Provence, Australie",
        "Sauternes (Bordeaux), Pessac-Léognan (Bordeaux), Bergerac(Dordogne), Monbazillac (Dordogne), Valle de Hunter (Australie)",
        &regions_set,
        &aoc_set,
    );

    let mut chenin_blanc = Vin {
        regions: Vec::new(),
        aoc: Vec::new(),
        primaires_froid: Vec::new(),
        primaires_chaud: Vec::new(),
        acidite: acidite[2],
        noms_alt: String::from("Pineau de la Loire, Blanc d'Aunis, Anjou, Steen"),
        botrytis: true,
        rouge: false,
        elevage: true,
        malo: true,
    };

    chenin_blanc.add_aromes(
        "pomme verte, poire, citron vert, tilleul, fleurs blanches, craie, miel, acacia",
        "ananas, coing, citron confit, orange, pêche jaune, abricot, fleur d'oranger",
        &primaires_set,
    );

    chenin_blanc.add_regions(
        "Loire, Afrique du Sud, Australie",
        "Vouvrai (Loire), Coteaux du Layon (Loire), Stellenbosch (Afrique du Sud)",
        &regions_set,
        &aoc_set,
    );

    let mut furmint = Vin {
        regions: Vec::new(),
        aoc: Vec::new(),
        primaires_froid: Vec::new(),
        primaires_chaud: Vec::new(),
        acidite: acidite[2],
        noms_alt: String::from(""),
        botrytis: true,
        rouge: false,
        elevage: true,
        malo: true,
    };

    furmint.add_aromes(
        "pomme verte, citron, citron vert, pamplemousse, shiste, craie, pierre à fusil, menthe, verveine",
        "coing, citron confit, orange, pêche jaune, abricot, abricot sec, miel, raisin sec, datte, canelle, gingembre",
        &primaires_set,
    );
    furmint.add_regions(
        "Hongrie, Slovaquie, Croatie, Slovénie",
        "Tokaj (Hongrie) ",
        &regions_set,
        &aoc_set,
    );

    // Random wine is selected

    let cepage_enum = random_cepage();

    match cepage_enum {
        Cepage::Chardonnay(cep) => {
            quizz(chardonnay, cep);
        }
        Cepage::Gewurztraminer(cep) => {
            quizz(gewurztraminer, cep);
        }
        Cepage::Riesling(cep) => {
            quizz(riesling, cep);
        }
        Cepage::Semillon(cep) => {
            quizz(semillon, cep);
        }
        Cepage::CheninBlanc(cep) => {
            quizz(chenin_blanc, cep);
        }
        Cepage::Furmint(cep) => {
            quizz(furmint, cep);
        }
        _ => println!("Autre"),
    };
}
