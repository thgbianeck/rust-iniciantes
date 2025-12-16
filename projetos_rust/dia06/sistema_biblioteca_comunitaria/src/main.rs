struct Livro {
    titulo: String,
    autor: String,
    ano_publicacao: u32,
    isbn: String,
    disponivel: bool,
    numero_paginas: u32,
}

impl Livro {
    fn novo(
        titulo: String,
        autor: String,
        ano_publicacao: u32,
        isbn: String,
        numero_paginas: u32,
    ) -> Self {
        Self {
            titulo,
            autor,
            ano_publicacao,
            isbn,
            disponivel: true, // Por padrão, livro novo está disponível
            numero_paginas,
        }
    }

    fn criar_classico(titulo: String, autor: String, ano_publicacao: u32) -> Self {
        Self {
            titulo,
            autor,
            ano_publicacao,
            isbn: String::from("N/A"), // Clássicos podem não ter ISBN
            disponivel: true,
            numero_paginas: 0, // Número de páginas não especificado
        }
    }

    // Método: exibir todas as informações
    fn exibir_detalhes(&self) {
        println!("\n📖 Detalhes do Livro:");
        println!("   Título: {}", self.titulo);
        println!("   Autor: {}", self.autor);
        println!("   Ano: {}", self.ano_publicacao);
        println!("   ISBN: {}", self.isbn);
        println!("   Páginas: {}", self.numero_paginas);
        println!(
            "   Status: {}",
            if self.disponivel {
                "✅ Disponível"
            } else {
                "❌ Emprestado"
            }
        );
    }

    fn emprestar(&mut self) {
        if !self.disponivel {
            self.disponivel = true;
            println!("✅ '{}' foi devolvido. Obrigado!", self.titulo);
        } else {
            println!("⚠️ '{}' não estava emprestado.", self.titulo);
        }
    }

    // Método: devolver livro (modifica estado)
    fn devolver(&mut self) {
        if !self.disponivel {
            self.disponivel = true;
            println!("✅ '{}' foi devolvido. Obrigado!", self.titulo);
        } else {
            println!("⚠️ '{}' não estava emprestado.", self.titulo);
        }
    }

    // Método: verificar se é clássico (leitura)
    fn eh_classico(&self) -> bool {
        let ano_atual = 2025;
        ano_atual - self.ano_publicacao > 50
    }

}

// ========== STRUCT BIBLIOTECA ==========
struct Biblioteca {
    livros: Vec<Livro>,
}

impl Biblioteca {
    // Função associada: criar biblioteca vazia
    fn nova() -> Self {
        Self {
            livros: Vec::new(),
        }
    }

    // Método: adicionar livro à coleção
    fn adicionar_livro(&mut self, livro: Livro) {
        println!("➕ Adicionando '{}' à biblioteca...", livro.titulo);
        self.livros.push(livro);
    }

    // Método: listar livros disponíveis
    fn listar_disponiveis(&self) {
        println!("\n📚 Livros Disponíveis:");
        let mut encontrados = 0;

        for livro in &self.livros {
            if livro.disponivel {
                println!("   • {} - {} ({})", 
                    livro.titulo, 
                    livro.autor, 
                    livro.ano_publicacao
                );
                encontrados += 1;
            }
        }

        if encontrados == 0 {
            println!("   Nenhum livro disponível no momento.");
        } else {
            println!("\nTotal: {} livro(s) disponível(is)", encontrados);
        }
    }

    // Método: buscar livros por autor
    fn buscar_por_autor(&self, autor: &str) {
        println!("\n🔍 Buscando livros de '{}':", autor);
        let mut encontrados = 0;
        
        for livro in &self.livros {
            // Busca case-insensitive
            if livro.autor.to_lowercase().contains(&autor.to_lowercase()) {
                let status = if livro.disponivel { "✅" } else { "❌" };
                println!("   {} {} ({}) - {}", 
                    status,
                    livro.titulo, 
                    livro.ano_publicacao,
                    if livro.eh_classico() { "📜 CLÁSSICO" } else { "" }
                );
                encontrados += 1;
            }
        }
        
        if encontrados == 0 {
            println!("   Nenhum livro encontrado para este autor.");
        }
    }

    // Método extra: estatísticas da biblioteca
    fn estatisticas(&self) {
        let total = self.livros.len();
        let disponiveis = self.livros.iter().filter(|l| l.disponivel).count();
        let emprestados = total - disponiveis;
        let classicos = self.livros.iter().filter(|l| l.eh_classico()).count();
        
        println!("\n📊 Estatísticas da Biblioteca:");
        println!("   Total de livros: {}", total);
        println!("   Disponíveis: {}", disponiveis);
        println!("   Emprestados: {}", emprestados);
        println!("   Clássicos: {}", classicos);
    }


}

// ========== PROGRAMA PRINCIPAL ==========

fn main() {
    println!("=== SISTEMA DE GERENCIAMENTO DE BIBLIOTECA ===\n");
    
    // Criar biblioteca
    let mut biblioteca = Biblioteca::nova();
    
    // Adicionar livros usando construtor padrão
    let livro1 = Livro::novo(
        String::from("O Senhor dos Anéis"),
        String::from("J.R.R. Tolkien"),
        1954,
        String::from("978-0544003415"),
        1178
    );
    
    let livro2 = Livro::novo(
        String::from("Clean Code"),
        String::from("Robert C. Martin"),
        2008,
        String::from("978-0132350884"),
        464
    );
    
    // Adicionar clássicos usando atalho
    let livro3 = Livro::criar_classico(
        String::from("Dom Casmurro"),
        String::from("Machado de Assis"),
        1899
    );
    
    let livro4 = Livro::criar_classico(
        String::from("1984"),
        String::from("George Orwell"),
        1949
    );
    
    // Adicionar à biblioteca
    biblioteca.adicionar_livro(livro1);
    biblioteca.adicionar_livro(livro2);
    biblioteca.adicionar_livro(livro3);
    biblioteca.adicionar_livro(livro4);
    
    // Listar disponíveis
    biblioteca.listar_disponiveis();
    
    // Emprestar livro (precisamos de acesso mutável)
    // NOTA: Como movemos os livros para o Vec, precisamos acessá-los via índice
    println!("\n--- OPERAÇÕES DE EMPRÉSTIMO ---");
    biblioteca.livros[0].emprestar();  // Emprestar "O Senhor dos Anéis"
    biblioteca.livros[1].emprestar();  // Emprestar "Clean Code"
    
    // Listar disponíveis novamente
    biblioteca.listar_disponiveis();
    
    // Devolver um livro
    println!("\n--- DEVOLUÇÃO ---");
    biblioteca.livros[0].devolver();
    
    // Buscar por autor
    biblioteca.buscar_por_autor("Machado");
    biblioteca.buscar_por_autor("Martin");
    
    // Exibir detalhes de um livro específico
    println!("\n--- DETALHES DE UM LIVRO ---");
    biblioteca.livros[2].exibir_detalhes();
    
    // Estatísticas finais
    biblioteca.estatisticas();
    
    println!("\n=== FIM DO PROGRAMA ===");
}