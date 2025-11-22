fn main() {
    let nome: &str = "Thiago Moreira Bianeck";
    let idade: u8 = 41;
    let profissao: &str = "Desenvolvedor de software";
    let ano_atual: u16 = 2025;
    // Cálculo do ano de nascimento
    let ano_nascimento: u16 = ano_atual - idade as u16;

    println!("Olá! 👋");
    println!("Meu nome é {nome}");
    println!("Sou {profissao} e apaixonado por tecnologia.");
    println!("Tenho {idade} anos de idade.");
    println!("Nasci no ano de {ano_nascimento}.");


}
