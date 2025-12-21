# 💾 Análise Detalhada da Struct `Storage` em Rust

Vou explicar este código que implementa um **gerenciador de armazenamento em JSON**, permitindo salvar e carregar dados de forma persistente em arquivos.

---

## 📦 **Imports: Bibliotecas Necessárias**

```rust {.line-numbers}
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
```

### Análise de Cada Import:

| Import | Descrição | Uso |
|--------|-----------|-----|
| `serde::{Deserialize, Serialize}` | Traits para serialização | Converter Rust ↔ JSON |
| `std::fs` | Sistema de arquivos | Ler/escrever arquivos |
| `std::io` | Input/Output | Tipos de erro e Result |
| `std::path::Path` | Manipulação de caminhos | Verificar existência, criar diretórios |

**Analogia:**
- **`serde`** = Tradutor (Rust ↔ JSON)
- **`fs`** = Arquivista (gerencia arquivos)
- **`io`** = Protocolo de comunicação (erros e resultados)
- **`Path`** = GPS (navega no sistema de arquivos)

---

## 🏗️ **Struct `Storage` - Estrutura Principal**

```rust {.line-numbers}
/// Gerenciador de armazenamento em JSON
pub struct Storage {
    file_path: String,
}
```

### Estrutura Simples:

**Campo único:**
- **`file_path: String`** = Caminho do arquivo onde os dados serão salvos

**Exemplo:**
```rust {.line-numbers}
let storage = Storage {
    file_path: "data/tarefas.json".to_string(),
};
```

**Por que apenas um campo?**
- Storage é um **wrapper** (embrulho) ao redor de um caminho de arquivo
- Encapsula operações de I/O (Input/Output)
- Segue o princípio de **Single Responsibility** (uma única responsabilidade)

**Analogia:** É como um **chaveiro** que guarda a chave (caminho) de um cofre (arquivo).

---

## 🔧 **Método 1: `new()` - Construtor**

```rust {.line-numbers}
/// Cria uma nova instância de Storage
pub fn new(file_path: String) -> Self {
    Storage { file_path }
}
```

### Análise:

**Assinatura:**
- **`pub fn new`** = Função pública (construtor)
- **`file_path: String`** = Recebe o caminho do arquivo
- **`-> Self`** = Retorna uma instância de `Storage`

**Uso:**
```rust {.line-numbers}
let storage = Storage::new("data/tarefas.json".to_string());
```

**Analogia:** É como **configurar** o chaveiro com a chave do cofre específico.

---

## 💾 **Método 2: `save()` - Salvar Dados em JSON**

```rust {.line-numbers}
/// Salva dados em arquivo JSON
pub fn save<T>(&self, data: &T) -> io::Result<()>
where
    T: Serialize,
{
    // Serializa para JSON com formatação bonita
    let json = serde_json::to_string_pretty(data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Cria o diretório se não existir
    if let Some(parent) = Path::new(&self.file_path).parent() {
        fs::create_dir_all(parent)?;
    }

    // Escreve no arquivo
    fs::write(&self.file_path, json)?;

    Ok(())
}
```

### Análise Detalhada:

---

#### a) **Assinatura com Genéricos**

```rust {.line-numbers}
pub fn save<T>(&self, data: &T) -> io::Result<()>
where
    T: Serialize,
```

**Componentes:**

**1. `<T>` - Tipo Genérico**
- **`T`** = Qualquer tipo (placeholder)
- Permite salvar **qualquer** struct que implemente `Serialize`

**2. `&self` - Referência Imutável**
- Apenas lê o caminho do arquivo
- Não modifica a instância de `Storage`

**3. `data: &T` - Referência aos Dados**
- **`&T`** = Empresta os dados sem consumi-los
- Mais eficiente (não copia)

**4. `-> io::Result<()>` - Retorno**
- **`io::Result<T>`** = `Result<T, io::Error>`
- **`()`** = Unit type (vazio) - não retorna valor, apenas sucesso/erro

**5. `where T: Serialize` - Trait Bound**
- **Restrição:** `T` DEVE implementar o trait `Serialize`
- Garante que podemos converter `T` para JSON

**Analogia:**
- **`<T>`** = "Aceito qualquer tipo de documento"
- **`where T: Serialize`** = "Desde que seja traduzível para JSON"

---

#### b) **Passo 1: Serialização para JSON**

```rust {.line-numbers}
let json = serde_json::to_string_pretty(data)
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
```

**Quebrando em partes:**

