fn main() {
    //ex1();
    ex2();
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