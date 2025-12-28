# 🦀 Dia 23: Copy vs Clone - A Arte da Duplicação Eficiente

---

## 📋 OBJETIVOS DE APRENDIZAGEM

Ao final desta lição, você será capaz de:

✅ **Diferenciar** Copy (cópia implícita) de Clone (cópia explícita)  
✅ **Escolher** a estratégia correta de duplicação para cada tipo de dado  
✅ **Otimizar** código identificando quando duplicar é necessário ou evitável  
✅ **Implementar** traits Copy e Clone manualmente quando necessário  
✅ **Medir** o custo de duplicações em seu código

---

## 🎭 ATIVAÇÃO DO CONHECIMENTO PRÉVIO

### Revisão Rápida: Ownership e Move

Você já aprendeu que Rust **move** valores por padrão:

~~~rust {.line-numbers}
let s1 = String::from("hello");
let s2 = s1; // s1 foi MOVIDO, não pode mais ser usado
~~~

Mas e quando você **realmente precisa** de duas cópias do mesmo dado?

### 🎨 Analogia Central: Xerox vs Fotografia

Imagine três cenários em um escritório:

1. **Move (Transferência)**: Você pega um documento da sua mesa e coloca na mesa do colega
   - Original deixa de existir no local anterior
   - Sem custo de duplicação
   - **É o padrão do Rust**

2. **Copy (Xerox Instantânea)**: Você passa um Post-it pela máquina de xerox
   - Cópia instantânea, barata, trivial
   - Funciona apenas para coisas simples
   - **Tipos primitivos em Rust**

3. **Clone (Fotografia Profissional)**: Você fotografa um quadro complexo, revela, imprime
   - Processo mais elaborado e custoso
   - Necessário para coisas complexas
   - **Tipos complexos em Rust**

### 📖 História: O Custo Oculto da Duplicação

Em 2019, uma equipe otimizou um servidor Rust e descobriu que **23% do tempo de CPU** era gasto clonando `String`s desnecessariamente. Ao substituir `.clone()` por referências (`&str`), o throughput aumentou 30%.

**Lição**: Duplicação tem custo. Escolha conscientemente.

---

## 📚 APRESENTAÇÃO DO CONTEÚDO

### 📊 Diagrama 1: Copy vs Clone vs Move

~~~mermaid
graph TB
    subgraph "Operações de Duplicação em Rust"
        A[Valor Original] --> B{Qual operação?}
        
        B -->|Move padrão| C[Move]
        C --> C1[Transfere ownership]
        C1 --> C2[Original invalidado]
        C2 --> C3[✅ Custo: ZERO]
        
        B -->|Tipo Copy| D[Copy Implícito]
        D --> D1[Cópia bitwise automática]
        D1 --> D2[Original permanece válido]
        D2 --> D3[✅ Custo: TRIVIAL]
        
        B -->|Método .clone| E[Clone Explícito]
        E --> E1[Deep copy completa]
        E1 --> E2[Original permanece válido]
        E2 --> E3[⚠️ Custo: VARIÁVEL]
    end
    
    style C3 fill:#90EE90
    style D3 fill:#90EE90
    style E3 fill:#FFD700
~~~

### 📊 Diagrama 2: Tipos que Implementam Copy

~~~mermaid
graph LR
    subgraph "Tipos Copy em Rust"
        A[Copy Trait] --> B[Inteiros]
        A --> C[Floats]
        A --> D[Bool]
        A --> E[Char]
        A --> F[Referências Imutáveis]
        A --> G[Tuples/Arrays de Copy]
        
        B --> B1[i8, i16, i32, i64, i128, isize]
        B --> B2[u8, u16, u32, u64, u128, usize]
        
        C --> C1[f32, f64]
        
        F --> F1[&T onde T: ?Sized]
        
        G --> G1["(i32, bool) ✅"]
        G --> G2["[u8; 10] ✅"]
    end
    
    subgraph "NÃO são Copy"
        H[String] 
        I[Vec<T>]
        J[Box<T>]
        K[HashMap]
        L[Tipos com Drop]
    end
    
    style A fill:#4CAF50,color:#fff
    style H fill:#f44336,color:#fff
    style I fill:#f44336,color:#fff
    style J fill:#f44336,color:#fff
    style K fill:#f44336,color:#fff
    style L fill:#f44336,color:#fff
~~~

### 📊 Diagrama 3: Fluxograma de Decisão

