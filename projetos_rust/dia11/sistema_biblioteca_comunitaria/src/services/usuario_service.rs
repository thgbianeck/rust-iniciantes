//! Serviço de gerenciamento de usuários

use crate::models::Usuario;
use std::collections::HashMap;

/// Gerencia operações com usuários
pub struct UsuarioService {
    usuarios: HashMap<u32, Usuario>,
    proximo_id: u32,
}

impl UsuarioService {
    /// Cria novo serviço
    pub fn novo() -> Self {
        UsuarioService {
            usuarios: HashMap::new(),
            proximo_id: 1,
        }
    }
    
    /// Cadastra usuário
    pub fn cadastrar(&mut self, nome: String, cpf: String) -> Result<u32, String> {
        // Verifica se CPF já existe
        if self.usuarios.values().any(|u| u.cpf == cpf) {
            return Err("CPF já cadastrado".to_string());
        }
        
        let id = self.proximo_id;
        let usuario = Usuario::novo(id, nome, cpf)?;
        
        self.usuarios.insert(id, usuario);
        self.proximo_id += 1;
        
        Ok(id)
    }
    
    /// Busca usuário por ID
    pub fn buscar(&self, id: u32) -> Option<&Usuario> {
        self.usuarios.get(&id)
    }
    
    /// Lista todos os usuários
    pub fn listar(&self) -> Vec<&Usuario> {
        self.usuarios.values().collect()
    }
}