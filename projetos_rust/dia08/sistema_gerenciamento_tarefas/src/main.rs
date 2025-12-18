use std::io::{self, Write};

// 1. ESTRUTURAS DE DADOS

#[derive(Debug, Clone)]
struct Tarefa {
    descricao: String,
    completa: bool,
}

impl Tarefa {
    // Construtor para criar nova tarefa
    fn nova(descricao: String) -> Self {
        Tarefa {
            descricao,
            completa: false,
        }
    }

    // Marcar como completa
    fn marcar_completa(&mut self) {
        self.completa = true;
    }

    // Retornar status como string
    fn status(&self) -> &str {
        if self.completa {
            "✓ Completa"
        } else {
            "○ Pendente"
        }
    }
}

// 2. GERENCIADOR DE TAREFAS

struct GerenciadorTarefas {
    tarefas: Vec<Tarefa>,
}

impl GerenciadorTarefas {
    // Criar gerenciador vazio
    fn novo() -> Self {
        GerenciadorTarefas {
            tarefas: Vec::new(),
        }
    }

    // Adicionar tarefa
    fn adicionar(&mut self, descricao: String) {
        let tarefa = Tarefa::nova(descricao);
        self.tarefas.push(tarefa);
        println!("✅ Tarefa adicionada com sucesso!");
    }

    // Listar todas as tarefas
    fn listar_todas(&self) {
        if self.tarefas.is_empty() {
            println!("📭 Nenhuma tarefa cadastrada.");
            return;
        }

        println!("\n📋 === TODAS AS TAREFAS ===");
        for (indice, tarefa) in self.tarefas.iter().enumerate() {
            println!("[{}] {} - {}", indice, tarefa.status(), tarefa.descricao);
        }
        println!();
    }

    // Listar apenas pendentes
    fn listar_pendentes(&self) {
        println!("\n⏳ === TAREFAS PENDENTES ===");
        let mut encontrou = false;

        for (indice, tarefa) in self.tarefas.iter().enumerate() {
            if !tarefa.completa {
                println!("[{}] {}", indice, tarefa.descricao);
                encontrou = true;
            }
        }

        if !encontrou {
            println!("🎉 Nenhuma tarefa pendente!");
        }
        println!();
    }

    // Listar apenas completas
    fn listar_completas(&self) {
        println!("\n✅ === TAREFAS COMPLETAS ===");
        let mut encontrou = false;

        for (indice, tarefa) in self.tarefas.iter().enumerate() {
            if tarefa.completa {
                println!("[{}] {}", indice, tarefa.descricao);
                encontrou = true;
            }
        }

        if !encontrou {
            println!("❌ Nenhuma tarefa completa ainda.");
        }
        println!();
    }

    // Marcar tarefa como completa (acesso seguro)
    fn marcar_completa(&mut self, indice: usize) {
        match self.tarefas.get_mut(indice) {
            Some(tarefa) => {
                if tarefa.completa {
                    println!("⚠️ Tarefa já estava completa!");
                } else {
                    tarefa.marcar_completa();
                    println!("✅ Tarefa marcada como completa!");
                }
            }
            None => {
                println!("❌ Índice inválido! Tarefa não encontrada.");
            }
        }
    }

    // Remover tarefa
    fn remover(&mut self, indice: usize) {
        if indice < self.tarefas.len() {
            let tarefa = self.tarefas.remove(indice);
            println!("🗑️ Tarefa removida: {}", tarefa.descricao);
        } else {
            println!("❌ Índice inválido! Tarefa não encontrada.");
        }
    }