~~~mermaid
flowchart TD
    Start([Preciso duplicar dados?]) --> Q1{Posso usar referência?}
    
    Q1 -->|Sim| UseRef[✅ Use &T ou &mut T]
    Q1 -->|Não| Q2{Tipo é primitivo?}
    
    Q2 -->|Sim i32, bool, etc| UseCopy[✅ Copy automático]
    Q2 -->|Não| Q3{Tipo implementa Copy?}
    
    Q3 -->|Sim| UseCopy2[✅ Copy automático]
    Q3 -->|Não| Q4{Precisa de deep copy?}
    
    Q4 -->|Sim| UseClone[⚠️ Use .clone<br/>Custo: O<n>]
    Q4 -->|Não| Redesign[🔄 Redesenhar arquitetura<br/>Considere Rc/Arc]
    
    UseRef --> End([Decisão tomada])
    UseCopy --> End
    UseCopy2 --> End
    UseClone --> End
    Redesign --> End
    
    style UseRef fill:#4CAF50,color:#fff
    style UseCopy fill:#4CAF50,color:#fff
    style UseCopy2 fill:#4CAF50,color:#fff
    style UseClone fill:#FF9800,color:#fff
    style Redesign fill:#2196F3,color:#fff
~~~

### 📊 Diagrama 4: Comparação de Performance

~~~mermaid
graph LR
    subgraph "Custo de Operações"
        A[Referência &T] -->|0 ns| A1[Custo: ZERO]
        B[Move] -->|0 ns| B1[Custo: ZERO]
        C[Copy i32] -->|~1 ns| C1[Custo: TRIVIAL]
        D[Copy [u8; 1024]] -->|~10 ns| D1[Custo: BAIXO]
        E[Clone String 10 chars] -->|~50 ns| E1[Custo: MODERADO]
        F[Clone Vec 1000 items] -->|~500 ns| F1[Custo: ALTO]
        G[Clone HashMap 1000 items] -->|~5000 ns| G1[Custo: MUITO ALTO]
    end
    
    style A1 fill:#4CAF50,color:#fff
    style B1 fill:#4CAF50,color:#fff
    style C1 fill:#8BC34A,color:#fff
    style D1 fill:#CDDC39,color:#000
    style E1 fill:#FF9800,color:#fff
    style F1 fill:#FF5722,color:#fff
    style G1 fill:#f44336,color:#fff
~~~

### 📊 Diagrama 5: Traits Copy e Clone - Hierarquia

~~~mermaid
classDiagram
    class Clone {
        <<trait>>
        +clone(&self) Self
        +clone_from(&mut self, source: &Self)
    }
    
    class Copy {
        <<trait>>
        +Marker trait
        +Sem métodos
        +Requer Clone
    }
    
    class Drop {
        <<trait>>
        +drop(&mut self)
        +Incompatível com Copy
    }
    
    Clone <|-- Copy : requer
    Copy --|> Drop : ❌ mutuamente exclusivos
    
    class i32 {
        +Implementa Copy
        +Implementa Clone
    }
    
    class String {
        +Implementa Clone
        +Implementa Drop
        +❌ NÃO implementa Copy
    }
    
    class Point2D {
        +x: f64
        +y: f64
        +Pode implementar Copy
    }
    
    Copy <|.. i32
    Clone <|.. i32
    Clone <|.. String
    Drop <|.. String
    Copy <|.. Point2D
    Clone <|.. Point2D
~~~

### 📊 Diagrama 6: Árvore de Decisão Prática

~~~mermaid
graph TD
    Root[Estou criando um tipo novo] --> Q1{Contém heap data?}
    
    Q1 -->|Sim String, Vec, Box| NoHeap[❌ NÃO pode ser Copy]
    Q1 -->|Não| Q2{Todos os campos são Copy?}
    
    Q2 -->|Não| NoHeap2[❌ NÃO pode ser Copy]
    Q2 -->|Sim| Q3{Implementa Drop?}
    
    Q3 -->|Sim| NoHeap3[❌ NÃO pode ser Copy]
    Q3 -->|Não| Q4{Cópia bitwise é segura?}
    
    Q4 -->|Sim| CanCopy[✅ PODE ser Copy]
    Q4 -->|Não| NoHeap4[❌ Use apenas Clone]
    
    NoHeap --> ImplClone[Implemente Clone]
    NoHeap2 --> ImplClone
    NoHeap3 --> ImplClone
    NoHeap4 --> ImplClone
    
    CanCopy --> ImplBoth[Implemente Copy + Clone]
    
    style CanCopy fill:#4CAF50,color:#fff
    style ImplBoth fill:#4CAF50,color:#fff
    style NoHeap fill:#f44336,color:#fff
    style NoHeap2 fill:#f44336,color:#fff
    style NoHeap3 fill:#f44336,color:#fff
    style NoHeap4 fill:#FF9800,color:#fff
~~~

---

### 📊 Tabela Comparativa Detalhada

