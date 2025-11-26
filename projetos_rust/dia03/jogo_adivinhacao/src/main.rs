use std::io;
use std::cmp::Ordering;

fn main() {
    println!("🎯 BEM-VINDO AO JOGO DE ADIVINHAÇÃO! 🎯");
    println!("======================================");
    println!("Tente adivinhar o número entre 1 e 100!\n");
    
    // Número secreto (em um jogo real, seria aleatório)
    let numero_secreto = 42;
    
    // Variáveis de controle do jogo
    let mut tentativas = 0;
    let tentativas_maximas = 7;
    let mut acertou = false;
    
    // Loop principal do jogo
    loop {
        tentativas += 1;
        println!("\n--- Tentativa {}/{} ---", tentativas, tentativas_maximas);
        print!("Digite seu palpite: ");
        
        // Garante que o print apareça antes do input
        use std::io::Write;
        io::stdout().flush().unwrap();
        
        // Lê a entrada do usuário
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Falha ao ler entrada");
        
        // Converte para número e valida
        let palpite: i32 = match entrada.trim().parse() {
            Ok(num) => {
                // Valida se está no intervalo correto
                if num < 1 || num > 100 {
                    println!("⚠️ Digite um número entre 1 e 100!");
                    tentativas -= 1; // Não conta como tentativa
                    continue;
                }
                num
            }
            Err(_) => {
                println!("❌ Por favor, digite um número válido!");
                tentativas -= 1; // Não conta como tentativa
                continue;
            }
        };
        
        // Compara o palpite com o número secreto
        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("📉 Seu palpite é MENOR que o número secreto!"),
            Ordering::Greater => println!("📈 Seu palpite é MAIOR que o número secreto!"),
            Ordering::Equal => {
                println!("\n🎉🎉🎉 PARABÉNS! Você acertou! 🎉🎉🎉");
                acertou = true;
                break; // Sai do loop
            }
        }
        
        // Verifica se as tentativas acabaram
        if tentativas >= tentativas_maximas {
            println!("\n💀 Game Over! Suas tentativas acabaram!");
            println!("O número secreto era: {}", numero_secreto);
            break;
        }
        
        // Dica extra baseada na proximidade
        let diferenca = (palpite - numero_secreto).abs();
        match diferenca {
            1..=5 => println!("🔥 Muito quente!"),
            6..=10 => println!("🌡️ Quente!"),
            11..=20 => println!("🧊 Frio!"),
            _ => println!("❄️ Muito frio!"),
        }
    }
    
    // Cálculo da pontuação
    if acertou {
        let pontuacao = 100 - ((tentativas - 1) * 10);
        
        println!("\n📊 ESTATÍSTICAS FINAIS");
        println!("====================");
        println!("Tentativas usadas: {}", tentativas);
        println!("Pontuação: {} pontos", pontuacao);
        
        // Avaliação baseada na pontuação
        let avaliacao = match pontuacao {
            100 => "🏆 PERFEITO! Primeira tentativa!",
            80..=90 => "⭐ Excelente!",
            60..=79 => "👍 Muito bom!",
            40..=59 => "👌 Bom!",
            _ => "📈 Continue praticando!",
        };
        
        println!("Avaliação: {}", avaliacao);
    }
    
    println!("\n✨ Obrigado por jogar! ✨");
}