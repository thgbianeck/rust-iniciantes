# 🦀 Dia 21: Rc/Arc - Propriedade Compartilhada em Rust

## 📋 OBJETIVOS DE APRENDIZAGEM

Ao final desta lição, você será capaz de:

- ✅ Entender o conceito de **shared ownership** (propriedade compartilhada)
- ✅ Usar `Rc<T>` para compartilhar dados em contextos single-thread
- ✅ Diferenciar `Rc<T>` de `Arc<T>` (thread-safe)
- ✅ Usar `Rc::clone()` corretamente (shallow copy)
- ✅ Debugar com `strong_count()` e `weak_count()`
- ✅ Evitar memory leaks usando `Weak<T>`
- ✅ Escolher entre `Box`, `Rc`, `Arc` e `&` no contexto certo

---

## 🎭 ATIVAÇÃO DO CONHECIMENTO PRÉVIO

### Revisão Rápida: Box e Ownership Único

Você já aprendeu sobre `Box<T>`:
- **Um único dono** por vez
- Ownership pode ser **movido**, mas não **compartilhado**
- Quando o dono sai de escopo, o valor é **dropado**

~~~rust {.line-numbers}
let x = Box::new(5);
let y = x; // x foi MOVIDO para y
// println!("{}", x); // ❌ ERRO: x não é mais válido
~~~

### 🏢 Analogia Central: O Condomínio

Imagine a propriedade de imóveis:

| Tipo | Analogia | Ownership |
|------|----------|-----------|
| **`Box<T>`** | 🏠 Casa individual | Um único dono |
| **`Rc<T>`** | 🏢 Apartamento com co-proprietários | Múltiplos donos |
| **Contador** | 📊 Registro de quantos donos existem | `strong_count()` |
| **Regra** | 💡 Último a sair apaga as luzes | Drop quando count = 0 |

### 📖 História: A Biblioteca Compartilhada

Imagine uma **biblioteca de dados** que precisa ser acessada por múltiplas partes do seu programa:

- Um **módulo de UI** precisa ler os dados
- Um **módulo de logging** precisa registrar os dados
- Um **módulo de validação** precisa verificar os dados

Com ownership tradicional, você teria que escolher **quem é o dono**. Mas e se todos precisassem **acessar simultaneamente**? É aí que entra `Rc<T>`!

---

## 📚 APRESENTAÇÃO DO CONTEÚDO

### 1️⃣ O Problema: Ownership Único é Limitante

~~~rust {.line-numbers}
struct Dados {
    valor: i32,
}

fn main() {
    let dados = Dados { valor: 42 };
    
    let modulo_a = dados; // dados movido para modulo_a
    // let modulo_b = dados; // ❌ ERRO: dados já foi movido!
}
~~~

**Problema**: Rust não permite múltiplos owners por padrão!

### 2️⃣ A Solução: Rc<T> (Reference Counting)

`Rc<T>` = **R**eference **C**ounted smart pointer

- Permite **múltiplos owners** do mesmo dado
- Mantém um **contador** de quantas referências existem
- Quando o contador chega a **0**, o dado é **dropado**
- **Apenas para single-thread** (não é thread-safe)

~~~rust {.line-numbers}
use std::rc::Rc;

fn main() {
    let dados = Rc::new(42);
    
    let modulo_a = Rc::clone(&dados); // Incrementa contador
    let modulo_b = Rc::clone(&dados); // Incrementa contador
    
    println!("Valor: {}", dados); // 42
    println!("Contador: {}", Rc::strong_count(&dados)); // 3
}
~~~

### 3️⃣ Arc<T> (Atomic Reference Counting)

`Arc<T>` = **A**tomic **R**eference **C**ounted

- Versão **thread-safe** do `Rc<T>`
- Usa operações **atômicas** (mais lento que `Rc`)
- Use quando precisar compartilhar entre **threads**

~~~rust {.line-numbers}
use std::sync::Arc;
use std::thread;

fn main() {
    let dados = Arc::new(42);
    
    let dados_clone = Arc::clone(&dados);
    let handle = thread::spawn(move || {
        println!("Thread: {}", dados_clone);
    });
    
    println!("Main: {}", dados);
    handle.join().unwrap();
}
~~~

---

## 📊 DIAGRAMAS VISUAIS

### Diagrama 1: Comparação Box vs Rc vs Arc