**1. `serde_json::to_string_pretty(data)`**
- Converte `data` para JSON formatado (com indentação)
- Retorna `Result<String, serde_json::Error>`

**Exemplo:**
```rust {.line-numbers}
// Struct
struct Task { id: 1, title: "Estudar" }

// JSON formatado
{
  "id": 1,
  "title": "Estudar"
}
```

**2. `.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))`**
- **Converte** o tipo de erro de `serde_json::Error` para `io::Error`
- Necessário porque a função retorna `io::Result`

**O que é `map_err`?**
- Método de `Result` que transforma o erro (se houver)
- Mantém o valor de sucesso intacto

**Estrutura:**
```rust {.line-numbers}
Result<String, serde_json::Error>
    ↓ map_err
Result<String, io::Error>
```

**3. `?` - Operador de Propagação de Erro**
- Se `Ok(json)` → extrai `json` e continua
- Se `Err(e)` → retorna o erro imediatamente

**Equivalente sem `?`:**
```rust {.line-numbers}
let json = match serde_json::to_string_pretty(data) {
    Ok(j) => j,
    Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
};
```

**Analogia:**
- **`to_string_pretty`** = Tradutor que converte documento para JSON
- **`map_err`** = Ajusta o tipo de erro para o formato esperado
- **`?`** = "Se der erro, pare tudo e retorne o erro"

---

#### c) **Passo 2: Criar Diretório (se necessário)**

```rust {.line-numbers}
if let Some(parent) = Path::new(&self.file_path).parent() {
    fs::create_dir_all(parent)?;
}
```

**Quebrando em partes:**

**1. `Path::new(&self.file_path)`**
- Cria um objeto `Path` a partir da string do caminho
- `Path` oferece métodos para manipular caminhos

**Exemplo:**
```rust {.line-numbers}
let path = Path::new("data/tarefas/2024/janeiro.json");
```

**2. `.parent()`**
- Retorna o **diretório pai** (sem o nome do arquivo)
- Retorna `Option<&Path>`

**Exemplo:**
```rust {.line-numbers}
let path = Path::new("data/tarefas/janeiro.json");
let parent = path.parent();  // Some("data/tarefas")

let path2 = Path::new("arquivo.json");
let parent2 = path2.parent();  // Some("") (diretório atual)
```

**3. `if let Some(parent) = ...`**
- Extrai o diretório pai se existir
- Se for `None`, pula o bloco

**4. `fs::create_dir_all(parent)?`**
- **Cria** todos os diretórios necessários no caminho
- Similar ao `mkdir -p` no Linux
- Se já existir, não faz nada (não dá erro)

**Exemplo:**
```rust {.line-numbers}
// Se o caminho é "data/tarefas/2024/janeiro.json"
// E apenas "data" existe, cria:
// data/tarefas/
// data/tarefas/2024/
```

**Por que isso é importante?**
- Evita erro "diretório não encontrado" ao salvar
- Cria estrutura de pastas automaticamente

**Analogia:**
- **`parent()`** = "Qual é o endereço da rua onde fica o prédio?"
- **`create_dir_all`** = "Construa todas as ruas necessárias até chegar lá"

---

#### d) **Passo 3: Escrever no Arquivo**

```rust {.line-numbers}
fs::write(&self.file_path, json)?;
```

**O que faz:**
- **Escreve** a string `json` no arquivo especificado por `file_path`
- Se o arquivo já existir, **sobrescreve** completamente
- Se não existir, **cria** o arquivo

**Retorno:**
- `Ok(())` se sucesso
- `Err(io::Error)` se falhar (permissões, disco cheio, etc.)

**Exemplo:**
```rust {.line-numbers}
// Antes: arquivo não existe ou tem conteúdo antigo
fs::write("data/tarefas.json", json)?;
// Depois: arquivo contém o novo JSON
```

**Analogia:** É como **gravar** um documento no HD, substituindo a versão antiga.

---

#### e) **Passo 4: Retornar Sucesso**

```rust {.line-numbers}
Ok(())
```

- Retorna `Ok` com valor vazio `()`
- Indica que a operação foi bem-sucedida

---

### Fluxo Completo do `save()`:

```
┌─────────────────────────────────────────────────────────┐
│                  FLUXO DO MÉTODO save()                 │
└─────────────────────────────────────────────────────────┘

    Dados (struct Task)
        │
        ▼
┌───────────────────────┐
│ Serializar para JSON  │ ← serde_json::to_string_pretty()
└──────────┬────────────┘
           │
           ▼
    ┌──────────────┐
    │ JSON String  │
    └──────┬───────┘
           │
           ▼
┌─────────────────────────┐
│ Extrair diretório pai   │ ← Path::new().parent()
└──────────┬──────────────┘
           │
           ▼
┌─────────────────────────┐
│ Criar diretórios        │ ← fs::create_dir_all()
└──────────┬──────────────┘
           │
           ▼
┌─────────────────────────┐
│ Escrever no arquivo     │ ← fs::write()
└──────────┬──────────────┘
           │
           ▼
       Ok(())
```

---

### Uso do `save()`:

```rust {.line-numbers}
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
}

fn main() -> io::Result<()> {
    let storage = Storage::new("data/tarefas.json".to_string());
    
    let tarefa = Task {
        id: 1,
        title: "Estudar Rust".to_string(),
    };
    
    // Salva a tarefa no arquivo
    storage.save(&tarefa)?;
    
    println!("Tarefa salva com sucesso!");
    Ok(())
}
```

**Arquivo gerado (`data/tarefas.json`):**
```json
{
  "id": 1,
  "title": "Estudar Rust"
}
```

---

## 📂 **Método 3: `load()` - Carregar Dados do JSON**

```rust {.line-numbers}
/// Carrega dados do arquivo JSON
pub fn load<T>(&self) -> io::Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    // Verifica se o arquivo existe
    if !Path::new(&self.file_path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Arquivo não encontrado",
        ));
    }

    // Lê o arquivo
    let json = fs::read_to_string(&self.file_path)?;

    // Deserializa o JSON
    let data = serde_json::from_str(&json)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(data)
}
```

### Análise Detalhada:

---

#### a) **Assinatura com Higher-Rank Trait Bounds (HRTB)**

```rust {.line-numbers}
pub fn load<T>(&self) -> io::Result<T>
where
    T: for<'de> Deserialize<'de>,
```

**Componentes:**

**1. `<T>` - Tipo Genérico**
- Retorna qualquer tipo `T`

**2. `-> io::Result<T>`**
- Retorna `T` em caso de sucesso
- Retorna `io::Error` em caso de falha

**3. `where T: for<'de> Deserialize<'de>` - HRTB**

**O que é isso?** 🤯

- **`for<'de>`** = "Para qualquer lifetime `'de`"
- **`Deserialize<'de>`** = Trait que permite deserializar com lifetime `'de`

**Por que isso é necessário?**

Quando deserializamos JSON, o Rust precisa saber por quanto tempo os dados emprestados são válidos.

**Comparação:**

| Trait Bound | Significado |
|-------------|-------------|
| `T: Deserialize<'static>` | Só funciona com dados que vivem para sempre |
| `T: for<'de> Deserialize<'de>` | Funciona com qualquer lifetime (mais flexível) |

**Analogia:**
- **`Deserialize<'static>`** = "Só aceito documentos permanentes"
- **`for<'de> Deserialize<'de>`** = "Aceito documentos temporários ou permanentes"

**Na prática, você não precisa se preocupar muito com isso - apenas use `for<'de> Deserialize<'de>` para `load()`!**

---

#### b) **Passo 1: Verificar se o Arquivo Existe**

```rust {.line-numbers}
if !Path::new(&self.file_path).exists() {
    return Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Arquivo não encontrado",
    ));
}
```

**O que faz:**

**1. `Path::new(&self.file_path).exists()`**
- Verifica se o arquivo existe no sistema de arquivos
- Retorna `bool` (true = existe, false = não existe)

**2. `!` - Negação**
- Se **NÃO** existe, entra no bloco

**3. `io::Error::new()`**
- Cria um novo erro de I/O
- **`io::ErrorKind::NotFound`** = Tipo de erro (arquivo não encontrado)
- **`"Arquivo não encontrado"`** = Mensagem descritiva

**4. `return Err(...)`**
- Retorna o erro imediatamente
- Interrompe a execução da função

**Por que verificar manualmente?**
- Fornece mensagem de erro mais clara
- Evita tentar ler arquivo inexistente

**Analogia:** É como verificar se a **chave está na fechadura** antes de tentar abrir a porta.

---

#### c) **Passo 2: Ler o Arquivo**

```rust {.line-numbers}
let json = fs::read_to_string(&self.file_path)?;
```

**O que faz:**
- **Lê** todo o conteúdo do arquivo como `String`
- Retorna `io::Result<String>`

