//! Funções de validação

/// Valida CPF (validação simplificada)
pub fn validar_cpf(cpf: &str) -> bool {
    // Remove caracteres não numéricos
    let cpf_numeros: String = cpf.chars().filter(|c| c.is_numeric()).collect();
    
    // Verifica se tem 11 dígitos
    if cpf_numeros.len() != 11 {
        return false;
    }
    
    // Verifica se não são todos iguais (ex: 111.111.111-11)
    if cpf_numeros.chars().all(|c| c == cpf_numeros.chars().next().unwrap()) {
        return false;
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cpf_valido() {
        assert!(validar_cpf("12345678901"));
        assert!(validar_cpf("123.456.789-01"));
    }
    
    #[test]
    fn test_cpf_invalido() {
        assert!(!validar_cpf("123"));
        assert!(!validar_cpf("11111111111"));
    }
}