~~~mermaid
graph TB
    subgraph "Box&lt;T&gt; - Ownership Único"
        B1[Box] --> D1[Dados na Heap]
        style B1 fill:#ff6b6b
        style D1 fill:#ffd93d
    end
    
    subgraph "Rc&lt;T&gt; - Shared Ownership Single-Thread"
        R1[Rc clone 1] --> D2[Dados na Heap<br/>Counter: 3]
        R2[Rc clone 2] --> D2
        R3[Rc clone 3] --> D2
        style R1 fill:#51cf66
        style R2 fill:#51cf66
        style R3 fill:#51cf66
        style D2 fill:#ffd93d
    end
    
    subgraph "Arc&lt;T&gt; - Shared Ownership Multi-Thread"
        A1[Arc clone 1<br/>Thread 1] --> D3[Dados na Heap<br/>Atomic Counter: 2]
        A2[Arc clone 2<br/>Thread 2] --> D3
        style A1 fill:#339af0
        style A2 fill:#339af0
        style D3 fill:#ffd93d
    end
~~~

### Diagrama 2: Ciclo de Vida com Rc - Contagem de Referências

~~~mermaid
stateDiagram-v2
    [*] --> Criado: Rc new(valor)<br/>count = 1
    Criado --> Count2: Rc clone()<br/>count = 2
    Count2 --> Count3: Rc clone()<br/>count = 3
    Count3 --> Count2: drop(rc1)<br/>count = 2
    Count2 --> Criado: drop(rc2)<br/>count = 1
    Criado --> [*]: drop(rc3)<br/>count = 0<br/>🗑️ Memória liberada
~~~

### Diagrama 3: Rc::clone() vs .clone() - Diferença Crucial

~~~mermaid
graph LR
    subgraph "Rc::clone() - SHALLOW COPY"
        RC1[Rc original] --> HEAP1[Dados na Heap]
        RC2[Rc::clone] --> HEAP1
        NOTE1[Apenas incrementa<br/>o contador<br/>⚡ RÁPIDO]
        style NOTE1 fill:#51cf66
    end
    
    subgraph ".clone() - DEEP COPY"
        OBJ1[Objeto original] --> DATA1[Dados 1]
        OBJ2[objeto.clone] --> DATA2[Dados 2<br/>CÓPIA COMPLETA]
        NOTE2[Duplica todos<br/>os dados<br/>🐌 LENTO]
        style NOTE2 fill:#ff6b6b
    end
~~~

### Diagrama 4: Problema - Ciclo de Referências (Memory Leak!)

~~~mermaid
graph LR
    A[Nó A<br/>Rc count: 2] --> B[Nó B<br/>Rc count: 2]
    B --> A
    
    style A fill:#ff6b6b
    style B fill:#ff6b6b
    
    NOTE[⚠️ MEMORY LEAK!<br/>Contador nunca chega a 0<br/>Memória nunca é liberada]
    style NOTE fill:#ffd93d
~~~

### Diagrama 5: Solução - Weak<T> para Quebrar Ciclos

~~~mermaid
graph LR
    A[Nó A<br/>Rc strong: 1] -->|Strong Rc| B[Nó B<br/>Rc strong: 1]
    B -.->|Weak| A
    
    style A fill:#51cf66
    style B fill:#51cf66
    
    NOTE[✅ SEM LEAK!<br/>Weak não incrementa strong_count<br/>Permite que memória seja liberada]
    style NOTE fill:#51cf66
~~~

### Diagrama 6: Fluxograma - Qual Smart Pointer Usar?

~~~mermaid
flowchart TD
    START([Preciso alocar na heap?])
    START --> Q1{Múltiplos owners?}
    
    Q1 -->|Não| Q2{Precisa mutar?}
    Q2 -->|Não| BOX[Use Box&lt;T&gt;]
    Q2 -->|Sim| BOXREF[Use Box&lt;T&gt; + &mut]
    
    Q1 -->|Sim| Q3{Multi-thread?}
    Q3 -->|Não| Q4{Precisa mutar?}
    Q3 -->|Sim| Q5{Precisa mutar?}
    
    Q4 -->|Não| RC[Use Rc&lt;T&gt;]
    Q4 -->|Sim| RCREFCELL[Use Rc&lt;RefCell&lt;T&gt;&gt;<br/>Dia 22!]
    
    Q5 -->|Não| ARC[Use Arc&lt;T&gt;]
    Q5 -->|Sim| ARCMUTEX[Use Arc&lt;Mutex&lt;T&gt;&gt;<br/>Dia 23!]
    
    style BOX fill:#51cf66
    style RC fill:#51cf66
    style ARC fill:#51cf66
    style RCREFCELL fill:#ffd93d
    style ARCMUTEX fill:#ffd93d