**Exemplo:**
```rust {.line-numbers}
// Arquivo: data/tarefas.json
// Conteúdo:
// {
//   "id": 1,
//   "title": "Estudar Rust"
// }

let json = fs::read_to_string("data/tarefas.json")?;
// json = "{\n  \"id\": 1,\n  \"title\": \"Estudar Rust\"\n}"
```

**Analogia:** É como **abrir** o arquivo e **ler** todo o texto de uma vez.

---

#### d) **Passo 3: Deserializar o JSON**

```rust {.line-numbers}
let data = serde_json::from_str(&json)
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
```

**O que faz:**

**1. `serde_json::from_str(&json)`**
- Converte a string JSON de volta para o tipo `T`
- Retorna `Result<T, serde_json::Error>`

**Exemplo:**
```rust {.line-numbers}
// JSON string
let json = r#"{"id": 1, "title": "Estudar Rust"}"#;

// Deserializa para Task
let tarefa: Task = serde_json::from_str(&json)?;
// tarefa = Task { id: 1, title: "Estudar Rust" }
```

**2. `.map_err(...)`**
- Converte `serde_json::Error` para `io::Error`
- Necessário para manter o tipo de retorno consistente

**3. `?`**
- Propaga o erro se a deserialização falhar

**Analogia:** É como **traduzir** o documento JSON de volta para a linguagem Rust.

---

#### e) **Passo 4: Retornar os Dados**

```rust {.line-numbers}
Ok(data)
```

- Retorna os dados deserializados envolvidos em `Ok`

---

### Fluxo Completo do `load()`:

```
┌─────────────────────────────────────────────────────────┐
│                  FLUXO DO MÉTODO load()                 │
└─────────────────────────────────────────────────────────┘

    Caminho do arquivo
        │
        ▼
┌───────────────────────┐
│ Arquivo existe?       │
└──────────┬────────────┘
           │
      ┌────▼────┐
      │   Não   │ → Err("Arquivo não encontrado")
      └─────────┘
           │
      ┌────▼────┐
      │   Sim   │
      └────┬────┘
           │
           ▼
┌─────────────────────────┐
│ Ler arquivo como String │ ← fs::read_to_string()
└──────────┬──────────────┘
           │
           ▼
    ┌──────────────┐
    │ JSON String  │
    └──────┬───────┘
           │
           ▼
┌─────────────────────────┐
│ Deserializar JSON       │ ← serde_json::from_str()
└──────────┬──────────────┘
           │
           ▼
    ┌──────────────┐
    │ Dados (T)    │
    └──────┬───────┘
           │
           ▼
       Ok(data)
```

---

### Uso do `load()`:

```rust {.line-numbers}
fn main() -> io::Result<()> {
    let storage = Storage::new("data/tarefas.json".to_string());
    
    // Carrega a tarefa do arquivo
    let tarefa: Task = storage.load()?;
    
    println!("Tarefa carregada: {} - {}", tarefa.id, tarefa.title);
    // Saída: Tarefa carregada: 1 - Estudar Rust
    
    Ok(())
}
```

**Tratamento de erro:**
```rust {.line-numbers}
match storage.load::<Task>() {
    Ok(tarefa) => println!("Carregado: {:?}", tarefa),
    Err(e) => eprintln!("Erro ao carregar: {}", e),
}
```

---

## ✅ **Método 4: `exists()` - Verificar se o Arquivo Existe**

```rust {.line-numbers}
/// Verifica se o arquivo existe
pub fn exists(&self) -> bool {
    Path::new(&self.file_path).exists()
}
```

### Análise:

**Simples e direto:**
- Retorna `true` se o arquivo existe
- Retorna `false` se não existe

**Uso:**
```rust {.line-numbers}
let storage = Storage::new("data/tarefas.json".to_string());

if storage.exists() {
    println!("Arquivo encontrado!");
} else {
    println!("Arquivo não existe.");
}
```

**Analogia:** É como **verificar** se a chave está no chaveiro antes de procurar o cofre.

---

## 🗑️ **Método 5: `delete()` - Deletar o Arquivo**

```rust {.line-numbers}
/// Deleta o arquivo de armazenamento
pub fn delete(&self) -> io::Result<()> {
    if self.exists() {
        fs::remove_file(&self.file_path)?;
    }
    Ok(())
}
```

### Análise:

**Passo a passo:**

**1. Verifica se existe**
```rust {.line-numbers}
if self.exists() {
```
- Só tenta deletar se o arquivo existir
- Evita erro "arquivo não encontrado"

