# ⌨️ Análise Detalhada do Módulo `input` em Rust

Vou explicar este código que implementa **funções utilitárias para captura de entrada do usuário** em aplicações CLI (Command Line Interface), com foco em interação via terminal.

---

## 📦 **Imports: Bibliotecas de I/O**

```rust {.line-numbers}
use std::io::{self, Write};
```

### Análise dos Imports:

| Import | O que é | Para que serve |
|--------|---------|----------------|
| `std::io` | Módulo de Input/Output | Operações de entrada/saída |
| `self` | Alias para `std::io` | Permite usar `io::stdin()` em vez de `std::io::stdin()` |
| `Write` | Trait | Necessário para usar `.flush()` |

**Por que `Write` é necessário?**
- `flush()` é um método do trait `Write`
- Sem importar `Write`, não podemos chamar `flush()` em `stdout()`

**Analogia:**
- **`std::io`** = Biblioteca de comunicação com o terminal
- **`Write`** = Permissão para "forçar" a escrita imediata na tela

---

## 📝 **Função 1: `read_line()` - Ler Linha de Texto**

```rust {.line-numbers}
/// Lê uma linha de entrada do usuário
pub fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler entrada");

    input.trim().to_string()
}
```

### Análise Detalhada:

---

#### a) **Assinatura da Função**

```rust {.line-numbers}
pub fn read_line(prompt: &str) -> String
```

**Componentes:**
- **`pub fn`** = Função pública (pode ser usada fora do módulo)
- **`prompt: &str`** = Mensagem a exibir antes de ler (ex: "Digite seu nome: ")
- **`-> String`** = Retorna a entrada do usuário como `String`

---

#### b) **Passo 1: Exibir o Prompt**

```rust {.line-numbers}
print!("{}", prompt);
```

**O que faz:**
- **`print!`** = Macro que imprime **sem** quebra de linha
- Diferente de `println!` (que adiciona `\n` no final)

**Exemplo:**
```rust {.line-numbers}
print!("Digite seu nome: ");
// Cursor fica na mesma linha: "Digite seu nome: _"

println!("Digite seu nome: ");
// Cursor vai para próxima linha:
// "Digite seu nome: "
// "_"
```

---

#### c) **Passo 2: Forçar Flush do Buffer**

```rust {.line-numbers}
io::stdout().flush().unwrap();
```

**O que é isso?** 🤔

**Problema: Buffering**
- Por padrão, a saída do terminal é **bufferizada** (armazenada em buffer)
- Texto só aparece quando:
  - Buffer está cheio, OU
  - Encontra uma quebra de linha (`\n`), OU
  - Programa termina

**Exemplo do problema:**
```rust {.line-numbers}
// SEM flush
print!("Digite seu nome: ");
let nome = read_input();  // Usuário não vê o prompt ainda! 😱

// COM flush
print!("Digite seu nome: ");
io::stdout().flush().unwrap();
let nome = read_input();  // Agora o prompt aparece! ✅
```

**Quebrando o código:**

**1. `io::stdout()`**
- Retorna um handle para a **saída padrão** (terminal)
- Tipo: `Stdout`

**2. `.flush()`**
- **Força** o buffer a ser escrito imediatamente
- Retorna `io::Result<()>`

**3. `.unwrap()`**
- Extrai o valor de `Ok`
- **Panic** se for `Err` (raro em stdout)

**Analogia:**
- **Buffer** = Caixa de correio que só é esvaziada quando cheia
- **`flush()`** = Forçar o carteiro a esvaziar a caixa **agora**

---

#### d) **Passo 3: Criar String Mutável**

```rust {.line-numbers}
let mut input = String::new();
```

- Cria uma `String` vazia e **mutável**
- Será preenchida com a entrada do usuário

---

#### e) **Passo 4: Ler Entrada do Usuário**

```rust {.line-numbers}
io::stdin()
    .read_line(&mut input)
    .expect("Falha ao ler entrada");
```

**Quebrando em partes:**

**1. `io::stdin()`**
- Retorna um handle para a **entrada padrão** (teclado)
- Tipo: `Stdin`

**2. `.read_line(&mut input)`**
- **Lê** uma linha completa do terminal (até o usuário pressionar Enter)
- **Adiciona** o texto (incluindo `\n`) à string `input`
- **`&mut input`** = Referência mutável (permite modificar)
- Retorna `io::Result<usize>` (número de bytes lidos)