~~~

### Diagrama 7: Thread-Safety - Rc vs Arc

~~~mermaid
graph TB
    subgraph "❌ Rc&lt;T&gt; - NÃO Thread-Safe"
        RC[Rc&lt;T&gt;]
        T1[Thread 1] -.->|ERRO!| RC
        T2[Thread 2] -.->|ERRO!| RC
        NOTE1[Contador simples<br/>Não usa operações atômicas<br/>⚡ Mais rápido<br/>❌ Não pode cruzar threads]
        style RC fill:#ff6b6b
        style NOTE1 fill:#ff6b6b
    end
    
    subgraph "✅ Arc&lt;T&gt; - Thread-Safe"
        ARC[Arc&lt;T&gt;]
        T3[Thread 1] -->|OK!| ARC
        T4[Thread 2] -->|OK!| ARC
        NOTE2[Contador atômico<br/>Usa operações atômicas<br/>🐌 Um pouco mais lento<br/>✅ Pode cruzar threads]
        style ARC fill:#51cf66
        style NOTE2 fill:#51cf66
    end
~~~

---

## 💡 DEMONSTRAÇÃO E MODELAGEM

### Exemplo 1: Rc Básico - Compartilhando Configuração

~~~rust {.line-numbers}
use std::rc::Rc;

#[derive(Debug)]
struct Config {
    servidor: String,
    porta: u16,
}

fn main() {
    // Criando configuração compartilhada
    let config = Rc::new(Config {
        servidor: String::from("localhost"),
        porta: 8080,
    });
    
    println!("📊 Contador inicial: {}", Rc::strong_count(&config)); // 1
    
    // Módulo de logging precisa da config
    let config_logging = Rc::clone(&config);
    println!("📊 Após clone 1: {}", Rc::strong_count(&config)); // 2
    
    // Módulo de autenticação precisa da config
    let config_auth = Rc::clone(&config);
    println!("📊 Após clone 2: {}", Rc::strong_count(&config)); // 3
    
    // Todos podem acessar
    println!("Logging: {:?}", config_logging);
    println!("Auth: {:?}", config_auth);
    println!("Main: {:?}", config);
    
    // Quando saem de escopo, contador decrementa
    drop(config_logging);
    println!("📊 Após drop 1: {}", Rc::strong_count(&config)); // 2
    
    drop(config_auth);
    println!("📊 Após drop 2: {}", Rc::strong_count(&config)); // 1
    
    // Quando último sai, memória é liberada
} // config é dropado aqui, memória liberada
~~~

**Saída:**
~~~
📊 Contador inicial: 1
📊 Após clone 1: 2
📊 Após clone 2: 3
Logging: Config { servidor: "localhost", porta: 8080 }
Auth: Config { servidor: "localhost", porta: 8080 }
Main: Config { servidor: "localhost", porta: 8080 }
📊 Após drop 1: 2
📊 Após drop 2: 1
~~~

### Exemplo 2: Rc::clone() é Barato

~~~rust {.line-numbers}
use std::rc::Rc;

fn main() {
    // Dado grande
    let vetor_grande = vec![1; 1_000_000];
    let rc_vetor = Rc::new(vetor_grande);
    
    // Clone é RÁPIDO - apenas incrementa contador
    let clone1 = Rc::clone(&rc_vetor); // ⚡ Instantâneo!
    let clone2 = Rc::clone(&rc_vetor); // ⚡ Instantâneo!
    let clone3 = Rc::clone(&rc_vetor); // ⚡ Instantâneo!
    
    println!("Todos compartilham o mesmo vetor na memória!");
    println!("Contador: {}", Rc::strong_count(&rc_vetor)); // 4
}
~~~

### Exemplo 3: Problema - Ciclo de Referências

