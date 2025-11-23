fn main() {
    // Armazenando temperatura em Celsius
    // Usamos f64 porque temperatura pode ter decimais
    let celsius: f64 = 25.0;
    
    // Convertendo para Fahrenheit
    // Fórmula: F = C × 1.8 + 32
    let fahrenheit = celsius * 1.8 + 32.0;
    
    // Mostrando os resultados
    // {:.1} significa "mostre com 1 casa decimal"
    println!("Temperatura: {:.1}°C", celsius);
    println!("Equivalente: {:.1}°F", fahrenheit);
    
    // EXTRA: Verificando se está quente
    let esta_quente = celsius > 30.0;
    println!("Está quente? {}", esta_quente);
}