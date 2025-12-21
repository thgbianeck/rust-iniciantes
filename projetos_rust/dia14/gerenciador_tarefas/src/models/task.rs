use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};

use super::enums::{Category, Priority, Status};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub priority: Priority,
    pub status: Status,
    pub due_date: Option<NaiveDate>,
    pub created_at: DateTime<Local>,
    pub completed_at: Option<DateTime<Local>>,
}

impl Task {
    /// Cria uma nova tarefa
    pub fn new(
        id: u32,
        title: String,
        description: String,
        category: Category,
        priority: Priority,
        due_date: Option<NaiveDate>,
    ) -> Self {
        Task {
            id,
            title,
            description,
            category,
            priority,
            status: Status::Pending,
            due_date,
            created_at: Local::now(),
            completed_at: None,
        }
    }

    /// Marca a tarefa como concluída
    pub fn complete(&mut self) {
        self.status = Status::Completed;
        self.completed_at = Some(Local::now());
    }

    /// Verifica se a tarefa está atrasada
    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            if self.status != Status::Completed {
                let today = Local::now().date_naive();
                return due_date < today;
            }
        }
        false
    }

    /// Inicia a tarefa (muda status para InProgress)
    pub fn start(&mut self) {
        if self.status == Status::Pending {
            self.status = Status::InProgress;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_new_task() {
        let task = Task::new(
            1,
            "Estudar Rust".to_string(),
            "Completar Fase 1".to_string(),
            Category::Study,
            Priority::High,
            None,
        );

        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Estudar Rust");
        assert_eq!(task.status, Status::Pending);
        assert!(task.completed_at.is_none());
    }

    #[test]
    fn test_complete_task() {
        let mut task = Task::new(
            1,
            "Test".to_string(),
            "Desc".to_string(),
            Category::Work,
            Priority::Medium,
            None,
        );

        task.complete();

        assert_eq!(task.status, Status::Completed);
        assert!(task.completed_at.is_some());
    }

    #[test]
    fn test_is_overdue() {
        let yesterday = Local::now().date_naive() - Duration::days(1);
        let task = Task::new(
            1,
            "Test".to_string(),
            "Desc".to_string(),
            Category::Work,
            Priority::High,
            Some(yesterday),
        );

        assert!(task.is_overdue());
    }

    #[test]
    fn test_not_overdue_when_completed() {
        let yesterday = Local::now().date_naive() - Duration::days(1);
        let mut task = Task::new(
            1,
            "Test".to_string(),
            "Desc".to_string(),
            Category::Work,
            Priority::High,
            Some(yesterday),
        );

        task.complete();

        assert!(!task.is_overdue());
    }

    #[test]
    fn test_start_task() {
        let mut task = Task::new(
            1,
            "Test".to_string(),
            "Desc".to_string(),
            Category::Work,
            Priority::Medium,
            None,
        );

        task.start();

        assert_eq!(task.status, Status::InProgress);
    }
}