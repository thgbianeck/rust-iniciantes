# 📋 Análise Detalhada da Struct `Task` em Rust

Vou explicar este código que implementa a estrutura principal de uma tarefa (Task) em um sistema de gerenciamento, incluindo seus métodos e testes automatizados.

---

## 📦 **Imports: Bibliotecas Necessárias**

```rust {.line-numbers}
use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};

use super::enums::{Category, Priority, Status};
```

### 1️⃣ **Chrono - Manipulação de Datas e Horários**

```rust {.line-numbers}
use chrono::{DateTime, Local, NaiveDate};
```

**O que é Chrono?**
- Biblioteca mais popular de Rust para trabalhar com datas e horários
- Similar ao `java.time` (Java) ou `datetime` (Python)

**Tipos importados:**

| Tipo | Descrição | Exemplo |
|------|-----------|---------|
| `DateTime<Local>` | Data + Hora com timezone local | `2024-01-15 14:30:45 -03:00` |
| `NaiveDate` | Apenas data (sem hora, sem timezone) | `2024-01-15` |
| `Local` | Timezone do sistema | Fuso horário da máquina |

**Analogia:**
- **`DateTime<Local>`** = Relógio de parede completo (hora, minuto, segundo, fuso horário)
- **`NaiveDate`** = Calendário simples (apenas dia, mês, ano)

---

### 2️⃣ **Serde - Serialização**

```rust {.line-numbers}
use serde::{Deserialize, Serialize};
```

Já vimos antes: permite converter `Task` para/de JSON, YAML, etc.

---

### 3️⃣ **Imports Relativos - Enums do Projeto**

```rust {.line-numbers}
use super::enums::{Category, Priority, Status};
```

**O que significa `super`?**
- **`super`** = Módulo pai (um nível acima)
- **`super::enums`** = Vai para o módulo pai e acessa `enums`

**Estrutura de diretórios:**
```
src/models/
├── mod.rs          ← Declara os submódulos
├── task.rs         ← VOCÊ ESTÁ AQUI
└── enums.rs        ← super::enums aponta para aqui
```

**Analogia do prédio:**
```
🏢 5º ANDAR - MODELS (super)
   ├── 🗂️ SALA "task" (você está aqui)
   └── 🗂️ SALA "enums" (super::enums)
```

**Alternativas de import:**

```rust {.line-numbers}
use super::enums::{Category, Priority, Status};  // ✅ Relativo (módulo pai)
use crate::models::enums::{Category, Priority, Status};  // ✅ Absoluto (da raiz)
```

---

## 🏗️ **Struct `Task` - Estrutura de Dados**

```rust {.line-numbers}
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
```

### 🎨 **Traits Derivados**

| Trait | Por que está aqui? |
|-------|-------------------|
| `Debug` | Debugging e logs |
| `Clone` | Criar cópias da tarefa |
| `Serialize` | Salvar em JSON/arquivo |
| `Deserialize` | Carregar de JSON/arquivo |

**⚠️ Nota:** `Task` **NÃO** implementa `Copy` porque contém `String` (dados no heap).

---

### 📊 **Campos da Struct**

Vamos analisar cada campo em detalhes:

#### 1️⃣ **`id: u32`** - Identificador Único

```rust {.line-numbers}
pub id: u32,
```

- **Tipo:** `u32` = Unsigned integer de 32 bits (0 a 4.294.967.295)
- **Uso:** Identificador único da tarefa
- **Por que `u32`?** IDs nunca são negativos

**Analogia:** É como o **número de protocolo** de um documento.

---

#### 2️⃣ **`title: String`** - Título da Tarefa

```rust {.line-numbers}
pub title: String,
```

- **Tipo:** `String` = String alocada no heap (tamanho dinâmico)
- **Uso:** Nome curto e descritivo da tarefa
- **Exemplo:** "Estudar Rust", "Comprar mantimentos"

**Analogia:** É o **assunto** de um email.

---

#### 3️⃣ **`description: String`** - Descrição Detalhada

