fn main() {
    println!("=== CALCULADORA DE MÉDIA ===\n");
    
    // 1. Variáveis imutáveis para notas
    let nota1: f64 = 8.5;
    let nota2: f64 = 9.0;
    let nota3: f64 = 7.5;
    
    println!("Nota 1: {:.1}", nota1);
    println!("Nota 2: {:.1}", nota2);
    println!("Nota 3: {:.1}", nota3);
    
    // 2. Cálculo da média
    let soma = nota1 + nota2 + nota3;
    let media = soma / 3.0;
    
    println!("\nMédia: {:.2}", media);
    
    // 3. Usando booleano para verificação
    let aprovado = media >= 7.0;
    println!("Aprovado? {}", aprovado);
    
    // 4. Shadowing para arredondar
    let media = media as i32; // Agora é inteiro!
    println!("Média arredondada: {}", media);
    
    // 5. Tupla com informações do aluno
    let aluno = ("João", media, aprovado);
    println!("\n=== RESUMO ===");
    println!("Aluno: {}", aluno.0);
    println!("Média: {}", aluno.1);
    println!("Status: {}", if aluno.2 {"Aprovado"} else {"Reprovado"});
    
    // 6. Array com todas as notas
    let notas = [nota1, nota2, nota3];
    println!("\nNotas registradas: {:?}", notas);
}