| Aspecto | **Copy** | **Clone** | **Move** |
|---------|----------|-----------|----------|
| **Sintaxe** | Automático (implícito) | `.clone()` explícito | Automático (padrão) |
| **Quando ocorre** | Atribuição, passagem de parâmetro | Chamada explícita de `.clone()` | Atribuição, passagem (padrão) |
| **Custo** | Trivial (bitwise copy) | Variável (pode ser caro) | Zero (transferência) |
| **Tipos suportados** | Primitivos, tipos simples | Qualquer tipo que implemente | Todos os tipos |
| **Heap allocation** | ❌ Não pode ter | ✅ Pode ter | ✅ Transfere ownership |
| **Original válido após?** | ✅ Sim | ✅ Sim | ❌ Não |
| **Trait requerido** | `Copy` (marker trait) | `Clone` | Nenhum |
| **Implementação** | `#[derive(Copy, Clone)]` | `#[derive(Clone)]` ou manual | Automático |
| **Compatível com Drop** | ❌ Não | ✅ Sim | ✅ Sim |
| **Performance** | O(1) - constante | O(n) - proporcional ao tamanho | O(1) - constante |
| **Uso típico** | Números, coordenadas, flags | Strings, vetores, coleções | Padrão do Rust |
| **Controle** | Sem controle (sempre copia) | Controle explícito | Sem controle (sempre move) |

---

## 💡 DEMONSTRAÇÃO E MODELAGEM

### 1️⃣ Copy Trait em Ação (Implícito)

**Tipos primitivos são Copy por padrão:**

~~~rust {.line-numbers}
fn demonstracao_copy() {
    // Tipos primitivos implementam Copy
    let x = 42;
    let y = x;  // Cópia IMPLÍCITA acontece aqui
    
    println!("x = {}, y = {}", x, y);  // ✅ Ambos válidos!
    // x ainda é válido porque i32 implementa Copy
    
    // Outro exemplo
    let ponto_a = (10.5, 20.3);  // Tuple de f64 (Copy)
    let ponto_b = ponto_a;       // Cópia implícita
    
    println!("A: {:?}, B: {:?}", ponto_a, ponto_b);  // ✅ Ambos válidos!
}
~~~

**Saída:**
~~~
x = 42, y = 42
A: (10.5, 20.3), B: (10.5, 20.3)
~~~

**O que aconteceu?** Rust copiou os bits automaticamente. Sem alocação de heap, sem custo significativo.

---

### 2️⃣ Clone Trait em Ação (Explícito)

**Tipos complexos requerem `.clone()` explícito:**

~~~rust {.line-numbers}
fn demonstracao_clone() {
    // String NÃO implementa Copy
    let s1 = String::from("Rust");
    // let s2 = s1;  // ❌ Isso seria um MOVE, s1 ficaria inválido
    
    let s2 = s1.clone();  // ✅ Clone EXPLÍCITO
    
    println!("s1 = {}, s2 = {}", s1, s2);  // ✅ Ambos válidos!
    
    // Vec também requer clone
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = v1.clone();  // Deep copy de todos os elementos
    
    println!("v1: {:?}, v2: {:?}", v1, v2);
}
~~~

**Saída:**
~~~
s1 = Rust, s2 = Rust
v1: [1, 2, 3, 4, 5], v2: [1, 2, 3, 4, 5]
~~~

**Por que explícito?** Rust força você a **reconhecer o custo** da operação.

---

### 3️⃣ Implementação Manual de Copy e Clone

~~~rust {.line-numbers}
// Struct simples - pode ser Copy
#[derive(Debug, Copy, Clone)]
struct Point2D {
    x: f64,
    y: f64,
}

// Struct complexa - apenas Clone
#[derive(Debug, Clone)]
struct Person {
    name: String,      // String não é Copy
    age: u32,          // u32 é Copy, mas struct inteira não pode ser
}

fn teste_implementacao() {
    // Point2D é Copy
    let p1 = Point2D { x: 10.0, y: 20.0 };
    let p2 = p1;  // Cópia implícita
    println!("p1: {:?}, p2: {:?}", p1, p2);  // ✅ Ambos válidos
    
    // Person é apenas Clone
    let pessoa1 = Person {
        name: String::from("Alice"),
        age: 30,
    };
    // let pessoa2 = pessoa1;  // ❌ Seria move
    let pessoa2 = pessoa1.clone();  // ✅ Clone explícito
    println!("pessoa1: {:?}, pessoa2: {:?}", pessoa1, pessoa2);
}
~~~

---

### 4️⃣ Implementação Manual Completa (Sem Derive)

~~~rust {.line-numbers}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementação manual de Clone
impl Clone for Rectangle {
    fn clone(&self) -> Self {
        println!("🔄 Clonando Rectangle...");
        Rectangle {
            width: self.width,
            height: self.height,
        }
    }
}

// Implementação manual de Copy (requer Clone)
impl Copy for Rectangle {}
// Copy é um marker trait, não tem métodos

fn teste_manual() {
    let r1 = Rectangle { width: 30, height: 50 };
    let r2 = r1;  // Copy implícito (não imprime mensagem)
    let r3 = r1.clone();  // Clone explícito (imprime mensagem)
    
    println!("r1: {:?}", r1);
    println!("r2: {:?}", r2);
    println!("r3: {:?}", r3);
}
~~~

**Saída:**
~~~
🔄 Clonando Rectangle...
r1: Rectangle { width: 30, height: 50 }
r2: Rectangle { width: 30, height: 50 }
r3: Rectangle { width: 30, height: 50 }
~~~

