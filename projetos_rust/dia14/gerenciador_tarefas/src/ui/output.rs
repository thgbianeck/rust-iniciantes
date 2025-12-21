use crate::models::Task;
use crate::services::task_service::Statistics;

/// Limpa a tela
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

/// Exibe o cabeçalho
pub fn print_header(title: &str) {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║  {:^54}  ║", title);
    println!("╚══════════════════════════════════════════════════════════╝\n");
}

/// Exibe uma mensagem de sucesso
pub fn print_success(message: &str) {
    println!("\n✅ {}", message);
}

/// Exibe uma mensagem de erro
pub fn print_error(message: &str) {
    println!("\n❌ Erro: {}", message);
}

/// Exibe uma mensagem de aviso
pub fn print_warning(message: &str) {
    println!("\n⚠️  {}", message);
}

/// Exibe uma tarefa formatada
pub fn print_task(task: &Task) {
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ ID: {:<52} │", task.id);
    println!("│ Título: {:<48} │", task.title);
    println!("│ Descrição: {:<45} │", task.description);
    println!("│ Categoria: {:<45} │", task.category.as_str());
    println!("│ Prioridade: {:<44} │", task.priority.as_str());
    println!("│ Status: {:<48} │", task.status.as_str());

    if let Some(due_date) = task.due_date {
        let overdue = if task.is_overdue() { " (ATRASADA!)" } else { "" };
        println!("│ Vencimento: {:<40}{} │", due_date, overdue);
    }

    println!(
        "│ Criada em: {:<45} │",
        task.created_at.format("%d/%m/%Y %H:%M")
    );

    if let Some(completed_at) = task.completed_at {
        println!(
            "│ Concluída em: {:<42} │",
            completed_at.format("%d/%m/%Y %H:%M")
        );
    }

    println!("└─────────────────────────────────────────────────────────┘");
}

/// Exibe uma lista de tarefas
pub fn print_task_list(tasks: &[&Task]) {
    if tasks.is_empty() {
        print_warning("Nenhuma tarefa encontrada.");
        return;
    }

    println!("\n{:<4} {:<25} {:<12} {:<10} {:<12}", "ID", "Título", "Categoria", "Prioridade", "Status");
    println!("{}", "─".repeat(70));

    for task in tasks {
        let title = if task.title.len() > 25 {
            format!("{}...", &task.title[..22])
        } else {
            task.title.clone()
        };

        let overdue = if task.is_overdue() { "⚠️ " } else { "" };

        println!(
            "{:<4} {:<25} {:<12} {:<10} {}{}",
            task.id,
            title,
            task.category.as_str(),
            task.priority.as_str(),
            overdue,
            task.status.as_str()
        );
    }

    println!("\nTotal: {} tarefa(s)", tasks.len());
}

/// Exibe estatísticas
pub fn print_statistics(stats: &Statistics) {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║                      ESTATÍSTICAS                        ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    println!("\n📊 Resumo Geral:");
    println!("   Total de tarefas: {}", stats.total);
    println!("   ✅ Concluídas: {}", stats.completed);
    println!("   🔄 Em andamento: {}", stats.in_progress);
    println!("   ⏳ Pendentes: {}", stats.pending);
    println!("   ⚠️  Atrasadas: {}", stats.overdue);

    println!("\n📁 Por Categoria:");
    for (category, count) in &stats.by_category {
        if *count > 0 {
            println!("   {}: {}", category.as_str(), count);
        }
    }

    println!("\n⭐ Por Prioridade:");
    for (priority, count) in &stats.by_priority {
        if *count > 0 {
            println!("   {}: {}", priority.as_str(), count);
        }
    }
}