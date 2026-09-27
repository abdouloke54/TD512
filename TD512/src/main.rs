
use rand::Rng;

fn main() {

    let mut input = String::new();
    let mut nombre_utilisateur: i32;
    

    let variable_alea = rand::thread_rng().gen_range(1..101);
    loop {
        input.clear();

        println!("Entrez un nombre:");

    
        let _ = std::io::stdin().read_line(&mut input);
        println!("Vous avez entré: {}", input.trim());
        match input.trim().parse::<i32>() {
            Ok(n) => nombre_utilisateur = n,
            Err(_) => {
                println!("Veuillez entrer un nombre valide.");
                continue;
            }
        }
    
        match nombre_utilisateur.cmp(&variable_alea) {
            std::cmp::Ordering::Less => println!("Le nombre aléatoire est plus grand que votre nombre."),
        
            std::cmp::Ordering::Greater => println!("Le nombre aléatoire est plus petit que votre nombre."),

            std::cmp::Ordering::Equal => {println!("Bravo ! Vous avez trouvé le nombre aléatoire."); 
                break;
            }
        }
    }
println!("C'est fini !");
}

