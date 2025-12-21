use gerenciador_tarefas::services::TaskService;
use gerenciador_tarefas::ui::Menu;

fn main() {
    // Caminho do arquivo de armazenamento
    let storage_path = "data/tasks.json".to_string();

    // Cria o serviço de tarefas
    let service = TaskService::new(storage_path);

    // Cria e executa o menu
    let mut menu = Menu::new(service);
    menu.run();
}