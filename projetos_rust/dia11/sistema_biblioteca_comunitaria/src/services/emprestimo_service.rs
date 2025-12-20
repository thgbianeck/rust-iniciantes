//! Serviço de processamento de empréstimos

use crate::models::Emprestimo;
use crate::services::{LivroService, UsuarioService};
use std::collections::HashMap;

/// Gerencia operações de empréstimos
pub struct EmprestimoService {
    emprestimos: HashMap<u32, Emprestimo>,
    proximo_id: u32,
}

impl EmprestimoService {
    /// Cria novo serviço
    pub fn novo() -> Self {
        EmprestimoService {
            emprestimos: HashMap::new(),
            proximo_id: 1,
        }
    }
    
    /// Processa empréstimo
    pub fn emprestar(
        &mut self,
        usuario_service: &UsuarioService,
        livro_service: &mut LivroService,
        usuario_id: u32,
        livro_id: u32,
        data: String,
    ) -> Result<u32, String> {
        // Valida usuário
        usuario_service
            .buscar(usuario_id)
            .ok_or("Usuário não encontrado")?;
        
        // Valida e empresta livro
        let livro = livro_service
            .buscar_mut(livro_id)
            .ok_or("Livro não encontrado")?;
        
        livro.emprestar()?;
        
        // Cria empréstimo
        let id = self.proximo_id;
        let emprestimo = Emprestimo::novo(id, usuario_id, livro_id, data);
        
        self.emprestimos.insert(id, emprestimo);
        self.proximo_id += 1;
        
        Ok(id)
    }
    
    /// Processa devolução
    pub fn devolver(
        &mut self,
        livro_service: &mut LivroService,
        emprestimo_id: u32,
        data: String,
    ) -> Result<(), String> {
        let emprestimo = self.emprestimos
            .get_mut(&emprestimo_id)
            .ok_or("Empréstimo não encontrado")?;
        
        if emprestimo.foi_devolvido() {
            return Err("Livro já foi devolvido".to_string());
        }
        
        // Marca devolução
        emprestimo.devolver(data);
        
        // Atualiza livro
        let livro = livro_service
            .buscar_mut(emprestimo.livro_id)
            .ok_or("Livro não encontrado")?;
        
        livro.devolver();
        
        Ok(())
    }
    
    /// Lista empréstimos ativos
    pub fn listar_ativos(&self) -> Vec<&Emprestimo> {
        self.emprestimos
            .values()
            .filter(|e| !e.foi_devolvido())
            .collect()
    }
    
    /// Lista todos os empréstimos
    pub fn listar(&self) -> Vec<&Emprestimo> {
        self.emprestimos.values().collect()
    }
}