```rust {.line-numbers}
pub description: String,
```

- **Tipo:** `String`
- **Uso:** Detalhes adicionais sobre a tarefa
- **Exemplo:** "Completar os capítulos 1-5 do livro de Rust"

**Analogia:** É o **corpo** de um email.

---

#### 4️⃣ **`category: Category`** - Categoria

```rust {.line-numbers}
pub category: Category,
```

- **Tipo:** `Category` enum (Work, Personal, Study, Health, Other)
- **Uso:** Classificar a tarefa
- **Implementa `Copy`:** Cópia automática (não move)

**Analogia:** É a **pasta** onde você arquiva o documento.

---

#### 5️⃣ **`priority: Priority`** - Prioridade

```rust {.line-numbers}
pub priority: Priority,
```

- **Tipo:** `Priority` enum (High, Medium, Low)
- **Uso:** Indicar urgência/importância
- **Implementa `Copy`:** Cópia automática

**Analogia:** É a **etiqueta colorida** (vermelha = urgente, verde = pode esperar).

---

#### 6️⃣ **`status: Status`** - Status Atual

```rust {.line-numbers}
pub status: Status,
```

- **Tipo:** `Status` enum (Pending, InProgress, Completed)
- **Uso:** Rastrear progresso da tarefa
- **Implementa `Copy`:** Cópia automática

**Analogia:** É o **carimbo** no documento (Pendente, Em Análise, Aprovado).

---

#### 7️⃣ **`due_date: Option<NaiveDate>`** - Data de Vencimento (Opcional)

```rust {.line-numbers}
pub due_date: Option<NaiveDate>,
```

**Análise detalhada:**

**a) `Option<T>` - Tipo Opcional**

`Option` é um enum que pode ser:
- **`Some(valor)`** - Contém um valor
- **`None`** - Não contém valor (equivalente a `null` em outras linguagens)

```rust {.line-numbers}
enum Option<T> {
    Some(T),
    None,
}
```

**Por que usar `Option`?**
- Nem toda tarefa tem prazo
- Rust **não tem `null`** - usa `Option` para valores opcionais
- Compilador **força** você a tratar ambos os casos

**Exemplos:**

```rust {.line-numbers}
// Tarefa COM prazo
let tarefa1 = Task {
    due_date: Some(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
    // ...
};

// Tarefa SEM prazo
let tarefa2 = Task {
    due_date: None,
    // ...
};

// Acessando o valor
match tarefa1.due_date {
    Some(data) => println!("Vence em: {}", data),
    None => println!("Sem prazo definido"),
}
```

**b) `NaiveDate` - Data Sem Timezone**

- **"Naive"** = Ingênua (não sabe sobre fusos horários)
- Armazena apenas: ano, mês, dia
- Perfeito para prazos (não importa a hora exata)

**Analogia:**
- **`Option<NaiveDate>`** = Caixa que **pode ou não** conter um calendário
- Você precisa **abrir a caixa** (`match` ou `if let`) para ver se tem algo dentro

---

#### 8️⃣ **`created_at: DateTime<Local>`** - Data/Hora de Criação

```rust {.line-numbers}
pub created_at: DateTime<Local>,
```

- **Tipo:** `DateTime<Local>` = Data + Hora com timezone local
- **Uso:** Registrar quando a tarefa foi criada
- **Sempre preenchido:** Não é `Option` (toda tarefa tem data de criação)

**Exemplo:**
```rust {.line-numbers}
created_at: 2024-01-15T14:30:45.123456789-03:00
            ↑ Data    ↑ Hora          ↑ Timezone
```

**Analogia:** É o **carimbo de data/hora** automático quando você protocola um documento.

---

#### 9️⃣ **`completed_at: Option<DateTime<Local>>`** - Data/Hora de Conclusão (Opcional)

```rust {.line-numbers}
pub completed_at: Option<DateTime<Local>>,
```

