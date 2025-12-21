use crate::models::{Category, Priority};
use crate::services::TaskService;
use crate::ui::{input, output};
use chrono::NaiveDate;

pub struct Menu {
    service: TaskService,
}

impl Menu {
    pub fn new(service: TaskService) -> Self {
        Menu { service }
    }

    pub fn run(&mut self) {
        loop {
            output::clear_screen();
            self.display_main_menu();

            let option = input::read_option("Escolha uma opção: ", 7);

            match option {
                Some(1) => self.create_task(),
                Some(2) => self.list_tasks(),
                Some(3) => self.view_task(),
                Some(4) => self.update_task(),
                Some(5) => self.delete_task(),
                Some(6) => self.filter_tasks(),
                Some(7) => self.show_statistics(),
                Some(0) => {
                    output::print_success("Até logo! 👋");
                    break;
                }
                _ => {
                    output::print_error("Opção inválida!");
                    input::pause();
                }
            }
        }
    }

    fn display_main_menu(&self) {
        output::print_header("GERENCIADOR DE TAREFAS");
        println!("1. ➕ Criar nova tarefa");
        println!("2. 📋 Listar todas as tarefas");
        println!("3. 🔍 Ver detalhes de uma tarefa");
        println!("4. ✏️  Atualizar tarefa");
        println!("5. 🗑️  Deletar tarefa");
        println!("6. 🔎 Filtrar tarefas");
        println!("7. 📊 Estatísticas");
        println!("0. 🚪 Sair");
        println!();
    }

    fn create_task(&mut self) {
        output::clear_screen();
        output::print_header("CRIAR NOVA TAREFA");

        let title = input::read_line("Título: ");
        if title.is_empty() {
            output::print_error("Título não pode ser vazio!");
            input::pause();
            return;
        }

        let description = input::read_line("Descrição: ");

        // Categoria
        println!("\nCategorias:");
        for (i, cat) in Category::all().iter().enumerate() {
            println!("{}. {}", i + 1, cat.as_str());
        }
        let cat_option = input::read_option("Escolha a categoria: ", Category::all().len() as u32);
        let category = match cat_option {
            Some(n) => Category::all()[(n - 1) as usize],
            None => {
                output::print_error("Categoria inválida!");
                input::pause();
                return;
            }
        };

        // Prioridade
        println!("\nPrioridades:");
        for (i, pri) in Priority::all().iter().enumerate() {
            println!("{}. {}", i + 1, pri.as_str());
        }
        let pri_option = input::read_option("Escolha a prioridade: ", Priority::all().len() as u32);
        let priority = match pri_option {
            Some(n) => Priority::all()[(n - 1) as usize],
            None => {
                output::print_error("Prioridade inválida!");
                input::pause();
                return;
            }
        };

        // Data de vencimento
        let due_date_str = input::read_line("Data de vencimento (DD/MM/AAAA) ou Enter para pular: ");
        let due_date = if due_date_str.is_empty() {
            None
        } else {
            match NaiveDate::parse_from_str(&due_date_str, "%d/%m/%Y") {
                Ok(date) => Some(date),
                Err(_) => {
                    output::print_error("Data inválida! Use o formato DD/MM/AAAA");
                    input::pause();
                    return;
                }
            }
        };

        // Criar tarefa
        match self.service.add_task(title, description, category, priority, due_date) {
            Ok(task) => {
                output::print_success(&format!("Tarefa criada com ID: {}", task.id));
            }
            Err(e) => {
                output::print_error(&format!("Erro ao criar tarefa: {}", e));
            }
        }

        input::pause();
    }

    fn list_tasks(&self) {
        output::clear_screen();
        output::print_header("TODAS AS TAREFAS");

        let tasks: Vec<&crate::models::Task> = self.service.list_all().iter().collect();
        output::print_task_list(&tasks);

        input::pause();
    }

    fn view_task(&self) {
        output::clear_screen();
        output::print_header("VER DETALHES DA TAREFA");

        let id = match input::read_number("ID da tarefa: ") {
            Some(id) => id,
            None => {
                output::print_error("ID inválido!");
                input::pause();
                return;
            }
        };

        match self.service.get_by_id(id) {
            Some(task) => {
                println!();
                output::print_task(task);

                // Submenu de ações
                println!("\nAções:");
                println!("1. ▶️  Iniciar tarefa");
                println!("2. ✅ Marcar como concluída");
                println!("0. Voltar");

                let option = input::read_option("\nEscolha uma ação: ", 2);

                match option {
                    Some(1) => {
                        if let Err(e) = self.service.start_task(id) {
                            output::print_error(&format!("Erro: {}", e));
                        } else {
                            output::print_success("Tarefa iniciada!");
                        }
                    }
                    Some(2) => {
                        if let Err(e) = self.service.complete_task(id) {
                            output::print_error(&format!("Erro: {}", e));
                        } else {
                            output::print_success("Tarefa concluída!");
                        }
                    }
                    _ => {}
                }
            }
            None => {
                output::print_error("Tarefa não encontrada!");
            }
        }

        input::pause();
    }

