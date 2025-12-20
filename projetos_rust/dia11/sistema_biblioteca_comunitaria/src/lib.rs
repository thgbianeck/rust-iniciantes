//! # Sistema de Biblioteca
//!
//! Sistema completo de gerenciamento de biblioteca com livros, usuários e empréstimos.

pub mod models;
pub mod services;
pub mod ui;
pub mod utils;

// Re-exports públicos
pub use models::{Emprestimo, Livro, Usuario};
pub use services::{EmprestimoService, LivroService, UsuarioService};
