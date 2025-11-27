// Valida se o email contém @ e .
fn validar_email(email: &str) -> (bool, String) {
    let tem_arroba = email.contains('@');
    let tem_ponto = email.contains('.');
    
    if tem_arroba && tem_ponto {
        (true, String::from("✅ Email válido"))
    } else {
        (false, String::from("❌ Email inválido: deve conter @ e ."))
    }
}

// Valida se a senha tem mínimo 8 caracteres e pelo menos 1 número
fn validar_senha(senha: &str) -> (bool, String) {
    let tamanho_ok = senha.len() >= 8;
    let tem_numero = senha.chars().any(|c| c.is_numeric());
    
    if tamanho_ok && tem_numero {
        (true, String::from("✅ Senha válida"))
    } else if !tamanho_ok {
        (false, String::from("❌ Senha inválida: mínimo 8 caracteres"))
    } else {
        (false, String::from("❌ Senha inválida: deve conter pelo menos 1 número"))
    }
}

// Valida se a idade está entre 18 e 120 anos
fn validar_idade(idade: u8) -> (bool, String) {
    if idade >= 18 && idade <= 120 {
        (true, String::from("✅ Idade válida"))
    } else if idade < 18 {
        (false, String::from("❌ Idade inválida: menor de 18 anos"))
    } else {
        (false, String::from("❌ Idade inválida: valor não realista"))
    }
}

// Valida se o CPF tem exatamente 11 dígitos numéricos
fn validar_cpf(cpf: &str) -> (bool, String) {
    let tamanho_correto = cpf.len() == 11;
    let apenas_numeros = cpf.chars().all(|c| c.is_numeric());
    
    if tamanho_correto && apenas_numeros {
        (true, String::from("✅ CPF válido (formato)"))
    } else if !tamanho_correto {
        (false, format!("❌ CPF inválido: deve ter 11 dígitos (tem {})", cpf.len()))
    } else {
        (false, String::from("❌ CPF inválido: deve conter apenas números"))
    }
}

// Função coordenadora que processa todo o cadastro
fn processar_cadastro(nome: &str, email: &str, senha: &str, idade: u8, cpf: &str) {
    println!("=== VALIDAÇÃO DE CADASTRO ===");
    println!("Nome: {}\n", nome);
    
    // Validar cada campo usando as funções especializadas
    let (email_valido, msg_email) = validar_email(email);
    let (senha_valida, msg_senha) = validar_senha(senha);
    let (idade_valida, msg_idade) = validar_idade(idade);
    let (cpf_valido, msg_cpf) = validar_cpf(cpf);
    
    // Exibir resultado de cada validação
    println!("{}", msg_email);
    println!("{}", msg_senha);
    println!("{}", msg_idade);
    println!("{}", msg_cpf);
    
    // Verificar se TODOS os campos são válidos
    let tudo_valido = email_valido && senha_valida && idade_valida && cpf_valido;
    
    // Mensagem final
    println!();
    if tudo_valido {
        println!("🎉 CADASTRO APROVADO!");
        println!("Usuário {} cadastrado com sucesso!", nome);
    } else {
        println!("⚠️ CADASTRO REJEITADO!");
        println!("Por favor, corrija os erros acima e tente novamente.");
    }
}

fn main() {
    // Teste 1: Cadastro válido
    println!(">>> TESTE 1: Cadastro válido\n");
    processar_cadastro(
        "Carlos Silva",
        "carlos@email.com",
        "senha123",
        25,
        "12345678901"
    );
    
    println!("\n{}\n", "=".repeat(50));
    
    // Teste 2: Cadastro com erros
    println!(">>> TESTE 2: Cadastro com erros\n");
    processar_cadastro(
        "Ana Costa",
        "ana.email.com",    // Email sem @
        "curta1",           // Senha muito curta
        16,                 // Menor de idade
        "123456789"         // CPF incompleto
    );
}