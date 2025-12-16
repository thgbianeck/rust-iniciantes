use std::io::{self};

fn main() {
    // Cabeçalho do sistema
    println!("╔════════════════════════════════╗");
    println!("║   📝 CADASTRO DE CLIENTE       ║");
    println!("╚════════════════════════════════╝\n");
    
    // Passo 1: Ler nome com validação
    let nome = ler_nome();
    
    // Passo 2: Ler email com validação
    let email = ler_email();
    
    // Passo 3: Ler telefone com validação
    let telefone = ler_telefone();
    
    // Passo 4: Mostrar resumo formatado
    exibir_resumo(&nome, &email, &telefone);
    
    // Passo 5: Confirmar dados
    let confirmado = confirmar();
    
    // Passo 6: Mensagem final baseada na confirmação
    if confirmado {
        println!("\n✅ Cadastro realizado com sucesso!");
        println!("🎉 Bem-vindo(a), {}!", nome);
        println!("📬 Enviaremos um email de confirmação para: {}", email);
    } else {
        println!("\n❌ Cadastro cancelado.");
        println!("💡 Você pode tentar novamente quando quiser!");
    }
}

/// Lê e valida o nome do usuário
/// Retorna uma String com o nome válido
fn ler_nome() -> String {
    loop {
        // Solicitar entrada
        println!("👤 Digite seu nome completo:");
        let mut entrada = String::new();

        // Ler do teclado
        io::stdin()
            .read_line(&mut entrada)
            .expect("❌ Erro ao ler entrada");

        // Limpar entrada (remove \n e espaços extras)
        let nome = entrada.trim().to_string();

        // Validação 1: Não vazio
        if nome.is_empty() {
            println!("❌ Nome não pode estar vazio!\n");
            continue; // Volta ao início do loop
        }

        // Validação 2: Tamanho mínimo
        if nome.len() < 3 {
            println!("❌ Nome muito curto (mínimo 3 caracteres). Tente novamente.\n");
            continue;
        }

        // Validação 3: Apenas letras e espaços (opcional)
        let apenas_letras = nome.chars().all(|c| c.is_alphabetic()  || c.is_whitespace());
        if !apenas_letras {
            println!("❌ Nome deve conter apenas letras.\n");
            continue;
        }

        // Todas as validações passaram!
        println!("✅ Nome aceito!\n");
        return nome; // Sai da função retornando o nome
    }
}

/// Lê e valida o email do usuário
/// Retorna uma String com o email válido
fn ler_email() -> String {
    loop {
        println!("📧 Digite seu email:");
        let mut entrada = String::new();
        
        io::stdin()
            .read_line(&mut entrada)
            .expect("❌ Erro ao ler entrada");
        
        let email = entrada.trim().to_string();
        
        // Validação 1: Não vazio
        if email.is_empty() {
            println!("❌ Email não pode estar vazio!\n");
            continue;
        }
        
        // Validação 2: Tamanho mínimo
        if email.len() < 5 {
            println!("❌ Email muito curto.\n");
            continue;
        }
        
        // Validação 3: Contém @
        if !email.contains("@") {
            println!("❌ Email inválido (deve conter @).\n");
            continue;
        }
        
        // Validação 4: Tem algo antes e depois do @
        let partes: Vec<&str> = email.split("@").collect();
        if partes.len() !=2 || partes[0].is_empty() || partes[1].is_empty() {
            println!("❌ Formato de email inválido.\n");
            continue;
        }
        
        println!("✅ Email aceito!\n");
        return email;
    }
}

/// Lê e valida o telefone do usuário
/// Retorna uma String com o telefone válido
fn ler_telefone() -> String {
    loop {
        println!("📱 Digite seu telefone (mínimo 8 dígitos):");
        let mut entrada = String::new();
        
        io::stdin()
            .read_line(&mut entrada)
            .expect("❌ Erro ao ler entrada");
        
        let telefone = entrada.trim().to_string();
        
        // Validação 1: Não vazio
        if telefone.is_empty() {
            println!("❌ Telefone não pode estar vazio!\n");
            continue;
        }
        
        // Validação 2: Contar apenas dígitos
        let digitos: String = telefone.chars().filter(|c| c.is_numeric()).collect();

        if digitos.len() < 8 {
            println!("❌ Telefone deve ter pelo menos 8 dígitos.\n");
            continue;
        }
        
        println!("✅ Telefone aceito!\n");
        return telefone;
    }
}

/// Exibe um resumo formatado dos dados cadastrados
/// Usa referências (&str) porque não precisa modificar
fn exibir_resumo(nome: &str, email: &str, telefone: &str) {
    println!("╔════════════════════════════════════════════╗");
    println!("║         📋 RESUMO DO CADASTRO              ║");
    println!("╠════════════════════════════════════════════╣");
    
    // Formatação com padding (preenchimento)
    println!("║ 👤 Nome:     {:<30}║", nome);
    println!("║ 📧 Email:    {:<30}║", email);
    println!("║ 📱 Telefone: {:<30}║", telefone);
    
    println!("╚════════════════════════════════════════════╝\n");
}
fn confirmar() -> bool {
    loop {
         println!("Os dados estão corretos? (S/N):");
        let mut resposta = String::new();
        
        io::stdin()
            .read_line(&mut resposta)
            .expect("❌ Erro ao ler entrada");
        
        // Converter para minúscula e pegar primeiro caractere
        let resposta = resposta.trim().to_lowercase();

        match resposta.as_str() {
            "s" | "sim" | "y" | "yes" => {
                return true; // Confirmado
            }
            "n" | "nao" | "não" | "no" => {
                return false; // Cancelado
            }
            _ => {
                println!("❌ Resposta inválida. Digite S para Sim ou N para Não.\n");
                // Loop continua
            }
        }
    }
}

