# 🎨 Análise Detalhada do Módulo `output` em Rust

Vou explicar este código que implementa **funções de formatação e exibição** para criar uma interface de usuário (UI) bonita e organizada em aplicações CLI (Command Line Interface).

---

## 📦 **Imports: Dependências do Módulo**

```rust {.line-numbers}
use crate::models::Task;
use crate::services::Statistics;
```

### Análise dos Imports:

| Import | O que é | Para que serve |
|--------|---------|----------------|
| `crate::models::Task` | Struct de tarefa | Exibir informações de tarefas |
| `crate::services::Statistics` | Struct de estatísticas | Exibir dados agregados |

---

## 🧹 **Função 1: `clear_screen()` - Limpar a Tela**

```rust {.line-numbers}
/// Limpa a tela
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
```

### Análise Detalhada:

---

#### a) **Códigos de Escape ANSI**

```rust {.line-numbers}
print!("\x1B[2J\x1B[1;1H");
```

**O que são códigos de escape ANSI?**
- Sequências especiais que controlam o terminal
- Começam com `\x1B` (ESC em hexadecimal = 27 em decimal)
- Seguidos por comandos específicos

**Quebrando a string:**

| Código | O que faz |
|--------|-----------|
| `\x1B` | Caractere ESC (escape) |
| `[2J` | Limpar toda a tela |
| `\x1B` | Caractere ESC novamente |
| `[1;1H` | Mover cursor para posição (1,1) - topo esquerdo |

**Alternativas:**

```rust {.line-numbers}
// Usando biblioteca crossterm (mais portável)
use crossterm::{execute, terminal};
execute!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();

// Usando comando do sistema (menos portável)
#[cfg(target_os = "windows")]
std::process::Command::new("cmd").args(&["/C", "cls"]).status().unwrap();

#[cfg(not(target_os = "windows"))]
std::process::Command::new("clear").status().unwrap();
```

**Por que usar códigos ANSI?**
- ✅ Rápido (não cria processo externo)
- ✅ Funciona na maioria dos terminais modernos
- ❌ Pode não funcionar em terminais muito antigos

**Uso:**
```rust {.line-numbers}
clear_screen();
println!("Tela limpa!");
```

**Analogia:** É como **apagar o quadro negro** antes de escrever algo novo.

---

## 📋 **Função 2: `print_header()` - Exibir Cabeçalho**

```rust {.line-numbers}
/// Exibe o cabeçalho
pub fn print_header(title: &str) {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║  {:^54}  ║", title);
    println!("╚══════════════════════════════════════════════════════════╝\n");
}
```

### Análise Detalhada:

---

#### a) **Caracteres de Desenho de Caixa (Box Drawing)**

```rust {.line-numbers}
╔══════════════════════════════════════════════════════════╗
║  {:^54}  ║
╚══════════════════════════════════════════════════════════╝
```

**Caracteres Unicode usados:**

| Caractere | Nome | Código Unicode |
|-----------|------|----------------|
| `╔` | Box Drawings Double Down and Right | U+2554 |
| `═` | Box Drawings Double Horizontal | U+2550 |
| `╗` | Box Drawings Double Down and Left | U+2557 |
| `║` | Box Drawings Double Vertical | U+2551 |
| `╚` | Box Drawings Double Up and Right | U+255A |
| `╝` | Box Drawings Double Up and Left | U+255D |

**Estrutura visual:**
```
╔══════════════════════════════════════════════════════════╗  ← Topo
║  TÍTULO CENTRALIZADO AQUI                                ║  ← Meio
╚══════════════════════════════════════════════════════════╝  ← Base
```

---

#### b) **Formatação com `{:^54}`**

```rust {.line-numbers}
println!("║  {:^54}  ║", title);
```

**O que significa `{:^54}`?**

- **`:`** = Início da especificação de formato
- **`^`** = Centralizar (align center)
- **`54`** = Largura total (padding)

**Alternativas de alinhamento:**


**Exemplo:**
```rust {.line-numbers}
let title = "MENU PRINCIPAL";

println!("{:^54}", title);
// Saída: "                    MENU PRINCIPAL                    "
//         ↑ 20 espaços       ↑ 14 chars      ↑ 20 espaços

println!("{:<54}", title);
// Saída: "MENU PRINCIPAL                                        "

println!("{:>54}", title);
// Saída: "                                        MENU PRINCIPAL"
```

