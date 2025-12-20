//! Serviço de gerenciamento de livros

use crate::models::Livro;
use std::collections::HashMap;

/// Gerencia operações com livros
pub struct LivroService {
    livros: HashMap<u32, Livro>,
    proximo_id: u32,
}

impl LivroService {
    /// Cria novo serviço
    pub fn novo() -> Self {
        LivroService {
            livros: HashMap::new(),
            proximo_id: 1,
        }
    }
    
    /// Adiciona livro
    pub fn adicionar(&mut self, titulo: String, autor: String, ano: u32) -> Result<u32, String> {
        let id = self.proximo_id;
        let livro = Livro::novo(id, titulo, autor, ano)?;
        
        self.livros.insert(id, livro);
        self.proximo_id += 1;
        
        Ok(id)
    }
    
    /// Busca livro por ID
    pub fn buscar(&self, id: u32) -> Option<&Livro> {
        self.livros.get(&id)
    }
    
    /// Busca livro mutável por ID
    pub fn buscar_mut(&mut self, id: u32) -> Option<&mut Livro> {
        self.livros.get_mut(&id)
    }
    
    /// Lista todos os livros
    pub fn listar(&self) -> Vec<&Livro> {
        self.livros.values().collect()
    }
    
    /// Lista livros disponíveis
    pub fn listar_disponiveis(&self) -> Vec<&Livro> {
        self.livros
            .values()
            .filter(|l| l.disponivel)
            .collect()
    }
}