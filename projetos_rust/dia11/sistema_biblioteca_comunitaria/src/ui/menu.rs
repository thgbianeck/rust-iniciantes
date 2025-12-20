//! Menus da aplicação

/// Exibe menu principal
pub fn exibir_menu_principal() {
    println!("\n{}", "=".repeat(50));
    println!("           SISTEMA DE BIBLIOTECA");
    println!("{}", "=".repeat(50));
    println!("1. Gerenciar Livros");
    println!("2. Gerenciar Usuários");
    println!("3. Gerenciar Empréstimos");
    println!("4. Relatórios");
    println!("0. Sair");
    println!("{}", "=".repeat(50));
}

/// Exibe menu de livros
pub fn exibir_menu_livros() {
    println!("\n--- LIVROS ---");
    println!("1. Adicionar Livro");
    println!("2. Listar Todos");
    println!("3. Listar Disponíveis");
    println!("0. Voltar");
}

/// Exibe menu de usuários
pub fn exibir_menu_usuarios() {
    println!("\n--- USUÁRIOS ---");
    println!("1. Cadastrar Usuário");
    println!("2. Listar Usuários");
    println!("0. Voltar");
}

/// Exibe menu de empréstimos
pub fn exibir_menu_emprestimos() {
    println!("\n--- EMPRÉSTIMOS ---");
    println!("1. Emprestar Livro");
    println!("2. Devolver Livro");
    println!("3. Listar Ativos");
    println!("0. Voltar");
}