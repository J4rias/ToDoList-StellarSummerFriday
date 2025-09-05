#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_add_task_and_get_taks_by_id() {
    let env = Env::default();
    let contract_id = env.register(ToDoListContract, ());
    let client = ToDoListContractClient::new(&env, &contract_id);

    // Test: Crear tarea
    let description = String::from_str(&env, "Movilizar a todo el ejército para el combate.");
    let owner = String::from_str(&env, "Sun Tzu");
    let timestamp: u64 = env.ledger().timestamp();

    let result = client.add_task(&description, &owner);
    assert_eq!(result, 1);

    // Test: Recuperar la tarea
    let task = client.get_task_by_id(&1);
    assert!(task.is_some());

    let task = task.unwrap();
    assert_eq!(task.id, 1);
    assert_eq!(task.description, description);
    assert_eq!(task.owner, owner);
    assert_eq!(task.status, TaskStatus::Pending);
    assert_eq!(task.timestamp, timestamp);
}

#[test]
fn test_invalid_task_data() {
    let env = Env::default();
    let contract_id = env.register(ToDoListContract, ());
    let client = ToDoListContractClient::new(&env, &contract_id);

    // Test: Agregar tarea con descripción vacía
    let empty_description = String::from_str(&env, "");
    let empty_owner = String::from_str(&env, "Leo Tolstoy");

    let result = client.try_add_task(&empty_description, &empty_owner);
    assert_eq!(result, Err(Ok(TaskError::InvalidTaskData)));
}

#[test]
fn test_task_not_found() {
    let env = Env::default();
    let contract_id = env.register(ToDoListContract, ());
    let client = ToDoListContractClient::new(&env, &contract_id);

    // Test: Tarea que no existe
    let task = client.get_task_by_id(&123);
    assert!(task.is_none());
}

#[test]
fn test_get_all() {
    let env = Env::default();
    let contract_id = env.register(ToDoListContract, ());
    let client = ToDoListContractClient::new(&env, &contract_id);

    // Agregar varias tareas
    client.add_task(&String::from_str(&env, "Preparar los dos minutos de odio"), &String::from_str(&env, "George Orwell"));
    client.add_task(&String::from_str(&env, "Consumir el soma de la mañana"), &String::from_str(&env, "Aldous Huxley"));
    client.add_task(&String::from_str(&env, "Ayudar a los bomberos con los libros"), &String::from_str(&env, "Ray Bradbury"));

    // Test: Completar una tarea
    client.task_completed(&1);
    
    // Test: Eliminar una tarea
    client.task_deleted(&2);

    // Test: Obtener todas las tareas (excluye las eliminadas)
    let available = client.get_all();
    assert_eq!(available.len(), 2); // Solo 2 tareas

    // Verificar que son las correctos
    let task1 = available.get(0).unwrap();
    let task3 = available.get(1).unwrap();
    assert_eq!(task1.id, 1);
    assert_eq!(task3.id, 3);
}