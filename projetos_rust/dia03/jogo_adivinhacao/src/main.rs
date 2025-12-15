use std::io;

fn main() {
    println!("======================================\n");
    println!("🎯 BEM-VINDO AO JOGO DE ADIVINHAÇÃO! 🎯");
    println!("======================================\n");
    
    let numero_secreto = 42;
    
    let mut tentativas = 0;
    let tentativas_maximas = 7;
    
    loop {
        // TODO: Ler o palpite do usuário
        println!("Por favor, insira seu palpite (tentativa {}/{}):", tentativas + 1, tentativas_maximas);
        let mut palpite = String::new();
        io::stdin().read_line(&mut palpite).expect("Falha ao ler a linha");
        // TODO: Validar se é um número válido
        let _palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, insira um número válido!");
                continue;
            }
        };
        tentativas += 1;
        match _palpite.cmp(&numero_secreto) {
            std::cmp::Ordering::Less => println!("Muito baixo! Tente novamente."),
            std::cmp::Ordering::Greater => println!("Muito alto! Tente novamente."),
            std::cmp::Ordering::Equal => {
                println!("🎉 Parabéns! Você acertou o número secreto em {} tentativas! 🎉", tentativas);
                break;
            }
            
        }
        if tentativas >= tentativas_maximas {
            println!("😞 Suas tentativas acabaram! O número secreto era {}. Melhor sorte na próxima vez! 😞", numero_secreto);
            break;
        }
    }
    println!("\nObrigado por jogar! Até a próxima! 👋");
}