use std::fmt;

// ========================================
// 1. DEFINIÇÃO DE ERROS CUSTOMIZADOS
// ========================================

#[derive(Debug)]
enum ErroValidacao {
    EmailInvalido { email: String, motivo: String },
    SenhaFraca { problemas: Vec<String> },
    CpfInvalido { cpf: String, motivo: String },
    CampoVazio { campo: String },
}

impl fmt::Display for ErroValidacao {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErroValidacao::EmailInvalido { email, motivo } => {
                write!(f, "❌ Email inválido '{}': {}", email, motivo)
            }
            ErroValidacao::SenhaFraca { problemas } => {
                writeln!(f, "❌ Senha não atende aos requisitos:")?;
                for problema in problemas {
                    writeln!(f, "   • {}", problema)?;
                }
                Ok(())
            }
            ErroValidacao::CpfInvalido { cpf, motivo } => {
                write!(f, "❌ CPF inválido '{}': {}", cpf, motivo)
            }
            ErroValidacao::CampoVazio { campo } => {
                write!(f, "❌ Campo obrigatório não preenchido: '{}'", campo)
            }
        }
    }
}

impl std::error::Error for ErroValidacao {}

// ========================================
// 2. ESTRUTURA DE DADOS
// ========================================

#[derive(Debug)]
struct Usuario {
    email: String,
    senha: String,
    cpf: String,
}

// ========================================
// 3. FUNÇÕES DE VALIDAÇÃO
// ========================================

fn validar_email(email: &str) -> Result<(), ErroValidacao> {
    // Verifica se está vazio
    if email.trim().is_empty() {
        return Err(ErroValidacao::CampoVazio {
            campo: "email".to_string(),
        });
    }

    // Verifica se contém @
    if !email.contains('@') {
        return Err(ErroValidacao::EmailInvalido {
            email: email.to_string(),
            motivo: "deve conter '@'".to_string(),
        });
    }

    // Verifica se tem domínio após @
    let partes: Vec<&str> = email.split('@').collect();
    if partes.len() != 2 || partes[1].is_empty() || !partes[1].contains('.') {
        return Err(ErroValidacao::EmailInvalido {
            email: email.to_string(),
            motivo: "formato deve ser usuario@dominio.com".to_string(),
        });
    }

    Ok(())
}

fn validar_senha(senha: &str) -> Result<(), ErroValidacao> {
    if senha.is_empty() {
        return Err(ErroValidacao::CampoVazio {
            campo: "senha".to_string(),
        });
    }

    let mut problemas = Vec::new();

    // Verifica tamanho mínimo
    if senha.len() < 8 {
        problemas.push(format!(
            "Deve ter no mínimo 8 caracteres (atual: {})",
            senha.len()
        ));
    }

    // Verifica se tem número
    if !senha.chars().any(|c| c.is_numeric()) {
        problemas.push("Deve conter pelo menos um número".to_string());
    }

    // Verifica se tem letra maiúscula
    if !senha.chars().any(|c| c.is_uppercase()) {
        problemas.push("Deve conter pelo menos uma letra maiúscula".to_string());
    }

    // Verifica se tem letra minúscula
    if !senha.chars().any(|c| c.is_lowercase()) {
        problemas.push("Deve conter pelo menos uma letra minúscula".to_string());
    }

    if !problemas.is_empty() {
        return Err(ErroValidacao::SenhaFraca { problemas });
    }

    Ok(())
}

fn validar_cpf(cpf: &str) -> Result<(), ErroValidacao> {
    if cpf.trim().is_empty() {
        return Err(ErroValidacao::CampoVazio {
            campo: "cpf".to_string(),
        });
    }

    // Remove caracteres não numéricos
    let cpf_limpo: String = cpf.chars().filter(|c| c.is_numeric()).collect();

    // Verifica se tem 11 dígitos
    if cpf_limpo.len() != 11 {
        return Err(ErroValidacao::CpfInvalido {
            cpf: cpf.to_string(),
            motivo: format!("deve ter 11 dígitos (encontrados: {})", cpf_limpo.len()),
        });
    }

    // Verifica se não são todos dígitos iguais (000.000.000-00, etc)
    if cpf_limpo
        .chars()
        .all(|c| c == cpf_limpo.chars().next().unwrap())
    {
        return Err(ErroValidacao::CpfInvalido {
            cpf: cpf.to_string(),
            motivo: "não pode ter todos os dígitos iguais".to_string(),
        });
    }

    // Em produção, implementar validação completa com dígitos verificadores

    Ok(())
}

// ========================================
// 4. FUNÇÃO PRINCIPAL DE CADASTRO
// ========================================

fn cadastrar_usuario(email: &str, senha: &str, cpf: &str) -> Result<Usuario, ErroValidacao> {
    // Usa ? para propagar erros automaticamente
    validar_email(email)?;
    validar_senha(senha)?;
    validar_cpf(cpf)?;

    // Se chegou aqui, todos os campos são válidos!
    Ok(Usuario {
        email: email.trim().to_lowercase(), // Normaliza email
        senha: senha.to_string(),           // Em produção: hash a senha!
        cpf: cpf.chars().filter(|c| c.is_numeric()).collect(), // Remove formatação
    })
}

// ========================================
// 5. FUNÇÃO COM LOGGING E RECOVERY
// ========================================

fn processar_cadastro(email: &str, senha: &str, cpf: &str) {
    println!("\n🔄 Processando cadastro...");
    println!("📧 Email: {}", email);
    println!("🔐 Senha: {}", "*".repeat(senha.len()));
    println!("🆔 CPF: {}", cpf);
    println!("{}", "=".repeat(50));

    match cadastrar_usuario(email, senha, cpf) {
        Ok(usuario) => {
            println!("✅ SUCESSO! Usuário cadastrado:");
            println!("   Email: {}", usuario.email);
            println!("   CPF: {}", usuario.cpf);
            println!("   Senha: [HASH ARMAZENADO COM SEGURANÇA]");
        }
        Err(erro) => {
            eprintln!("\n{}", erro);
            eprintln!("\n💡 Dica: Corrija os erros acima e tente novamente.");

            // Logging para monitoramento (em produção, usar biblioteca de log)
            eprintln!("\n📊 [LOG] Tentativa de cadastro falhou: {:?}", erro);
        }
    }
}

// ========================================
// 6. FUNÇÃO MAIN COM CASOS DE TESTE
// ========================================

fn main() {
    println!("🦀 Sistema de Validação Robusto em Rust\n");

    // Caso 1: Todos os campos válidos ✅
    processar_cadastro("joao.silva@email.com", "Senha123", "123.456.789-09");

    // Caso 2: Email inválido ❌
    processar_cadastro("emailsemarroba", "Senha123", "12345678909");

    // Caso 3: Senha fraca ❌
    processar_cadastro("maria@email.com", "123", "98765432100");

    // Caso 4: CPF inválido ❌
    processar_cadastro(
        "pedro@email.com",
        "SenhaForte123",
        "123", // Muito curto
    );

    // Caso 5: Múltiplos erros (testa propagação) ❌
    processar_cadastro(
        "", // Email vazio
        "fraca",
        "00000000000", // CPF com dígitos iguais
    );

    // Caso 6: CPF com formatação (recovery) ✅
    processar_cadastro(
        "ana@email.com",
        "SenhaSegura123",
        "123.456.789-09", // Com pontos e traço - será limpo
    );
}
