# 🎮 Análise Detalhada da Struct `Menu` em Rust

Vou explicar este código que implementa a **camada de apresentação** (UI/Controller) do gerenciador de tarefas, orquestrando a interação entre o usuário e o serviço de tarefas.

---

## 📦 **Imports: Dependências do Módulo**

```rust {.line-numbers}
use crate::models::{Category, Priority};
use crate::services::TaskService;
use crate::ui::{input, output};
use chrono::NaiveDate;
```

### Análise dos Imports:

| Import | O que é | Para que serve |
|--------|---------|----------------|
| `crate::models::{Category, Priority}` | Enums de domínio | Criar/filtrar tarefas |
| `crate::services::TaskService` | Camada de serviço | Lógica de negócio |
| `crate::ui::{input, output}` | Módulos de UI | Entrada/saída do usuário |
| `chrono::NaiveDate` | Tipo de data | Parsing de datas |

**Arquitetura em camadas:**
```
┌─────────────────────────────────────┐
│  Menu (Presentation Layer)         │ ← VOCÊ ESTÁ AQUI
│  - Interação com usuário           │
│  - Validação de entrada             │
│  - Formatação de saída              │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│  TaskService (Business Logic)      │
│  - CRUD de tarefas                  │
│  - Filtros e estatísticas           │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│  Storage (Data Access)              │
│  - Persistência em JSON             │
└─────────────────────────────────────┘
```

**Analogia:** `Menu` é o **garçom** que interage com o cliente, `TaskService` é a **cozinha** que prepara os pedidos, e `Storage` é o **estoque** onde os ingredientes são guardados.

---

## 🏗️ **Struct `Menu` - Controlador Principal**

```rust {.line-numbers}
pub struct Menu {
    service: TaskService,
}
```

### Estrutura:

**Campo único:**
- **`service: TaskService`** = Instância do serviço de tarefas

**Por que apenas um campo?**
- Menu é um **wrapper** ao redor do serviço
- Segue o padrão **Controller** (MVC)
- Mantém estado mínimo (apenas referência ao serviço)

**Analogia:** É como um **controle remoto** que tem apenas os botões necessários para controlar a TV (serviço).

---

## 🔧 **Método 1: `new()` - Construtor**

```rust {.line-numbers}
impl Menu {
    pub fn new(service: TaskService) -> Self {
        Menu { service }
    }
```

### Análise:

**Construtor simples:**
- Recebe `TaskService` já inicializado
- Encapsula o serviço dentro do menu

**Uso:**
```rust {.line-numbers}
let service = TaskService::new("data/tarefas.json".to_string());
let mut menu = Menu::new(service);
menu.run();
```

---

## 🔄 **Método 2: `run()` - Loop Principal**

```rust {.line-numbers}
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
```

### Análise Detalhada:

---

#### a) **Loop Infinito**

```rust {.line-numbers}
loop {
    // ...
}
```

**Por que `loop` e não `while true`?**
- `loop` é mais idiomático em Rust
- Indica intenção clara de loop infinito
- Compilador sabe que só sai com `break`

---

#### b) **Passo 1: Limpar Tela e Exibir Menu**

```rust {.line-numbers}
output::clear_screen();
self.display_main_menu();
```

- **`clear_screen()`** = Limpa a tela do terminal
- **`self.display_main_menu()`** = Exibe opções do menu

**Por que limpar a cada iteração?**
- Interface limpa e organizada
- Remove mensagens antigas
- Foco na ação atual

---

#### c) **Passo 2: Ler Opção do Usuário**

```rust {.line-numbers}
let option = input::read_option("Escolha uma opção: ", 7);
```

- **`read_option()`** = Lê número entre 0 e 7
- Retorna `Option<u32>`
- Valida automaticamente o intervalo

---

#### d) **Passo 3: Despachar Ação (Pattern Matching)**

```rust {.line-numbers}
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
```

**Padrão de design: Command Pattern**

Cada opção dispara um método específico:

| Opção | Método | Ação |
|-------|--------|------|
| 1 | `create_task()` | Criar nova tarefa |
| 2 | `list_tasks()` | Listar todas |
| 3 | `view_task()` | Ver detalhes |
| 4 | `update_task()` | Atualizar |
| 5 | `delete_task()` | Deletar |
| 6 | `filter_tasks()` | Filtrar |
| 7 | `show_statistics()` | Estatísticas |
| 0 | `break` | Sair |
| _ | Erro | Opção inválida |

