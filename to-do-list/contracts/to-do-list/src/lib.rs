#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, contracttype, Env, String, Symbol, Vec, symbol_short};

// Enum con los posibles estados de las tareas
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskStatus {
    Completed,
    Pending,
    Deleted,
}

// Estructura de una tarea
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub owner: String,
    pub status: TaskStatus,
    pub timestamp: u64,
}

// Enum de errores personalizados
#[contracterror]
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum TaskError {
    TaskNotFound = 1,
    InvalidTaskData = 2,
}

#[contract]
pub struct ToDoListContract;

// Constante para la clave del próximo ID
const NEXT_ID_KEY: Symbol = symbol_short!("next_id");



#[contractimpl]
impl ToDoListContract {
    pub fn add_task(env: Env, description: String, owner: String) -> Result<u32, TaskError> {
        // Validar que los parámetros no están vacíos
        if description.len() == 0 || owner.len() == 0 {
            return Err(TaskError::InvalidTaskData);
        }
        
        // Obtener el próximo ID disponible
        let next_id = Self::get_next_task_id(&env);
        
        // Timestamp del bloque en epoch UNIX
        let timestamp: u64 = env.ledger().timestamp();

        let new_task = Task {
            id: next_id,
            description: description.clone(),
            owner: owner.clone(),
            status: TaskStatus::Pending,
            timestamp: timestamp,
        };

        // Guardar la tarea en el data storage
        env.storage().instance().set(&next_id, &new_task);

        // Actualizar el indice de IDs
        env.storage().instance().set(&NEXT_ID_KEY, &(next_id + 1));

        Ok(next_id)
    }

    // Obtener tarea por ID
    pub fn get_task_by_id(env: Env, task_id: u32) -> Option<Task> {
        env.storage().instance().get(&task_id)
    }

    // Retorna todas las tareas pendientes y concluidas
    pub fn get_all(env: Env) -> Vec<Task> {
        let mut tasks = Vec::new(&env);
        let last_id = Self::get_next_task_id(&env);

        // Iterar las tareas y excluir las eliminadas
        for id in 1..last_id {
            if let Some(task) = Self::get_task_by_id(env.clone(), id) {
                if task.status == TaskStatus::Completed || task.status == TaskStatus::Pending {
                    tasks.push_back(task);
                }
            }
        }
        tasks
    }

    // Concluir tarea
    pub fn task_completed(env: Env, task_id: u32) -> Result<(), TaskError> {
        let mut task: Task = env
            .storage()
            .instance()
            .get(&task_id)
            .ok_or(TaskError::TaskNotFound)?;

        task.status = TaskStatus::Completed;

        env.storage().instance().set(&task_id, &task);

        Ok(())
    }

    // Eliminar tarea
    pub fn task_deleted(env: Env, task_id: u32) -> Result<(), TaskError> {
        let mut task: Task = env
            .storage()
            .instance()
            .get(&task_id)
            .ok_or(TaskError::TaskNotFound)?;

        task.status = TaskStatus::Deleted;

        env.storage().instance().set(&task_id, &task);

        Ok(())
    }

    /// Función helper para obtener el próximo ID disponible
    fn get_next_task_id(env: &Env) -> u32 {
        env.storage().instance().get(&NEXT_ID_KEY).unwrap_or(1)
    }
}

mod test;