---

#### c) **Cálculo da Largura**

**Por que 54?**

```
╔══════════════════════════════════════════════════════════╗
║  {:^54}  ║
   ↑      ↑
   2      54 = 56 caracteres internos
   espaços

Total da linha: 1 (║) + 2 (espaços) + 54 (conteúdo) + 2 (espaços) + 1 (║) = 60
```

**Contando os `═`:**
- Linha superior tem 58 `═` entre `╔` e `╗`
- Total: 1 + 58 + 1 = 60 caracteres

---

### Uso do `print_header()`:

```rust {.line-numbers}
print_header("GERENCIADOR DE TAREFAS");

// Saída:
// 
// ╔══════════════════════════════════════════════════════════╗
// ║              GERENCIADOR DE TAREFAS                      ║
// ╚══════════════════════════════════════════════════════════╝
//
```

**Analogia:** É como criar um **banner** ou **título de seção** destacado.

---

## ✅ **Funções 3-5: Mensagens de Status**

### Função 3: `print_success()` - Mensagem de Sucesso

```rust {.line-numbers}
/// Exibe uma mensagem de sucesso
pub fn print_success(message: &str) {
    println!("\n✅ {}", message);
}
```

**Emoji usado:** ✅ (U+2705 - White Heavy Check Mark)

**Uso:**
```rust {.line-numbers}
print_success("Tarefa adicionada com sucesso!");
// Saída:
// 
// ✅ Tarefa adicionada com sucesso!
```

---

### Função 4: `print_error()` - Mensagem de Erro

```rust {.line-numbers}
/// Exibe uma mensagem de erro
pub fn print_error(message: &str) {
    println!("\n❌ Erro: {}", message);
}
```

**Emoji usado:** ❌ (U+274C - Cross Mark)

**Uso:**
```rust {.line-numbers}
print_error("Tarefa não encontrada");
// Saída:
// 
// ❌ Erro: Tarefa não encontrada
```

---

### Função 5: `print_warning()` - Mensagem de Aviso

```rust {.line-numbers}
/// Exibe uma mensagem de aviso
pub fn print_warning(message: &str) {
    println!("\n⚠️  {}", message);
}
```

**Emoji usado:** ⚠️ (U+26A0 - Warning Sign)

**Uso:**
```rust {.line-numbers}
print_warning("Nenhuma tarefa encontrada.");
// Saída:
// 
// ⚠️  Nenhuma tarefa encontrada.
```

---

### **Por que usar emojis?**

✅ **Vantagens:**
- Feedback visual imediato
- Universalmente reconhecidos
- Não dependem de cores (funciona em terminais sem suporte a cores)

❌ **Desvantagens:**
- Podem não renderizar em terminais muito antigos
- Ocupam 2 caracteres de largura em alguns terminais

**Alternativa com cores ANSI:**
```rust {.line-numbers}
pub fn print_success(message: &str) {
    println!("\n\x1B[32m✓\x1B[0m {}", message);  // Verde
}

pub fn print_error(message: &str) {
    println!("\n\x1B[31m✗\x1B[0m Erro: {}", message);  // Vermelho
}
```

**Analogia:** É como usar **semáforos** (verde = sucesso, vermelho = erro, amarelo = aviso).

---

## 📄 **Função 6: `print_task()` - Exibir Tarefa Detalhada**

```rust {.line-numbers}
/// Exibe uma tarefa formatada
pub fn print_task(task: &Task) {
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│ ID: {:<52} │", task.id);
    println!("│ Título: {:<48} │", task.title);
    println!("│ Descrição: {:<45} │", task.description);
    println!("│ Categoria: {:<45} │", task.category.as_str());
    println!("│ Prioridade: {:<44} │", task.priority.as_str());
    println!("│ Status: {:<48} │", task.status.as_str());

    if let Some(due_date) = task.due_date {
        let overdue = if task.is_overdue() { " (ATRASADA!)" } else { "" };
        println!("│ Vencimento: {:<40}{} │", due_date, overdue);
    }

    println!(
        "│ Criada em: {:<45} │",
        task.created_at.format("%d/%m/%Y %H:%M")
    );

    if let Some(completed_at) = task.completed_at {
        println!(
            "│ Concluída em: {:<42} │",
            completed_at.format("%d/%m/%Y %H:%M")
        );
    }

    println!("└─────────────────────────────────────────────────────────┘");
}
```

