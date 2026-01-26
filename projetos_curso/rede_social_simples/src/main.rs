use std::collections::HashMap;
use std::io::{self, Read};

struct User{
    name: String,
    age: u32,
    friends: Vec<String>
}

fn main() {
    let mut users = HashMap::new();

    loop{
        println!("Escolha uma opção:");
        println!("1. Adicionar usuário");
        println!("2. Adicionar amigo");
        println!("3. Ver lista de amigos");
        println!("4. Sair");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap();

        match choice{
            1 => {
                println!("Digite o nome do usuário: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                println!("Digite a idade do usuário: ");
                let mut age = String::new();
                io::stdin().read_line(&mut age).unwrap();
                let age: u32 = age.trim().parse().unwrap();
                let user = User{
                    name: name.clone(),
                    age,
                    friends: Vec::new()
                };

                users.insert(name, user);
            },

            2 => {

                println!("Digite o nome do usuário: ");
                let mut username = String::new();
                io::stdin().read_line(&mut username).unwrap();
                let mut username = username.trim().to_string();

                println!("Digite o nome do amigo: ");
                let mut friendname = String::new();
                io::stdin().read_line( &mut friendname).unwrap();
                let friendname = friendname.trim().to_string();

                let user = users.get_mut(&mut username).unwrap();
                user.friends.push(friendname);
            }

            3 => {
                println!("Digite o nome do usuário: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                let user = users.get(&name).unwrap();
                println!("Amigos de {}", name);
                for friend in &user.friends{
                    println!("- {}", friend)
                }
            },

            4 => {
                break;
            },

            _ => {
                println!("Opção inválida");
            }
        }
    }
}
