use std::collections::HashMap;

fn processar_funcional(transacoes: Vec<Transacao>) {
    const TAXA_CAMBIO: f64 = 5.20;
    const TAXA_PROCESSAMENTO: f64 = 0.02;
    const LIMITE_USD: f64 = 100.0 / TAXA_CAMBIO;

    // Pipeline completo em uma única expressão
    let por_categoria: HashMap<Categoria, Vec<f64>> = transacoes
        .into_iter()
        .filter(|t| t.valor_usd > LIMITE_USD)
        .map(|t| {
            let valor_brl = t.valor_usd * TAXA_CAMBIO;
            let valor_final = valor_brl * (1.0 + TAXA_PROCESSAMENTO);
            (t.categoria, valor_final)
        })
        .fold(HashMap::new(), |mut acc, (cat, valor)| {
            acc.entry(cat).or_insert(Vec::new()).push(valor);
            acc
        });

    // Exibir relatório
    println!("\n📊 RELATÓRIO FINANCEIRO (Versão Funcional)\n");
    println!("{:-<60}", "");

    let total_geral: f64 = por_categoria
        .iter()
        .map(|(categoria, valores)| {
            let soma: f64 = valores.iter().sum();
            let media = soma / valores.len() as f64;
            let count = valores.len();

            println!("📁 {}", categoria);
            println!("   Total: R$ {:.2}", soma);
            println!("   Média: R$ {:.2}", media);
            println!("   Transações: {}", count);
            println!();

            soma
        })
        .sum();

    println!("{:-<60}", "");
    println!("💰 TOTAL GERAL: R$ {:.2}", total_geral);
}
