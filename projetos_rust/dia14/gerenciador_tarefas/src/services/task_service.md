# 🔧 Análise Detalhada da Struct `TaskService` em Rust

Vou explicar este código que implementa a **camada de lógica de negócio** (service layer) para gerenciar tarefas, incluindo operações CRUD, filtros e estatísticas.

---

## 📦 **Imports: Dependências do Módulo**

```rust {.line-numbers}
use crate::models::{Category, Priority, Status, Task};
use crate::storage::Storage;
use std::io;
```

### Análise de Cada Import:

| Import | Origem | O que é 

**`crate::` - Caminho Absoluto**
- **`crate`** = Raiz do projeto (equivalente a `src/lib.rs`)
- Caminho absoluto a partir da raiz

**Estrutura de diretórios:**
```
src/
├── lib.rs              ← crate (raiz)
├── models/
│   ├── mod.rs
│   ├── task.rs         ← Task
│   └── enums.rs        ← Category, Priority, Status
├── storage/
│   └── json_storage.rs ← Storage
└── services/
    └── task_service.rs ← VOCÊ ESTÁ AQUI
```

**Analogia do prédio:**
```
🏢 PRÉDIO (crate)
   ├── 5º Andar - MODELS (crate::models)
   │   ├── Task, Category, Priority, Status
   ├── 7º Andar - STORAGE (crate::storage)
   │   └── Storage
   └── 6º Andar - SERVICES (crate::services)
       └── TaskService ← Você está aqui
```

---

## 🏗️ **Struct `TaskService` - Gerenciador de Tarefas**

```rust {.line-numbers}
/// Serviço para gerenciar tarefas
pub struct TaskService {
    tasks: Vec<Task>,
    next_id: u32,
    storage: Storage,
}
```

### Campos da Struct:

| Campo | Tipo | Propósito 

**Analogia:**
- **`TaskService`** = Gerente de um escritório
- **`tasks`** = Lista de tarefas na mesa do gerente
- **`next_id`** = Contador de protocolos (próximo número disponível)
- **`storage`** = Arquivo físico onde os documentos são guardados

---

## 🔧 **Método 1: `new()` - Construtor**

```rust {.line-numbers}
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
```

### Análise Detalhada:

**Passo a passo:**

**1. Criar instância de Storage**
```rust {.line-numbers}
let storage = Storage::new(storage_path);
```
- Cria o gerenciador de persistência
- `storage_path` = Caminho do arquivo JSON (ex: "data/tarefas.json")

**2. Carregar tarefas existentes**
```rust {.line-numbers}
let (tasks, next_id) = Self::load_from_storage(&storage);
```
- **`Self::load_from_storage`** = Chama função associada (método estático)
- Retorna **tupla** `(Vec<Task>, u32)`
- **Desestruturação** da tupla em duas variáveis

**3. Construir a struct**
```rust {.line-numbers}
TaskService {
    tasks,
    next_id,
    storage,
}
```

**Uso:**
```rust {.line-numbers}
let service = TaskService::new("data/tarefas.json".to_string());
```

**Analogia:** É como **contratar um gerente** que já vem com a lista de tarefas existentes carregada.

---

## 📂 **Método 2: `load_from_storage()` - Carregar do Arquivo**

```rust {.line-numbers}
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
```

### Análise Detalhada:

---

#### a) **Assinatura**

```rust {.line-numbers}
fn load_from_storage(storage: &Storage) -> (Vec<Task>, u32)
```

**Características:**
- **`fn`** (sem `pub`) = Função **privada** (só usada internamente)
- **`storage: &Storage`** = Referência ao Storage (não consome)
- **`-> (Vec<Task>, u32)`** = Retorna **tupla** com tarefas e próximo ID

**Por que retornar tupla?**
- Precisa retornar **dois valores** ao mesmo tempo
- Alternativa seria criar uma struct, mas tupla é mais simples para casos pequenos

---

#### b) **Tentativa de Carregar Tarefas**

```rust {.line-numbers}
match storage.load::<Vec<Task>>() {
```

- **`storage.load::<Vec<Task>>()`** = Tenta carregar vetor de tarefas do arquivo
- **Turbofish `::<Vec<Task>>`** = Especifica o tipo explicitamente
- Retorna `io::Result<Vec<Task>>`

**Por que turbofish?**
- Rust não consegue inferir o tipo automaticamente aqui
- Precisamos dizer explicitamente que queremos `Vec<Task>`

