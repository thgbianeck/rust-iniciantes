use serde::{Deserialize, Serialize}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category { 
    Work, 
    Personal, 
    Study, 
    Health, 
    Other,
}

impl Category {
    pub fn all() -> Vec<Category> {
        vec![
            Category::Work,
            Category::Personal,
            Category::Study,
            Category::Health,
            Category::Other,
        ]
    }

    pub fn as_str(&self) -> &str {
        match self {
            Category::Work => "Trabalho",
            Category::Personal => "Pessoal",
            Category::Study => "Estudos",
            Category::Health => "Saúde",
            Category::Other => "Outro",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Priority {
    pub fn all() -> Vec<Priority> {
        vec![
            Priority::High, 
            Priority::Medium, 
            Priority::Low
        ]
    }

    pub fn as_str(&self) -> &str {
        match self {
            Priority::High => "Alta",
            Priority::Medium => "Média",
            Priority::Low => "Baixa",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::Pending => "Pendente",
            Status::InProgress => "Em Andamento",
            Status::Completed => "Concluída",
        }
    }
}