~~~rust {.line-numbers}
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    valor: i32,
    proximo: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let node_a = Rc::new(RefCell::new(Node {
        valor: 1,
        proximo: None,
    }));
    
    let node_b = Rc::new(RefCell::new(Node {
        valor: 2,
        proximo: Some(Rc::clone(&node_a)), // B aponta para A
    }));
    
    // Criando o CICLO! A aponta para B
    node_a.borrow_mut().proximo = Some(Rc::clone(&node_b));
    
    println!("Count A: {}", Rc::strong_count(&node_a)); // 2
    println!("Count B: {}", Rc::strong_count(&node_b)); // 2
    
    // ⚠️ MEMORY LEAK! Quando main termina:
    // - node_a tem count 2 (node_a e node_b.proximo)
    // - node_b tem count 2 (node_b e node_a.proximo)
    // - Nenhum chega a 0, memória nunca é liberada!
}
~~~

### Exemplo 4: Solução - Weak<T>

~~~rust {.line-numbers}
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    valor: i32,
    proximo: Option<Rc<RefCell<Node>>>,
    anterior: Option<Weak<RefCell<Node>>>, // Weak para evitar ciclo!
}

fn main() {
    let node_a = Rc::new(RefCell::new(Node {
        valor: 1,
        proximo: None,
        anterior: None,
    }));
    
    let node_b = Rc::new(RefCell::new(Node {
        valor: 2,
        proximo: None,
        anterior: Some(Rc::downgrade(&node_a)), // Weak reference!
    }));
    
    node_a.borrow_mut().proximo = Some(Rc::clone(&node_b));
    
    println!("Strong count A: {}", Rc::strong_count(&node_a)); // 1
    println!("Weak count A: {}", Rc::weak_count(&node_a)); // 1
    println!("Strong count B: {}", Rc::strong_count(&node_b)); // 2
    
    // ✅ SEM LEAK! Weak não impede a liberação de memória
    
    // Para acessar Weak, use upgrade()
    if let Some(anterior) = &node_b.borrow().anterior {
        if let Some(node) = anterior.upgrade() {
            println!("Node anterior existe: {}", node.borrow().valor);
        } else {
            println!("Node anterior foi dropado");
        }
    }
}
~~~

---

## 🎯 PRÁTICA GUIADA - Exercício Completo

### 🎓 Exercício: Sistema de Grafo de Dependências de Projetos

**Contexto**: Você está construindo um sistema para gerenciar dependências entre projetos de software. Cada projeto pode depender de múltiplos outros projetos, e múltiplos projetos podem depender do mesmo projeto base.

**Objetivo**: Implementar um grafo onde:
- Nós representam projetos
- Múltiplos nós podem referenciar o mesmo projeto (shared ownership)
- Podemos adicionar dependências
- Podemos contar quantas referências cada projeto tem
- Evitar memory leaks com referências circulares

### Parte 1: Implementação Básica com Rc

~~~rust {.line-numbers}
use std::rc::Rc;
use std::cell::RefCell;

// Estrutura de um Projeto
#[derive(Debug)]
struct Projeto {
    nome: String,
    versao: String,
    dependencias: Vec<Rc<RefCell<Projeto>>>,
}

impl Projeto {
    fn new(nome: &str, versao: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Projeto {
            nome: nome.to_string(),
            versao: versao.to_string(),
            dependencias: Vec::new(),
        }))
    }
    
    fn adicionar_dependencia(&mut self, dep: Rc<RefCell<Projeto>>) {
        self.dependencias.push(dep);
    }
    
    fn listar_dependencias(&self) {
        println!("📦 {} v{} depende de:", self.nome, self.versao);
        for dep in &self.dependencias {
            let dep_ref = dep.borrow();
            println!("  └─ {} v{}", dep_ref.nome, dep_ref.versao);
        }
    }
}

