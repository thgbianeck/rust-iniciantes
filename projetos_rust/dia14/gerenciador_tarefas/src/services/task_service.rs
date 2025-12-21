use crate::models::{Category, Priority, Status, Task};
use crate::storage::Storage;
use std::io;

/// Serviço para gerenciar tarefas
pub struct TaskService {
    tasks: Vec<Task>,
    next_id: u32,
    pub storage: Storage,
}

impl TaskService {
    /// Cria um novo TaskService
    pub fn new(storage_path: String) -> Self {
        let storage = Storage::new(storage_path);
        let (tasks, next_id) = Self::load_from_storage(&storage);

        TaskService {
            tasks,
            next_id,
            storage,
        }
    }

    /// Carrega tarefas do storage
    fn load_from_storage(storage: &Storage) -> (Vec<Task>, u32) {
        match storage.load::<Vec<Task>>() {
            Ok(tasks) => {
                let max_id = tasks.iter().map(|t| t.id).max().unwrap_or(0);
                (tasks, max_id + 1)
            }
            Err(_) => (Vec::new(), 1),
        }
    }

    /// Salva tarefas no storage
    fn save(&self) -> io::Result<()> {
        self.storage.save(&self.tasks)
    }

    /// Adiciona uma nova tarefa
    pub fn add_task(
        &mut self,
        title: String,
        description: String,
        category: Category,
        priority: Priority,
        due_date: Option<chrono::NaiveDate>,
    ) -> io::Result<&Task> {
        let task = Task::new(
            self.next_id,
            title,
            description,
            category,
            priority,
            due_date,
        );

        self.tasks.push(task);
        self.next_id += 1;
        self.save()?;

        Ok(self.tasks.last().unwrap())
    }

    /// Retorna todas as tarefas
    pub fn list_all(&self) -> &[Task] {
        &self.tasks
    }

    /// Busca uma tarefa por ID
    pub fn get_by_id(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }

    /// Atualiza uma tarefa
    pub fn update_task(
        &mut self,
        id: u32,
        title: Option<String>,
        description: Option<String>,
        category: Option<Category>,
        priority: Option<Priority>,
        due_date: Option<Option<chrono::NaiveDate>>,
    ) -> io::Result<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Tarefa não encontrada"))?;

        if let Some(t) = title {
            task.title = t;
        }
        if let Some(d) = description {
            task.description = d;
        }
        if let Some(c) = category {
            task.category = c;
        }
        if let Some(p) = priority {
            task.priority = p;
        }
        if let Some(dd) = due_date {
            task.due_date = dd;
        }

        self.save()
    }

    /// Deleta uma tarefa
    pub fn delete_task(&mut self, id: u32) -> io::Result<()> {
        let index = self
            .tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Tarefa não encontrada"))?;

        self.tasks.remove(index);
        self.save()
    }

    /// Marca uma tarefa como concluída
    pub fn complete_task(&mut self, id: u32) -> io::Result<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Tarefa não encontrada"))?;

        task.complete();
        self.save()
    }

    /// Inicia uma tarefa
    pub fn start_task(&mut self, id: u32) -> io::Result<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Tarefa não encontrada"))?;

        task.start();
        self.save()
    }

    /// Filtra tarefas por status
    pub fn filter_by_status(&self, status: Status) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.status == status)
            .collect()
    }

    /// Filtra tarefas por categoria
    pub fn filter_by_category(&self, category: Category) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.category == category)
            .collect()
    }

    /// Filtra tarefas por prioridade
    pub fn filter_by_priority(&self, priority: Priority) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.priority == priority)
            .collect()
    }

    /// Retorna tarefas atrasadas
    pub fn get_overdue(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.is_overdue()).collect()
    }

    /// Retorna estatísticas
    pub fn get_statistics(&self) -> Statistics {
        let total = self.tasks.len();
        let completed = self.filter_by_status(Status::Completed).len();
        let in_progress = self.filter_by_status(Status::InProgress).len();
        let pending = self.filter_by_status(Status::Pending).len();
        let overdue = self.get_overdue().len();

        let by_category = Category::all()
            .iter()
            .map(|c| (*c, self.filter_by_category(*c).len()))
            .collect();

        let by_priority = Priority::all()
            .iter()
            .map(|p| (*p, self.filter_by_priority(*p).len()))
            .collect();

        Statistics {
            total,
            completed,
            in_progress,
            pending,
            overdue,
            by_category,
            by_priority,
        }
    }
}

