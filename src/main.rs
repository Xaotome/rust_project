use std::collections::HashMap;
use rand::RngExt;

fn main() {
    //ex1();
    //ex2();
    //ex3();
    ex4();
}

fn ex4() {
    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=100);

    let mut guesses: Vec<u32> = Vec::new();
    let mut remaining_guesses = 10;

    let mut scores: HashMap<String, u32> = HashMap::new();

    loop {
        println!("Devinez le nombre secret entre 1 et 100 :");

        if !guesses.is_empty() {
            println!("Vos tentatives précédentes : {:?}", guesses);
        }

        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Erreur de lecture");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide.");
                continue;
            }
        };

        guesses.push(guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => {
                println!("Trop petit !");
                if remaining_guesses > 0 {
                    remaining_guesses -= 1;
                }
            },
            std::cmp::Ordering::Greater => {
                println!("Trop grand !");
                if remaining_guesses > 0 {
                    remaining_guesses -= 1;
                }
            },
            std::cmp::Ordering::Equal => {
                println!("Félicitations ! Vous avez deviné le nombre secret !");
                break;
            }
        }
    }

    // Gestion du score final

    let final_score = remaining_guesses * 10; // Par exemple, chaque tentative restante vaut 10 points
    println!("Votre score final est : {}", final_score);

    // Demander au jour son nom 

    let mut player_name = String::new();
    println!("Entrez votre nom pour enregistrer votre score :");
    std::io::stdin().read_line(&mut player_name).expect("Erreur de lecture");
    let player_name = player_name.trim().to_string();

    scores.entry(player_name).or_insert(final_score);
    println!("Meilleurs scores : {:?}", scores);
}

#[derive(Debug)]
enum TaskStatus {
    Incomplete,
    Complete
}

trait Task {
    fn description(&self) -> String;
    fn status(&self) -> TaskStatus;
    fn mark_complete(&mut self);
}

impl Task for String {
    fn description(&self) -> String {
        self.clone()
    }

    fn status(&self) -> TaskStatus {
        TaskStatus::Incomplete
    }

    fn mark_complete(&mut self) {
        // Implementation for marking a task as complete
    }
}


fn ex3() {

    let mut tasks: HashMap<i32, (String, TaskStatus)> = HashMap::new();
    let mut next_task_id = 1;

    loop {
        println!("Bienvenue dans votre application de To-do list");
        println!("1. Ajouter une tâche");
        println!("2. Afficher les tâches");
        println!("3. Marquer une tâche comme terminée");
        println!("4. Supprimer une tâche");
        println!("5. Quitter");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Erreur de lecture");
        let choice: u32 = choice.trim().parse().expect("Veuillez entrer un nombre valide");

        match choice {
            1 => {
                println!("Entrez la description de la tâche :");
                let mut description = String::new();
                std::io::stdin().read_line(&mut description).expect("Erreur de lecture");
                let description = description.trim().to_string();

                tasks.insert(next_task_id, (description, TaskStatus::Incomplete));
                println!("Tâche ajoutée avec l'ID : {}", next_task_id);
                next_task_id += 1;
            },
            2 => {
                println!("Tâches actuelles :");
                for (id, (description, status)) in &tasks {
                    println!("ID : {}, Description : {}, Statut : {:?}", id, description, status);
                }
            },
            3 => {
                println!("Entrez l'ID de la tâche à marquer comme terminée :");
                let mut id_input = String::new();
                std::io::stdin().read_line(&mut id_input).expect("Erreur de lecture");
                let id: i32 = id_input.trim().parse().expect("Veuillez entrer un nombre valide");

                if let Some(task) = tasks.get_mut(&id) {
                    task.1 = TaskStatus::Complete;
                    println!("Tâche ID {} marquée comme terminée.", id);
                } else {
                    println!("Tâche avec l'ID {} non trouvée.", id);
                }
            },
            4 => {
                println!("Entrez l'ID de la tâche à supprimer :");
                let mut id_input = String::new();
                std::io::stdin().read_line(&mut id_input).expect("Erreur de lecture");
                let id: i32 = id_input.trim().parse().expect("Veuillez entrer un nombre valide");

                if tasks.remove(&id).is_some() {
                    println!("Tâche ID {} supprimée.", id);
                } else {
                    println!("Tâche avec l'ID {} non trouvée.", id);
                }
            },
            5 => {
                println!("Merci d'avoir utilisé notre application de To-do list. Au revoir !");
                break;
            },
            _ => println!("Option invalide, veuillez réessayer."),
        }
    }



    

}
fn ex2() {
    println!("Bienvenue sur votre calculatrice personnelle !");

    let mut option = 1;

    while option != 0 {
        println!("Veuillez choisir une option :");
        println!("1. Addition");
        println!("2. Soustraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("0. Quitter");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Erreur de lecture");
        option = input.trim().parse().expect("Veuillez entrer un nombre valide");

        match option {
            1 => 
            {
                println!("Entrez le premier nombre :");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Erreur de lecture");
                let num1: f64 = input.trim().parse().expect("Veuillez entrer un nombre valide");

                println!("Entrez le deuxième nombre :");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Erreur de lecture");
                let num2: f64 = input.trim().parse().expect("Veuillez entrer un nombre valide");

                let result = num1 + num2;
                println!("Le résultat de l'addition est : {}", result);
            },
            2 => 
            {
                println!("Entrez le premier nombre :");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Erreur de lecture");
                let num1: f64 = input.trim().parse().expect("Veuillez entrer un nombre valide");

                println!("Entrez le deuxième nombre :");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Erreur de lecture");
                let num2: f64 = input.trim().parse().expect("Veuillez entrer un nombre valide");

                let result = num1 - num2;
                println!("Le résultat de la soustraction est : {}", result);
            },
            3 => 
            {
                println!("Entrez le premier nombre :");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Erreur de lecture");
                let num1: f64 = input.trim().parse().expect("Veuillez entrer un nombre valide");

                println!("Entrez le deuxième nombre :");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Erreur de lecture");
                let num2: f64 = input.trim().parse().expect("Veuillez entrer un nombre valide");

                let result = num1 * num2;
                println!("Le résultat de la multiplication est : {}", result);
            },
            4 => 
            {
                println!("Entrez le premier nombre :");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Erreur de lecture");
                let num1: f64 = input.trim().parse().expect("Veuillez entrer un nombre valide");

                println!("Entrez le deuxième nombre :");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Erreur de lecture");
                let num2: f64 = input.trim().parse().expect("Veuillez entrer un nombre valide");

                if num2 == 0.0 {
                    println!("Erreur : Division par zéro n'est pas autorisée.");
                } else {
                    let result = num1 / num2;
                    println!("Le résultat de la division est : {}", result);
                }
            },
            0 => println!("Merci d'avoir utilisé notre calculatrice personnelle. Au revoir !"),
            _ => println!("Option invalide, veuillez réessayer."),
        }
    }


}

