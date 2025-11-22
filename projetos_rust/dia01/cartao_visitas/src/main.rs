// Códigos ANSI para cores
const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
//const BLUE: &str = "\x1b[34m";
//const YELLOW: &str = "\x1b[33m";

fn main() {
    // TODO: Imprimir a borda superior
    println!("{GREEN}╔═════════════════════════════════════════╗{RESET}");
    println!("{GREEN}║           ___                           ║{RESET}");
    println!("{GREEN}║          (o o)                          ║{RESET}");
    println!("{GREEN}║       ooO--(_)--Ooo                     ║{RESET}");
    // TODO: Imprimir linha vazia
    println!("{GREEN}║                                         ║{RESET}");
    // TODO: Imprimir título "CARTÃO DE VISITAS"
    println!("{GREEN}║{RESET}         🦀 CARTÃO DE VISITAS 🦀         {GREEN}║{RESET}");
    // TODO: Imprimir linha vazia
    println!("{GREEN}║                                         ║{RESET}");
    // TODO: Imprimir seu nome
    println!("{GREEN}║{RESET} 👤 Nome: Thiago Moreira Bianeck         {GREEN}║{RESET}");
    // TODO: Imprimir sua profissão
    println!("{GREEN}║{RESET} 🛠️  Profissão: Engenheiro de Software    {GREEN}║{RESET}");
    // TODO: Imprimir linha vazia
    println!("{GREEN}║                                         ║{RESET}");
    // TODO: Imprimir email
    println!("{GREEN}║{RESET} 📧 Email: thiagobianeck@gmail.com       {GREEN}║{RESET}");
    // TODO: Imprimir GitHub
    println!("{GREEN}║{RESET} 🐙 GitHub: github.com/thgbianeck        {GREEN}║{RESET}");
    // TODO: Imprimir linha vazia
    println!("{GREEN}║                                         ║{RESET}");
    // TODO: Imprimir mensagem
    println!("{GREEN}║{RESET} 💬 Mensagem: Obrigado por visitar       {GREEN}║{RESET}");
    println!("{GREEN}║{RESET} meu cartão!                             {GREEN}║{RESET}");
    // TODO: Imprimir linha vazia
    println!("{GREEN}║                                         ║{RESET}");
    // TODO: Imprimir borda inferior
    println!("{GREEN}╚═════════════════════════════════════════╝{RESET}");
}