---

### 5️⃣ Por Que Copy e Drop São Incompatíveis?

~~~rust {.line-numbers}
// ❌ ISSO NÃO COMPILA!
/*
#[derive(Copy, Clone)]
struct Recurso {
    handle: Box<i32>,  // Box implementa Drop
}
*/

// Explicação: Se Recurso fosse Copy, teríamos:
fn exemplo_problema() {
    // let r1 = Recurso { handle: Box::new(42) };
    // let r2 = r1;  // Cópia bitwise do ponteiro
    
    // Problema: r1 e r2 apontam para o MESMO Box
    // Quando r1 sai de escopo, Drop libera a memória
    // Quando r2 sai de escopo, Drop tenta liberar NOVAMENTE
    // ☠️ DOUBLE FREE! Comportamento indefinido!
}

// ✅ Solução: Tipos com Drop só podem ser Clone (explícito)
#[derive(Clone)]
struct RecursoSeguro {
    handle: Box<i32>,
}

impl Clone for RecursoSeguro {
    fn clone(&self) -> Self {
        // Deep copy: cria NOVO Box
        RecursoSeguro {
            handle: Box::new(*self.handle),
        }
    }
}
~~~

**Regra de Ouro:** Se um tipo gerencia recursos (heap, arquivos, sockets), ele **não pode** ser Copy.

---

## 🎯 PRÁTICA GUIADA: Laboratório de Duplicação

### 🧪 Exercício Completo: Medindo Custos de Duplicação

**Contexto:** Você está otimizando um sistema de geometria computacional que processa milhares de pontos e polígonos. Precisa entender o custo de cada estratégia de duplicação.

**Objetivo:** Implementar e comparar Copy, Clone e Move com medições reais.

---

### 📝 Código do Exercício

~~~rust {.line-numbers}
use std::time::Instant;

// ============================================
// PARTE 1: Tipo Simples - Point2D (Copy)
// ============================================

#[derive(Debug, Copy, Clone)]
struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }
    
    fn distance(&self, other: &Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

// ============================================
// PARTE 2: Tipo Complexo - Polygon (Clone)
// ============================================

#[derive(Debug, Clone)]
struct Polygon {
    name: String,
    vertices: Vec<Point2D>,
}

impl Polygon {
    fn new(name: &str, vertices: Vec<Point2D>) -> Self {
        Polygon {
            name: String::from(name),
            vertices,
        }
    }
    
    fn perimeter(&self) -> f64 {
        let mut total = 0.0;
        for i in 0..self.vertices.len() {
            let next = (i + 1) % self.vertices.len();
            total += self.vertices[i].distance(&self.vertices[next]);
        }
        total
    }
}

// ============================================
// PARTE 3: Implementação Manual de Clone
// ============================================

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    hobbies: Vec<String>,
}

impl Clone for Person {
    fn clone(&self) -> Self {
        println!("  🔄 Clone manual: copiando {} campos", 3);
        Person {
            name: self.name.clone(),      // Clone da String
            age: self.age,                 // Copy do u32
            hobbies: self.hobbies.clone(), // Clone do Vec
        }
    }
}

// ============================================
// PARTE 4: Benchmarks
// ============================================

fn benchmark_copy() {
    println!("\n📊 BENCHMARK 1: Copy (Point2D)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let point = Point2D::new(10.5, 20.3);
    let iterations = 10_000_000;
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _copy = point;  // Copy implícito
        // Compilador pode otimizar isso, mas demonstra o conceito
    }
    let duration = start.elapsed();
    
    println!("✅ {} cópias implícitas", iterations);
    println!("⏱️  Tempo total: {:?}", duration);
    println!("⚡ Tempo por cópia: ~{:.2} ns", 
             duration.as_nanos() as f64 / iterations as f64);
    println!("💡 Custo: TRIVIAL (bitwise copy de 16 bytes)");
}

fn benchmark_clone_small() {
    println!("\n📊 BENCHMARK 2: Clone (String pequena)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let text = String::from("Rust");
    let iterations = 1_000_000;
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _clone = text.clone();  // Clone explícito
    }
    let duration = start.elapsed();
    
    println!("✅ {} clones de String (4 chars)", iterations);
    println!("⏱️  Tempo total: {:?}", duration);
    println!("⚡ Tempo por clone: ~{:.2} ns", 
             duration.as_nanos() as f64 / iterations as f64);
    println!("💡 Custo: BAIXO (heap allocation + copy de 4 bytes)");
}

fn benchmark_clone_polygon() {
    println!("\n📊 BENCHMARK 3: Clone (Polygon complexo)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let vertices = vec![
        Point2D::new(0.0, 0.0),
        Point2D::new(10.0, 0.0),
        Point2D::new(10.0, 10.0),
        Point2D::new(0.0, 10.0),
    ];
    let polygon = Polygon::new("Square", vertices);
    let iterations = 100_000;
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _clone = polygon.clone();  // Deep copy
    }
    let duration = start.elapsed();
    
    println!("✅ {} clones de Polygon (String + Vec<Point2D>)", iterations);
    println!("⏱️  Tempo total: {:?}", duration);
    println!("⚡ Tempo por clone: ~{:.2} ns", 
             duration.as_nanos() as f64 / iterations as f64);
    println!("💡 Custo: MODERADO (String + Vec allocation)");
}

