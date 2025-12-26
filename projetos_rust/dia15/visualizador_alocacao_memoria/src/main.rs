// Visualizador de Alocações de Memória
// Este programa demonstra Stack vs Heap, Copy vs Move

fn main() {
    println!("=== VISUALIZADOR DE MEMÓRIA ===\n");
    
    // ========================================
    // PARTE 1: TIPOS NO STACK (Copy)
    // ========================================
    println!("📦 PARTE 1: Tipos no Stack (Copy)");
    println!("----------------------------------");
    
    // Criando variáveis no Stack
    let x = 42;
    println!("✅ Criado: x = {} (Stack, 4 bytes)", x);
    
    let y = 3.144;
    println!("✅ Criado: y = {} (Stack, 8 bytes)", y);
    
    let ativo = true;
    println!("✅ Criado: ativo = {} (Stack, 1 byte)", ativo);
    
    // Copy em ação
    println!("\n🔄 Testando Copy:");
    let x2 = x;  // x é COPIADO para x2
    println!("   let x2 = x;");
    println!("   x = {} (ainda válido! ✅)", x);
    println!("   x2 = {} (cópia independente)", x2);
    println!("   Motivo: i32 implementa Copy (barato copiar 4 bytes)");
    
    // ========================================
    // PARTE 2: TIPOS NO HEAP (Move)
    // ========================================
    println!("\n📦 PARTE 2: Tipos no Heap (Move)");
    println!("----------------------------------");
    
    // Criando String (usa Heap)
    let s1 = String::from("Rust");
    println!("✅ Criado: s1 = \"{}\"", s1);
    println!("   Stack: ptr, len=4, cap=4 (12 bytes)");
    println!("   Heap: \"Rust\" (4 bytes)");
    
    // Move em ação
    println!("\n🔄 Testando Move:");
    let s2 = s1;  // s1 é MOVIDO para s2
    println!("   let s2 = s1;");
    // println!("   s1 = {}", s1); // ❌ Descomente para ver o erro!
    println!("   s1 = [INVÁLIDO] (movido! ❌)");
    println!("   s2 = \"{}\" (novo dono ✅)", s2);
    println!("   Motivo: String não implementa Copy (caro copiar Heap)");
    
    // ========================================
    // PARTE 3: CLONE EXPLÍCITO
    // ========================================
    println!("\n📦 PARTE 3: Clone Explícito");
    println!("----------------------------------");
    
    let s3 = String::from("Clone");
    println!("✅ Criado: s3 = \"{}\"", s3);
    
    let s4 = s3.clone();  // Clone explícito
    println!("🔄 let s4 = s3.clone();");
    println!("   s3 = \"{}\" (ainda válido! ✅)", s3);
    println!("   s4 = \"{}\" (cópia profunda no Heap)", s4);
    println!("   Motivo: .clone() copia dados do Heap explicitamente");
    
    // ========================================
    // PARTE 4: VEC (DINÂMICO)
    // ========================================
    println!("\n📦 PARTE 4: Vec (Crescimento Dinâmico)");
    println!("----------------------------------");
    
    let mut v = Vec::new();
    println!("✅ Criado: v = Vec vazio");
    println!("   len=0, cap=0");
    
    v.push(10);
    println!("🔄 v.push(10);");
    println!("   v = {:?}", v);
    println!("   len=1, cap=4 (alocou espaço extra!)");
    
    v.push(20);
    println!("🔄 v.push(20);");
    println!("   v = {:?}", v);
    println!("   len=2, cap=4 (ainda cabe)");
    
    v.push(30);
    println!("🔄 v.push(30);");
    println!("   v = {:?}", v);
    println!("   len=3, cap=4");
    
    // ========================================
    // PARTE 5: ESCOPO E DROP
    // ========================================
    println!("\n📦 PARTE 5: Escopo e Drop Automático");
    println!("----------------------------------");
    
    {
        let s_temp = String::from("Temporária");
        println!("✅ Criado: s_temp = \"{}\" (dentro do bloco)", s_temp);
        println!("   Esta String existe apenas neste bloco");
    } // ← Drop é chamado aqui!
    
    println!("❌ s_temp foi destruída (Drop automático)");
    println!("   Memória do Heap foi liberada");
    // println!("{}", s_temp); // ❌ Descomente para ver o erro!
    
    // ========================================
    // PARTE 6: PREVISÃO
    // ========================================
    println!("\n📦 PARTE 6: Exercício de Previsão");
    println!("----------------------------------");
    println!("O que acontece com cada variável ao final de main()?");
    println!("   x, y, ativo, x2 → Removidos do Stack");
    println!("   s2, s3, s4, v → Drop chamado, Heap liberado");
    println!("   Tudo automático! Sem leaks! ✅");
    
    println!("\n=== FIM DO VISUALIZADOR ===");
} // ← Aqui TODAS as variáveis são limpas automaticamente!