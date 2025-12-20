//! Modelo de usuário

use crate::utils::validacao;

/// Representa um usuário da biblioteca
#[derive(Debug, Clone)]
pub struct Usuario {
    pub id: u32,
    pub nome: String,
    pub cpf: String,
}

impl Usuario {
    /// Cria um novo usuário
    pub fn novo(id: u32, nome: String, cpf: String) -> Result<Self, String> {
        if nome.is_empty() {
            return Err("Nome não pode ser vazio".to_string());
        }

        if !validacao::validar_cpf(&cpf) {
            return Err("CPF inválido".to_string());
        }

        Ok(Usuario { id, nome, cpf })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criar_usuario_valido() {
        let usuario = Usuario::novo(1, "Alice".to_string(), "12345678901".to_string());
        assert!(usuario.is_ok());
    }

    #[test]
    fn test_cpf_invalido() {
        let usuario = Usuario::novo(1, "Alice".to_string(), "123".to_string());
        assert!(usuario.is_err());
    }
}