---

#### c) **Caso de Sucesso: `Ok(tasks)`**

```rust {.line-numbers}
Ok(tasks) => {
    let max_id = tasks.iter().map(|t| t.id).max().unwrap_or(0);
    (tasks, max_id + 1)
}
```

**Passo a passo:**

**1. Encontrar o maior ID**
```rust {.line-numbers}
let max_id = tasks.iter().map(|t| t.id).max().unwrap_or(0);
```

Vamos quebrar isso:

**a) `tasks.iter()`**
- Cria um **iterador** sobre as tarefas
- Não consome o vetor (apenas empresta)

**b) `.map(|t| t.id)`**
- **Transforma** cada tarefa em seu ID
- `|t| t.id` = Closure (função anônima) que extrai o ID

**Exemplo:**
```rust {.line-numbers}
// Tarefas: [Task{id:1}, Task{id:5}, Task{id:3}]
// Após map: [1, 5, 3]
```

**c) `.max()`**
- Encontra o **maior valor** no iterador
- Retorna `Option<u32>` (pode não haver tarefas)

**Exemplo:**
```rust {.line-numbers}
// [1, 5, 3] → Some(5)
// [] → None
```

**d) `.unwrap_or(0)`**
- Se `Some(valor)` → retorna `valor`
- Se `None` → retorna `0` (padrão)

**Exemplo:**
```rust {.line-numbers}
Some(5).unwrap_or(0)  // 5
None.unwrap_or(0)     // 0
```

**Fluxo completo:**
```
Tarefas: [Task{id:1}, Task{id:5}, Task{id:3}]
   ↓ iter()
Iterador: &Task, &Task, &Task
   ↓ map(|t| t.id)
IDs: 1, 5, 3
   ↓ max()
Maior: Some(5)
   ↓ unwrap_or(0)
Resultado: 5
```

**2. Calcular próximo ID**
```rust {.line-numbers}
(tasks, max_id + 1)
```
- Se `max_id = 5`, então `next_id = 6`
- Garante que o próximo ID seja único

**Analogia:** É como verificar o **último número de protocolo** usado e pegar o próximo.

---

#### d) **Caso de Erro: `Err(_)`**

```rust {.line-numbers}
Err(_) => (Vec::new(), 1),
```

**O que significa `_`?**
- **Placeholder** que ignora o valor do erro
- "Não me importo com o erro específico"

**O que retorna:**
- **`Vec::new()`** = Vetor vazio (nenhuma tarefa)
- **`1`** = Próximo ID começa em 1

**Quando isso acontece?**
- Arquivo não existe (primeira execução)
- Arquivo corrompido
- Erro de permissão

**Analogia:** É como começar com uma **lista em branco** quando não há arquivo anterior.

---

### Fluxo Completo do `load_from_storage()`:

```
┌─────────────────────────────────────────────────────────┐
│          FLUXO DO load_from_storage()                   │
└─────────────────────────────────────────────────────────┘

    Tentar carregar arquivo
        │
        ▼
    ┌───────────┐
    │ Sucesso?  │
    └─────┬─────┘
          │
     ┌────▼────┐
     │   Sim   │
     └────┬────┘
          │
          ▼
    ┌─────────────────────┐
    │ Encontrar maior ID  │ ← tasks.iter().map(|t| t.id).max()
    └──────────┬──────────┘
               │
               ▼
    ┌─────────────────────┐
    │ next_id = max_id+1  │
    └──────────┬──────────┘
               │
               ▼
         (tasks, next_id)
          
     ┌────▼────┐
     │   Não   │
     └────┬────┘
          │
          ▼
    ┌─────────────────────┐
    │ Retornar vazio      │
    └──────────┬──────────┘
               │
               ▼
         ([], 1)
```

---

## 💾 **Método 3: `save()` - Salvar no Arquivo**

```rust {.line-numbers}
/// Salva tarefas no storage
fn save(&self) -> io::Result<()> {
    self.storage.save(&self.tasks)
}
```

### Análise:

**Função privada simples:**
- **`&self`** = Referência imutável (apenas lê)
- Delega para `storage.save()`
- Salva **todas** as tarefas de uma vez

**Por que privada?**
- Usuários externos não devem chamar `save()` diretamente
- É chamada automaticamente após operações que modificam tarefas

**Analogia:** É como **arquivar** todos os documentos da mesa no arquivo físico.

---

