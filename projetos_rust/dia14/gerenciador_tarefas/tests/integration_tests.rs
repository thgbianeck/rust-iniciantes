use gerenciador_tarefas::models::{Category, Priority, Status};
use gerenciador_tarefas::services::TaskService;

#[test]
fn test_full_crud_workflow() {
    // Cria serviço
    let mut service = TaskService::new("data/test_integration.json".to_string());

    // CREATE
    let task = service
        .add_task(
            "Integration Test".to_string(),
            "Testing full workflow".to_string(),
            Category::Work,
            Priority::High,
            None,
        )
        .unwrap();

    let task_id = task.id;

    // READ
    let found = service.get_by_id(task_id);
    assert!(found.is_some());
    assert_eq!(found.unwrap().title, "Integration Test");

    // UPDATE
    service
        .update_task(
            task_id,
            Some("Updated Title".to_string()),
            None,
            None,
            None,
            None,
        )
        .unwrap();

    let updated = service.get_by_id(task_id).unwrap();
    assert_eq!(updated.title, "Updated Title");

    // Complete
    service.complete_task(task_id).unwrap();
    let completed = service.get_by_id(task_id).unwrap();
    assert_eq!(completed.status, Status::Completed);

    // DELETE
    service.delete_task(task_id).unwrap();
    assert!(service.get_by_id(task_id).is_none());

    // Cleanup
    service.storage.delete().ok();
}

#[test]
fn test_persistence() {
    let storage_path = "data/test_persistence.json".to_string();

    // Cria e adiciona tarefa
    {
        let mut service = TaskService::new(storage_path.clone());
        service
            .add_task(
                "Persistent Task".to_string(),
                "Should survive restart".to_string(),
                Category::Personal,
                Priority::Medium,
                None,
            )
            .unwrap();
    }

    // Carrega novamente
    {
        let service = TaskService::new(storage_path.clone());
        let tasks = service.list_all();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Persistent Task");

        // Cleanup
        service.storage.delete().ok();
    }
}

#[test]
fn test_filters() {
    let mut service = TaskService::new("data/test_filters.json".to_string());

    // Adiciona várias tarefas
    service
        .add_task(
            "Work Task".to_string(),
            "Desc".to_string(),
            Category::Work,
            Priority::High,
            None,
        )
        .unwrap();

    service
        .add_task(
            "Personal Task".to_string(),
            "Desc".to_string(),
            Category::Personal,
            Priority::Low,
            None,
        )
        .unwrap();

    let task3 = service
        .add_task(
            "Study Task".to_string(),
            "Desc".to_string(),
            Category::Study,
            Priority::High,
            None,
        )
        .unwrap();

    service.complete_task(task3.id).unwrap();

    // Testa filtros
    let work_tasks = service.filter_by_category(Category::Work);
    assert_eq!(work_tasks.len(), 1);

    let high_priority = service.filter_by_priority(Priority::High);
    assert_eq!(high_priority.len(), 2);

    let completed = service.filter_by_status(Status::Completed);
    assert_eq!(completed.len(), 1);

    // Cleanup
    service.storage.delete().ok();
}

#[test]
fn test_statistics() {
    let mut service = TaskService::new("data/test_stats.json".to_string());

    // Adiciona tarefas
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
            Priority::Medium,
            None,
        )
        .unwrap();

    service
        .add_task(
            "Task 3".to_string(),
            "Desc".to_string(),
            Category::Personal,
            Priority::Low,
            None,
        )
        .unwrap();

    service.complete_task(task2.id).unwrap();

    // Testa estatísticas
    let stats = service.get_statistics();
    assert_eq!(stats.total, 3);
    assert_eq!(stats.completed, 1);
    assert_eq!(stats.pending, 2);

    // Cleanup
    service.storage.delete().ok();
}