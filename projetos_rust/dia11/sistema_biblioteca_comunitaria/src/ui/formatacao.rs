//! Formatação de dados para exibição

use crate::models::{Livro, Usuario, Emprestimo};

/// Formata livro para exibição
pub fn formatar_livro(livro: &Livro) -> String {
    let status = if livro.disponivel { "✅ Disponível" } else { "❌ Emprestado" };
    format!(
        "ID: {} | {} - {} ({}) | {}",
        livro.id, livro.titulo, livro.autor, livro.ano, status
    )
}

/// Formata usuário para exibição
pub fn formatar_usuario(usuario: &Usuario) -> String {
    format!(
        "ID: {} | Nome: {} | CPF: {}",
        usuario.id, usuario.nome, usuario.cpf
    )
}

/// Formata empréstimo para exibição
pub fn formatar_emprestimo(emprestimo: &Emprestimo) -> String {
    let status = match &emprestimo.data_devolucao {
        Some(data) => format!("Devolvido em {}", data),
        None => "Em aberto".to_string(),
    };
    
    format!(
        "ID: {} | Usuário: {} | Livro: {} | Empréstimo: {} | {}",
        emprestimo.id,
        emprestimo.usuario_id,
        emprestimo.livro_id,
        emprestimo.data_emprestimo,
        status
    )
}