## ➕ **Método 4: `add_task()` - Adicionar Tarefa**

```rust {.line-numbers}
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
```

### Análise Detalhada:

---

#### a) **Assinatura**

```rust {.line-numbers}
pub fn add_task(
    &mut self,
    title: String,
    description: String,
    category: Category,
    priority: Priority,
    due_date: Option<chrono::NaiveDate>,
) -> io::Result<&Task>
```

**Características:**
- **`&mut self`** = Referência **mutável** (vai modificar o estado)
- **Parâmetros:** Dados necessários para criar a tarefa
- **`-> io::Result<&Task>`** = Retorna **referência** à tarefa criada

**Por que retornar `&Task` e não `Task`?**
- Evita clonar a tarefa
- Permite ao chamador acessar o ID gerado
- Mais eficiente

---

#### b) **Passo 1: Criar a Tarefa**

```rust {.line-numbers}
let task = Task::new(
    self.next_id,
    title,
    description,
    category,
    priority,
    due_date,
);
```

- Usa o construtor de `Task`
- **`self.next_id`** = ID único gerado automaticamente

---

#### c) **Passo 2: Adicionar ao Vetor**

```rust {.line-numbers}
self.tasks.push(task);
```

- **`push()`** = Adiciona ao final do vetor
- **Move** `task` para dentro do vetor (ownership transferido)

---

#### d) **Passo 3: Incrementar Contador**

```rust {.line-numbers}
self.next_id += 1;
```

- Prepara o próximo ID para a próxima tarefa
- Garante IDs únicos

**Exemplo:**
```rust {.line-numbers}
// Antes: next_id = 5
self.next_id += 1;
// Depois: next_id = 6
```

---

#### e) **Passo 4: Persistir**

```rust {.line-numbers}
self.save()?;
```

- Salva **todas** as tarefas no arquivo
- **`?`** = Propaga erro se falhar

**Por que salvar após cada operação?**
- Garante que dados não sejam perdidos
- Mantém arquivo sempre atualizado
- Trade-off: performance vs segurança

---

#### f) **Passo 5: Retornar Referência**

```rust {.line-numbers}
Ok(self.tasks.last().unwrap())
```

**Quebrando:**

**1. `self.tasks.last()`**
- Retorna `Option<&Task>` (referência ao último elemento)
- `Some(&task)` se vetor não vazio
- `None` se vetor vazio

**2. `.unwrap()`**
- Extrai o valor de `Some`
- **Panic** se for `None`

**Por que `unwrap()` é seguro aqui?**
- Acabamos de fazer `push()`, então **sabemos** que há pelo menos um elemento
- O vetor **nunca** estará vazio neste ponto

**3. `Ok(...)`**
- Envolve a referência em `Ok` (sucesso)

---

### Uso do `add_task()`:

```rust {.line-numbers}
let mut service = TaskService::new("data/tarefas.json".to_string());

let tarefa = service.add_task(
    "Estudar Rust".to_string(),
    "Completar Fase 1".to_string(),
    Category::Study,
    Priority::High,
    None,
)?;

println!("Tarefa criada com ID: {}", tarefa.id);
```

---

## 📋 **Método 5: `list_all()` - Listar Todas as Tarefas**

```rust {.line-numbers}
/// Retorna todas as tarefas
pub fn list_all(&self) -> &[Task] {
    &self.tasks
}
```

### Análise:

**Retorno: `&[Task]` - Slice**
- **`&[Task]`** = Referência a um slice (fatia) de tarefas
- **Não** é `&Vec<Task>` (mais genérico)
- Permite passar para funções que aceitam slices

**Diferença entre `&Vec<T>` e `&[T]`:**


**Uso:**
```rust {.line-numbers}
let todas = service.list_all();
for tarefa in todas {
    println!("{}: {}", tarefa.id, tarefa.title);
}
```

**Analogia:** É como **mostrar** a lista completa de tarefas na mesa.

---

## 🔍 **Método 6: `get_by_id()` - Buscar por ID**

```rust {.line-numbers}
/// Busca uma tarefa por ID
pub fn get_by_id(&self, id: u32) -> Option<&Task> {
    self.tasks.iter().find(|t| t.id == id)
}
```

### Análise Detalhada:

---

#### a) **Retorno: `Option<&Task>`**

- **`Some(&task)`** = Tarefa encontrada
- **`None`** = Tarefa não encontrada

---

#### b) **Busca com `find()`**