**2. Remove o arquivo**
```rust {.line-numbers}
fs::remove_file(&self.file_path)?;
```
- **Deleta** o arquivo permanentemente
- Retorna erro se falhar (permissões, arquivo em uso, etc.)

**3. Retorna sucesso**
```rust {.line-numbers}
Ok(())
```
- Mesmo se o arquivo não existir, retorna `Ok` (operação idempotente)

**Uso:**
```rust {.line-numbers}
let storage = Storage::new("data/tarefas.json".to_string());

// Deleta o arquivo
storage.delete()?;

println!("Arquivo deletado!");
```

**Analogia:** É como **jogar fora** o documento do arquivo.

---

## 🧪 **Testes Automatizados**

```rust {.line-numbers}
#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestData {
        id: u32,
        name: String,
    }

    // Testes aqui
}
```

### Struct de Teste:

```rust {.line-numbers}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestData {
    id: u32,
    name: String,
}
```

**Por que `PartialEq`?**
- Permite comparar instâncias com `==`
- Necessário para `assert_eq!` nos testes

---

### Teste 1: `test_save_and_load` - Salvar e Carregar

```rust {.line-numbers}
#[test]
fn test_save_and_load() {
    let storage = Storage::new("data/test_storage.json".to_string());

    let data = TestData {
        id: 1,
        name: "Test".to_string(),
    };

    // Salva
    storage.save(&data).unwrap();

    // Carrega
    let loaded: TestData = storage.load().unwrap();

    assert_eq!(data, loaded);

    // Limpa
    storage.delete().unwrap();
}
```

**O que Este Teste Verifica?**

✅ Dados salvos podem ser carregados corretamente  
✅ Serialização e deserialização funcionam  
✅ Dados carregados são idênticos aos salvos  

**Fluxo:**
```
Criar dados → Salvar → Carregar → Comparar → Limpar
```

---

### Teste 2: `test_load_nonexistent_file` - Arquivo Inexistente

```rust {.line-numbers}
#[test]
fn test_load_nonexistent_file() {
    let storage = Storage::new("data/nonexistent.json".to_string());

    let result: io::Result<TestData> = storage.load();

    assert!(result.is_err());
}
```

**O que Este Teste Verifica?**

✅ Carregar arquivo inexistente retorna erro  
✅ Não causa panic (falha controlada)  

**Método `.is_err()`:**
```rust {.line-numbers}
result.is_err()  // true se for Err, false se for Ok
```

---

### Teste 3: `test_exists` - Verificar Existência

```rust {.line-numbers}
#[test]
fn test_exists() {
    let storage = Storage::new("data/test_exists.json".to_string());

    assert!(!storage.exists());

    let data = TestData {
        id: 1,
        name: "Test".to_string(),
    };
    storage.save(&data).unwrap();

    assert!(storage.exists());

    storage.delete().unwrap();
    assert!(!storage.exists());
}
```

**O que Este Teste Verifica?**

✅ Arquivo não existe inicialmente  
✅ Após salvar, arquivo existe  
✅ Após deletar, arquivo não existe mais  

**Fluxo:**
```
Não existe → Salvar → Existe → Deletar → Não existe
```

---

## 🎯 **Conceitos-Chave Demonstrados**

### 1. **Genéricos (Generics)**
- `<T>` permite trabalhar com qualquer tipo
- Reutilização de código

### 2. **Trait Bounds**
- `where T: Serialize` restringe tipos aceitos
- Garante que operações são possíveis

### 3. **Result e Error Handling**
- `io::Result<T>` para operações que podem falhar
- `?` para propagação de erros
- `map_err` para conversão de tipos de erro

### 4. **Sistema de Arquivos**
- `fs::write` / `fs::read_to_string` para I/O
- `Path` para manipulação de caminhos
- `create_dir_all` para criar diretórios

### 5. **Serialização com Serde**
- `to_string_pretty` para JSON formatado
- `from_str` para deserialização

### 6. **Higher-Rank Trait Bounds (HRTB)**
- `for<'de> Deserialize<'de>` para flexibilidade de lifetimes

---

## 💡 **Boas Práticas Demonstradas**

✅ **Encapsulamento** - Operações de I/O centralizadas  
✅ **Genéricos** - Funciona com qualquer tipo serializável  
✅ **Error handling** - Erros bem tratados e propagados  
✅ **Criação automática de diretórios** - UX melhor  
✅ **Testes abrangentes** - Casos de sucesso e falha  
✅ **Documentação** - Doc comments claros  
✅ **Idempotência** - `delete()` não falha se arquivo não existe  