fn main() {
    // Criando projetos base (bibliotecas compartilhadas)
    let serde = Projeto::new("serde", "1.0.0");
    let tokio = Projeto::new("tokio", "1.35.0");
    
    println!("🔢 Referências iniciais:");
    println!("  serde: {}", Rc::strong_count(&serde));
    println!("  tokio: {}", Rc::strong_count(&tokio));
    
    // Criando projeto que usa serde
    let meu_app = Projeto::new("meu_app", "0.1.0");
    meu_app.borrow_mut().adicionar_dependencia(Rc::clone(&serde));
    
    println!("\n🔢 Após meu_app usar serde:");
    println!("  serde: {}", Rc::strong_count(&serde)); // 2
    
    // Criando outro projeto que TAMBÉM usa serde
    let outro_app = Projeto::new("outro_app", "0.2.0");
    outro_app.borrow_mut().adicionar_dependencia(Rc::clone(&serde));
    outro_app.borrow_mut().adicionar_dependencia(Rc::clone(&tokio));
    
    println!("\n🔢 Após outro_app usar serde e tokio:");
    println!("  serde: {}", Rc::strong_count(&serde)); // 3
    println!("  tokio: {}", Rc::strong_count(&tokio)); // 2
    
    // Listando dependências
    println!("\n📋 Estrutura de dependências:");
    meu_app.borrow().listar_dependencias();
    outro_app.borrow().listar_dependencias();
    
    // Simulando remoção de um projeto
    drop(meu_app);
    println!("\n🔢 Após dropar meu_app:");
    println!("  serde: {}", Rc::strong_count(&serde)); // 2
    
    drop(outro_app);
    println!("\n🔢 Após dropar outro_app:");
    println!("  serde: {}", Rc::strong_count(&serde)); // 1
    println!("  tokio: {}", Rc::strong_count(&tokio)); // 1
}
~~~

**Saída Esperada:**
~~~
🔢 Referências iniciais:
  serde: 1
  tokio: 1

🔢 Após meu_app usar serde:
  serde: 2

🔢 Após outro_app usar serde e tokio:
  serde: 3
  tokio: 2

📋 Estrutura de dependências:
📦 meu_app v0.1.0 depende de:
  └─ serde v1.0.0
📦 outro_app v0.2.0 depende de:
  └─ serde v1.0.0
  └─ tokio v1.35.0

🔢 Após dropar meu_app:
  serde: 2

🔢 Após dropar outro_app:
  serde: 1
  tokio: 1
~~~

### Parte 2: Problema - Dependência Circular (Memory Leak)

~~~rust {.line-numbers}
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Projeto {
    nome: String,
    dependencias: Vec<Rc<RefCell<Projeto>>>,
}

impl Projeto {
    fn new(nome: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Projeto {
            nome: nome.to_string(),
            dependencias: Vec::new(),
        }))
    }
}

fn main() {
    let projeto_a = Projeto::new("projeto_a");
    let projeto_b = Projeto::new("projeto_b");
    
    // Criando CICLO: A depende de B, B depende de A
    projeto_a.borrow_mut().dependencias.push(Rc::clone(&projeto_b));
    projeto_b.borrow_mut().dependencias.push(Rc::clone(&projeto_a));
    
    println!("Count A: {}", Rc::strong_count(&projeto_a)); // 2
    println!("Count B: {}", Rc::strong_count(&projeto_b)); // 2
    
    // ⚠️ MEMORY LEAK!
    // Quando main termina, ambos ainda têm count = 1
    // (referenciados um pelo outro)
    // Memória NUNCA é liberada!
}
~~~

### Parte 3: Solução - Usando Weak para Dependências Reversas

~~~rust {.line-numbers}
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Projeto {
    nome: String,
    // Dependências diretas (strong)
    dependencias: Vec<Rc<RefCell<Projeto>>>,
    // Projetos que dependem deste (weak para evitar ciclos)
    dependentes: Vec<Weak<RefCell<Projeto>>>,
}

impl Projeto {
    fn new(nome: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Projeto {
            nome: nome.to_string(),
            dependencias: Vec::new(),
            dependentes: Vec::new(),
        }))
    }
    
    fn adicionar_dependencia(
        projeto: &Rc<RefCell<Projeto>>,
        dependencia: &Rc<RefCell<Projeto>>
    ) {
        // Adiciona dependência forte
        projeto.borrow_mut().dependencias.push(Rc::clone(dependencia));
        
        // Adiciona referência fraca reversa
        dependencia.borrow_mut().dependentes.push(Rc::downgrade(projeto));
    }
    
    fn listar_dependentes(&self) {
        println!("\n👥 Projetos que dependem de '{}':", self.nome);
        for weak_dep in &self.dependentes {
            if let Some(dep) = weak_dep.upgrade() {
                println!("  └─ {}", dep.borrow().nome);
            } else {
                println!("  └─ [projeto removido]");
            }
        }
    }
}