```rust {.line-numbers}
self.tasks.iter().find(|t| t.id == id)
```

**Passo a passo:**

**1. `self.tasks.iter()`**
- Cria iterador sobre as tarefas

**2. `.find(|t| t.id == id)`**
- **`find`** = Busca o primeiro elemento que satisfaz a condição
- **`|t| t.id == id`** = Closure que verifica se o ID corresponde
- Retorna `Option<&Task>`

**Exemplo:**
```rust {.line-numbers}
// Tarefas: [Task{id:1}, Task{id:5}, Task{id:3}]
// Buscar ID 5:
tasks.iter().find(|t| t.id == 5)  // Some(&Task{id:5})

// Buscar ID 99:
tasks.iter().find(|t| t.id == 99)  // None
```

**Complexidade:** O(n) - busca linear

---

### Uso do `get_by_id()`:

```rust {.line-numbers}
match service.get_by_id(5) {
    Some(tarefa) => println!("Encontrada: {}", tarefa.title),
    None => println!("Tarefa não encontrada"),
}

// Ou com if let:
if let Some(tarefa) = service.get_by_id(5) {
    println!("Encontrada: {}", tarefa.title);
}
```

**Analogia:** É como **procurar** um documento específico pelo número de protocolo.

---

## ✏️ **Método 7: `update_task()` - Atualizar Tarefa**

```rust {.line-numbers}
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
```

### Análise Detalhada:

---

#### a) **Parâmetros Opcionais**

```rust {.line-numbers}
title: Option<String>,
description: Option<String>,
category: Option<Category>,
priority: Option<Priority>,
due_date: Option<Option<chrono::NaiveDate>>,
```

**Por que `Option`?**
- Permite **atualização parcial**
- `Some(valor)` = Atualizar este campo
- `None` = Manter valor atual

**`Option<Option<T>>` - Nested Option** 🤯

```rust {.line-numbers}
due_date: Option<Option<chrono::NaiveDate>>
```

**Por que dois `Option`?**
- **Primeiro `Option`:** "Quero atualizar o prazo?"
  - `Some(...)` = Sim, atualizar
  - `None` = Não, manter atual
- **Segundo `Option`:** "Qual é o novo prazo?"
  - `Some(data)` = Definir prazo
  - `None` = Remover prazo

**Exemplos:**
```rust {.line-numbers}
// Não atualizar prazo (manter atual)
due_date: None

// Definir prazo para 31/12/2024
due_date: Some(Some(NaiveDate::from_ymd(2024, 12, 31)))

// Remover prazo (definir como None)
due_date: Some(None)
```

---

#### b) **Buscar Tarefa Mutável**

```rust {.line-numbers}
let task = self
    .tasks
    .iter_mut()
    .find(|t| t.id == id)
    .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Tarefa não encontrada"))?;
```

**Quebrando em partes:**

**1. `self.tasks.iter_mut()`**
- **`iter_mut()`** = Iterador **mutável**
- Permite modificar os elementos

**Diferença:**


**2. `.find(|t| t.id == id)`**
- Busca a tarefa com o ID especificado
- Retorna `Option<&mut Task>`

**3. `.ok_or_else(|| ...)`**
- **Converte** `Option` em `Result`
- `Some(task)` → `Ok(task)`
- `None` → `Err(erro)`

**O que é `ok_or_else`?**
- Método de `Option` que transforma em `Result`
- **`ok_or_else`** = Lazy (closure só executada se `None`)
- **`ok_or`** = Eager (valor sempre criado)

**Exemplo:**
```rust {.line-numbers}
let opt: Option<i32> = None;

// ok_or - valor sempre criado
let result = opt.ok_or(expensive_error());  // expensive_error() sempre executado

// ok_or_else - closure só executada se None
let result = opt.ok_or_else(|| expensive_error());  // só executa se None
```

**4. `?` - Propagação de Erro**
- Se `Ok(task)` → extrai `task` e continua
- Se `Err(e)` → retorna erro imediatamente

---

#### c) **Atualização Condicional**

```rust {.line-numbers}
if let Some(t) = title {
    task.title = t;
}
if let Some(d) = description {
    task.description = d;
}
// ... etc
```

**Padrão de atualização parcial:**
- Verifica se o campo foi fornecido (`Some`)
- Se sim, atualiza
- Se não (`None`), mantém valor atual