---

## 🚀 **Exemplo Completo de Uso**

```rust {.line-numbers}
use serde::{Serialize, Deserialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

fn main() -> io::Result<()> {
    let storage = Storage::new("data/tarefas.json".to_string());
    
    // Criar tarefas
    let tarefas = vec![
        Task { id: 1, title: "Estudar Rust".to_string(), completed: false },
        Task { id: 2, title: "Fazer exercícios".to_string(), completed: true },
    ];
    
    // Salvar
    storage.save(&tarefas)?;
    println!("✅ Tarefas salvas!");
    
    // Verificar existência
    if storage.exists() {
        println!("📁 Arquivo existe!");
    }
    
    // Carregar
    let tarefas_carregadas: Vec<Task> = storage.load()?;
    println!("📂 Tarefas carregadas:");
    for tarefa in &tarefas_carregadas {
        println!("  - {} ({})", tarefa.title, 
            if tarefa.completed { "✓" } else { "○" });
    }
    
    // Deletar (opcional)
    // storage.delete()?;
    
    Ok(())
}
```

**Saída:**
```
✅ Tarefas salvas!
📁 Arquivo existe!
📂 Tarefas carregadas:
  - Estudar Rust (○)
  - Fazer exercícios (✓)
```

**Arquivo gerado (`data/tarefas.json`):**
```json
[
  {
    "id": 1,
    "title": "Estudar Rust",
    "completed": false
  },
  {
    "id": 2,
    "title": "Fazer exercícios",
    "completed": true
  }
]
```

---

## 🔍 **Possíveis Melhorias**

### 1. **Backup Antes de Sobrescrever**

```rust {.line-numbers}
impl Storage {
    pub fn save_with_backup<T>(&self, data: &T) -> io::Result<()>
    where
        T: Serialize,
    {
        // Cria backup se arquivo existir
        if self.exists() {
            let backup_path = format!("{}.bak", self.file_path);
            fs::copy(&self.file_path, backup_path)?;
        }
        
        self.save(data)
    }
}
```

### 2. **Append (Adicionar sem Sobrescrever)**

```rust {.line-numbers}
impl Storage {
    pub fn append<T>(&self, item: &T) -> io::Result<()>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        let mut items: Vec<T> = if self.exists() {
            self.load()?
        } else {
            Vec::new()
        };
        
        items.push(item.clone());
        self.save(&items)
    }
}
```

### 3. **Compressão (Gzip)**

```rust {.line-numbers}
use flate2::write::GzEncoder;
use flate2::Compression;

impl Storage {
    pub fn save_compressed<T>(&self, data: &T) -> io::Result<()>
    where
        T: Serialize,
    {
        let json = serde_json::to_string(data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        let file = fs::File::create(&self.file_path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(json.as_bytes())?;
        encoder.finish()?;
        
        Ok(())
    }
}
```

### 4. **Criptografia**

```rust {.line-numbers}
impl Storage {
    pub fn save_encrypted<T>(&self, data: &T, key: &[u8]) -> io::Result<()>
    where
        T: Serialize,
    {
        let json = serde_json::to_string(data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        // Criptografar json com key
        let encrypted = encrypt(json.as_bytes(), key)?;
        
        fs::write(&self.file_path, encrypted)?;
        Ok(())
    }
}
```

### 5. **Versionamento**

```rust {.line-numbers}
#[derive(Serialize, Deserialize)]
struct VersionedData<T> {
    version: u32,
    data: T,
}

impl Storage {
    pub fn save_versioned<T>(&self, data: &T, version: u32) -> io::Result<()>
    where
        T: Serialize,
    {
        let versioned = VersionedData {
            version,
            data,
        };
        
        self.save(&versioned)
    }
}
```

---

## 📚 **Resumo Final**

Esta struct `Storage` demonstra **excelente design** de persistência em Rust:

1. **Genéricos** - Funciona com qualquer tipo serializável
2. **Error handling robusto** - Tratamento adequado de erros de I/O
3. **Criação automática de diretórios** - Melhor experiência do usuário
4. **API simples e intuitiva** - Métodos claros e diretos
5. **Testes abrangentes** - Cobertura de casos principais
6. **Serialização JSON** - Formato legível e portável
7. **Encapsulamento** - Detalhes de I/O escondidos

É um exemplo perfeito de como implementar **persistência de dados** em Rust! 💾✨