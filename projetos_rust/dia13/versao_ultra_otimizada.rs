use std::collections::HashMap;

#[derive(Debug)]
struct Estatisticas {
    total: f64,
    count: usize,
}

fn processar_ultra_otimizado(transacoes: Vec<Transacao>) {
    const TAXA_CAMBIO: f64 = 5.20;
    const TAXA_PROCESSAMENTO: f64 = 0.02;
    const LIMITE_USD: f64 = 100.0 / TAXA_CAMBIO;

    // Uma única passagem, acumulando estatísticas diretamente
    let estatisticas: HashMap<Categoria, Estatisticas> = transacoes
        .into_iter()
        .filter(|t| t.valor_usd > LIMITE_USD)
        .map(|t| {
            let valor_brl = t.valor_usd * TAXA_CAMBIO;
            let valor_final = valor_brl * (1.0 + TAXA_PROCESSAMENTO);
            (t.categoria, valor_final)
        })
        .fold(HashMap::new(), |mut acc, (cat, valor)| {
            acc.entry(cat)
                .and_modify(|e| {
                    e.total += valor;
                    e.count += 1;
                })
                .or_insert(Estatisticas {
                    total: valor,
                    count: 1,
                });
            acc
        });

    // Exibir relatório
    println!("\n📊 RELATÓRIO FINANCEIRO (Versão Ultra-Otimizada)\n");
    println!("{:-<60}", "");

    let total_geral: f64 = estatisticas
        .iter()
        .map(|(categoria, stats)| {
            let media = stats.total / stats.count as f64;

            println!("📁 {}", categoria);
            println!("   Total: R$ {:.2}", stats.total);
            println!("   Média: R$ {:.2}", media);
            println!("   Transações: {}", stats.count);
            println!();

            stats.total
        })
        .sum();

    println!("{:-<60}", "");
    println!("💰 TOTAL GERAL: R$ {:.2}", total_geral);
}