- **Tipo:** `Option<DateTime<Local>>`
- **Uso:** Registrar quando a tarefa foi concluída
- **Opcional:** `None` enquanto não concluída, `Some(data)` quando concluída

**Fluxo de vida:**

```rust {.line-numbers}
// Tarefa criada
let mut task = Task::new(...);
assert_eq!(task.completed_at, None);  // ✅ Ainda não concluída

// Tarefa concluída
task.complete();
assert!(task.completed_at.is_some());  // ✅ Agora tem data de conclusão
```

**Analogia:** É o **carimbo de "CONCLUÍDO"** que só é aplicado quando você termina o trabalho.

---

## 🔧 **Implementação de Métodos: `impl Task`**

```rust {.line-numbers}
impl Task {
    // Métodos aqui
}
```

### Método 1: `new()` - Construtor

```rust {.line-numbers}
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
```

**Análise detalhada:**

#### a) **Comentário de Documentação**

```rust {.line-numbers}
/// Cria uma nova tarefa
```

- **`///`** = Comentário de documentação (doc comment)
- Aparece na documentação gerada por `cargo doc`
- Diferente de `//` (comentário normal)

**Gerando documentação:**
```bash
cargo doc --open
```

---

#### b) **Assinatura da Função**

```rust {.line-numbers}
pub fn new(
    id: u32,
    title: String,
    description: String,
    category: Category,
    priority: Priority,
    due_date: Option<NaiveDate>,
) -> Self
```

**Características:**
- **`pub fn`** = Função pública (pode ser chamada de fora)
- **`new`** = Convenção para construtores em Rust
- **Parâmetros:** Dados fornecidos pelo usuário
- **`-> Self`** = Retorna uma instância de `Task` (`Self` é um alias para `Task`)

**Por que `Self` em vez de `Task`?**
- Mais genérico e reutilizável
- Se você renomear `Task`, não precisa mudar `Self`

---

#### c) **Corpo da Função - Inicialização de Campos**

```rust {.line-numbers}
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
```

**Sintaxe de inicialização de struct:**

**Forma longa (explícita):**
```rust {.line-numbers}
Task {
    id: id,
    title: title,
    description: description,
    // ...
}
```

**Forma curta (field init shorthand):**
```rust {.line-numbers}
Task {
    id,        // Equivalente a: id: id
    title,     // Equivalente a: title: title
    // ...
}
```

**Quando o nome do campo é igual ao nome da variável, você pode omitir a repetição!**

---

#### d) **Campos com Valores Padrão**

```rust {.line-numbers}
status: Status::Pending,      // ← Sempre começa como Pendente
created_at: Local::now(),     // ← Timestamp atual
completed_at: None,           // ← Ainda não concluída
```

**Por que esses campos não são parâmetros?**
- **`status`:** Toda tarefa nova começa como `Pending` (lógica de negócio)
- **`created_at`:** É sempre "agora" (não faz sentido o usuário escolher)
- **`completed_at`:** Sempre `None` no início (tarefa não está concluída)

**Analogia:** É como um formulário onde alguns campos são **preenchidos automaticamente**.

---

#### e) **`Local::now()` - Timestamp Atual**

```rust {.line-numbers}
created_at: Local::now(),
```

- **`Local`** = Timezone do sistema
- **`now()`** = Momento atual
- Retorna `DateTime<Local>` com data, hora e timezone

**Exemplo de valor:**
```
2024-01-15T14:30:45.123456789-03:00
```

---

#### f) **Uso do Construtor**

```rust {.line-numbers}
let tarefa = Task::new(
    1,
    "Estudar Rust".to_string(),
    "Completar Fase 1".to_string(),
    Category::Study,
    Priority::High,
    Some(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
);

println!("Tarefa criada: {:?}", tarefa);
```

---

### Método 2: `complete()` - Marcar como Concluída

```rust {.line-numbers}
/// Marca a tarefa como concluída
pub fn complete(&mut self) {
    self.status = Status::Completed;
    self.completed_at = Some(Local::now());
}
```

**Análise detalhada:**