/// Estrutura para estatísticas
#[derive(Debug)]
pub struct Statistics {
    pub total: usize,
    pub completed: usize,
    pub in_progress: usize,
    pub pending: usize,
    pub overdue: usize,
    pub by_category: Vec<(Category, usize)>,
    pub by_priority: Vec<(Priority, usize)>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    fn create_test_service() -> TaskService {
        TaskService::new("data/test_tasks.json".to_string())
    }

    #[test]
    fn test_add_task() {
        let mut service = create_test_service();
        let initial_count = service.list_all().len();

        service
            .add_task(
                "Test Task".to_string(),
                "Description".to_string(),
                Category::Work,
                Priority::High,
                None,
            )
            .unwrap();

        assert_eq!(service.list_all().len(), initial_count + 1);

        // Cleanup
        service.storage.delete().ok();
    }

    #[test]
    fn test_get_by_id() {
        let mut service = create_test_service();

        let task = service
            .add_task(
                "Test".to_string(),
                "Desc".to_string(),
                Category::Personal,
                Priority::Medium,
                None,
            )
            .unwrap();

        let id = task.id;
        let found = service.get_by_id(id);

        assert!(found.is_some());
        assert_eq!(found.unwrap().title, "Test");

        // Cleanup
        service.storage.delete().ok();
    }

    #[test]
    fn test_update_task() {
        let mut service = create_test_service();

        let task = service
            .add_task(
                "Original".to_string(),
                "Desc".to_string(),
                Category::Work,
                Priority::Low,
                None,
            )
            .unwrap();

        let id = task.id;

        service
            .update_task(
                id,
                Some("Updated".to_string()),
                None,
                None,
                Some(Priority::High),
                None,
            )
            .unwrap();

        let updated = service.get_by_id(id).unwrap();
        assert_eq!(updated.title, "Updated");
        assert_eq!(updated.priority, Priority::High);

        // Cleanup
        service.storage.delete().ok();
    }

    #[test]
    fn test_delete_task() {
        let mut service = create_test_service();

        let task = service
            .add_task(
                "To Delete".to_string(),
                "Desc".to_string(),
                Category::Other,
                Priority::Low,
                None,
            )
            .unwrap();

        let id = task.id;
        let count_before = service.list_all().len();

        service.delete_task(id).unwrap();

        assert_eq!(service.list_all().len(), count_before - 1);
        assert!(service.get_by_id(id).is_none());

        // Cleanup
        service.storage.delete().ok();
    }

    #[test]
    fn test_complete_task() {
        let mut service = create_test_service();

        let task = service
            .add_task(
                "To Complete".to_string(),
                "Desc".to_string(),
                Category::Study,
                Priority::Medium,
                None,
            )
            .unwrap();

        let id = task.id;

        service.complete_task(id).unwrap();

        let completed = service.get_by_id(id).unwrap();
        assert_eq!(completed.status, Status::Completed);
        assert!(completed.completed_at.is_some());

        // Cleanup
        service.storage.delete().ok();
    }

    #[test]
    fn test_filter_by_status() {
        let mut service = create_test_service();

        service
            .add_task(
                "Task 1".to_string(),
                "Desc".to_string(),
                Category::Work,
                Priority::High,
                None,
            )
            .unwrap();

        let task2 = service
            .add_task(
                "Task 2".to_string(),
                "Desc".to_string(),
                Category::Work,
                Priority::High,
                None,
            )
            .unwrap();

        service.complete_task(task2.id).unwrap();

        let pending = service.filter_by_status(Status::Pending);
        let completed = service.filter_by_status(Status::Completed);

        assert_eq!(pending.len(), 1);
        assert_eq!(completed.len(), 1);

        // Cleanup
        service.storage.delete().ok();
    }

    #[test]
    fn test_statistics() {
        let mut service = create_test_service();

        service
            .add_task(
                "Task 1".to_string(),
                "Desc".to_string(),
                Category::Work,
                Priority::High,
                None,
            )
            .unwrap();

        service
            .add_task(
                "Task 2".to_string(),
                "Desc".to_string(),
                Category::Personal,
                Priority::Medium,
                None,
            )
            .unwrap();

        let stats = service.get_statistics();

        assert_eq!(stats.total, 2);
        assert_eq!(stats.pending, 2);
        assert_eq!(stats.completed, 0);

        // Cleanup
        service.storage.delete().ok();
    }
}