### Análise Detalhada:

---

#### a) **Estrutura da Caixa**

```rust {.line-numbers}
┌─────────────────────────────────────────────────────────┐  ← Topo
│ Campo: Valor                                            │  ← Linhas
└─────────────────────────────────────────────────────────┘  ← Base
```

**Caracteres usados:**

| Caractere | Nome | Código Unicode |
|-----------|------|----------------|
| `┌` | Box Drawings Light Down and Right | U+250C |
| `─` | Box Drawings Light Horizontal | U+2500 |
| `┐` | Box Drawings Light Down and Left | U+2510 |
| `│` | Box Drawings Light Vertical | U+2502 |
| `└` | Box Drawings Light Up and Right | U+2514 |
| `┘` | Box Drawings Light Up and Left | U+2518 |

---

#### b) **Formatação de Campos**

```rust {.line-numbers}
println!("│ ID: {:<52} │", task.id);
```

**Estrutura:**
- **`│ `** = Borda esquerda + espaço (2 chars)
- **`ID: `** = Label (4 chars)
- **`{:<52}`** = Valor alinhado à esquerda com 52 chars de largura
- **` │`** = Espaço + borda direita (2 chars)

**Total:** 2 + 4 + 52 + 2 = 60 caracteres

**Por que `{:<52}` e não `{:<48}` para todos?**
- Cada campo tem label de tamanho diferente
- Ajusta a largura para manter alinhamento total de 60 chars

**Exemplos:**
```rust {.line-numbers}
// ID: (4 chars) + valor (52 chars) = 56 + bordas (4) = 60
println!("│ ID: {:<52} │", task.id);

// Título: (8 chars) + valor (48 chars) = 56 + bordas (4) = 60
println!("│ Título: {:<48} │", task.title);

// Descrição: (11 chars) + valor (45 chars) = 56 + bordas (4) = 60
println!("│ Descrição: {:<45} │", task.description);
```

---

#### c) **Campos Opcionais com `if let`**

```rust {.line-numbers}
if let Some(due_date) = task.due_date {
    let overdue = if task.is_overdue() { " (ATRASADA!)" } else { "" };
    println!("│ Vencimento: {:<40}{} │", due_date, overdue);
}
```

**Lógica:**

**1. Verificar se há data de vencimento**
```rust {.line-numbers}
if let Some(due_date) = task.due_date {
```
- Só exibe o campo se `due_date` for `Some`
- Se for `None`, pula completamente

**2. Verificar se está atrasada**
```rust {.line-numbers}
let overdue = if task.is_overdue() { " (ATRASADA!)" } else { "" };
```
- Se atrasada, adiciona sufixo " (ATRASADA!)"
- Se não, string vazia

**3. Exibir com formatação dinâmica**
```rust {.line-numbers}
println!("│ Vencimento: {:<40}{} │", due_date, overdue);
```
- `{:<40}` = Data com 40 chars de largura
- `{}` = Sufixo (vazio ou " (ATRASADA!)")

**Exemplo de saída:**
```
│ Vencimento: 2024-01-15 (ATRASADA!)                      │
│ Vencimento: 2024-12-31                                   │
```

---

#### d) **Formatação de Datas**

```rust {.line-numbers}
println!(
    "│ Criada em: {:<45} │",
    task.created_at.format("%d/%m/%Y %H:%M")
);
```

**Método `.format()`:**
- Método de `DateTime` (do crate `chrono`)
- Formata data/hora usando padrão strftime

**Padrão `"%d/%m/%Y %H:%M"`:**

| Código | Significado | Exemplo |
|--------|-------------|---------|
| `%d` | Dia (01-31) | 15 |
| `%m` | Mês (01-12) | 01 |
| `%Y` | Ano (4 dígitos) | 2024 |
| `%H` | Hora (00-23) | 14 |
| `%M` | Minuto (00-59) | 30 |