fn main() {
    let serde = Projeto::new("serde");
    let tokio = Projeto::new("tokio");
    let meu_app = Projeto::new("meu_app");
    let outro_app = Projeto::new("outro_app");
    
    // Criando dependências
    Projeto::adicionar_dependencia(&meu_app, &serde);
    Projeto::adicionar_dependencia(&outro_app, &serde);
    Projeto::adicionar_dependencia(&outro_app, &tokio);
    
    println!("🔢 Contadores:");
    println!("  serde - strong: {}, weak: {}", 
        Rc::strong_count(&serde), 
        Rc::weak_count(&serde)); // strong: 3, weak: 2
    
    // Listando quem depende de serde
    serde.borrow().listar_dependentes();
    
    // Removendo meu_app
    drop(meu_app);
    
    println!("\n🗑️ Após remover meu_app:");
    println!("  serde - strong: {}, weak: {}", 
        Rc::strong_count(&serde), 
        Rc::weak_count(&serde)); // strong: 2, weak: 1
    
    serde.borrow().listar_dependentes();
    
    // ✅ SEM MEMORY LEAK! Weak não impede liberação
}
~~~

**Saída Esperada:**
~~~
🔢 Contadores:
  serde - strong: 3, weak: 2

👥 Projetos que dependem de 'serde':
  └─ meu_app
  └─ outro_app

🗑️ Após remover meu_app:
  serde - strong: 2, weak: 1

👥 Projetos que dependem de 'serde':
  └─ [projeto removido]
  └─ outro_app
~~~

### 🎓 Análise do Exercício

**O que aprendemos:**

1. **Shared Ownership**: Múltiplos projetos podem compartilhar a mesma dependência
2. **Reference Counting**: `strong_count()` mostra quantas referências fortes existem
3. **Weak References**: `Weak<T>` permite referências que não impedem liberação de memória
4. **Evitar Leaks**: Usar `Weak` para referências reversas/circulares

**Padrão de Design:**
- **Dependências diretas** → `Rc<T>` (strong)
- **Dependências reversas** → `Weak<T>` (para evitar ciclos)

---

## 🔄 FEEDBACK E AVALIAÇÃO

### ✅ Checklist de Conceitos

Marque o que você dominou:

- [ ] Entendo o que é **reference counting**
- [ ] Sei quando usar `Rc<T>` vs `Box<T>`
- [ ] Sei quando usar `Arc<T>` vs `Rc<T>`
- [ ] Entendo que `Rc::clone()` é **barato** (só incrementa contador)
- [ ] Sei usar `strong_count()` para debugging
- [ ] Entendo o problema de **ciclos de referências**
- [ ] Sei usar `Weak<T>` para evitar memory leaks
- [ ] Sei escolher entre `Box`, `Rc`, `Arc` e `&`

### 🧠 Quiz Rápido

**1. Qual a diferença entre `Rc::clone()` e `.clone()`?**

<details>
<summary>Ver resposta</summary>

- `Rc::clone(&rc)`: Apenas incrementa o contador (shallow copy) - **RÁPIDO** ⚡
- `.clone()`: Clona o dado interno completamente (deep copy) - **LENTO** 🐌

</details>

**2. Quando usar `Arc<T>` ao invés de `Rc<T>`?**

<details>
<summary>Ver resposta</summary>

Use `Arc<T>` quando precisar compartilhar dados entre **múltiplas threads**. `Rc<T>` não é thread-safe.

</details>

**3. Por que ciclos de referências causam memory leaks?**

<details>
<summary>Ver resposta</summary>

Porque o contador nunca chega a 0:
- A tem referência para B (count de B = 1)
- B tem referência para A (count de A = 1)
- Quando ambos saem de escopo, ainda têm count > 0
- Memória nunca é liberada!

</details>

**4. Como `Weak<T>` resolve o problema de ciclos?**

<details>
<summary>Ver resposta</summary>

`Weak<T>` não incrementa o `strong_count`, apenas o `weak_count`. Quando todas as referências **fortes** são dropadas, a memória é liberada, mesmo que existam referências **fracas**.

</details>

**5. Qual o custo de `Rc::clone()`?**

<details>
<summary>Ver resposta</summary>

**Muito baixo**: apenas incrementa um contador (uma operação aritmética simples). Não copia os dados!

</details>

### 🎯 Exercícios de Fixação

**Exercício 1: Detecção de Leaks**

Identifique se há memory leak:

~~~rust {.line-numbers}
use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let a = Rc::new(RefCell::new(Node { next: None }));
    let b = Rc::new(RefCell::new(Node { next: Some(Rc::clone(&a)) }));
    a.borrow_mut().next = Some(Rc::clone(&b));
}
~~~