**Exemplo:**
```rust {.line-numbers}
// Atualizar apenas título e prioridade
service.update_task(
    5,
    Some("Novo Título".to_string()),  // ← Atualiza
    None,                              // ← Mantém atual
    None,                              // ← Mantém atual
    Some(Priority::High),              // ← Atualiza
    None,                              // ← Mantém atual
)?;
```

---

#### d) **Persistir Mudanças**

```rust {.line-numbers}
self.save()
```

- Salva todas as tarefas no arquivo
- Retorna `io::Result<()>`

---

### Uso do `update_task()`:

```rust {.line-numbers}
// Atualizar apenas título
service.update_task(
    5,
    Some("Título Atualizado".to_string()),
    None,
    None,
    None,
    None,
)?;

// Atualizar título e remover prazo
service.update_task(
    5,
    Some("Novo Título".to_string()),
    None,
    None,
    None,
    Some(None),  // ← Remove prazo
)?;
```

---

## 🗑️ **Método 8: `delete_task()` - Deletar Tarefa**

```rust {.line-numbers}
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
```

### Análise Detalhada:

---

#### a) **Encontrar Índice**

```rust {.line-numbers}
let index = self
    .tasks
    .iter()
    .position(|t| t.id == id)
    .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Tarefa não encontrada"))?;
```

**O que é `position()`?**
- Método de iteradores que retorna o **índice** do primeiro elemento que satisfaz a condição
- Retorna `Option<usize>`

**Exemplo:**
```rust {.line-numbers}
// Tarefas: [Task{id:1}, Task{id:5}, Task{id:3}]
tasks.iter().position(|t| t.id == 5)  // Some(1) - índice 1
tasks.iter().position(|t| t.id == 99)  // None
```

**Por que `position()` em vez de `find()`?**
- **`find()`** retorna `Option<&T>` (referência ao elemento)
- **`position()`** retorna `Option<usize>` (índice)
- Precisamos do **índice** para remover do vetor

---

#### b) **Remover do Vetor**

```rust {.line-numbers}
self.tasks.remove(index);
```

**O que faz `remove()`?**
- Remove o elemento no índice especificado
- **Desloca** todos os elementos seguintes para a esquerda
- Retorna o elemento removido (que ignoramos aqui)

**Exemplo:**
```rust {.line-numbers}
// Antes: [Task{id:1}, Task{id:5}, Task{id:3}]
tasks.remove(1);
// Depois: [Task{id:1}, Task{id:3}]
```

**Complexidade:** O(n) - precisa deslocar elementos

---

#### c) **Persistir**

```rust {.line-numbers}
self.save()
```

---

### Uso do `delete_task()`:

```rust {.line-numbers}
match service.delete_task(5) {
    Ok(()) => println!("Tarefa deletada!"),
    Err(e) => eprintln!("Erro: {}", e),
}
```

**Analogia:** É como **jogar fora** um documento do arquivo.

---

## ✅ **Método 9: `complete_task()` - Marcar como Concluída**

```rust {.line-numbers}
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
```

### Análise:

**Padrão similar aos métodos anteriores:**
1. Buscar tarefa mutável
2. Chamar método `complete()` da tarefa
3. Salvar

**Uso:**
```rust {.line-numbers}
service.complete_task(5)?;
println!("Tarefa concluída!");
```

---

## ▶️ **Método 10: `start_task()` - Iniciar Tarefa**

```rust {.line-numbers}
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
```

### Análise:

**Idêntico a `complete_task()`, mas chama `start()` em vez de `complete()`**

---

## 🔍 **Métodos de Filtro**

### Método 11: `filter_by_status()` - Filtrar por Status

```rust {.line-numbers}
/// Filtra tarefas por status
pub fn filter_by_status(&self, status: Status) -> Vec<&Task> {
    self.tasks
        .iter()
        .filter(|t| t.status == status)
        .collect()
}
```

### Análise Detalhada:

---

#### a) **Retorno: `Vec<&Task>`**

- Vetor de **referências** às tarefas
- Não clona as tarefas (mais eficiente)

---

#### b) **Método `filter()`**

```rust {.line-numbers}
self.tasks
    .iter()
    .filter(|t| t.status == status)
    .collect()
```

**Passo a passo:**

**1. `self.tasks.iter()`**
- Cria iterador

**2. `.filter(|t| t.status == status)`**
- **Filtra** elementos que satisfazem a condição
- Retorna iterador com apenas elementos que passam no teste

