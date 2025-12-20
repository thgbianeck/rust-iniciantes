//! Modelo de livro

/// Representa um livro da biblioteca
#[derive(Debug, Clone)]
pub struct Livro {
    pub id: u32,
    pub titulo: String,
    pub autor: String,
    pub ano: u32,
    pub disponivel: bool,
}

impl Livro {
    /// Cria um novo livro
    pub fn novo(id: u32, titulo: String, autor: String, ano: u32) -> Result<Self, String> {
        if titulo.is_empty() {
            return Err("Título não pode ser vazio".to_string());
        }

        if autor.is_empty() {
            return Err("Autor não pode ser vazio".to_string());
        }

        if ano < 1000 || ano > 2100 {
            return Err("Ano inválido".to_string());
        }

        Ok(Livro {
            id,
            titulo,
            autor,
            ano,
            disponivel: true,
        })
    }

    /// Marca livro como emprestado
    pub fn emprestar(&mut self) -> Result<(), String> {
        if !self.disponivel {
            return Err("Livro já está emprestado".to_string());
        }
        self.disponivel = false;
        Ok(())
    }

    /// Marca livro como devolvido
    pub fn devolver(&mut self) {
        self.disponivel = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criar_livro_valido() {
        let livro = Livro::novo(1, "1984".to_string(), "George Orwell".to_string(), 1949);
        assert!(livro.is_ok());
    }

    #[test]
    fn test_emprestar_livro() {
        let mut livro =
            Livro::novo(1, "1984".to_string(), "George Orwell".to_string(), 1949).unwrap();

        assert!(livro.emprestar().is_ok());
        assert!(!livro.disponivel);
        assert!(livro.emprestar().is_err());
    }
}