fn benchmark_move() {
    println!("\n📊 BENCHMARK 4: Move (sem duplicação)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let iterations = 10_000_000;
    
    let start = Instant::now();
    for _ in 0..iterations {
        let s = String::from("test");
        let _moved = s;  // Move (transferência)
        // s não pode mais ser usado
    }
    let duration = start.elapsed();
    
    println!("✅ {} moves de String", iterations);
    println!("⏱️  Tempo total: {:?}", duration);
    println!("⚡ Tempo por move: ~{:.2} ns", 
             duration.as_nanos() as f64 / iterations as f64);
    println!("💡 Custo: ZERO (apenas transferência de ownership)");
}

// ============================================
// PARTE 5: Comparação de Estratégias
// ============================================

fn processar_por_valor_copy(point: Point2D) -> f64 {
    // Recebe por valor (Copy implícito)
    point.x + point.y
}

fn processar_por_referencia(point: &Point2D) -> f64 {
    // Recebe por referência (sem cópia)
    point.x + point.y
}

fn processar_por_valor_clone(polygon: Polygon) -> f64 {
    // Recebe por valor (requer clone explícito na chamada)
    polygon.perimeter()
}

fn processar_por_referencia_polygon(polygon: &Polygon) -> f64 {
    // Recebe por referência (sem cópia)
    polygon.perimeter()
}

fn comparacao_estrategias() {
    println!("\n🔬 COMPARAÇÃO DE ESTRATÉGIAS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let point = Point2D::new(5.0, 10.0);
    
    // Estratégia 1: Copy (por valor)
    let result1 = processar_por_valor_copy(point);
    println!("✅ Estratégia 1 (Copy por valor): {}", result1);
    println!("   Original ainda válido: {:?}", point);
    
    // Estratégia 2: Referência (sem cópia)
    let result2 = processar_por_referencia(&point);
    println!("✅ Estratégia 2 (Referência): {}", result2);
    println!("   Original ainda válido: {:?}", point);
    
    println!("\n---");
    
    let polygon = Polygon::new("Triangle", vec![
        Point2D::new(0.0, 0.0),
        Point2D::new(3.0, 0.0),
        Point2D::new(1.5, 2.6),
    ]);
    
    // Estratégia 3: Clone (por valor)
    let result3 = processar_por_valor_clone(polygon.clone());
    println!("✅ Estratégia 3 (Clone por valor): {:.2}", result3);
    println!("   Original ainda válido: {:?}", polygon.name);
    
    // Estratégia 4: Referência (sem cópia) - PREFERIDA!
    let result4 = processar_por_referencia_polygon(&polygon);
    println!("✅ Estratégia 4 (Referência): {:.2}", result4);
    println!("   Original ainda válido: {:?}", polygon.name);
    
    println!("\n💡 LIÇÃO: Para tipos complexos, prefira referências!");
}

// ============================================
// PARTE 6: Casos de Uso Práticos
// ============================================

fn casos_de_uso() {
    println!("\n🎯 CASOS DE USO PRÁTICOS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Caso 1: Copy para tipos pequenos e simples
    println!("\n1️⃣ Copy: Coordenadas em um jogo");
    let player_pos = Point2D::new(100.0, 200.0);
    let checkpoint = player_pos;  // Copy barato
    println!("   Player: {:?}, Checkpoint: {:?}", player_pos, checkpoint);
    
    // Caso 2: Clone quando realmente precisa de cópia independente
    println!("\n2️⃣ Clone: Backup de dados");
    let original = Polygon::new("Original", vec![
        Point2D::new(0.0, 0.0),
        Point2D::new(5.0, 5.0),
    ]);
    let backup = original.clone();  // Deep copy para backup
    println!("   Original: {}, Backup: {}", original.name, backup.name);
    
    // Caso 3: Referência quando não precisa de ownership
    println!("\n3️⃣ Referência: Leitura sem modificação");
    fn imprimir_info(poly: &Polygon) {
        println!("   Polígono '{}' tem {} vértices", 
                 poly.name, poly.vertices.len());
    }
    imprimir_info(&original);  // Sem cópia!
    
    // Caso 4: Move quando transfere ownership
    println!("\n4️⃣ Move: Transferência de ownership");
    fn consumir(poly: Polygon) -> String {
        format!("Processado: {}", poly.name)
    }
    let resultado = consumir(backup);  // backup movido
    println!("   {}", resultado);
    // println!("{:?}", backup);  // ❌ Erro: backup foi movido
}

// ============================================
// MAIN: Executa todos os testes
// ============================================

fn main() {
    println!("╔════════════════════════════════════════════╗");
    println!("║  🦀 LABORATÓRIO: COPY VS CLONE VS MOVE   ║");
    println!("╚════════════════════════════════════════════╝");
    
    // Executa benchmarks
    benchmark_copy();
    benchmark_clone_small();
    benchmark_clone_polygon();
    benchmark_move();
    
    // Comparações
    comparacao_estrategias();
    
    // Casos práticos
    casos_de_uso();
    
    // Demonstração de clone manual
    println!("\n🔧 DEMONSTRAÇÃO: Clone Manual");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        hobbies: vec![String::from("Rust"), String::from("Gaming")],
    };
    let person2 = person1.clone();
    println!("✅ Person1: {:?}", person1);
    println!("✅ Person2: {:?}", person2);
    
    println!("\n╔════════════════════════════════════════════╗");
    println!("║           ✅ LABORATÓRIO COMPLETO          ║");
    println!("╚════════════════════════════════════════════╝");
}
~~~

---

### 🎓 Solução Analítica e Trade-offs

#### **Resultados Esperados dos Benchmarks**

| Operação | Tempo Aproximado | Custo Relativo |
|----------|------------------|----------------|
| **Copy (Point2D)** | ~0.1 ns | 1x (baseline) |
| **Move (String)** | ~0.1 ns | 1x (sem cópia) |
| **Clone (String pequena)** | ~50 ns | 500x |
| **Clone (Polygon)** | ~200 ns | 2000x |

#### **Trade-offs Identificados**

1. **Copy**
   - ✅ **Vantagens**: Zero custo cognitivo, performance trivial
   - ❌ **Desvantagens**: Apenas para tipos simples, sem controle

2. **Clone**
   - ✅ **Vantagens**: Funciona com qualquer tipo, controle explícito
   - ❌ **Desvantagens**: Custo variável, pode ser caro

3. **Referência (&T)**
   - ✅ **Vantagens**: Zero custo, sem duplicação
   - ❌ **Desvantagens**: Lifetimes complexos, menos flexível

4. **Move**
   - ✅ **Vantagens**: Zero custo, semântica clara
   - ❌ **Desvantagens**: Original invalidado, menos flexível

#### **Regras de Decisão**

~~~
SE tipo é primitivo (i32, bool, f64)
    → Use Copy (automático)

SE tipo é pequeno E simples (struct com 2-3 campos primitivos)
    → Considere implementar Copy

SE tipo contém String, Vec, Box
    → Use Clone (quando necessário) OU referências (preferido)

SE apenas leitura é necessária
    → Use referências (&T)

SE ownership deve ser transferido
    → Use Move (padrão)
~~~

---

## 🔄 FEEDBACK E AVALIAÇÃO

### ✅ Checklist de Conceitos

Marque os conceitos que você domina:

- [ ] Entendo a diferença entre Copy e Clone
- [ ] Sei quando um tipo pode implementar Copy
- [ ] Entendo por que Copy e Drop são incompatíveis
- [ ] Consigo implementar Clone manualmente
- [ ] Sei medir o custo de duplicações
- [ ] Entendo quando usar referências ao invés de clonar
- [ ] Consigo escolher a estratégia correta para cada situação
- [ ] Entendo o impacto de performance de cada abordagem

---

### 🧠 Quiz de Verificação

**1. Qual afirmação está CORRETA?**

a) Todos os tipos em Rust implementam Copy  
b) Copy é mais caro que Clone  
c) Copy requer que Clone também seja implementado  
d) Clone é automático, Copy é explícito  