**Importante:** `read_line` **adiciona** ao final da string, não substitui!

**Exemplo:**
```rust {.line-numbers}
let mut input = String::from("Olá ");
io::stdin().read_line(&mut input).unwrap();
// Usuário digita: "Mundo"
// input agora é: "Olá Mundo\n"
```

**3. `.expect("Falha ao ler entrada")`**
- Similar a `.unwrap()`, mas com mensagem customizada
- Se `Err`, faz **panic** com a mensagem

**Quando pode falhar?**
- Stdin foi fechado
- Erro de I/O (muito raro)

---

#### f) **Passo 5: Limpar e Retornar**

```rust {.line-numbers}
input.trim().to_string()
```

**Quebrando:**

**1. `input.trim()`**
- **Remove** espaços em branco no início e fim
- Remove `\n` (quebra de linha) do Enter
- Retorna `&str` (slice, não String)

**Exemplo:**
```rust {.line-numbers}
let input = "  Olá Mundo\n  ";
let trimmed = input.trim();
// trimmed = "Olá Mundo"
```

**2. `.to_string()`**
- **Converte** `&str` para `String` (owned)
- Necessário porque a função retorna `String`, não `&str`

**Por que não retornar `input.trim()` diretamente?**
- `trim()` retorna `&str` que **empresta** de `input`
- `input` é local (será destruído ao sair da função)
- Não podemos retornar referência a variável local!

**Analogia:**
- **`trim()`** = Cortar as bordas de um papel
- **`to_string()`** = Fazer uma cópia do papel cortado para levar embora

---

### Fluxo Completo do `read_line()`:

```
┌─────────────────────────────────────────────────────────┐
│              FLUXO DO read_line()                       │
└─────────────────────────────────────────────────────────┘

    Exibir prompt
        │
        ▼
    print!("Digite: ")
        │
        ▼
    Forçar flush
        │
        ▼
    io::stdout().flush()
        │
        ▼
    Criar String vazia
        │
        ▼
    let mut input = String::new()
        │
        ▼
    Aguardar entrada do usuário
        │
        ▼
    io::stdin().read_line(&mut input)
        │
        ▼
    Usuário digita: "Olá Mundo" + Enter
        │
        ▼
    input = "Olá Mundo\n"
        │
        ▼
    Limpar espaços
        │
        ▼
    input.trim() = "Olá Mundo"
        │
        ▼
    Converter para String
        │
        ▼
    .to_string()
        │
        ▼
    Retornar "Olá Mundo"
```

---

### Uso do `read_line()`:

```rust {.line-numbers}
let nome = read_line("Digite seu nome: ");
println!("Olá, {}!", nome);

// Saída:
// Digite seu nome: João
// Olá, João!
```

---

## 🔢 **Função 2: `read_number()` - Ler Número**

```rust {.line-numbers}
/// Lê um número do usuário
pub fn read_number(prompt: &str) -> Option<u32> {
    let input = read_line(prompt);
    input.parse::<u32>().ok()
}
```

### Análise Detalhada:

---

#### a) **Assinatura**

```rust {.line-numbers}
pub fn read_number(prompt: &str) -> Option<u32>
```

**Retorno: `Option<u32>`**
- **`Some(numero)`** = Conversão bem-sucedida
- **`None`** = Entrada inválida (não é número)

**Por que `Option` e não `Result`?**
- Simplifica o uso (não precisa lidar com tipo de erro específico)
- Suficiente para validação simples

---

#### b) **Passo 1: Ler Entrada**

```rust {.line-numbers}
let input = read_line(prompt);
```