**Caso especial: Opção 0 (Sair)**
```rust {.line-numbers}
Some(0) => {
    output::print_success("Até logo! 👋");
    break;
}
```
- **`break`** = Sai do loop infinito
- Termina o programa

**Caso padrão: `_` (Wildcard)**
```rust {.line-numbers}
_ => {
    output::print_error("Opção inválida!");
    input::pause();
}
```
- Captura qualquer outro valor (`None` ou número fora do intervalo)
- Exibe erro e pausa

---

### Fluxo Completo do `run()`:

```
┌─────────────────────────────────────────────────────────┐
│              FLUXO DO LOOP PRINCIPAL                    │
└─────────────────────────────────────────────────────────┘

    ┌──────────────┐
    │ Limpar tela  │
    └──────┬───────┘
           │
           ▼
    ┌──────────────┐
    │ Exibir menu  │
    └──────┬───────┘
           │
           ▼
    ┌──────────────┐
    │ Ler opção    │
    └──────┬───────┘
           │
           ▼
    ┌──────────────┐
    │ Match opção  │
    └──────┬───────┘
           │
     ┌─────┴─────┐
     │           │
     ▼           ▼
  Opção 0?    Outra opção
     │           │
     │           ▼
     │    ┌──────────────┐
     │    │ Executar ação│
     │    └──────┬───────┘
     │           │
     │           ▼
     │    ┌──────────────┐
     │    │ Voltar ao    │
     │    │ início loop  │
     │    └──────────────┘
     │
     ▼
  ┌──────────────┐
  │ break (sair) │
  └──────────────┘
```

**Analogia:** É como um **menu de restaurante** onde você escolhe um prato e o garçom executa a ação correspondente.

---

## 📋 **Método 3: `display_main_menu()` - Exibir Menu Principal**

```rust {.line-numbers}
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
```

### Análise:

**Método privado (`fn` sem `pub`):**
- Apenas usado internamente pelo `Menu`
- Não exposto para código externo

**Emojis usados:**

| Emoji | Significado | Unicode |
|-------|-------------|---------|
| ➕ | Adicionar | U+2795 |
| 📋 | Lista | U+1F4CB |
| 🔍 | Lupa | U+1F50D |
| ✏️ | Lápis | U+270F |
| 🗑️ | Lixeira | U+1F5D1 |
| 🔎 | Lupa | U+1F50E |
| 📊 | Gráfico | U+1F4CA |
| 🚪 | Porta | U+1F6AA |

**Exemplo de saída:**
```

╔══════════════════════════════════════════════════════════╗
║              GERENCIADOR DE TAREFAS                      ║
╚══════════════════════════════════════════════════════════╝

1. ➕ Criar nova tarefa
2. 📋 Listar todas as tarefas
3. 🔍 Ver detalhes de uma tarefa
4. ✏️  Atualizar tarefa
5. 🗑️  Deletar tarefa
6. 🔎 Filtrar tarefas
7. 📊 Estatísticas
0. 🚪 Sair

```

---

## ➕ **Método 4: `create_task()` - Criar Nova Tarefa**

```rust {.line-numbers}
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
```

### Análise Detalhada:

---

#### a) **Assinatura: `&mut self`**

```rust {.line-numbers}
fn create_task(&mut self)
```

**Por que `&mut self`?**
- Vai modificar o estado do serviço (adicionar tarefa)
- `TaskService::add_task()` requer `&mut self`

---

#### b) **Validação de Título**

```rust {.line-numbers}
let title = input::read_line("Título: ");
if title.is_empty() {
    output::print_error("Título não pode ser vazio!");
    input::pause();
    return;
}
```

**Padrão: Early Return**
- Valida entrada
- Se inválida, exibe erro e **retorna cedo**
- Evita aninhamento excessivo de `if`

**Alternativa sem early return (pior):**
```rust {.line-numbers}
let title = input::read_line("Título: ");
if !title.is_empty() {
    let description = input::read_line("Descrição: ");
    // ... todo o resto aninhado
} else {
    output::print_error("Título não pode ser vazio!");
    input::pause();
}
```

---

#### c) **Seleção de Categoria**

```rust {.line-numbers}
println!("\nCategorias:");
for (i, cat) in Category::all().iter().enumerate() {
    println!("{}. {}", i + 1, cat.as_str());
}
```