**Exemplo:**
```rust {.line-numbers}
// DateTime: 2024-01-15T14:30:45
task.created_at.format("%d/%m/%Y %H:%M")
// Resultado: "15/01/2024 14:30"
```

**Outros padrões úteis:**
```rust {.line-numbers}
"%Y-%m-%d"              // 2024-01-15
"%d/%m/%Y"              // 15/01/2024
"%d/%m/%Y %H:%M:%S"     // 15/01/2024 14:30:45
"%A, %d de %B de %Y"    // Segunda-feira, 15 de Janeiro de 2024
```

---

### Exemplo de Saída do `print_task()`:

```
┌─────────────────────────────────────────────────────────┐
│ ID: 1                                                    │
│ Título: Estudar Rust                                     │
│ Descrição: Completar Fase 1 do curso                     │
│ Categoria: Estudos                                       │
│ Prioridade: Alta                                         │
│ Status: Em Andamento                                     │
│ Vencimento: 2024-01-20 (ATRASADA!)                      │
│ Criada em: 15/01/2024 14:30                             │
└─────────────────────────────────────────────────────────┘
```

**Analogia:** É como um **cartão de visita** ou **ficha** detalhada da tarefa.

---

## 📋 **Função 7: `print_task_list()` - Exibir Lista de Tarefas**

```rust {.line-numbers}
/// Exibe uma lista de tarefas
pub fn print_task_list(tasks: &[&Task]) {
    if tasks.is_empty() {
        print_warning("Nenhuma tarefa encontrada.");
        return;
    }

    println!("\n{:<4} {:<25} {:<12} {:<10} {:<12}", "ID", "Título", "Categoria", "Prioridade", "Status");
    println!("{}", "─".repeat(70));

    for task in tasks {
        let title = if task.title.len() > 25 {
            format!("{}...", &task.title[..22])
        } else {
            task.title.clone()
        };

        let overdue = if task.is_overdue() { "⚠️ " } else { "" };

        println!(
            "{:<4} {:<25} {:<12} {:<10} {}{}",
            task.id,
            title,
            task.category.as_str(),
            task.priority.as_str(),
            overdue,
            task.status.as_str()
        );
    }

    println!("\nTotal: {} tarefa(s)", tasks.len());
}
```

### Análise Detalhada:

---

#### a) **Parâmetro: `&[&Task]`**

```rust {.line-numbers}
pub fn print_task_list(tasks: &[&Task])
```

**O que é `&[&Task]`?**
- **`&[...]`** = Slice (fatia) - referência a uma sequência
- **`&Task`** = Cada elemento é uma referência a `Task`
- **Dupla referência:** Não clona as tarefas, apenas empresta

**Por que não `&[Task]`?**
- `&[Task]` = Slice de tarefas **owned** (precisaria clonar)
- `&[&Task]` = Slice de **referências** (mais eficiente)

**De onde vem esse tipo?**
- Métodos de filtro retornam `Vec<&Task>`
- Podemos passar `&Vec<&Task>` que coerce para `&[&Task]`

---

#### b) **Verificação de Lista Vazia**

```rust {.line-numbers}
if tasks.is_empty() {
    print_warning("Nenhuma tarefa encontrada.");
    return;
}
```

- Se não há tarefas, exibe aviso e retorna cedo
- Evita exibir cabeçalho de tabela vazia

---

#### c) **Cabeçalho da Tabela**

```rust {.line-numbers}
println!("\n{:<4} {:<25} {:<12} {:<10} {:<12}", "ID", "Título", "Categoria", "Prioridade", "Status");
println!("{}", "─".repeat(70));
```

**Formatação de colunas:**

| Coluna | Largura | Alinhamento |
|--------|---------|-------------|
| ID | 4 chars | Esquerda `{:<4}` |
| Título | 25 chars | Esquerda `{:<25}` |
| Categoria | 12 chars | Esquerda `{:<12}` |
| Prioridade | 10 chars | Esquerda `{:<10}` |
| Status | 12 chars | Esquerda `{:<12}` |

**Total:** 4 + 25 + 12 + 10 + 12 = 63 chars (+ espaços entre colunas)

