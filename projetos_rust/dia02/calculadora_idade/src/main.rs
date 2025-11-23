fn main() {
    let ano_atual: u32 = 2025;
    let ano_nascimento: u32 = 1984;
    let idade: u32 = ano_atual - ano_nascimento;
    let maior_idade: bool = idade >= 18;
    
    println!("Idade: {}", idade);
    println!("Maior de idade: {}", maior_idade);
}