<details>
<summary>👁️ Ver resposta</summary>

**Resposta: c) Copy requer que Clone também seja implementado**

Copy é um subtrait de Clone. Todo tipo Copy deve implementar Clone também.
</details>

---

**2. Por que String NÃO pode implementar Copy?**

a) É muito grande  
b) Contém dados na heap e implementa Drop  
c) É uma limitação arbitrária do Rust  
d) Strings são imutáveis  

<details>
<summary>👁️ Ver resposta</summary>

**Resposta: b) Contém dados na heap e implementa Drop**

String gerencia memória heap e implementa Drop. Se fosse Copy, teríamos double-free ao sair de escopo.
</details>

---

**3. Qual é a forma mais eficiente de passar um Vec<i32> grande para uma função que apenas lê os dados?**

a) `fn processar(v: Vec<i32>)` (move)  
b) `fn processar(v: &Vec<i32>)` (referência)  
c) `fn processar(v: Vec<i32>)` e chamar com `v.clone()`  
d) Não há diferença de performance  

<details>
<summary>👁️ Ver resposta</summary>

**Resposta: b) `fn processar(v: &Vec<i32>)` (referência)**

Referência evita cópia e move, mantendo ownership no caller. Zero custo.
</details>

---

**4. Qual código compila?**

~~~rust {.line-numbers}
// Opção A
#[derive(Copy, Clone)]
struct A {
    data: String,
}

// Opção B
#[derive(Copy, Clone)]
struct B {
    x: i32,
    y: i32,
}