**Linha separadora:**
```rust {.line-numbers}
println!("{}", "─".repeat(70));
```
- **`"─".repeat(70)`** = Cria string com 70 caracteres `─`
- Separa cabeçalho dos dados

---

#### d) **Truncamento de Título Longo**

```rust {.line-numbers}
let title = if task.title.len() > 25 {
    format!("{}...", &task.title[..22])
} else {
    task.title.clone()
};
```

**Lógica:**

**Se título tem mais de 25 caracteres:**
```rust {.line-numbers}
format!("{}...", &task.title[..22])
```
- **`&task.title[..22]`** = Slice dos primeiros 22 caracteres
- **`format!("{}...", ...)`** = Adiciona "..." no final
- **Total:** 22 + 3 = 25 caracteres

**Se título tem 25 ou menos:**
```rust {.line-numbers}
task.title.clone()
```
- Usa o título completo

**Exemplo:**
```rust {.line-numbers}
// Título curto
"Estudar Rust"  →  "Estudar Rust"

// Título longo
"Completar todos os exercícios do curso de Rust"
→ "Completar todos os ex..."
   ↑ 22 chars      ↑ 3 chars = 25 total
```

**Por que truncar?**
- Mantém tabela alinhada
- Evita quebra de linha
- Melhora legibilidade

---

#### e) **Indicador de Tarefa Atrasada**

```rust {.line-numbers}
let overdue = if task.is_overdue() { "⚠️ " } else { "" };
```

- Se atrasada, adiciona emoji ⚠️ antes do status
- Destaque visual imediato

---

#### f) **Exibição de Cada Linha**

```rust {.line-numbers}
println!(
    "{:<4} {:<25} {:<12} {:<10} {}{}",
    task.id,
    title,
    task.category.as_str(),
    task.priority.as_str(),
    overdue,
    task.status.as_str()
);
```

**Formatação:**
- Mesmas larguras do cabeçalho
- `overdue` e `status` sem largura fixa (dinâmico)

---

#### g) **Rodapé com Total**

```rust {.line-numbers}
println!("\nTotal: {} tarefa(s)", tasks.len());
```

- Exibe contagem total de tarefas

---

### Exemplo de Saída do `print_task_list()`:

```

ID   Título                    Categoria    Prioridade Status      
──────────────────────────────────────────────────────────────────────
1    Estudar Rust              Estudos      Alta       Em Andamento
2    Comprar mantimentos       Pessoal      Média      Pendente    
3    Completar todos os ex...  Trabalho     Alta       ⚠️ Pendente  
4    Consulta médica           Saúde        Baixa      Concluída   

Total: 4 tarefa(s)
```

**Analogia:** É como uma **planilha** ou **tabela resumida** das tarefas.

---

## 📊 **Função 8: `print_statistics()` - Exibir Estatísticas**

```rust {.line-numbers}
/// Exibe estatísticas
pub fn print_statistics(stats: &Statistics) {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║                      ESTATÍSTICAS                        ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    println!("\n📊 Resumo Geral:");
    println!("   Total de tarefas: {}", stats.total);
    println!("   ✅ Concluídas: {}", stats.completed);
    println!("   🔄 Em andamento: {}", stats.in_progress);
    println!("   ⏳ Pendentes: {}", stats.pending);
    println!("   ⚠️  Atrasadas: {}", stats.overdue);

    println!("\n📁 Por Categoria:");
    for (category, count) in &stats.by_category {
        if *count > 0 {
            println!("   {}: {}", category.as_str(), count);
        }
    }

    println!("\n⭐ Por Prioridade:");
    for (priority, count) in &stats.by_priority {
        if *count > 0 {
            println!("   {}: {}", priority.as_str(), count);
        }
    }
}
```

### Análise Detalhada:

---

#### a) **Cabeçalho com Caixa Dupla**

```rust {.line-numbers}
println!("\n╔══════════════════════════════════════════════════════════╗");
println!("║                      ESTATÍSTICAS                        ║");
println!("╚══════════════════════════════════════════════════════════╝");
```

- Usa caracteres de caixa dupla (mesmo estilo de `print_header()`)
- Título fixo "ESTATÍSTICAS"

---

#### b) **Seção: Resumo Geral**

