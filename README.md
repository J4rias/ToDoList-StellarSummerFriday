# ToDoList-StellarSummerFriday
Respuesta al reto de hacer un "To Do List" para ejecutar en un Smart Contract de Stellar. 

Los siguientes códigos solo funcionan dentro de GitHub CodeSpace:

**Script para instalar rust y complementos en la maquina GitHub**
```plaintext
bash ./scripts/install.sh
```

**Creación de una entidad un “alias” de una billetera:** Para poder desplegar contratos y hacer operaciones de escritura en los contratos necesitamos crear una billetera con fondos en la red de pruebas de la siguiente manera:
```plaintext
stellar keys generate <entity> --network testnet --fund
stellar keys address <alias>
```

Aquí se recomienda cerrar la terminal y volver a iniciarla para que no genere error al compilar el contrato.

**Compilar el contrato**
```plaintext
stellar contract build
```

**Funciones disponibles**
```plaintext
     • add_task - Añade una nueva tarea. 
     • get_all - Lista las tareas existentes (menos las eliminadas).
     • get_task_by_id - Recupera una tarea por su ID.
     • task_completed - Marca como completa una tarea.
     • task_deleted - Elimina una tarea.
```

**Despliegue dentro de GitHub CodeSpace**
```plaintext
stellar contract deploy \
 --wasm target/wasm32v1-none/release/to_do_list.wasm \
  --source <entity> \
  --network testnet \
  --alias to_do_list
```

Se recomienda copiar el ID que aparece en la última línea de la ejecución anterior por si el alias no funciona.

**Añadir una tarea**
```plaintext
stellar contract invoke \
  --id to_do_list \
  --network testnet \
  --source <entity> \
  --send=yes \
  -- add_task --description "Dejar tarea a Maria Bolkonsky" --owner "Leo Tolstoi"
```

**Traer una tarea por ID**
```plaintext
stellar contract invoke \
  --id to_do_list \
  --network testnet \
  --source <entity> \
  --send=yes \
  -- get_task_by_id --task_id 1
```

**Eliminar una tarea por ID**
```plaintext
stellar contract invoke \
  --id to_do_list \
  --network testnet \
  --source <entity> \
  --send=yes \
  -- task_deleted --task_id 1
```

**Marcar como completa una tarea por ID**
```plaintext
stellar contract invoke \
  --id to_do_list \
  --network testnet \
  --source <entity> \
  --send=yes \
  -- task_completed --task_id 1
```

**Recuperar todas las tareas (a excepción de las eliminadas)**
```plaintext
stellar contract invoke \
  --id to_do_list \
  --network testnet \
  --source <entity> \
  --send=yes \
  -- get_all
```

**Correr test**
```plaintext
cargo test
```