**Quebrando:**

**1. `Category::all()`**
- Retorna `Vec<Category>` com todas as categorias
- `[Work, Personal, Study, Health, Other]`

**2. `.iter().enumerate()`**
- **`iter()`** = Cria iterador
- **`enumerate()`** = Adiciona índice (0, 1, 2, ...)
- Retorna tuplas `(índice, &categoria)`

**3. `i + 1`**
- Exibe índice começando de 1 (mais amigável)
- Internamente usa índice 0

**Exemplo de saída:**
```
Categorias:
1. Trabalho
2. Pessoal
3. Estudos
4. Saúde
5. Outro
```

---

#### d) **Conversão de Opção para Categoria**

```rust {.line-numbers}
let cat_option = input::read_option("Escolha a categoria: ", Category::all().len() as u32);
let category = match cat_option {
    Some(n) => Category::all()[(n - 1) as usize],
    None => {
        output::print_error("Categoria inválida!");
        input::pause();
        return;
    }
};
```

**Lógica:**

**1. Ler opção (1 a 5)**
```rust {.line-numbers}
input::read_option("Escolha a categoria: ", Category::all().len() as u32)
```
- `Category::all().len()` = 5
- Aceita valores de 0 a 5

**2. Converter para índice de array**
```rust {.line-numbers}
Category::all()[(n - 1) as usize]
```
- Usuário digita: 1, 2, 3, 4, 5
- Índice do array: 0, 1, 2, 3, 4
- **`(n - 1)`** = Ajusta para índice base-0

**Exemplo:**
```rust {.line-numbers}
// Usuário escolhe: 3 (Estudos)
// n = 3
// Índice: 3 - 1 = 2
// Category::all()[2] = Study
```

**3. Tratar erro**
```rust {.line-numbers}
None => {
    output::print_error("Categoria inválida!");
    input::pause();
    return;
}
```
- Se `None` (entrada inválida), retorna cedo

---

#### e) **Parsing de Data Opcional**

```rust {.line-numbers}
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
```

**Lógica:**

**1. Se string vazia → `None`**
```rust {.line-numbers}
if due_date_str.is_empty() {
    None
}
```
- Usuário pressionou Enter sem digitar
- Tarefa sem prazo

**2. Se não vazia → tentar fazer parsing**
```rust {.line-numbers}
NaiveDate::parse_from_str(&due_date_str, "%d/%m/%Y")
```
- **`parse_from_str`** = Método de `NaiveDate` (chrono)
- **`"%d/%m/%Y"`** = Formato esperado (dia/mês/ano)
- Retorna `Result<NaiveDate, ParseError>`

**Exemplo:**
```rust {.line-numbers}
// Entrada válida
"15/01/2024" → Ok(NaiveDate::from_ymd(2024, 1, 15))

// Entradas inválidas
"2024-01-15" → Err (formato errado)
"32/01/2024" → Err (dia inválido)
"15/13/2024" → Err (mês inválido)
"abc"        → Err (não é data)
```

**3. Tratar resultado**
```rust {.line-numbers}
match ... {
    Ok(date) => Some(date),
    Err(_) => {
        output::print_error("Data inválida! Use o formato DD/MM/AAAA");
        input::pause();
        return;
    }
}
```

---

#### f) **Criar Tarefa no Serviço**

```rust {.line-numbers}
match self.service.add_task(title, description, category, priority, due_date) {
    Ok(task) => {
        output::print_success(&format!("Tarefa criada com ID: {}", task.id));
    }
    Err(e) => {
        output::print_error(&format!("Erro ao criar tarefa: {}", e));
    }
}
```

**Tratamento de erro:**
- `Ok(task)` = Sucesso, exibe ID gerado
- `Err(e)` = Falha (erro de I/O), exibe mensagem

**Macro `format!`:**
```rust {.line-numbers}
format!("Tarefa criada com ID: {}", task.id)
```
- Cria `String` formatada
- Similar a `println!`, mas retorna string em vez de imprimir

---

#### g) **Pausar Antes de Voltar**

```rust {.line-numbers}
input::pause();
```

- Aguarda usuário pressionar Enter
- Permite ler mensagem de sucesso/erro antes de voltar ao menu

---

### Fluxo Completo do `create_task()`:

```
┌─────────────────────────────────────────────────────────┐
│              FLUXO DE CRIAÇÃO DE TAREFA                 │
└─────────────────────────────────────────────────────────┘

    Limpar tela
        │
        ▼
    Ler título
        │
        ▼
    ┌──────────────┐
    │ Vazio?       │
    └──────┬───────┘
           │
      ┌────▼────┐
      │   Sim   │ → Erro e retorna
      └─────────┘
           │
      ┌────▼────┐
      │   Não   │
      └────┬────┘
           │
           ▼
    Ler descrição
        │
        ▼
    Listar categorias
        │
        ▼
    Ler opção categoria
        │
        ▼
    ┌──────────────┐
    │ Válida?      │
    └──────┬───────┘
           │
      ┌────▼────┐
      │   Não   │ → Erro e retorna
      └─────────┘
           │
      ┌────▼────┐
      │   Sim   │
      └────┬────┘
           │
           ▼
    Listar prioridades
        │
        ▼
    Ler opção prioridade
        │
        ▼
    ┌──────────────┐
    │ Válida?      │
    └──────┬───────┘
           │
      ┌────▼────┐
      │   Não   │ → Erro e retorna
      └─────────┘
           │
      ┌────▼────┐
      │   Sim   │
      └────┬────┘
           │
           ▼
    Ler data vencimento
        │
        ▼
    ┌──────────────┐
    │ Vazia?       │
    └──────┬───────┘
           │
      ┌────▼────┐
      │   Sim   │ → due_date = None
      └────┬────┘
           │
      ┌────▼────┐
      │   Não   │
      └────┬────┘
           │
           ▼
    ┌──────────────┐
    │ Parse válido?│
    └──────┬───────┘
           │
      ┌────▼────┐
      │   Não   │ → Erro e retorna
      └─────────┘
           │
      ┌────▼────┐
      │   Sim   │ → due_date = Some(date)
      └────┬────┘
           │
           ▼
    Chamar service.add_task()
        │
        ▼
    ┌──────────────┐
    │ Sucesso?     │
    └──────┬───────┘
           │
      ┌────▼────┐
      │   Sim   │ → Exibir sucesso
      └────┬────┘
           │
      ┌────▼────┐
      │   Não   │ → Exibir erro
      └────┬────┘
           │
           ▼
    Pausar
        │
        ▼
    Retornar ao menu
```

---

## 📋 **Método 5: `list_tasks()` - Listar Todas as Tarefas**

```rust {.line-numbers}
fn list_tasks(&self) {
    output::clear_screen();
    output::print_header("TODAS AS TAREFAS");

    let tasks: Vec<&crate::models::Task> = self.service.list_all().iter().collect();
    output::print_task_list(&tasks);

    input::pause();
}
```

### Análise:

---

#### a) **Conversão de Tipo**

```rust {.line-numbers}
let tasks: Vec<&crate::models::Task> = self.service.list_all().iter().collect();
```

**Por que essa conversão?**

**1. `self.service.list_all()`**
- Retorna `&[Task]` (slice de tarefas)

**2. `.iter()`**
- Cria iterador sobre referências
- Tipo: `Iterator<Item = &Task>`

**3. `.collect()`**
- Coleta em `Vec<&Task>`

**Por que não passar `list_all()` diretamente?**
- `print_task_list()` espera `&[&Task]` (slice de referências)
- `list_all()` retorna `&[Task]` (slice de tarefas)
- Precisamos converter para o tipo esperado

**Alternativa mais simples:**
```rust {.line-numbers}
let tasks = self.service.list_all();
let refs: Vec<&Task> = tasks.iter().collect();
output::print_task_list(&refs);
```

---

## 🔍 **Método 6: `view_task()` - Ver Detalhes de Tarefa**

```rust {.line-numbers}
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
```

### Análise Detalhada:

---

#### a) **Submenu de Ações**

```rust {.line-numbers}
println!("\nAções:");
println!("1. ▶️  Iniciar tarefa");
println!("2. ✅ Marcar como concluída");
println!("0. Voltar");

let option = input::read_option("\nEscolha uma ação: ", 2);
```

**Padrão: Menu aninhado**
- Após exibir tarefa, oferece ações rápidas
- Evita voltar ao menu principal para ações simples

---

#### b) **Tratamento de Erro com `if let`**

```rust {.line-numbers}
if let Err(e) = self.service.start_task(id) {
    output::print_error(&format!("Erro: {}", e));
} else {
    output::print_success("Tarefa iniciada!");
}
```