    // Exibir estatísticas
    fn estatisticas(&self) {
        let total = self.tarefas.len();

        let completas = self.tarefas.iter().filter(|t| t.completa).count();

        let pendentes = total - completas;

        let percentual = if total > 0 {
            (completas as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        println!("\n📊 === ESTATÍSTICAS ===");
        println!("Total de tarefas: {}", total);
        println!("✅ Completas: {}", completas);
        println!("⏳ Pendentes: {}", pendentes);
        println!("📈 Progresso: {:.1}%", percentual);
        println!();
    }
    // Buscar tarefa por palavra-chave
    fn buscar(&self, termo: &str) {
        println!("\n🔍 === RESULTADOS DA BUSCA ===");
        let mut encontrou = false;

        for (indice, tarefa) in self.tarefas.iter().enumerate() {
            if tarefa
                .descricao
                .to_lowercase()
                .contains(&termo.to_lowercase())
            {
                println!("[{}] {} - {}", indice, tarefa.status(), tarefa.descricao);
                encontrou = true;
            }
        }

        if !encontrou {
            println!("❌ Nenhuma tarefa encontrada com o termo '{}'.", termo);
        }
        println!();
    }
}

// 3. INTERFACE DO USUÁRIO

fn exibir_menu() {
    println!("\n╔═══════════════════════════════════════╗");
    println!("║   📝 GERENCIADOR DE TAREFAS           ║");
    println!("╠═══════════════════════════════════════╣");
    println!("║  1. Adicionar tarefa                  ║");
    println!("║  2. Listar todas                      ║");
    println!("║  3. Listar pendentes                  ║");
    println!("║  4. Listar completas                  ║");
    println!("║  5. Marcar como completa              ║");
    println!("║  6. Remover tarefa                    ║");
    println!("║  7. Estatísticas                      ║");
    println!("║  8. Buscar tarefa                     ║");
    println!("║  0. Sair                              ║");
    println!("╚═══════════════════════════════════════╝");
    print!("\nEscolha uma opção: ");
    io::stdout().flush().unwrap();
}

fn ler_linha() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler entrada");
    input.trim().to_string()
}

fn ler_numero() -> usize {
    loop {
        let input = ler_linha();
        match input.parse::<usize>() {
            Ok(num) => return num,
            Err(_) => {
                print!("❌ Entrada inválida! Digite um número: ");
                io::stdout().flush().unwrap();
            }
        }
    }
}

// 4. FUNÇÃO PRINCIPAL

fn main() {
    let mut gerenciador = GerenciadorTarefas::novo();

    // Adicionar tarefas de exemplo
    gerenciador.adicionar("Estudar Rust - Vectors".to_string());
    gerenciador.adicionar("Fazer exercícios práticos".to_string());
    gerenciador.adicionar("Ler documentação oficial".to_string());

    println!("🎉 Bem-vindo ao Gerenciador de Tarefas!");
    println!("💡 3 tarefas de exemplo foram adicionadas.");

    loop {
        exibir_menu();
        let opcao = ler_numero();

        match opcao {
            1 => {
                print!("📝 Digite a descrição da tarefa: ");
                io::stdout().flush().unwrap();
                let descricao = ler_linha();
                if !descricao.is_empty() {
                    gerenciador.adicionar(descricao);
                } else {
                    println!("❌ Descrição não pode ser vazia!");
                }
            }

            2 => {
                gerenciador.listar_todas();
            }

            3 => {
                gerenciador.listar_pendentes();
            }

            4 => {
                gerenciador.listar_completas();
            }

            5 => {
                gerenciador.listar_todas();
                print!("Digite o índice da tarefa para marcar como completa: ");
                io::stdout().flush().unwrap();
                let indice = ler_numero();
                gerenciador.marcar_completa(indice);
            }

            6 => {
                gerenciador.listar_todas();
                print!("Digite o índice da tarefa para remover: ");
                io::stdout().flush().unwrap();
                let indice = ler_numero();
                gerenciador.remover(indice);
            }

            7 => {
                gerenciador.estatisticas();
            }

            8 => {
                print!("🔍 Digite o termo de busca: ");
                io::stdout().flush().unwrap();
                let termo = ler_linha();
                gerenciador.buscar(&termo);
            }

            0 => {
                println!("\n👋 Até logo! Suas tarefas foram salvas na memória.");
                println!("🎯 Continue praticando Rust!");
                break;
            }

            _ => {
                println!("❌ Opção inválida! Tente novamente.");
            }
        }
    }
}