```rust {.line-numbers}
println!("\n📊 Resumo Geral:");
println!("   Total de tarefas: {}", stats.total);
println!("   ✅ Concluídas: {}", stats.completed);
println!("   🔄 Em andamento: {}", stats.in_progress);
println!("   ⏳ Pendentes: {}", stats.pending);
println!("   ⚠️  Atrasadas: {}", stats.overdue);
```

**Emojis usados:**

| Emoji | Significado | Unicode |
|-------|-------------|---------|
| 📊 | Gráfico de barras | U+1F4CA |
| ✅ | Check mark | U+2705 |
| 🔄 | Setas circulares | U+1F504 |
| ⏳ | Ampulheta | U+23F3 |
| ⚠️ | Aviso | U+26A0 |

**Indentação:**
- `"   "` = 3 espaços para alinhar valores

---

#### c) **Seção: Por Categoria**

```rust {.line-numbers}
println!("\n📁 Por Categoria:");
for (category, count) in &stats.by_category {
    if *count > 0 {
        println!("   {}: {}", category.as_str(), count);
    }
}
```

**Lógica:**

**1. Iterar sobre tuplas**
```rust {.line-numbers}
for (category, count) in &stats.by_category {
```
- `stats.by_category` é `Vec<(Category, usize)>`
- Desestrutura cada tupla em `category` e `count`

**2. Filtrar categorias vazias**
```rust {.line-numbers}
if *count > 0 {
```
- **`*count`** = Desreferencia (porque `count` é `&usize`)
- Só exibe categorias com pelo menos 1 tarefa

**3. Exibir linha**
```rust {.line-numbers}
println!("   {}: {}", category.as_str(), count);
```
- Formato: "   Nome: Quantidade"

---

#### d) **Seção: Por Prioridade**

```rust {.line-numbers}
println!("\n⭐ Por Prioridade:");
for (priority, count) in &stats.by_priority {
    if *count > 0 {
        println!("   {}: {}", priority.as_str(), count);
    }
}
```

**Idêntico à seção de categoria, mas para prioridades**

---

### Exemplo de Saída do `print_statistics()`:

```

╔══════════════════════════════════════════════════════════╗
║                      ESTATÍSTICAS                        ║
╚══════════════════════════════════════════════════════════╝

📊 Resumo Geral:
   Total de tarefas: 10
   ✅ Concluídas: 3
   🔄 Em andamento: 2
   ⏳ Pendentes: 4
   ⚠️  Atrasadas: 1

📁 Por Categoria:
   Trabalho: 5
   Pessoal: 3
   Estudos: 2

⭐ Por Prioridade:
   Alta: 4
   Média: 3
   Baixa: 3
```

**Analogia:** É como um **dashboard** ou **painel de controle** com métricas agregadas.

---

## 🎯 **Conceitos-Chave Demonstrados**

### 1. **Formatação de Strings**
- `{:<N}` - Alinhamento à esquerda
- `{:^N}` - Centralização
- `{:>N}` - Alinhamento à direita

### 2. **Caracteres Unicode**
- Box drawing characters (┌─┐│└┘╔═╗║╚╝)
- Emojis (✅❌⚠️📊🔄⏳📁⭐)

### 3. **Códigos de Escape ANSI**
- `\x1B[2J` - Limpar tela
- `\x1B[1;1H` - Mover cursor

### 4. **Formatação de Datas**
- `.format()` com padrões strftime
- `%d/%m/%Y %H:%M`

### 5. **Manipulação de Strings**
- `.repeat(N)` - Repetir caractere
- `&str[..N]` - Slice de string
- `.len()` - Comprimento

### 6. **Pattern Matching**
- `if let Some(...)` - Campos opcionais
- Expressões condicionais inline

---

## 💡 **Boas Práticas Demonstradas**

✅ **Feedback visual claro** - Emojis e símbolos  
✅ **Formatação consistente** - Larguras alinhadas  
✅ **Truncamento inteligente** - Títulos longos  
✅ **Campos opcionais** - Só exibe se presente  
✅ **Indicadores visuais** - Tarefas atrasadas destacadas  
✅ **Separação de responsabilidades** - Cada função tem um propósito  
✅ **Reutilização** - Funções de mensagem genéricas  
✅ **Documentação** - Doc comments claros  