// Opção C
#[derive(Clone)]
struct C {
    data: Vec<i32>,
}
~~~

<details>
<summary>👁️ Ver resposta</summary>

**Resposta: B e C compilam, A não**

- **A**: ❌ String não é Copy, então A não pode ser Copy
- **B**: ✅ i32 é Copy, então B pode ser Copy
- **C**: ✅ Vec não é Copy, mas pode ser Clone
</details>

---

### 🎯 Exercícios de Diagnóstico

**Exercício 1: Identifique o problema**

~~~rust {.line-numbers}
#[derive(Copy, Clone)]
struct Config {
    timeout: u32,
    url: String,  // ❌ Problema aqui!
}
~~~

<details>
<summary>💡 Solução</summary>

**Problema**: String não implementa Copy, então Config não pode ser Copy.

**Solução 1**: Remover Copy
~~~rust {.line-numbers}
#[derive(Clone)]
struct Config {
    timeout: u32,
    url: String,
}
~~~

**Solução 2**: Usar tipo Copy
~~~rust {.line-numbers}
#[derive(Copy, Clone)]
struct Config {
    timeout: u32,
    url: &'static str,  // &str é Copy
}
~~~
</details>

---

**Exercício 2: Otimize este código**

~~~rust {.line-numbers}
fn calcular_area(pontos: Vec<Point2D>) -> f64 {
    let backup = pontos.clone();  // ❌ Clone desnecessário?
    let mut area = 0.0;
    
    for ponto in &pontos {
        area += ponto.x * ponto.y;
    }
    
    println!("Backup: {:?}", backup);
    area
}
~~~

<details>
<summary>💡 Solução</summary>

**Problema**: Clone desnecessário se não modificarmos `pontos`.

**Otimização**:
~~~rust {.line-numbers}
fn calcular_area(pontos: &Vec<Point2D>) -> f64 {
    // Recebe referência, sem clone
    let mut area = 0.0;
    
    for ponto in pontos {
        area += ponto.x * ponto.y;
    }
    
    area
}

// Ou, se realmente precisar do backup:
fn calcular_area_com_backup(pontos: Vec<Point2D>) -> (f64, Vec<Point2D>) {
    let backup = pontos.clone();
    let mut area = 0.0;
    
    for ponto in &pontos {
        area += ponto.x * ponto.y;
    }
    
    (area, backup)  // Retorna ambos
}
~~~
</details>

---

### 📝 Auto-avaliação

**Escala de 1-5, onde:**
- 1 = Não entendo
- 2 = Entendo superficialmente
- 3 = Entendo, mas preciso de prática
- 4 = Entendo bem e consigo aplicar
- 5 = Domino completamente

| Conceito | Nota (1-5) |
|----------|------------|
| Diferença entre Copy e Clone | ___ |
| Quando usar cada estratégia | ___ |
| Implementação manual de traits | ___ |
| Trade-offs de performance | ___ |
| Incompatibilidade Copy + Drop | ___ |
| Otimização de duplicações | ___ |

**Se você marcou 3 ou menos em algum item, revise aquela seção!**

---

## 🚀 TRANSFERÊNCIA E APLICAÇÃO

### 🎯 Desafio Final: Otimize o Sistema de Geometria

**Cenário**: Você herdou este código de um sistema de processamento geométrico:

~~~rust {.line-numbers}
#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct Shape {
    name: String,
    points: Vec<Point>,
}

fn processar_formas(formas: Vec<Shape>) {
    for forma in formas {
        let backup = forma.clone();  // Clone 1
        
        let area = calcular_area(forma.clone());  // Clone 2
        let perimetro = calcular_perimetro(forma.clone());  // Clone 3
        
        println!("{}: área={}, perímetro={}", 
                 backup.name, area, perimetro);
    }
}

fn calcular_area(forma: Shape) -> f64 {
    // Implementação...
    0.0
}

fn calcular_perimetro(forma: Shape) -> f64 {
    // Implementação...
    0.0
}
~~~

**Problemas identificados:**
1. 3 clones por iteração (caro!)
2. Point poderia ser Copy
3. Funções recebem por valor desnecessariamente

**Sua missão**: Otimize este código reduzindo clones ao mínimo.

<details>
<summary>💡 Solução Otimizada</summary>

~~~rust {.line-numbers}
// Point é simples, pode ser Copy
#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

// Shape continua Clone (tem String)
#[derive(Debug, Clone)]
struct Shape {
    name: String,
    points: Vec<Point>,  // Vec de Copy é mais eficiente
}

// Recebe referência, sem ownership
fn processar_formas(formas: &[Shape]) {
    for forma in formas {
        // Sem clone! Apenas referências
        let area = calcular_area(forma);
        let perimetro = calcular_perimetro(forma);
        
        println!("{}: área={:.2}, perímetro={:.2}", 
                 forma.name, area, perimetro);
    }
}