**Padrão alternativo ao `match`:**

**Com `match`:**
```rust {.line-numbers}
match self.service.start_task(id) {
    Ok(_) => output::print_success("Tarefa iniciada!"),
    Err(e) => output::print_error(&format!("Erro: {}", e)),
}
```

**Com `if let` (quando só interessa o erro):**
```rust {.line-numbers}
if let Err(e) = self.service.start_task(id) {
    output::print_error(&format!("Erro: {}", e));
} else {
    output::print_success("Tarefa iniciada!");
}
```

**Ambos são equivalentes, escolha por preferência de estilo.**

---

## ✏️ **Método 7: `update_task()` - Atualizar Tarefa**

```rust {.line-numbers}
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
```

### Análise:

---

#### a) **Verificação Prévia de Existência**

```rust {.line-numbers}
if self.service.get_by_id(id).is_none() {
    output::print_error("Tarefa não encontrada!");
    input::pause();
    return;
}
```

**Por que verificar antes?**
- Evita pedir dados ao usuário se a tarefa não existe
- Melhor UX (feedback imediato)

---

#### b) **Conversão de String Vazia para `None`**

```rust {.line-numbers}
let title = input::read_line("Novo título: ");
let title = if title.is_empty() { None } else { Some(title) };
```

**Lógica:**
- String vazia → `None` (manter valor atual)
- String não vazia → `Some(valor)` (atualizar)

**Padrão de atualização parcial:**
- Permite atualizar apenas campos específicos
- Campos não fornecidos mantêm valor original

---

## 🗑️ **Método 8: `delete_task()` - Deletar Tarefa**

```rust {.line-numbers}
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
```

### Análise:

---

#### a) **Confirmação Antes de Deletar**

```rust {.line-numbers}
if input::confirm("\nTem certeza que deseja deletar esta tarefa?") {
    // Deletar
} else {
    output::print_warning("Operação cancelada.");
}
```

**Padrão de segurança:**
- Operações destrutivas requerem confirmação
- Previne deleções acidentais
- Boa prática de UX

---

#### b) **Exibir Tarefa Antes de Deletar**

```rust {.line-numbers}
output::print_task(task);

if input::confirm("\nTem certeza que deseja deletar esta tarefa?") {
```

**Por que exibir?**
- Usuário vê o que será deletado
- Confirma que é a tarefa correta
- Evita erros

---

## 🔎 **Método 9: `filter_tasks()` - Menu de Filtros**

```rust {.line-numbers}
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
```

### Análise:

**Padrão: Submenu**
- Menu de filtros é um submenu
- Cada opção chama método específico
- Delega responsabilidade

---

## 📊 **Métodos de Filtro (10-14)**

Vou analisar um exemplo detalhado:

### Método 10: `filter_by_status()` - Filtrar por Status

```rust {.line-numbers}
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
```

### Análise:

---

#### a) **Import Local**

```rust {.line-numbers}
use crate::models::Status;
```

**Import dentro da função:**
- Válido em Rust
- Escopo limitado à função
- Útil quando tipo é usado apenas aqui

---

#### b) **Mapeamento Manual de Opção**

```rust {.line-numbers}
let status = match option {
    Some(1) => Status::Pending,
    Some(2) => Status::InProgress,
    Some(3) => Status::Completed,
    _ => { ... }
};
```

**Por que não usar `Status::all()` como em Category?**
- `Status` não tem método `all()` implementado
- Mapeamento manual é mais explícito aqui

---

### Métodos 11-13: Filtros por Categoria e Prioridade

**Idênticos ao padrão de `create_task()`:**
- Listam opções com `enumerate()`
- Convertem opção para enum
- Chamam método de filtro do serviço

---

### Método 14: `show_overdue()` - Tarefas Atrasadas

```rust {.line-numbers}
fn show_overdue(&self) {
    let tasks = self.service.get_overdue();
    output::print_task_list(&tasks);
    input::pause();
}
```

**Mais simples:**
- Não precisa de entrada do usuário
- Apenas chama serviço e exibe

---

## 📊 **Método 15: `show_statistics()` - Exibir Estatísticas**

```rust {.line-numbers}
fn show_statistics(&self) {
    output::clear_screen();
    let stats = self.service.get_statistics();
    output::print_statistics(&stats);
    input::pause();
}
```

**Método mais simples:**
- Obtém estatísticas do serviço
- Delega formatação para `output::print_statistics()`

