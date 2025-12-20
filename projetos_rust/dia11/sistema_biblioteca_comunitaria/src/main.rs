//! Ponto de entrada da aplicação

use biblioteca::ui::{
    exibir_menu_principal, formatar_emprestimo, formatar_livro, formatar_usuario,
};
use biblioteca::{EmprestimoService, LivroService, UsuarioService};

fn main() {
    let mut livro_service = LivroService::novo();
    let mut usuario_service = UsuarioService::novo();
    let mut emprestimo_service = EmprestimoService::novo();

    // Dados de exemplo
    println!("📚 Inicializando sistema...\n");

    // Adiciona livros
    let livro1 = livro_service
        .adicionar("1984".to_string(), "George Orwell".to_string(), 1949)
        .expect("Erro ao adicionar livro");

    let livro2 = livro_service
        .adicionar(
            "O Senhor dos Anéis".to_string(),
            "J.R.R. Tolkien".to_string(),
            1954,
        )
        .expect("Erro ao adicionar livro");

    // Cadastra usuários
    let usuario1 = usuario_service
        .cadastrar("Alice Silva".to_string(), "12345678901".to_string())
        .expect("Erro ao cadastrar usuário");

    let usuario2 = usuario_service
        .cadastrar("Bob Santos".to_string(), "98765432100".to_string())
        .expect("Erro ao cadastrar usuário");

    // Exibe menu
    exibir_menu_principal();

    // Lista livros
    println!("\n📖 LIVROS CADASTRADOS:");
    for livro in livro_service.listar() {
        println!("  {}", formatar_livro(livro));
    }

    // Lista usuários
    println!("\n👥 USUÁRIOS CADASTRADOS:");
    for usuario in usuario_service.listar() {
        println!("  {}", formatar_usuario(usuario));
    }

    // Processa empréstimo
    println!("\n📋 PROCESSANDO EMPRÉSTIMO...");
    match emprestimo_service.emprestar(
        &usuario_service,
        &mut livro_service,
        usuario1,
        livro1,
        "2024-01-15".to_string(),
    ) {
        Ok(id) => println!("✅ Empréstimo {} realizado com sucesso!", id),
        Err(e) => println!("❌ Erro: {}", e),
    }

    // Lista empréstimos ativos
    println!("\n📚 EMPRÉSTIMOS ATIVOS:");
    for emprestimo in emprestimo_service.listar_ativos() {
        println!("  {}", formatar_emprestimo(emprestimo));
    }

    // Lista livros disponíveis
    println!("\n✅ LIVROS DISPONÍVEIS:");
    for livro in livro_service.listar_disponiveis() {
        println!("  {}", formatar_livro(livro));
    }
}