**3. `.collect()`**
- **Coleta** os elementos do iterador em uma coleção
- Tipo inferido: `Vec<&Task>`

**Exemplo:**
```rust {.line-numbers}
// Tarefas:
// [Task{status:Pending}, Task{status:Completed}, Task{status:Pending}]

let pendentes = service.filter_by_status(Status::Pending);
// Resultado: [&Task{status:Pending}, &Task{status:Pending}]
```

---

### Método 12: `filter_by_category()` - Filtrar por Categoria

```rust {.line-numbers}
/// Filtra tarefas por categoria
pub fn filter_by_category(&self, category: Category) -> Vec<&Task> {
    self.tasks
        .iter()
        .filter(|t| t.category == category)
        .collect()
}
```

**Idêntico a `filter_by_status()`, mas filtra por categoria**

---

### Método 13: `filter_by_priority()` - Filtrar por Prioridade

```rust {.line-numbers}
/// Filtra tarefas por prioridade
pub fn filter_by_priority(&self, priority: Priority) -> Vec<&Task> {
    self.tasks
        .iter()
        .filter(|t| t.priority == priority)
        .collect()
}
```

**Idêntico aos anteriores, mas filtra por prioridade**

---

### Método 14: `get_overdue()` - Tarefas Atrasadas

```rust {.line-numbers}
/// Retorna tarefas atrasadas
pub fn get_overdue(&self) -> Vec<&Task> {
    self.tasks.iter().filter(|t| t.is_overdue()).collect()
}
```

**Usa o método `is_overdue()` da tarefa para filtrar**

---

## 📊 **Método 15: `get_statistics()` - Estatísticas**

```rust {.line-numbers}
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
```

### Análise Detalhada:

---

#### a) **Contagens Simples**

```rust {.line-numbers}
let total = self.tasks.len();
let completed = self.filter_by_status(Status::Completed).len();
let in_progress = self.filter_by_status(Status::InProgress).len();
let pending = self.filter_by_status(Status::Pending).len();
let overdue = self.get_overdue().len();
```

- Usa métodos de filtro existentes
- `.len()` para contar elementos

---

#### b) **Contagem por Categoria**

```rust {.line-numbers}
let by_category = Category::all()
    .iter()
    .map(|c| (*c, self.filter_by_category(*c).len()))
    .collect();
```

**Passo a passo:**

**1. `Category::all()`**
- Retorna `Vec<Category>` com todas as categorias
- `[Work, Personal, Study, Health, Other]`

**2. `.iter()`**
- Cria iterador sobre categorias

**3. `.map(|c| (*c, self.filter_by_category(*c).len()))`**
- **Transforma** cada categoria em uma tupla `(Category, usize)`
- **`*c`** = Desreferencia (copia o valor)
- **`self.filter_by_category(*c).len()`** = Conta tarefas nessa categoria

**Exemplo:**
```rust {.line-numbers}
// Entrada: [Work, Personal, Study, Health, Other]
// Saída: [(Work, 5), (Personal, 3), (Study, 2), (Health, 1), (Other, 0)]
```

**4. `.collect()`**
- Coleta em `Vec<(Category, usize)>`

---

#### c) **Contagem por Prioridade**

```rust {.line-numbers}
let by_priority = Priority::all()
    .iter()
    .map(|p| (*p, self.filter_by_priority(*p).len()))
    .collect();
```

**Idêntico a `by_category`, mas para prioridades**

---

#### d) **Construir Struct de Estatísticas**

```rust {.line-numbers}
Statistics {
    total,
    completed,
    in_progress,
    pending,
    overdue,
    by_category,
    by_priority,
}
```

---

### Uso do `get_statistics()`:

```rust {.line-numbers}
let stats = service.get_statistics();

println!("Total: {}", stats.total);
println!("Concluídas: {}", stats.completed);
println!("Em andamento: {}", stats.in_progress);
println!("Pendentes: {}", stats.pending);
println!("Atrasadas: {}", stats.overdue);

println!("\nPor categoria:");
for (cat, count) in &stats.by_category {
    println!("  {}: {}", cat.as_str(), count);
}

println!("\nPor prioridade:");
for (pri, count) in &stats.by_priority {
    println!("  {}: {}", pri.as_str(), count);
}
```

---

## 📊 **Struct `Statistics` - Estrutura de Estatísticas**

```rust {.line-numbers}
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
```

### Análise:

**Campos:**
- **Contadores simples:** `total`, `completed`, `in_progress`, `pending`, `overdue`
- **Distribuições:** `by_category`, `by_priority`

**Por que `Vec<(Category, usize)>`?**
- Tupla associa categoria com contagem
- Alternativa seria `HashMap<Category, usize>`, mas vetor é mais simples aqui

---

## 🧪 **Testes Automatizados**

Vou destacar alguns testes interessantes:

### Teste: `test_update_task`

```rust {.line-numbers}
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
```

**O que Este Teste Verifica?**

✅ Atualização parcial funciona  
✅ Campos não especificados mantêm valor original  
✅ Mudanças são persistidas  

---

### Teste: `test_filter_by_status`

```rust {.line-numbers}
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
```

**O que Este Teste Verifica?**

✅ Filtros funcionam corretamente  
✅ Tarefas são categorizadas por status  

---

## 🎯 **Conceitos-Chave Demonstrados**

### 1. **Service Layer Pattern**
- Encapsula lógica de negócio
- Coordena operações entre models e storage

### 2. **CRUD Operations**
- **Create:** `add_task()`
- **Read:** `list_all()`, `get_by_id()`
- **Update:** `update_task()`
- **Delete:** `delete_task()`

### 3. **Iteradores e Functional Programming**
- `iter()`, `iter_mut()`
- `map()`, `filter()`, `find()`, `position()`
- `collect()`

### 4. **Error Handling**
- `io::Result<T>` para operações que podem falhar
- `ok_or_else()` para converter `Option` em `Result`
- `?` para propagação de erros

### 5. **Ownership e Borrowing**
- `&self` vs `&mut self`
- Retornar referências (`&Task`) vs valores

### 6. **Option e Result**
- `Option<T>` para valores opcionais
- `Option<Option<T>>` para nested options
- `Result<T, E>` para operações que podem falhar

---

## 💡 **Boas Práticas Demonstradas**

✅ **Encapsulamento** - Lógica de negócio centralizada  
✅ **Persistência automática** - Salva após cada modificação  
✅ **IDs únicos** - Geração automática de IDs  
✅ **Atualização parcial** - Parâmetros opcionais  
✅ **Filtros reutilizáveis** - Métodos de filtro genéricos  
✅ **Estatísticas agregadas** - Visão geral dos dados  
✅ **Testes abrangentes** - Cobertura de casos principais  
✅ **Error handling robusto** - Tratamento adequado de erros  

---

## 🚀 **Exemplo Completo de Uso**

```rust {.line-numbers}
use std::io;

fn main() -> io::Result<()> {
    let mut service = TaskService::new("data/tarefas.json".to_string());
    
    // Adicionar tarefas
    service.add_task(
        "Estudar Rust".to_string(),
        "Completar Fase 1".to_string(),
        Category::Study,
        Priority::High,
        None,
    )?;
    
    service.add_task(
        "Comprar mantimentos".to_string(),
        "Leite, pão, ovos".to_string(),
        Category::Personal,
        Priority::Medium,
        Some(Local::now().date_naive()),
    )?;
    
    // Listar todas
    println!("Todas as tarefas:");
    for tarefa in service.list_all() {
        println!("  [{}] {} - {}", 
            tarefa.id, 
            tarefa.title, 
            tarefa.status.as_str()
        );
    }
    
    // Filtrar por prioridade
    println!("\nTarefas de alta prioridade:");
    for tarefa in service.filter_by_priority(Priority::High) {
        println!("  - {}", tarefa.title);
    }
    
    // Estatísticas
    let stats = service.get_statistics();
    println!("\nEstatísticas:");
    println!("  Total: {}", stats.total);
    println!("  Pendentes: {}", stats.pending);
    println!("  Concluídas: {}", stats.completed);
    
    Ok(())
}
```

---

## 📚 **Resumo Final**

Esta struct `TaskService` demonstra **excelente arquitetura** de service layer em Rust:

1. **Gerenciamento de estado** - Mantém tarefas em memória
2. **Persistência automática** - Sincroniza com arquivo
3. **CRUD completo** - Todas operações básicas
4. **Filtros flexíveis** - Busca por múltiplos critérios
5. **Estatísticas agregadas** - Visão analítica dos dados
6. **IDs únicos** - Geração automática e segura
7. **Error handling** - Tratamento robusto de erros
8. **Testes abrangentes** - Alta cobertura de código

É um exemplo perfeito de como implementar **lógica de negócio** em Rust! 🔧✨