---

## 🎯 **Conceitos-Chave Demonstrados**

### 1. **Padrão MVC (Model-View-Controller)**
- **Model:** `Task`, `Category`, `Priority`, `Status`
- **View:** Módulos `input` e `output`
- **Controller:** `Menu` (orquestra interação)

### 2. **Separation of Concerns**
- `Menu` não conhece detalhes de persistência
- `Menu` não conhece detalhes de formatação
- Cada camada tem responsabilidade única

### 3. **Early Return Pattern**
- Validações retornam cedo em caso de erro
- Evita aninhamento excessivo
- Código mais legível

### 4. **Command Pattern**
- Cada opção do menu dispara um comando
- Fácil adicionar novas opções

### 5. **Error Handling**
- `match` para tratar `Result` e `Option`
- `if let` para casos específicos
- Mensagens de erro claras

### 6. **User Experience**
- Confirmação para operações destrutivas
- Feedback visual claro (emojis, cores)
- Pausas para ler mensagens
- Validação de entrada

---

## 💡 **Boas Práticas Demonstradas**

✅ **Validação de entrada** - Verifica antes de processar  
✅ **Feedback claro** - Mensagens de sucesso/erro  
✅ **Confirmação de ações** - Operações destrutivas  
✅ **Early return** - Evita aninhamento  
✅ **Separação de responsabilidades** - Cada método tem um propósito  
✅ **Reutilização** - Métodos de `input` e `output`  
✅ **Tratamento de erros** - Nunca ignora `Result`  
✅ **UX consistente** - Padrões de interação uniformes  

---

## 🚀 **Exemplo de Uso Completo**

```rust {.line-numbers}
// src/main.rs
mod models;
mod services;
mod storage;
mod ui;

use services::TaskService;
use ui::menu::Menu;

fn main() {
    let service = TaskService::new("data/tarefas.json".to_string());
    let mut menu = Menu::new(service);
    menu.run();
}
```

---

## 🔍 **Possíveis Melhorias**

### 1. **Adicionar Busca por Texto**

```rust {.line-numbers}
fn search_tasks(&self) {
    let query = input::read_line("Buscar por: ");
    let tasks: Vec<&Task> = self.service
        .list_all()
        .iter()
        .filter(|t| t.title.contains(&query) || t.description.contains(&query))
        .collect();
    output::print_task_list(&tasks);
    input::pause();
}
```

### 2. **Adicionar Ordenação**

```rust {.line-numbers}
fn sort_tasks(&self) {
    println!("1. Por ID");
    println!("2. Por Título");
    println!("3. Por Prioridade");
    
    let option = input::read_option("Ordenar por: ", 3);
    // Implementar ordenação
}
```

### 3. **Adicionar Exportação**

```rust {.line-numbers}
fn export_tasks(&self) {
    let path = input::read_line("Caminho do arquivo: ");
    match export_to_csv(self.service.list_all(), &path) {
        Ok(_) => output::print_success("Exportado com sucesso!"),
        Err(e) => output::print_error(&format!("Erro: {}", e)),
    }
}
```

### 4. **Adicionar Histórico de Ações**

```rust {.line-numbers}
struct Menu {
    service: TaskService,
    history: Vec<String>,
}

impl Menu {
    fn log_action(&mut self, action: &str) {
        self.history.push(format!("{}: {}", Local::now(), action));
    }
}
```

### 5. **Adicionar Atalhos de Teclado**

```rust {.line-numbers}
// Usar biblioteca como `crossterm` para capturar teclas
fn run(&mut self) {
    loop {
        match read_key() {
            Key::Char('n') => self.create_task(),
            Key::Char('l') => self.list_tasks(),
            Key::Char('q') => break,
            // ...
        }
    }
}
```

---

## 📚 **Resumo Final**

Esta struct `Menu` demonstra **excelente arquitetura** de UI para CLI em Rust:

1. **Separação de responsabilidades** - Controller puro
2. **Validação robusta** - Entrada sempre validada
3. **UX polida** - Feedback claro e confirmações
4. **Error handling** - Tratamento adequado de erros
5. **Modularidade** - Fácil adicionar novas funcionalidades
6. **Manutenibilidade** - Código organizado e legível
7. **Padrões de design** - MVC, Command, Early Return

É um exemplo perfeito de como implementar **interface de usuário** para aplicações CLI em Rust! 🎮✨