#### a) **`&mut self` - Referência Mutável**

```rust {.line-numbers}
pub fn complete(&mut self)
```

**O que significa `&mut self`?**
- **`&`** = Referência (empresta, não consome)
- **`mut`** = Mutável (pode modificar)
- **`self`** = A instância atual de `Task`

**Comparação:**

| Assinatura | Significado | Exemplo |
|------------|-------------|---------|
| `fn foo(self)` | Consome o valor | `task.foo()` - task não pode ser usado depois |
| `fn foo(&self)` | Empresta imutável | `task.foo()` - task pode ser usado depois, mas não modificado |
| `fn foo(&mut self)` | Empresta mutável | `task.foo()` - task pode ser usado e modificado depois |

**Analogia:**
- **`self`** = Você **doa** o documento (não pode mais usá-lo)
- **`&self`** = Você **empresta** o documento para leitura (pode pedir de volta)
- **`&mut self`** = Você **empresta** o documento para edição (pode pedir de volta modificado)

---

#### b) **Modificando Campos**

```rust {.line-numbers}
self.status = Status::Completed;
self.completed_at = Some(Local::now());
```

**O que acontece:**
1. Muda o status para `Completed`
2. Registra o timestamp de conclusão

**Uso:**

```rust {.line-numbers}
let mut tarefa = Task::new(...);
println!("Status: {:?}", tarefa.status);  // Pending

tarefa.complete();  // ← Modifica a tarefa
println!("Status: {:?}", tarefa.status);  // Completed
println!("Concluída em: {:?}", tarefa.completed_at);  // Some(2024-01-15...)
```

**⚠️ Importante:** A variável precisa ser `mut` para chamar métodos `&mut self`:

```rust {.line-numbers}
let tarefa = Task::new(...);  // ❌ Não é mutável
tarefa.complete();  // ❌ ERRO: cannot borrow as mutable

let mut tarefa = Task::new(...);  // ✅ Mutável
tarefa.complete();  // ✅ Funciona!
```

---

### Método 3: `is_overdue()` - Verificar se Está Atrasada

```rust {.line-numbers}
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
```

**Análise detalhada:**

#### a) **`&self` - Referência Imutável**

```rust {.line-numbers}
pub fn is_overdue(&self) -> bool
```

- Apenas **lê** dados, não modifica
- Retorna `bool` (true = atrasada, false = não atrasada)

---

#### b) **`if let` - Pattern Matching Simplificado**

```rust {.line-numbers}
if let Some(due_date) = self.due_date {
    // due_date está disponível aqui
}
```

**O que faz:**
- Tenta extrair o valor de `Option`
- Se for `Some(valor)`, entra no bloco e `due_date` contém o valor
- Se for `None`, pula o bloco

**Equivalente com `match`:**

```rust {.line-numbers}
match self.due_date {
    Some(due_date) => {
        // Código aqui
    }
    None => {
        // Não faz nada
    }
}
```

**`if let` é mais conciso quando você só se importa com um caso!**

**Analogia:**
```
if let = "Se a caixa tiver algo dentro, abra e use o conteúdo"
```

---

#### c) **Lógica de Verificação**

```rust {.line-numbers}
if self.status != Status::Completed {
    let today = Local::now().date_naive();
    return due_date < today;
}
```

**Passo a passo:**

1. **Verifica se NÃO está concluída**
   - Tarefas concluídas nunca estão "atrasadas"

2. **Obtém a data de hoje**
   ```rust {.line-numbers}
   let today = Local::now().date_naive();
   ```
   - `Local::now()` = DateTime completo (2024-01-15T14:30:45...)
   - `.date_naive()` = Extrai apenas a data (2024-01-15)

3. **Compara datas**
   ```rust {.line-numbers}
   return due_date < today;
   ```
   - Se `due_date` (prazo) é **anterior** a `today` (hoje) → atrasada!

**Fluxo completo:**

