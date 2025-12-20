//! Modelo de empréstimo

/// Representa um empréstimo de livro
#[derive(Debug, Clone)]
pub struct Emprestimo {
    pub id: u32,
    pub usuario_id: u32,
    pub livro_id: u32,
    pub data_emprestimo: String,
    pub data_devolucao: Option<String>,
}

impl Emprestimo {
    /// Cria um novo empréstimo
    pub fn novo(id: u32, usuario_id: u32, livro_id: u32, data: String) -> Self {
        Emprestimo {
            id,
            usuario_id,
            livro_id,
            data_emprestimo: data,
            data_devolucao: None,
        }
    }
    
    /// Registra devolução
    pub fn devolver(&mut self, data: String) {
        self.data_devolucao = Some(data);
    }
    
    /// Verifica se foi devolvido
    pub fn foi_devolvido(&self) -> bool {
        self.data_devolucao.is_some()
    }
}