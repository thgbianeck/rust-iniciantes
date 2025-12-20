fn main() {
    let transacoes = gerar_transacoes();
    
    println!("\n{'='*60}");
    println!("🔴 ABORDAGEM IMPERATIVA");
    println!("{'='*60}");
    processar_imperativo(transacoes.clone());
    
    println!("\n\n{'='*60}");
    println!("🟢 ABORDAGEM FUNCIONAL");
    println!("{'='*60}");
    processar_funcional(transacoes.clone());
    
    println!("\n\n{'='*60}");
    println!("🚀 ABORDAGEM ULTRA-OTIMIZADA");
    println!("{'='*60}");
    processar_ultra_otimizado(transacoes);
}