```
┌─────────────────────────────────┐
│ Tarefa tem prazo (due_date)?    │
└────────┬────────────────────────┘
         │
    ┌────▼────┐
    │   Não   │ → Retorna false (não está atrasada)
    └─────────┘
         │
    ┌────▼────┐
    │   Sim   │
    └────┬────┘
         │
┌────────▼────────────────────────┐
│ Tarefa está concluída?          │
└────────┬────────────────────────┘
         │
    ┌────▼────┐
    │   Sim   │ → Retorna false (concluída não atrasa)
    └─────────┘
         │
    ┌────▼────┐
    │   Não   │
    └────┬────┘
         │
┌────────▼────────────────────────┐
│ Prazo < Hoje?                   │
└────────┬────────────────────────┘
         │
    ┌────▼────┐
    │   Sim   │ → Retorna true (ATRASADA!)
    └─────────┘
         │
    ┌────▼────┐
    │   Não   │ → Retorna false (no prazo)
    └─────────┘
```

---

#### d) **Retorno Padrão**

```rust {.line-numbers}
false
```

Se nenhuma das condições anteriores for verdadeira, retorna `false`.

**Casos que chegam aqui:**
- Tarefa sem prazo (`due_date = None`)
- Tarefa concluída
- Prazo ainda não venceu

---

#### e) **Uso do Método**

```rust {.line-numbers}
let ontem = Local::now().date_naive() - Duration::days(1);
let tarefa = Task::new(
    1,
    "Tarefa Atrasada".to_string(),
    "Descrição".to_string(),
    Category::Work,
    Priority::High,
    Some(ontem),  // ← Prazo era ontem!
);

if tarefa.is_overdue() {
    println!("⚠️ ATENÇÃO: Tarefa atrasada!");
}
```

---

### Método 4: `start()` - Iniciar Tarefa

```rust {.line-numbers}
/// Inicia a tarefa (muda status para InProgress)
pub fn start(&mut self) {
    if self.status == Status::Pending {
        self.status = Status::InProgress;
    }
}
```

**Análise detalhada:**

#### a) **Validação de Estado**

```rust {.line-numbers}
if self.status == Status::Pending {
    self.status = Status::InProgress;
}
```

**Lógica:**
- Só muda para `InProgress` se estiver `Pending`
- Impede transições inválidas (ex: `Completed` → `InProgress`)

**Máquina de estados:**

```
Pending ──start()──> InProgress ──complete()──> Completed
   ↑                                                  │
   └──────────────── (não permitido) ────────────────┘
```

**Uso:**

```rust {.line-numbers}
let mut tarefa = Task::new(...);
println!("Status: {:?}", tarefa.status);  // Pending

tarefa.start();
println!("Status: {:?}", tarefa.status);  // InProgress

tarefa.start();  // ← Não faz nada (já está InProgress)
println!("Status: {:?}", tarefa.status);  // InProgress
```

---

## 🧪 **Testes Automatizados: `#[cfg(test)]`**

```rust {.line-numbers}
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    // Testes aqui
}
```

### O que é `#[cfg(test)]`?

**Compilação Condicional:**
- **`#[cfg(test)]`** = "Compile este código APENAS quando rodar testes"
- Não é incluído no binário final (economiza espaço)

**Executando testes:**
```bash
cargo test
```

---

### `mod tests` - Módulo de Testes

```rust {.line-numbers}
mod tests {
    use super::*;
    // ...
}
```

**Por que um módulo separado?**
- Organização: mantém testes isolados
- Namespace: evita conflitos de nomes

---

### `use super::*;` - Importar Tudo do Módulo Pai

```rust {.line-numbers}
use super::*;
```

- **`super`** = Módulo pai (onde `Task` está definida)
- **`*`** = Importa tudo (Task, Category, Priority, Status, etc.)

**Analogia:** "Traga tudo da sala ao lado para cá"

---

### Teste 1: `test_new_task` - Criação de Tarefa

```rust {.line-numbers}
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
```

**Análise:**

#### a) **`#[test]` - Marca como Teste**

```rust {.line-numbers}
#[test]
fn test_new_task() { ... }
```

