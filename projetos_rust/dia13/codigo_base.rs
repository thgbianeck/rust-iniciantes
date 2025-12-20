#[derive(Debug, Clone)]
struct Transacao {
    id: u32,
    descricao: String,
    valor_usd: f64,
    categoria: Categoria,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Categoria {
    Alimentacao,
    Transporte,
    Entretenimento,
    Saude,
    Educacao,
}

impl std::fmt::Display for Categoria {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Categoria::Alimentacao => write!(f, "Alimentação"),
            Categoria::Transporte => write!(f, "Transporte"),
            Categoria::Entretenimento => write!(f, "Entretenimento"),
            Categoria::Saude => write!(f, "Saúde"),
            Categoria::Educacao => write!(f, "Educação"),
        }
    }
}

fn gerar_transacoes() -> Vec<Transacao> {
    vec![
        Transacao {
            id: 1,
            descricao: "Restaurante".into(),
            valor_usd: 45.0,
            categoria: Categoria::Alimentacao,
        },
        Transacao {
            id: 2,
            descricao: "Uber".into(),
            valor_usd: 15.0,
            categoria: Categoria::Transporte,
        },
        Transacao {
            id: 3,
            descricao: "Cinema".into(),
            valor_usd: 30.0,
            categoria: Categoria::Entretenimento,
        },
        Transacao {
            id: 4,
            descricao: "Supermercado".into(),
            valor_usd: 120.0,
            categoria: Categoria::Alimentacao,
        },
        Transacao {
            id: 5,
            descricao: "Consulta médica".into(),
            valor_usd: 80.0,
            categoria: Categoria::Saude,
        },
        Transacao {
            id: 6,
            descricao: "Curso online".into(),
            valor_usd: 200.0,
            categoria: Categoria::Educacao,
        },
        Transacao {
            id: 7,
            descricao: "Táxi".into(),
            valor_usd: 25.0,
            categoria: Categoria::Transporte,
        },
        Transacao {
            id: 8,
            descricao: "Show".into(),
            valor_usd: 150.0,
            categoria: Categoria::Entretenimento,
        },
        Transacao {
            id: 9,
            descricao: "Farmácia".into(),
            valor_usd: 60.0,
            categoria: Categoria::Saude,
        },
        Transacao {
            id: 10,
            descricao: "Livros".into(),
            valor_usd: 90.0,
            categoria: Categoria::Educacao,
        },
    ]
}
