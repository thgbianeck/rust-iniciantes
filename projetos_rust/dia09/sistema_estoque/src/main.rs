use std::collections::HashMap;

// Estrutura que representa um produto
#[derive(Debug, Clone)]
struct Produto {
    codigo: String,
    nome: String,
    preco: f64,
    quantidade: u32,
    categoria: String,
}

impl Produto {
    fn new(codigo: String, nome: String, preco: f64, quantidade: u32, categoria: String) -> Self {
        Produto {
            codigo,
            nome,
            preco,
            quantidade,
            categoria,
        }
    }

    fn valor_total(&self) -> f64 {
        self.preco * self.quantidade as f64
    }
}

// Sistema de gerenciamento de estoque
struct SistemaEstoque {
    produtos: HashMap<String, Produto>,
}

impl SistemaEstoque {
    fn new() -> Self {
        SistemaEstoque {
            produtos: HashMap::new(),
        }
    }

    // 1. Adicionar produto
    fn adicionar_produto(&mut self, produto: Produto) -> Result<(), String> {
        if self.produtos.contains_key(&produto.codigo) {
            return Err(format!("Produto com código {} já existe!", produto.codigo));
        }

        self.produtos.insert(produto.codigo.clone(), produto);
        Ok(())
    }

    // 2. Remover produto
    fn remover_produto(&mut self, codigo: &str) -> Result<Produto, String> {
        self.produtos
            .remove(codigo)
            .ok_or_else(|| format!("Produto {} não encontrado", codigo))
    }

    // 3. Atualizar quantidade (usando Entry API)
    fn atualizar_quantidade(&mut self, codigo: &str, quantidade: i32) -> Result<(), String> {
        let produto = self
            .produtos
            .get_mut(codigo)
            .ok_or_else(|| format!("Produto {} não encontrado", codigo))?;

        let nova_quantidade = produto.quantidade as i32 + quantidade;

        if nova_quantidade < 0 {
            return Err(String::from("Quantidade não pode ser negativa"));
        }

        produto.quantidade = nova_quantidade as u32;
        Ok(())
    }

    // 4. Buscar produto
    fn buscar_produto(&self, codigo: &str) -> Option<&Produto> {
        self.produtos.get(codigo)
    }

    // 5. Listar todo o estoque
    fn listar_estoque(&self) {
        if self.produtos.is_empty() {
            println!("Estoque vazio!");
            return;
        }

        println!("\n{'=':<70}");
        println!("ESTOQUE COMPLETO");
        println!("{'=':<70}");
        println!(
            "{:<12} {:<25} {:<12} {:<8} {:<15}",
            "CÓDIGO", "NOME", "PREÇO", "QTD", "CATEGORIA"
        );
        println!("{'-':<70}");

        for produto in self.produtos.values() {
            println!(
                "{:<12} {:<25} R$ {:>8.2} {:>8} {:<15}",
                produto.codigo, produto.nome, produto.preco, produto.quantidade, produto.categoria
            );
        }
        println!("{'=':<70}\n");
    }

    // 6. Produtos em falta (quantidade < 5)
    fn produtos_em_falta(&self) -> Vec<&Produto> {
        self.produtos
            .values()
            .filter(|p| p.quantidade < 5)
            .collect()
    }

    // 7. Valor total do estoque
    fn valor_total(&self) -> f64 {
        self.produtos.values().map(|p| p.valor_total()).sum()
    }

    // Função auxiliar: listar por categoria
    fn listar_por_categoria(&self, categoria: &str) {
        let produtos: Vec<_> = self
            .produtos
            .values()
            .filter(|p| p.categoria == categoria)
            .collect();

        if produtos.is_empty() {
            println!("Nenhum produto na categoria '{}'", categoria);
            return;
        }

        println!("\nProdutos na categoria '{}':", categoria);
        for produto in produtos {
            println!(
                "  - {} ({}): {} unidades",
                produto.nome, produto.codigo, produto.quantidade
            );
        }
    }
}

fn main() {
    // Adicionando produtos
    println!("=== ADICIONANDO PRODUTOS ===\n");

    let produtos = vec![
        Produto::new(
            String::from("MOUSE-001"),
            String::from("Mouse Gamer RGB"),
            150.00,
            25,
            String::from("Periféricos"),
        ),
        Produto::new(
            String::from("TECLADO-001"),
            String::from("Teclado Mecânico"),
            350.00,
            10,
            String::from("Periféricos"),
        ),
        Produto::new(
            String::from("MONITOR-001"),
            String::from("Monitor 27\" 144Hz"),
            1200.00,
            3,
            String::from("Monitores"),
        ),
        Produto::new(
            String::from("HEADSET-001"),
            String::from("Headset 7.1"),
            280.00,
            15,
            String::from("Áudio"),
        ),
        Produto::new(
            String::from("WEBCAM-001"),
            String::from("Webcam Full HD"),
            320.00,
            2,
            String::from("Vídeo"),
        ),
    ];

    for produto in produtos {
        match estoque.adicionar_produto(produto) {
            Ok(_) => println!("✓ Produto adicionado com sucesso"),
            Err(e) => println!("✗ Erro: {}", e),
        }
    }

    // Listar estoque completo
    estoque.listar_estoque();

    // Buscar produto específico
    println!("=== BUSCAR PRODUTO ===\n");
    match estoque.buscar_produto("MOUSE-001") {
        Some(produto) => {
            println!("Produto encontrado:");
            println!("  Nome: {}", produto.nome);
            println!("  Preço: R$ {:.2}", produto.preco);
            println!("  Quantidade: {}", produto.quantidade);
            println!("  Valor total: R$ {:.2}", produto.valor_total());
        }
        None => println!("Produto não encontrado"),
    }

    // Atualizar quantidade
    println!("\n=== ATUALIZAR QUANTIDADE ===\n");
    match estoque.atualizar_quantidade("MOUSE-001", -5) {
        Ok(_) => println!("✓ Quantidade atualizada (vendeu 5 unidades)"),
        Err(e) => println!("✗ Erro: {}", e),
    }

    match estoque.atualizar_quantidade("MONITOR-001", 10) {
        Ok(_) => println!("✓ Quantidade atualizada (recebeu 10 unidades)"),
        Err(e) => println!("✗ Erro: {}", e),
    }

    // Produtos em falta
    println!("\n=== PRODUTOS EM FALTA (< 5 unidades) ===\n");
    let em_falta = estoque.produtos_em_falta();

    if em_falta.is_empty() {
        println!("Nenhum produto em falta!");
    } else {
        println!(
            "⚠️  ATENÇÃO: {} produto(s) com estoque baixo:\n",
            em_falta.len()
        );
        for produto in em_falta {
            println!(
                "  - {} ({}): {} unidades restantes",
                produto.nome, produto.codigo, produto.quantidade
            );
        }
    }

    // Valor total do estoque
    println!("\n=== VALOR TOTAL DO ESTOQUE ===\n");
    println!("Valor total em estoque: R$ {:.2}", estoque.valor_total());
    
    // Listar por categoria
    println!("\n=== PRODUTOS POR CATEGORIA ===");
    estoque.listar_por_categoria("Periféricos");
    estoque.listar_por_categoria("Monitores");

    // Remover produto
    println!("\n=== REMOVER PRODUTO ===\n");
    match estoque.remover_produto("WEBCAM-001") {
        Ok(produto) => println!("✓ Produto '{}' removido com sucesso", produto.nome),
        Err(e) => println!("✗ Erro: {}", e),
    }

    // Listar estoque final
    println!("\n=== ESTOQUE FINAL ===");
    estoque.listar_estoque();
}