- Indica que esta função é um teste
- `cargo test` executa automaticamente

---

#### b) **Assertions - Verificações**

**`assert_eq!(a, b)` - Verifica Igualdade**

```rust {.line-numbers}
assert_eq!(task.id, 1);
assert_eq!(task.title, "Estudar Rust");
assert_eq!(task.status, Status::Pending);
```

- Se `a == b` → Teste passa ✅
- Se `a != b` → Teste falha ❌ e mostra a diferença

**`assert!(condição)` - Verifica Condição Booleana**

```rust {.line-numbers}
assert!(task.completed_at.is_none());
```

- Se `condição == true` → Teste passa ✅
- Se `condição == false` → Teste falha ❌

**Método `.is_none()`:**
```rust {.line-numbers}
task.completed_at.is_none()  // true se for None, false se for Some
```

---

#### c) **O que Este Teste Verifica?**

✅ ID é atribuído corretamente  
✅ Título é armazenado corretamente  
✅ Status inicial é `Pending`  
✅ `completed_at` é `None` (não concluída)  

---

### Teste 2: `test_complete_task` - Conclusão de Tarefa

```rust {.line-numbers}
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
```

**O que Este Teste Verifica?**

✅ Método `complete()` muda status para `Completed`  
✅ `completed_at` é preenchido (`Some`)  

**Método `.is_some()`:**
```rust {.line-numbers}
task.completed_at.is_some()  // true se for Some, false se for None
```

---

### Teste 3: `test_is_overdue` - Tarefa Atrasada

```rust {.line-numbers}
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
```

**Análise:**

#### a) **Criando Data no Passado**

```rust {.line-numbers}
let yesterday = Local::now().date_naive() - Duration::days(1);
```

- `Local::now().date_naive()` = Data de hoje
- `Duration::days(1)` = Duração de 1 dia
- `hoje - 1 dia` = Ontem

**Exemplo:**
```rust {.line-numbers}
// Se hoje é 2024-01-15
let yesterday = Local::now().date_naive() - Duration::days(1);
// yesterday = 2024-01-14
```

---

#### b) **Verificação**

```rust {.line-numbers}
assert!(task.is_overdue());
```

**O que Este Teste Verifica?**

✅ Tarefa com prazo no passado é detectada como atrasada  

---

### Teste 4: `test_not_overdue_when_completed` - Concluída Não Atrasa

```rust {.line-numbers}
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
```

**Análise:**

```rust {.line-numbers}
assert!(!task.is_overdue());
//      ↑ Negação: verifica que é FALSE
```

**O que Este Teste Verifica?**

✅ Tarefa concluída **NÃO** é considerada atrasada (mesmo com prazo vencido)  

**Lógica de negócio importante:** Tarefas concluídas não podem estar "atrasadas"!

---

### Teste 5: `test_start_task` - Iniciar Tarefa

```rust {.line-numbers}
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
```

**O que Este Teste Verifica?**

✅ Método `start()` muda status de `Pending` para `InProgress`  

---

## 🎯 **Executando os Testes**

```bash
# Executar todos os testes
cargo test

# Executar teste específico
cargo test test_new_task

# Executar com output detalhado
cargo test -- --nocapture

# Executar com threads (paralelização)
cargo test -- --test-threads=1
```

**Saída esperada:**

```
running 5 tests
test tests::test_new_task ... ok
test tests::test_complete_task ... ok
test tests::test_is_overdue ... ok
test tests::test_not_overdue_when_completed ... ok
test tests::test_start_task ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 📊 **Diagrama de Fluxo de Estados**

```
┌─────────────────────────────────────────────────────────┐
│                    CICLO DE VIDA DA TAREFA              │
└─────────────────────────────────────────────────────────┘

    Task::new()
        │
        ▼
   ┌─────────┐
   │ Pending │ ◄─── Status inicial
   └────┬────┘
        │
        │ .start()
        ▼