<details>
<summary>Ver resposta</summary>

**SIM, há memory leak!** A aponta para B, B aponta para A. Ciclo de referências.

**Solução**: Usar `Weak` em uma das direções:

~~~rust {.line-numbers}
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>, // Weak aqui!
}
~~~

</details>

**Exercício 2: Escolha o Smart Pointer Correto**

Para cada cenário, escolha: `Box`, `Rc`, `Arc`, ou `&`:

1. Dados grandes na heap, um único dono
2. Configuração compartilhada entre módulos (single-thread)
3. Contador compartilhado entre threads
4. Lista encadeada simples
5. Grafo com múltiplos pais por nó

<details>
<summary>Ver respostas</summary>

1. **`Box<T>`** - ownership único
2. **`Rc<T>`** - shared ownership, single-thread
3. **`Arc<T>`** - shared ownership, multi-thread
4. **`Box<T>`** - cada nó tem um dono
5. **`Rc<T>`** - múltiplos pais = shared ownership

</details>

### 📊 Auto-Avaliação

Avalie seu entendimento (1-5):

| Conceito | Nível |
|----------|-------|
| Rc básico | ⭐⭐⭐⭐⭐ |
| Arc vs Rc | ⭐⭐⭐⭐⭐ |
| Weak para ciclos | ⭐⭐⭐⭐⭐ |
| strong_count/weak_count | ⭐⭐⭐⭐⭐ |
| Escolher smart pointer | ⭐⭐⭐⭐⭐ |

---

## 🚀 TRANSFERÊNCIA E APLICAÇÃO

### 🎯 Desafio Final: Árvore Binária com Parent Pointers

Implemente uma árvore binária onde cada nó conhece seu **pai** (parent pointer) usando `Weak` para evitar ciclos:

~~~rust {.line-numbers}
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct TreeNode {
    valor: i32,
    pai: Option<Weak<RefCell<TreeNode>>>,
    esquerda: Option<Rc<RefCell<TreeNode>>>,
    direita: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(valor: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            valor,
            pai: None,
            esquerda: None,
            direita: None,
        }))
    }
    
    fn adicionar_filho_esquerda(
        pai: &Rc<RefCell<TreeNode>>,
        filho: Rc<RefCell<TreeNode>>
    ) {
        filho.borrow_mut().pai = Some(Rc::downgrade(pai));
        pai.borrow_mut().esquerda = Some(filho);
    }
    
    fn adicionar_filho_direita(
        pai: &Rc<RefCell<TreeNode>>,
        filho: Rc<RefCell<TreeNode>>
    ) {
        filho.borrow_mut().pai = Some(Rc::downgrade(pai));
        pai.borrow_mut().direita = Some(filho);
    }
    
    fn caminho_ate_raiz(&self) -> Vec<i32> {
        let mut caminho = vec![self.valor];
        let mut atual = self.pai.clone();
        
        while let Some(weak_pai) = atual {
            if let Some(pai) = weak_pai.upgrade() {
                caminho.push(pai.borrow().valor);
                atual = pai.borrow().pai.clone();
            } else {
                break;
            }
        }
        
        caminho.reverse();
        caminho
    }
}

fn main() {
    // Construindo árvore:
    //       10
    //      /  \
    //     5    15
    //    / \
    //   3   7
    
    let raiz = TreeNode::new(10);
    let node5 = TreeNode::new(5);
    let node15 = TreeNode::new(15);
    let node3 = TreeNode::new(3);
    let node7 = TreeNode::new(7);
    
    TreeNode::adicionar_filho_esquerda(&raiz, Rc::clone(&node5));
    TreeNode::adicionar_filho_direita(&raiz, Rc::clone(&node15));
    TreeNode::adicionar_filho_esquerda(&node5, Rc::clone(&node3));
    TreeNode::adicionar_filho_direita(&node5, Rc::clone(&node7));
    
    // Testando caminho até raiz
    println!("Caminho de 3 até raiz: {:?}", node3.borrow().caminho_ate_raiz());
    println!("Caminho de 7 até raiz: {:?}", node7.borrow().caminho_ate_raiz());
    println!("Caminho de 15 até raiz: {:?}", node15.borrow().caminho_ate_raiz());
    
    // Verificando contadores
    println!("\n🔢 Strong counts:");
    println!("  raiz: {}", Rc::strong_count(&raiz));
    println!("  node5: {}", Rc::strong_count(&node5));
    println!("  node3: {}", Rc::strong_count(&node3));
}
~~~

