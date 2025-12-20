// src/validador.rs

/// Valida formato de email
pub fn validar_email(email: &str) -> bool {
    if email.is_empty() {
        return false;
    }
    
    let partes: Vec<&str> = email.split('@').collect();
    if partes.len() != 2 {
        return false;
    }
    
    let (local, dominio) = (partes[0], partes[1]);
    
    !local.is_empty() && 
    !dominio.is_empty() && 
    dominio.contains('.')
}

/// Valida CPF (formato simplificado)
pub fn validar_cpf(cpf: &str) -> bool {
    let apenas_numeros: String = cpf.chars()
        .filter(|c| c.is_numeric())
        .collect();
    
    apenas_numeros.len() == 11
}

/// Valida senha forte
pub fn validar_senha_forte(senha: &str) -> bool {
    if senha.len() < 8 {
        return false;
    }
    
    let tem_maiuscula = senha.chars().any(|c| c.is_uppercase());
    let tem_minuscula = senha.chars().any(|c| c.is_lowercase());
    let tem_numero = senha.chars().any(|c| c.is_numeric());
    let tem_especial = senha.chars().any(|c| !c.is_alphanumeric());
    
    tem_maiuscula && tem_minuscula && tem_numero && tem_especial
}

/// Valida telefone brasileiro
pub fn validar_telefone(telefone: &str) -> bool {
    let apenas_numeros: String = telefone.chars()
        .filter(|c| c.is_numeric())
        .collect();
    
    apenas_numeros.len() == 10 || apenas_numeros.len() == 11
}

// ============================================
// 🧪 MÓDULO DE TESTES
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================
    // TESTES: validar_email
    // ========================================

    #[test]
    fn test_validar_email_formato_valido_simples() {
        assert!(validar_email("usuario@exemplo.com"));
    }

    #[test]
    fn test_validar_email_formato_valido_com_subdominio() {
        assert!(validar_email("usuario@mail.exemplo.com"));
    }

    #[test]
    fn test_validar_email_formato_valido_com_numeros() {
        assert!(validar_email("usuario123@exemplo.com"));
    }

    #[test]
    fn test_validar_email_sem_arroba_retorna_false() {
        assert!(!validar_email("usuarioexemplo.com"));
    }

    #[test]
    fn test_validar_email_sem_dominio_retorna_false() {
        assert!(!validar_email("usuario@"));
    }

    #[test]
    fn test_validar_email_sem_local_retorna_false() {
        assert!(!validar_email("@exemplo.com"));
    }

    #[test]
    fn test_validar_email_vazio_retorna_false() {
        assert!(!validar_email(""));
    }

    #[test]
    fn test_validar_email_sem_ponto_no_dominio_retorna_false() {
        assert!(!validar_email("usuario@exemplo"));
    }

    #[test]
    fn test_validar_email_multiplos_arrobas_retorna_false() {
        assert!(!validar_email("usuario@@exemplo.com"));
    }

    // ========================================
    // TESTES: validar_cpf
    // ========================================

    #[test]
    fn test_validar_cpf_formato_com_pontos_e_traco() {
        assert!(validar_cpf("123.456.789-00"));
    }

    #[test]
    fn test_validar_cpf_formato_apenas_numeros() {
        assert!(validar_cpf("12345678900"));
    }

    #[test]
    fn test_validar_cpf_com_espacos_retorna_false() {
        assert!(!validar_cpf("123 456 789 00"));
    }

    #[test]
    fn test_validar_cpf_com_menos_digitos_retorna_false() {
        assert!(!validar_cpf("123.456.789"));
    }

    #[test]
    fn test_validar_cpf_com_mais_digitos_retorna_false() {
        assert!(!validar_cpf("123.456.789-001"));
    }

    #[test]
    fn test_validar_cpf_vazio_retorna_false() {
        assert!(!validar_cpf(""));
    }

    #[test]
    fn test_validar_cpf_com_letras_retorna_false() {
        assert!(!validar_cpf("123.456.789-0A"));
    }

    // ========================================
    // TESTES: validar_senha_forte
    // ========================================

    #[test]
    fn test_validar_senha_forte_com_todos_requisitos() {
        assert!(validar_senha_forte("Senh@123"));
    }

    #[test]
    fn test_validar_senha_forte_complexa() {
        assert!(validar_senha_forte("C0mpl3x@Pass!"));
    }

    #[test]
    fn test_validar_senha_sem_maiuscula_retorna_false() {
        assert!(!validar_senha_forte("senh@123"));
    }

    #[test]
    fn test_validar_senha_sem_minuscula_retorna_false() {
        assert!(!validar_senha_forte("SENH@123"));
    }

    #[test]
    fn test_validar_senha_sem_numero_retorna_false() {
        assert!(!validar_senha_forte("Senh@abc"));
    }

    #[test]
    fn test_validar_senha_sem_especial_retorna_false() {
        assert!(!validar_senha_forte("Senha123"));
    }

    #[test]
    fn test_validar_senha_muito_curta_retorna_false() {
        assert!(!validar_senha_forte("Se@1"));
    }

    #[test]
    fn test_validar_senha_vazia_retorna_false() {
        assert!(!validar_senha_forte(""));
    }

    #[test]
    fn test_validar_senha_exatamente_8_caracteres() {
        assert!(validar_senha_forte("Senh@123"));
    }

    // ========================================
    // TESTES: validar_telefone
    // ========================================

    #[test]
    fn test_validar_telefone_fixo_com_formatacao() {
        assert!(validar_telefone("(11) 3333-4444"));
    }

    #[test]
    fn test_validar_telefone_celular_com_formatacao() {
        assert!(validar_telefone("(11) 98888-7777"));
    }

    #[test]
    fn test_validar_telefone_apenas_numeros_10_digitos() {
        assert!(validar_telefone("1133334444"));
    }

    #[test]
    fn test_validar_telefone_apenas_numeros_11_digitos() {
        assert!(validar_telefone("11988887777"));
    }

    #[test]
    fn test_validar_telefone_com_menos_digitos_retorna_false() {
        assert!(!validar_telefone("11 3333-444"));
    }

    #[test]
    fn test_validar_telefone_com_mais_digitos_retorna_false() {
        assert!(!validar_telefone("11 98888-77777"));
    }

    #[test]
    fn test_validar_telefone_vazio_retorna_false() {
        assert!(!validar_telefone(""));
    }
}