┌──────────────┐
│  InProgress  │ ◄─── Tarefa em andamento
└──────┬───────┘
       │
       │ .complete()
       ▼
  ┌───────────┐
  │ Completed │ ◄─── Estado final
  └───────────┘
       │
       │ completed_at = Some(now)
       ▼
   [FIM]
```

---

## 🔍 **Conceitos-Chave Demonstrados**

### 1. **Ownership e Borrowing**
- `&self` - Empresta imutável
- `&mut self` - Empresta mutável
- `self` - Consome o valor

### 2. **Option<T> - Valores Opcionais**
- `Some(valor)` - Contém valor
- `None` - Sem valor
- `if let` - Pattern matching simplificado

### 3. **Datas e Horários (Chrono)**
- `DateTime<Local>` - Timestamp completo
- `NaiveDate` - Apenas data
- `Duration` - Intervalo de tempo

### 4. **Métodos de Instância vs Associados**
- `Task::new()` - Função associada (construtor)
- `task.complete()` - Método de instância

### 5. **Testes Automatizados**
- `#[cfg(test)]` - Compilação condicional
- `#[test]` - Marca função como teste
- `assert_eq!` / `assert!` - Verificações

### 6. **Documentação**
- `///` - Doc comments
- `cargo doc` - Gera documentação HTML

---

## 💡 **Boas Práticas Demonstradas**

✅ **Construtor `new()`** - Padrão para criar instâncias  
✅ **Valores padrão sensatos** - Status inicial, timestamps automáticos  
✅ **Validação de estado** - `start()` só funciona se `Pending`  
✅ **Métodos descritivos** - `is_overdue()`, `complete()`, `start()`  
✅ **Testes abrangentes** - Cobertura de casos principais  
✅ **Documentação** - Doc comments em métodos públicos  
✅ **Imutabilidade por padrão** - `&self` quando possível  
✅ **Option para valores opcionais** - Sem `null`, type safety  

---

## 🚀 **Possíveis Melhorias**

### 1. **Adicionar Validação**

```rust {.line-numbers}
impl Task {
    pub fn new(...) -> Result<Self, String> {
        if title.trim().is_empty() {
            return Err("Título não pode ser vazio".to_string());
        }
        
        if description.len() > 500 {
            return Err("Descrição muito longa".to_string());
        }
        
        Ok(Task { ... })
    }
}
```

### 2. **Adicionar Método `pause()`**

```rust {.line-numbers}
impl Task {
    pub fn pause(&mut self) {
        if self.status == Status::InProgress {
            self.status = Status::Pending;
        }
    }
}
```

### 3. **Adicionar Método `days_until_due()`**

```rust {.line-numbers}
impl Task {
    pub fn days_until_due(&self) -> Option<i64> {
        self.due_date.map(|due| {
            let today = Local::now().date_naive();
            (due - today).num_days()
        })
    }
}
```

### 4. **Adicionar Método `duration()`**

```rust {.line-numbers}
impl Task {
    pub fn duration(&self) -> Option<Duration> {
        self.completed_at.map(|completed| {
            completed.signed_duration_since(self.created_at)
        })
    }
}
```

### 5. **Implementar `Display` Trait**

```rust {.line-numbers}
use std::fmt;

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}] {} - {} ({})",
            self.id,
            self.title,
            self.status.as_str(),
            self.priority.as_str()
        )
    }
}

// Uso:
println!("{}", task);  // [1] Estudar Rust - Pendente (Alta)
```

---

## 📚 **Resumo Final**

Esta struct `Task` demonstra **excelente design** em Rust:

1. **Modelagem de domínio** - Campos bem definidos
2. **Type safety** - Uso de enums e Option
3. **Encapsulamento** - Métodos controlam mudanças de estado
4. **Testabilidade** - Testes automatizados abrangentes
5. **Documentação** - Doc comments claros
6. **Serialização** - Integração com Serde
7. **Timestamps** - Rastreamento de datas importantes

É um exemplo perfeito de como estruturar dados e comportamento em Rust! 🦀✨