**Saída Esperada:**
~~~
Caminho de 3 até raiz: [10, 5, 3]
Caminho de 7 até raiz: [10, 5, 7]
Caminho de 15 até raiz: [10, 15]

🔢 Strong counts:
  raiz: 1
  node5: 2
  node3: 2
~~~

### 📚 Preparação para o Dia 22: RefCell

Você aprendeu a **compartilhar ownership** com `Rc<T>`, mas os dados são **imutáveis**. E se precisar **mutar** dados compartilhados?

**Próximo tópico**: `RefCell<T>` - Interior Mutability

~~~rust {.line-numbers}
use std::rc::Rc;
use std::cell::RefCell;

// Combinação poderosa: Rc<RefCell<T>>
let dados = Rc::new(RefCell::new(42));

let ref1 = Rc::clone(&dados);
let ref2 = Rc::clone(&dados);

// Múltiplos owners + mutabilidade!
*ref1.borrow_mut() = 100;
println!("{}", ref2.borrow()); // 100
~~~

### 🔗 Recursos Extras

**Documentação Oficial:**
- [std::rc::Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html)
- [std::sync::Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)
- [std::rc::Weak](https://doc.rust-lang.org/std/rc/struct.Weak.html)

**Artigos Recomendados:**
- "Understanding Rust's Reference Counting"
- "Avoiding Memory Leaks with Weak References"
- "When to Use Rc vs Arc"

**Vídeos:**
- "Rust Smart Pointers Explained"
- "Reference Cycles and Memory Leaks in Rust"

---

## 📊 TABELA COMPARATIVA FINAL

| Aspecto | Box<T> | Rc<T> | Arc<T> | Weak<T> |
|---------|--------|-------|--------|---------|
| **Ownership** | Único | Compartilhado | Compartilhado | Não possui |
| **Threads** | ✅ | ❌ | ✅ | ✅ (com Arc) |
| **Mutabilidade** | ✅ | ❌ (precisa RefCell) | ❌ (precisa Mutex) | ❌ |
| **Performance** | ⚡⚡⚡ | ⚡⚡ | ⚡ | ⚡⚡ |
| **Clone** | Deep | Shallow | Shallow | Shallow |
| **Uso** | Heap allocation | Shared data | Shared + threads | Evitar ciclos |
| **Overhead** | Nenhum | Contador | Contador atômico | Contador |

---

## 🎓 RESUMO EXECUTIVO

### 🔑 Pontos-Chave

1. **`Rc<T>`** permite **múltiplos owners** do mesmo dado (single-thread)
2. **`Arc<T>`** é a versão **thread-safe** do `Rc<T>`
3. **`Rc::clone()`** é **barato** - apenas incrementa contador
4. **`strong_count()`** mostra quantas referências fortes existem
5. **Ciclos de referências** causam **memory leaks**
6. **`Weak<T>`** resolve ciclos - não impede liberação de memória
7. Use `Rc/Arc` **apenas quando necessário** - há custo runtime

### ⚠️ Armadilhas Comuns

1. ❌ Usar `Rc` quando `&` seria suficiente
2. ❌ Criar ciclos sem usar `Weak`
3. ❌ Usar `Rc` em código multi-thread (use `Arc`)
4. ❌ Confundir `Rc::clone()` com `.clone()`
5. ❌ Esquecer de verificar `upgrade()` ao usar `Weak`

### ✅ Boas Práticas

1. ✅ Prefira `&` quando possível
2. ✅ Use `Rc` apenas quando realmente precisa de shared ownership
3. ✅ Use `Weak` para referências reversas/circulares
4. ✅ Use `strong_count()` para debugging
5. ✅ Documente por que `Rc` é necessário no código

---

## 🎉 PARABÉNS!

Você completou o **Dia 21** e agora domina:

- ✅ Shared ownership com `Rc<T>` e `Arc<T>`
- ✅ Reference counting e seus custos
- ✅ Detecção e prevenção de memory leaks
- ✅ Uso de `Weak<T>` para quebrar ciclos
- ✅ Escolha do smart pointer correto para cada situação

**Próximo passo**: Dia 22 - `RefCell<T>` e Interior Mutability! 🚀

---

**💬 Dúvidas? Pratique os exercícios e experimente os exemplos!**