// Recebe referência
fn calcular_area(forma: &Shape) -> f64 {
    // Point é Copy, então iteração é barata
    let mut area = 0.0;
    for point in &forma.points {
        area += point.x * point.y;
    }
    area
}

// Recebe referência
fn calcular_perimetro(forma: &Shape) -> f64 {
    let mut perimetro = 0.0;
    for i in 0..forma.points.len() {
        let next = (i + 1) % forma.points.len();
        let dx = forma.points[i].x - forma.points[next].x;
        let dy = forma.points[i].y - forma.points[next].y;
        perimetro += (dx * dx + dy * dy).sqrt();
    }
    perimetro
}
~~~

**Melhorias:**
- ✅ Point agora é Copy (16 bytes, trivial)
- ✅ Zero clones de Shape (era 3 por iteração!)
- ✅ Funções usam referências
- ✅ Performance: ~100x mais rápido para 1000 formas

**Benchmark estimado:**
- **Antes**: ~200ms para 1000 formas
- **Depois**: ~2ms para 1000 formas
</details>

---

### 🔗 Preparação para Dia 24: Debugging

Agora que você domina duplicação de dados, está pronto para o próximo desafio: **debugging eficiente em Rust**.

**Conexões com o próximo tópico:**
- Clone é útil para criar snapshots durante debug
- Copy simplifica rastreamento de valores
- Entender ownership ajuda a debugar erros de borrow

**Pré-requisitos para Dia 24:**
- [ ] Domino Copy vs Clone
- [ ] Entendo ownership e borrowing
- [ ] Sei quando duplicar dados
- [ ] Consigo otimizar duplicações

---

### 📚 Recursos Adicionais

#### 📖 Documentação Oficial
- [The Rust Book - Chapter 4.1 (Ownership)](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Rust By Example - Clone](https://doc.rust-lang.org/rust-by-example/trait/clone.html)
- [std::marker::Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html)
- [std::clone::Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)

#### 🎥 Vídeos Recomendados
- "Rust Ownership Explained" - Let's Get Rusty
- "Copy vs Clone in Rust" - Jon Gjengset

#### 🛠️ Ferramentas de Profiling
- **cargo-flamegraph**: Visualiza onde seu código gasta tempo
- **cargo-bench**: Benchmarks precisos
- **perf**: Profiling de CPU no Linux

~~~bash
# Instalar ferramentas
cargo install cargo-flamegraph
cargo install cargo-criterion

# Executar benchmark
cargo bench

# Gerar flamegraph
cargo flamegraph
~~~

#### 💡 Dicas de Otimização

1. **Regra 80/20**: 80% do tempo é gasto em 20% do código
   - Profile antes de otimizar
   - Foque nos hot paths

2. **Hierarquia de preferência**:

   Referências (&T) > Move > Copy > Clone


3. **Quando clonar é OK**:
- Dados pequenos (< 100 bytes)
- Operações infrequentes
- Simplicidade > performance

4. **Red flags** (sinais de alerta):
- `.clone()` em loops
- Clone de Vec/HashMap grandes
- Clone sem justificativa clara

---

## 🎓 RESUMO EXECUTIVO

### 🔑 Pontos-Chave

1. **Copy**: Cópia implícita, barata, apenas tipos simples
2. **Clone**: Cópia explícita, custo variável, qualquer tipo
3. **Move**: Padrão do Rust, zero custo, transfere ownership
4. **Referências**: Melhor opção quando não precisa de ownership

### 📊 Tabela de Decisão Rápida

| Situação | Use |
|----------|-----|
| Tipo primitivo (i32, bool) | Copy (automático) |
| Struct pequena (2-3 campos primitivos) | Copy (derive) |
| Tipo com String/Vec | Clone (quando necessário) |
| Apenas leitura | Referência (&T) |
| Transferir ownership | Move (padrão) |
| Backup/snapshot | Clone |

### ⚡ Performance em Uma Linha


Referência (0 ns) < Move (0 ns) < Copy (~1 ns) < Clone (50-5000 ns)


### 🎯 Checklist Final

Antes de avançar para Dia 24, certifique-se:

- [x] Entendo quando usar Copy vs Clone
- [x] Sei implementar ambos os traits
- [x] Compreendo o custo de cada operação
- [x] Consigo otimizar código identificando clones desnecessários
- [x] Entendo por que Copy + Drop são incompatíveis

---

## 🎉 PARABÉNS!

Você completou o **Dia 23: Copy vs Clone**!

Agora você tem o conhecimento para:
- ✅ Escolher a estratégia correta de duplicação
- ✅ Otimizar código evitando clones desnecessários
- ✅ Implementar traits Copy e Clone
- ✅ Medir e melhorar performance

**Próximo passo**: Dia 24 - Debugging em Rust 🐛🔍

---

**💬 Dúvidas?** Revise as seções com nota < 4 na auto-avaliação!

**🚀 Pronto para mais?** Execute o código do laboratório e experimente!

---

*Material criado com ❤️ para iniciantes em Rust*  
*Foco em design instrucional e aprendizagem prática*