    fn update_task(&mut self) {
        output::clear_screen();
        output::print_header("ATUALIZAR TAREFA");

        let id = match input::read_number("ID da tarefa: ") {
            Some(id) => id,
            None => {
                output::print_error("ID inválido!");
                input::pause();
                return;
            }
        };

        // Verifica se existe
        if self.service.get_by_id(id).is_none() {
            output::print_error("Tarefa não encontrada!");
            input::pause();
            return;
        }

        println!("\nDeixe em branco para manter o valor atual.\n");

        let title = input::read_line("Novo título: ");
        let title = if title.is_empty() { None } else { Some(title) };

        let description = input::read_line("Nova descrição: ");
        let description = if description.is_empty() {
            None
        } else {
            Some(description)
        };

        // Aqui você pode adicionar lógica para atualizar categoria, prioridade, etc.

        match self.service.update_task(id, title, description, None, None, None) {
            Ok(_) => {
                output::print_success("Tarefa atualizada com sucesso!");
            }
            Err(e) => {
                output::print_error(&format!("Erro ao atualizar: {}", e));
            }
        }

        input::pause();
    }

    fn delete_task(&mut self) {
        output::clear_screen();
        output::print_header("DELETAR TAREFA");

        let id = match input::read_number("ID da tarefa: ") {
            Some(id) => id,
            None => {
                output::print_error("ID inválido!");
                input::pause();
                return;
            }
        };

        // Mostra a tarefa
        match self.service.get_by_id(id) {
            Some(task) => {
                println!();
                output::print_task(task);

                if input::confirm("\nTem certeza que deseja deletar esta tarefa?") {
                    match self.service.delete_task(id) {
                        Ok(_) => {
                            output::print_success("Tarefa deletada com sucesso!");
                        }
                        Err(e) => {
                            output::print_error(&format!("Erro ao deletar: {}", e));
                        }
                    }
                } else {
                    output::print_warning("Operação cancelada.");
                }
            }
            None => {
                output::print_error("Tarefa não encontrada!");
            }
        }

        input::pause();
    }

    fn filter_tasks(&self) {
        output::clear_screen();
        output::print_header("FILTRAR TAREFAS");

        println!("1. Por Status");
        println!("2. Por Categoria");
        println!("3. Por Prioridade");
        println!("4. Tarefas Atrasadas");
        println!("0. Voltar");

        let option = input::read_option("\nEscolha o filtro: ", 4);

        match option {
            Some(1) => self.filter_by_status(),
            Some(2) => self.filter_by_category(),
            Some(3) => self.filter_by_priority(),
            Some(4) => self.show_overdue(),
            _ => {}
        }
    }

    fn filter_by_status(&self) {
        use crate::models::Status;

        println!("\n1. Pendentes");
        println!("2. Em Andamento");
        println!("3. Concluídas");

        let option = input::read_option("Escolha o status: ", 3);

        let status = match option {
            Some(1) => Status::Pending,
            Some(2) => Status::InProgress,
            Some(3) => Status::Completed,
            _ => {
                output::print_error("Opção inválida!");
                input::pause();
                return;
            }
        };

        let tasks = self.service.filter_by_status(status);
        output::print_task_list(&tasks);
        input::pause();
    }

    fn filter_by_category(&self) {
        println!();
        for (i, cat) in Category::all().iter().enumerate() {
            println!("{}. {}", i + 1, cat.as_str());
        }

        let option = input::read_option("Escolha a categoria: ", Category::all().len() as u32);

        let category = match option {
            Some(n) => Category::all()[(n - 1) as usize],
            None => {
                output::print_error("Opção inválida!");
                input::pause();
                return;
            }
        };

        let tasks = self.service.filter_by_category(category);
        output::print_task_list(&tasks);
        input::pause();
    }

    fn filter_by_priority(&self) {
        println!();
        for (i, pri) in Priority::all().iter().enumerate() {
            println!("{}. {}", i + 1, pri.as_str());
        }

        let option = input::read_option("Escolha a prioridade: ", Priority::all().len() as u32);

        let priority = match option {
            Some(n) => Priority::all()[(n - 1) as usize],
            None => {
                output::print_error("Opção inválida!");
                input::pause();
                return;
            }
        };

        let tasks = self.service.filter_by_priority(priority);
        output::print_task_list(&tasks);
        input::pause();
    }

    fn show_overdue(&self) {
        let tasks = self.service.get_overdue();
        output::print_task_list(&tasks);
        input::pause();
    }

    fn show_statistics(&self) {
        output::clear_screen();
        let stats = self.service.get_statistics();
        output::print_statistics(&stats);
        input::pause();
    }
}