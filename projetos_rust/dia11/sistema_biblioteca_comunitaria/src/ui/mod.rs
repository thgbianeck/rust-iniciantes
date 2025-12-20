//! Interface do usuário

pub mod menu;
pub mod formatacao;

pub use menu::{exibir_menu_principal, exibir_menu_livros, exibir_menu_usuarios};
pub use formatacao::{formatar_livro, formatar_usuario, formatar_emprestimo};