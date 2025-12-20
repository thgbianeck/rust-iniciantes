use std::collections::HashMap;

fn processar_imperativo(transacoes: Vec<Transacao>) {
    const TAXA_CAMBIO: f64 = 5.20;
    const TAXA_PROCESSAMENTO: f64 = 0.02;

    // Etapa 1: Filtrar transações > $100 USD
    let mut filtradas = Vec::new();
    for transacao in &transacoes {
        if transacao.valor_usd > 100.0 / TAXA_CAMBIO {
            filtradas.push(transacao.clone());
        }
    }

    // Etapa 2: Converter para BRL e aplicar taxa
    let mut processadas = Vec::new();
    for transacao in filtradas {
        let valor_brl = transacao.valor_usd * TAXA_CAMBIO;
        let valor_final = valor_brl * (1.0 + TAXA_PROCESSAMENTO);
        processadas.push((transacao.categoria.clone(), valor_final));
    }

    // Etapa 3: Agrupar por categoria
    let mut por_categoria: HashMap<Categoria, Vec<f64>> = HashMap::new();
    for (categoria, valor) in processadas {
        por_categoria
            .entry(categoria)
            .or_insert(Vec::new())
            .push(valor);
    }

    // Etapa 4: Calcular estatísticas
    println!("\n📊 RELATÓRIO FINANCEIRO (Versão Imperativa)\n");
    println!("{:-<60}", "");

    let mut total_geral = 0.0;
    for (categoria, valores) in &por_categoria {
        let soma: f64 = valores.iter().sum();
        let media = soma / valores.len() as f64;
        let count = valores.len();

        total_geral += soma;

        println!("📁 {}", categoria);
        println!("   Total: R$ {:.2}", soma);
        println!("   Média: R$ {:.2}", media);
        println!("   Transações: {}", count);
        println!();
    }

    println!("{:-<60}", "");
    println!("💰 TOTAL GERAL: R$ {:.2}", total_geral);
}
