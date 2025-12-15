fn main() {
    let celsius = 25.0;
    let fahrenheit = celsius * 1.8 + 32.0;

    println!("Temperatura em Celsius: {:.1}°C", celsius);
    println!("Temperatura em Fahrenheit: {:.1}°F", fahrenheit);

    let esta_quente = celsius > 30.0;
    println!("Está quente? {}", esta_quente);
}