- Reutiliza `read_line()` (DRY - Don't Repeat Yourself)

---

#### c) **Passo 2: Converter para Número**

```rust {.line-numbers}
input.parse::<u32>().ok()
```

**Quebrando:**

**1. `input.parse::<u32>()`**
- **`parse`** = Método que tenta converter string para outro tipo
- **`::<u32>`** = Turbofish especificando o tipo alvo
- Retorna `Result<u32, ParseIntError>`

**Exemplo:**
```rust {.line-numbers}
"123".parse::<u32>()    // Ok(123)
"abc".parse::<u32>()    // Err(ParseIntError)
"-5".parse::<u32>()     // Err (u32 não aceita negativos)
"1.5".parse::<u32>()    // Err (u32 não aceita decimais)
```

**2. `.ok()`**
- **Converte** `Result<T, E>` em `Option<T>`
- `Ok(valor)` → `Some(valor)`
- `Err(_)` → `None` (descarta o erro)

**Exemplo:**
```rust {.line-numbers}
let result: Result<u32, _> = "123".parse();
let option: Option<u32> = result.ok();
// option = Some(123)

let result: Result<u32, _> = "abc".parse();
let option: Option<u32> = result.ok();
// option = None
```

---

### Uso do `read_number()`:

```rust {.line-numbers}
match read_number("Digite sua idade: ") {
    Some(idade) => println!("Você tem {} anos", idade),
    None => println!("Idade inválida!"),
}

// Ou com if let:
if let Some(idade) = read_number("Digite sua idade: ") {
    println!("Você tem {} anos", idade);
} else {
    println!("Idade inválida!");
}
```

**Analogia:** É como ter um **validador** que só aceita números válidos.

---

## 🎯 **Função 3: `read_option()` - Ler Opção de Menu**

```rust {.line-numbers}
/// Lê uma opção do menu
pub fn read_option(prompt: &str, max: u32) -> Option<u32> {
    let option = read_number(prompt)?;
    if option <= max {
        Some(option)
    } else {
        None
    }
}
```

### Análise Detalhada:

---

#### a) **Assinatura**

```rust {.line-numbers}
pub fn read_option(prompt: &str, max: u32) -> Option<u32>
```

**Parâmetros:**
- **`prompt`** = Mensagem a exibir
- **`max`** = Valor máximo permitido

**Retorno:**
- **`Some(opcao)`** = Opção válida (1 a max)
- **`None`** = Entrada inválida ou fora do intervalo

---

#### b) **Passo 1: Ler Número**

```rust {.line-numbers}
let option = read_number(prompt)?;
```

**O que é `?` aqui?**

- **`?`** funciona com `Option` também (não só `Result`)!
- Se `read_number()` retorna `None`, a função **retorna `None` imediatamente**
- Se retorna `Some(valor)`, extrai `valor` e continua

**Equivalente sem `?`:**
```rust {.line-numbers}
let option = match read_number(prompt) {
    Some(val) => val,
    None => return None,
};
```

**Muito mais conciso com `?`!**

---

#### c) **Passo 2: Validar Intervalo**

```rust {.line-numbers}
if option <= max {
    Some(option)
} else {
    None
}
```

**Lógica:**
- Se `option` está no intervalo válido (0 a max) → `Some(option)`
- Se está fora do intervalo → `None`

**Por que `<= max` e não `< max`?**
- Permite opção 0 (útil para "sair" ou "voltar")
- Permite opção `max` (última opção do menu)

**Exemplo:**
```rust {.line-numbers}
// Menu com 3 opções (1, 2, 3)
read_option("Escolha (1-3): ", 3)

// Entradas válidas: 0, 1, 2, 3
// Entradas inválidas: 4, 5, 100, etc.
```

---

### Uso do `read_option()`:

```rust {.line-numbers}
println!("1. Adicionar tarefa");
println!("2. Listar tarefas");
println!("3. Sair");

match read_option("Escolha uma opção (1-3): ", 3) {
    Some(1) => adicionar_tarefa(),
    Some(2) => listar_tarefas(),
    Some(3) => println!("Saindo..."),
    _ => println!("Opção inválida!"),
}
```

**Analogia:** É como um **porteiro** que só deixa passar números válidos do menu.

---

## ✅ **Função 4: `confirm()` - Confirmar Ação**

```rust {.line-numbers}
/// Confirma uma ação (s/n)
pub fn confirm(prompt: &str) -> bool {
    let input = read_line(&format!("{} (s/n): ", prompt));
    matches!(input.to_lowercase().as_str(), "s" | "sim" | "y" | "yes")
}
```

### Análise Detalhada:

---

#### a) **Assinatura**

```rust {.line-numbers}
pub fn confirm(prompt: &str) -> bool
```

**Retorno:**
- **`true`** = Usuário confirmou (s/sim/y/yes)
- **`false`** = Usuário negou ou entrada inválida

---

#### b) **Passo 1: Ler Entrada com Sufixo**

```rust {.line-numbers}
let input = read_line(&format!("{} (s/n): ", prompt));
```

**Quebrando:**

**1. `format!("{} (s/n): ", prompt)`**
- **`format!`** = Macro que cria uma `String` formatada
- Adiciona " (s/n): " ao final do prompt

**Exemplo:**
```rust {.line-numbers}
let prompt = "Deseja continuar?";
let full_prompt = format!("{} (s/n): ", prompt);
// full_prompt = "Deseja continuar? (s/n): "
```

**2. `&format!(...)`**
- `format!` retorna `String`
- `&` cria referência `&String`
- Coerção automática para `&str`

---

#### c) **Passo 2: Verificar Resposta**

```rust {.line-numbers}
matches!(input.to_lowercase().as_str(), "s" | "sim" | "y" | "yes")
```

**Quebrando em partes:**

**1. `input.to_lowercase()`**
- Converte para minúsculas
- Permite aceitar "S", "Sim", "SIM", etc.
- Retorna `String`

**Exemplo:**
```rust {.line-numbers}
"SIM".to_lowercase()  // "sim"
"Yes".to_lowercase()  // "yes"
```

**2. `.as_str()`**
- Converte `String` para `&str`
- Necessário para usar com `matches!`

**3. `matches!(..., "s" | "sim" | "y" | "yes")`**
- **`matches!`** = Macro que verifica se um valor corresponde a um padrão
- **`|`** = Operador "ou" em pattern matching
- Retorna `bool`

**O que é `matches!`?**

É um atalho para pattern matching que retorna `bool`:

**Sem `matches!`:**
```rust {.line-numbers}
match input.to_lowercase().as_str() {
    "s" | "sim" | "y" | "yes" => true,
    _ => false,
}
```

**Com `matches!`:**
```rust {.line-numbers}
matches!(input.to_lowercase().as_str(), "s" | "sim" | "y" | "yes")
```

**Muito mais conciso!**

---

### Fluxo Completo do `confirm()`:

```
┌─────────────────────────────────────────────────────────┐
│              FLUXO DO confirm()                         │
└─────────────────────────────────────────────────────────┘

    Prompt: "Deseja continuar?"
        │
        ▼
    format!("{} (s/n): ", prompt)
        │
        ▼
    "Deseja continuar? (s/n): "
        │
        ▼
    read_line(...)
        │
        ▼
    Usuário digita: "SIM"
        │
        ▼
    input = "SIM"
        │
        ▼
    input.to_lowercase()
        │
        ▼
    "sim"
        │
        ▼
    .as_str()
        │
        ▼
    "sim" (como &str)
        │
        ▼
    matches!(..., "s" | "sim" | "y" | "yes")
        │
        ▼
    true ✅
```

---

### Uso do `confirm()`:

```rust {.line-numbers}
if confirm("Deseja deletar esta tarefa?") {
    deletar_tarefa();
    println!("Tarefa deletada!");
} else {
    println!("Operação cancelada.");
}

// Exemplos de entradas aceitas:
// "s" → true
// "S" → true
// "sim" → true
// "SIM" → true
// "y" → true
// "yes" → true
// "YES" → true
// "n" → false
// "não" → false
// "abc" → false
// "" → false
```

**Analogia:** É como um **diálogo de confirmação** que aceita várias formas de "sim".

---

## ⏸️ **Função 5: `pause()` - Pausar Execução**

```rust {.line-numbers}
/// Pausa até o usuário pressionar Enter
pub fn pause() {
    read_line("\nPressione Enter para continuar...");
}
```

### Análise:

**Função extremamente simples:**
- Chama `read_line()` com mensagem fixa
- **Ignora** o retorno (não importa o que o usuário digitou)
- Apenas aguarda o usuário pressionar Enter

**Uso:**
```rust {.line-numbers}
println!("Tarefa adicionada com sucesso!");
pause();
limpar_tela();
```

**Analogia:** É como um **botão "Continuar"** em jogos ou instaladores.

---

## 🎯 **Conceitos-Chave Demonstrados**

### 1. **I/O Bufferizado**
- `print!` vs `println!`
- `flush()` para forçar escrita imediata

### 2. **Ownership e Borrowing**
- `&str` vs `String`
- `&mut` para modificar variáveis

### 3. **Error Handling**
- `Result` vs `Option`
- `.ok()` para converter `Result` em `Option`
- `?` para propagação de erros/None

### 4. **Pattern Matching**
- `matches!` macro
- Padrões com `|` (ou)

### 5. **String Manipulation**
- `.trim()` para remover espaços
- `.to_lowercase()` para normalizar
- `.parse()` para conversão de tipos

### 6. **Macros**
- `print!`, `println!`
- `format!`
- `matches!`

---

## 💡 **Boas Práticas Demonstradas**

✅ **Reutilização de código** - `read_number()` usa `read_line()`  
✅ **Validação de entrada** - `read_option()` valida intervalo  
✅ **Flexibilidade** - `confirm()` aceita múltiplas respostas  
✅ **UX melhorada** - `flush()` garante que prompts apareçam  
✅ **Simplicidade** - Funções pequenas e focadas  
✅ **Documentação** - Doc comments claros  
✅ **Type safety** - `Option` para valores opcionais  

---

## 🚀 **Exemplo Completo de Uso**

```rust {.line-numbers}
use crate::ui::input;

fn main() {
    // Ler texto
    let nome = input::read_line("Digite seu nome: ");
    println!("Olá, {}!", nome);
    
    // Ler número
    loop {
        match input::read_number("Digite sua idade: ") {
            Some(idade) => {
                println!("Você tem {} anos", idade);
                break;
            }
            None => println!("Por favor, digite um número válido!"),
        }
    }
    
    // Menu com validação
    loop {
        println!("\n=== MENU ===");
        println!("1. Opção 1");
        println!("2. Opção 2");
        println!("3. Sair");
        
        match input::read_option("Escolha (1-3): ", 3) {
            Some(1) => println!("Você escolheu Opção 1"),
            Some(2) => println!("Você escolheu Opção 2"),
            Some(3) => {
                if input::confirm("Deseja realmente sair?") {
                    println!("Até logo!");
                    break;
                }
            }
            _ => println!("Opção inválida!"),
        }
        
        input::pause();
    }
}
```

---

## 🔍 **Possíveis Melhorias**

### 1. **Adicionar Validação de String Vazia**

```rust {.line-numbers}
pub fn read_non_empty(prompt: &str) -> String {
    loop {
        let input = read_line(prompt);
        if !input.is_empty() {
            return input;
        }
        println!("Entrada não pode ser vazia!");
    }
}
```

### 2. **Adicionar Leitura de Float**

```rust {.line-numbers}
pub fn read_float(prompt: &str) -> Option<f64> {
    let input = read_line(prompt);
    input.parse::<f64>().ok()
}
```

### 3. **Adicionar Leitura de Data**

```rust {.line-numbers}
use chrono::NaiveDate;

pub fn read_date(prompt: &str) -> Option<NaiveDate> {
    let input = read_line(prompt);
    NaiveDate::parse_from_str(&input, "%d/%m/%Y").ok()
}
```

### 4. **Adicionar Timeout**

```rust {.line-numbers}
use std::time::Duration;

pub fn read_line_timeout(prompt: &str, timeout: Duration) -> Option<String> {
    // Implementação com threads ou async
    // ...
}
```

### 5. **Adicionar Histórico (como bash)**

```rust {.line-numbers}
// Usar biblioteca como `rustyline` para histórico de comandos
use rustyline::Editor;

pub fn read_line_with_history(prompt: &str) -> String {
    let mut rl = Editor::<()>::new();
    rl.readline(prompt).unwrap_or_default()
}
```

### 6. **Adicionar Validação Customizada**

```rust {.line-numbers}
pub fn read_validated<F>(prompt: &str, validator: F) -> String
where
    F: Fn(&str) -> bool,
{
    loop {
        let input = read_line(prompt);
        if validator(&input) {
            return input;
        }
        println!("Entrada inválida!");
    }
}

// Uso:
let email = read_validated("Digite seu email: ", |s| s.contains('@'));
```

---

## 📚 **Resumo Final**

Este módulo `input` demonstra **excelente design** de utilitários CLI em Rust:

1. **Funções compostas** - Reutilização de código
2. **Validação integrada** - `read_option()` valida intervalo
3. **Flexibilidade** - `confirm()` aceita múltiplas respostas
4. **UX polida** - `flush()` garante feedback imediato
5. **Type safety** - `Option` para valores opcionais
6. **Simplicidade** - API intuitiva e fácil de usar
7. **Robustez** - Tratamento adequado de erros

É um exemplo perfeito de como criar **utilitários de entrada** para aplicações CLI em Rust! ⌨️✨