fn ex1() {
    // Message d'accueil
    println!("Bienvenue dans votre application de gestion de compte bancaire !");

    // Initialisation du compte
    let mut balance = 1000.0;
    println!("Votre solde initial est de : {}€", balance);
    
    let mut option = 1;
    while option != 0 {
        println!("Veuillez choisir une option :");
        println!("1. Afficher le solde");
        println!("2. Déposer de l'argent");
        println!("3. Retirer de l'argent");
        println!("0. Quitter");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Erreur de lecture");
        option = input.trim().parse().expect("Veuillez entrer un nombre valide");

        match option {
            1 => 
            {
                let borrowed_balance = &balance; // Emprunt du solde pour affichage
                println!("Votre solde actuel est de : {}€", borrowed_balance);
            },
            2 => 
             {
                println!("Entrez le montant à déposer :");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Erreur de lecture");

                    // Convertir un String vers un f64
                    let deposit: f64 = input.trim().parse().expect("Veuillez entrer un nombre valide");
                    // Gérer le dépot en faisant l'addition
                    balance += deposit;

                println!("Votre nouveau solde est de : {}€", balance);
             },
            3 => 
            {
                println!("Entrez le montant à retirer :");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Erreur de lecture");
                    
                let withdrawal: f64 = input.trim().parse().expect("Veuillez entrer un nombre valide");

                if withdrawal < 0.0 {
                    println!("Le montant de retrait ne peut pas être négatif.");
                } else {
                    if balance - withdrawal < 0.0 {
                        println!("Fonds insuffisants pour ce retrait.");
                    } else {
                        balance -= withdrawal;
                        println!("Votre solde après retrait est de : {}€", balance);
                    }
                }
            },
            0 => 
            {
                // Application des frais de gestion, affichage du solde final via le shadowing
                let withdrawal_fee = 10.0;
                let balance = balance - withdrawal_fee; // Shadowing pour appliquer les frais

                println!("Votre solde après application des frais de gestion de {}€ est de : {}€", withdrawal_fee, balance);
                println!("Merci d'avoir utilisé notre application de gestion de compte bancaire. Au revoir !");
            },
            _ => println!("Option invalide, veuillez réessayer."),
        }

    }
}