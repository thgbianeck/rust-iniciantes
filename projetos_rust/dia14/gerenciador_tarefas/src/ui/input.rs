use std::io::{self, Write};

/// Lê uma linha de entrada do usuário
pub fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler entrada");

    input.trim().to_string()
}

/// Lê um número do usuário
pub fn read_number(prompt: &str) -> Option<u32> {
    let input = read_line(prompt);
    input.parse::<u32>().ok()
}

/// Lê uma opção do menu
pub fn read_option(prompt: &str, max: u32) -> Option<u32> {
    let option = read_number(prompt)?;
    if option <= max {
        Some(option)
    } else {
        None
    }
}

/// Confirma uma ação (s/n)
pub fn confirm(prompt: &str) -> bool {
    let input = read_line(&format!("{} (s/n): ", prompt));
    matches!(input.to_lowercase().as_str(), "s" | "sim" | "y" | "yes")
}

/// Pausa até o usuário pressionar Enter
pub fn pause() {
    read_line("\nPressione Enter para continuar...");
}