---

## 🚀 **Exemplo Completo de Uso**

```rust {.line-numbers}
use crate::ui::output;
use crate::services::TaskService;

fn main() {
    let mut service = TaskService::new("data/tarefas.json".to_string());
    
    // Limpar tela e exibir cabeçalho
    output::clear_screen();
    output::print_header("GERENCIADOR DE TAREFAS");
    
    // Adicionar tarefa
    match service.add_task(...) {
        Ok(tarefa) => {
            output::print_success("Tarefa adicionada com sucesso!");
            output::print_task(tarefa);
        }
        Err(e) => output::print_error(&e.to_string()),
    }
    
    // Listar tarefas
    output::print_header("LISTA DE TAREFAS");
    let todas = service.list_all();
    let refs: Vec<&Task> = todas.iter().collect();
    output::print_task_list(&refs);
    
    // Exibir estatísticas
    output::print_header("ESTATÍSTICAS");
    let stats = service.get_statistics();
    output::print_statistics(&stats);
}
```

---

## 🔍 **Possíveis Melhorias**

### 1. **Adicionar Cores ANSI**

```rust {.line-numbers}
pub fn print_success(message: &str) {
    println!("\n\x1B[32m✅ {}\x1B[0m", message);  // Verde
}

pub fn print_error(message: &str) {
    println!("\n\x1B[31m❌ Erro: {}\x1B[0m", message);  // Vermelho
}
```

### 2. **Adicionar Paginação**

```rust {.line-numbers}
pub fn print_task_list_paginated(tasks: &[&Task], page: usize, per_page: usize) {
    let start = page * per_page;
    let end = (start + per_page).min(tasks.len());
    let page_tasks = &tasks[start..end];
    
    print_task_list(page_tasks);
    println!("\nPágina {} de {}", page + 1, (tasks.len() + per_page - 1) / per_page);
}
```

### 3. **Adicionar Ordenação**

```rust {.line-numbers}
pub fn print_task_list_sorted(tasks: &[&Task], sort_by: SortBy) {
    let mut sorted: Vec<&Task> = tasks.to_vec();
    match sort_by {
        SortBy::Id => sorted.sort_by_key(|t| t.id),
        SortBy::Title => sorted.sort_by_key(|t| &t.title),
        SortBy::Priority => sorted.sort_by_key(|t| t.priority),
    }
    print_task_list(&sorted);
}
```

### 4. **Adicionar Exportação**

```rust {.line-numbers}
pub fn export_task_list_csv(tasks: &[&Task], path: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "ID,Título,Categoria,Prioridade,Status")?;
    for task in tasks {
        writeln!(
            file,
            "{},{},{},{},{}",
            task.id,
            task.title,
            task.category.as_str(),
            task.priority.as_str(),
            task.status.as_str()
        )?;
    }
    Ok(())
}
```

### 5. **Adicionar Gráficos ASCII**

```rust {.line-numbers}
pub fn print_statistics_with_charts(stats: &Statistics) {
    print_statistics(stats);
    
    println!("\n📈 Gráfico de Status:");
    print_bar_chart("Concluídas", stats.completed, stats.total);
    print_bar_chart("Em andamento", stats.in_progress, stats.total);
    print_bar_chart("Pendentes", stats.pending, stats.total);
}

fn print_bar_chart(label: &str, value: usize, max: usize) {
    let bar_length = if max > 0 { (value * 30) / max } else { 0 };
    let bar = "█".repeat(bar_length);
    println!("   {:<15} {} {}", label, bar, value);
}
```

---

## 📚 **Resumo Final**

Este módulo `output` demonstra **excelente design** de UI para CLI em Rust:

1. **Formatação profissional** - Caixas, tabelas, alinhamento
2. **Feedback visual** - Emojis e símbolos
3. **Responsividade** - Truncamento de texto longo
4. **Clareza** - Separação de seções
5. **Consistência** - Padrões de formatação uniformes
6. **Flexibilidade** - Campos opcionais tratados adequadamente
7. **Usabilidade** - Informações organizadas e legíveis

É um exemplo perfeito de como criar **interfaces de terminal bonitas e funcionais** em Rust! 🎨✨