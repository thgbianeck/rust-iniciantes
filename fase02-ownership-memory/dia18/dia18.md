# 🦀 Dia 18: Slices - Janelas Eficientes para Dados

## 📋 Objetivos de Aprendizagem

Ao final desta lição, você será capaz de:

✅ **Compreender** o conceito de slice como "view" sem ownership  
✅ **Diferenciar** String vs &str e escolher apropriadamente  
✅ **Utilizar** todos os tipos de ranges (.., ..=, a.., etc)  
✅ **Criar** parsers eficientes usando slices  
✅ **Otimizar** código evitando cópias desnecessárias  

---

## 🎭 Ativação do Conhecimento Prévio

### 🔄 Revisão Rápida: Borrowing

Lembra do que aprendemos sobre borrowing?

- `&T` = referência imutável (emprestar para ler)
- `&mut T` = referência mutável (emprestar para modificar)
- Não há ownership, apenas acesso temporário

**Slices são uma evolução natural desse conceito!**

---

### 🪟 Analogia Central: A Janela Mágica

Imagine que você tem um **prédio inteiro** (String ou Vec):

~~~
┌─────────────────────────────────┐
│  P R É D I O   I N T E I R O    │ ← String (você é o dono)
│                                 │
│  [H][e][l][l][o][ ][W][o][r][l][d]
│                                 │
└─────────────────────────────────┘
~~~

Agora você quer mostrar apenas uma **parte** para alguém. Você tem duas opções:

**❌ Opção 1: Construir um prédio novo (cópia)**
- Caro (tempo e memória)
- Desnecessário se só quer "olhar"

**✅ Opção 2: Abrir uma JANELA (slice)**
- Rápido e eficiente
- Apenas uma "view" para parte dos dados
- Zero custo de cópia!

~~~
┌─────────────────────────────────┐
│  [H][e][l][l][o][ ][W][o][r][l][d]
│         ╔═══════════╗           │
│         ║ JANELA    ║           │ ← &str (slice)
│         ║ [W][o][r] ║           │
│         ╚═══════════╝           │
└─────────────────────────────────┘
~~~

**Slices são janelas eficientes para seus dados!**

---

### 📖 História: O Bibliotecário Eficiente

Era uma vez um bibliotecário que precisava mostrar trechos de livros para visitantes.

**Bibliotecário Inexperiente:**
- Cada vez que alguém pedia "páginas 10-20"
- Ele COPIAVA essas páginas em um novo livro
- Gastava papel, tempo e dinheiro

**Bibliotecário Expert (usando Slices):**
- Simplesmente APONTAVA: "Olhe aqui, páginas 10-20"
- Visitante via o conteúdo original
- Zero desperdício, velocidade máxima!

**Rust te transforma no bibliotecário expert! 🎯**

---

## 📚 Apresentação do Conteúdo

### 1️⃣ O que é um Slice?

**Definição Técnica:**
> Um slice é uma **referência a uma sequência contígua de elementos** em uma coleção, sem possuir ownership dos dados.

**Definição Simples:**
> Um slice é uma **janela** que permite ver parte de uma String, Vec ou array, sem copiar nada.

**Sintaxe:**
- `&str` → slice de String (texto)
- `&[T]` → slice de Vec<T> ou array

---

### 2️⃣ Diagrama: Memória de String com Slice

~~~mermaid
graph TB
    subgraph STACK["🗄️ STACK - Variáveis"]
        texto["<b>texto: String</b><br/>ptr: 0x1000<br/>len: 11<br/>capacity: 11"]
        slice["<b>slice: &str</b><br/>ptr: 0x1006<br/>len: 5"]
    end
    
    subgraph HEAP["💾 HEAP - Memória Dinâmica"]
        h["H<br/>0"]
        e1["e<br/>1"]
        l1["l<br/>2"]
        l2["l<br/>3"]
        o["o<br/>4"]
        space["_<br/>5"]
        r["R<br/>6"]
        u["u<br/>7"]
        s["s<br/>8"]
        t["t<br/>9"]
        excl["!<br/>10"]
    end
    
    texto -.->|"aponta para início"| h
    slice -.->|"aponta para posição 6"| r
    
    style texto fill:#4CAF50,stroke:#2E7D32,color:#fff
    style slice fill:#2196F3,stroke:#1565C0,color:#fff
    style HEAP fill:#FFF3E0,stroke:#E65100
    style STACK fill:#E8F5E9,stroke:#2E7D32
    style r fill:#FFEB3B,stroke:#F57F17
    style u fill:#FFEB3B,stroke:#F57F17
    style s fill:#FFEB3B,stroke:#F57F17
    style t fill:#FFEB3B,stroke:#F57F17
    style excl fill:#FFEB3B,stroke:#F57F17
~~~

**Pontos-chave:**
- String possui ownership (ptr + len + capacity)
- &str é apenas ptr + len (sem ownership!)
- Slice aponta para dados existentes
- **Zero cópias = Zero custo!**

---

### 3️⃣ String vs &str: Comparação Visual

~~~mermaid
graph LR
    subgraph String["<b>String</b> - Ownership"]
        s1["✓ Mutável"]
        s2["✓ Pode crescer"]
        s3["✓ Heap alocado"]
        s4["✗ Custa alocar"]
        s5["📦 Dono dos dados"]
    end
    
    subgraph Str["<b>&str</b> - Borrowed View"]
        t1["✗ Imutável"]
        t2["✗ Tamanho fixo"]
        t3["✓ Aponta para dados"]
        t4["✓ Zero custo"]
        t5["👁️ Apenas visualiza"]
    end
    
    style String fill:#4CAF50,stroke:#2E7D32,color:#fff
    style Str fill:#2196F3,stroke:#1565C0,color:#fff
~~~

**Quando usar cada um:**

| Situação | Use String | Use &str |
|----------|-----------|----------|
| Parâmetro de função | ❌ | ✅ (mais flexível) |
| Precisa modificar | ✅ | ❌ |
| Precisa crescer | ✅ | ❌ |
| Apenas leitura | ❌ | ✅ (mais eficiente) |
| Retornar substring | ❌ | ✅ (zero cópia) |

**Regra de Ouro:**
> Use `&str` para parâmetros de função (mais flexível)  
> Use `String` quando precisa ownership ou modificação

---

### 4️⃣ Ranges: Todos os Tipos

Ranges definem **qual parte** você quer ver através da janela:

| Sintaxe | Significado | Exemplo | Resultado |
|---------|-------------|---------|-----------|
| `..` | Tudo | `&texto[..]` | String inteira |
| `a..b` | De a até b (exclusivo) | `&texto[0..5]` | Índices 0,1,2,3,4 |
| `a..=b` | De a até b (inclusivo) | `&texto[0..=4]` | Índices 0,1,2,3,4 |
| `a..` | De a até o fim | `&texto[6..]` | Do índice 6 ao fim |
| `..b` | Do início até b | `&texto[..5]` | Do início até 4 |

**Exemplo Visual:**

~~~
Texto: "Hello Rust!"
       0123456789...

&texto[..]      → "Hello Rust!"  (tudo)
&texto[0..5]    → "Hello"        (0,1,2,3,4)
&texto[0..=4]   → "Hello"        (0,1,2,3,4)
&texto[6..]     → "Rust!"        (6 até fim)
&texto[..5]     → "Hello"        (início até 4)
&texto[6..10]   → "Rust"         (6,7,8,9)
~~~

~~~mermaid
graph TD
    A["Texto: 'Hello Rust!'<br/>Índices: 0-10"]
    
    A --> B[".. <br/>Tudo"]
    A --> C["a..b<br/>Exclusivo"]
    A --> D["a..=b<br/>Inclusivo"]
    A --> E["a..<br/>Do índice ao fim"]
    A --> F["..b<br/>Do início ao índice"]
    
    B --> B1["[..]<br/>'Hello Rust!'"]
    C --> C1["[0..5]<br/>'Hello'"]
    D --> D1["[0..=4]<br/>'Hello'"]
    E --> E1["[6..]<br/>'Rust!'"]
    F --> F1["[..5]<br/>'Hello'"]
    
    style A fill:#9C27B0,stroke:#4A148C,color:#fff
    style B fill:#4CAF50,stroke:#2E7D32,color:#fff
    style C fill:#2196F3,stroke:#1565C0,color:#fff
    style D fill:#FF9800,stroke:#E65100,color:#fff
    style E fill:#F44336,stroke:#B71C1C,color:#fff
    style F fill:#00BCD4,stroke:#006064,color:#fff
~~~

---

### 5️⃣ Diagrama: Camadas de Abstração

~~~mermaid
graph TB
    subgraph Layer1["🏢 OWNERSHIP - Responsável pela Memória"]
        own["String, Vec&lt;T&gt;<br/>• Aloca/Desaloca<br/>• Pode crescer<br/>• Dono dos dados"]
    end
    
    subgraph Layer2["🤝 BORROWING - Acesso Temporário"]
        bor["&String, &Vec&lt;T&gt;<br/>• Empresta tudo<br/>• Não modifica tamanho<br/>• Acesso completo"]
    end
    
    subgraph Layer3["🪟 SLICES - View Parcial"]
        slc["&str, &[T]<br/>• View de PARTE<br/>• Mais flexível<br/>• Zero-cost abstraction"]
    end
    
    Layer1 --> Layer2
    Layer2 --> Layer3
    
    style Layer1 fill:#4CAF50,stroke:#2E7D32
    style Layer2 fill:#2196F3,stroke:#1565C0
    style Layer3 fill:#FF9800,stroke:#E65100
    style own fill:#81C784,stroke:#2E7D32,color:#000
    style bor fill:#64B5F6,stroke:#1565C0,color:#000
    style slc fill:#FFB74D,stroke:#E65100,color:#000
~~~

**Slices são a camada mais flexível e eficiente!**

---

### 6️⃣ Criando Slices: Sequência Passo a Passo

~~~mermaid
sequenceDiagram
    participant P as Programador
    participant S as String
    participant H as HEAP
    participant SL as Slice
    
    Note over P,H: PASSO 1: Criar String
    P->>S: let texto = String::from("Rust")
    S->>H: Aloca memória
    H-->>S: [R][u][s][t]
    
    Note over P,SL: PASSO 2: Criar Slice
    P->>SL: let slice = &texto[0..2]
    SL->>H: Aponta para posição 0
    Note over SL: ptr + len (sem ownership!)
    
    Note over P,SL: PASSO 3: Usar Slice
    P->>SL: println!("{}", slice)
    SL-->>P: "Ru" (view, não cópia!)
    
    Note over P,H: PASSO 4: Slice válido enquanto texto existir
    Note over S,SL: ✓ Ambos válidos
~~~

**Pontos importantes:**
1. String aloca memória no HEAP
2. Slice apenas aponta (não aloca!)
3. Slice é válido enquanto String existir
4. Zero cópias = máxima eficiência

---

### 7️⃣ Performance: Copy vs View

~~~mermaid
graph TB
    subgraph Cenario["📊 Cenário: 1000 substrings de texto 1MB"]
    end
    
    subgraph Abordagem1["❌ ABORDAGEM 1: String::from - Copiar"]
        a1["⏱️ Tempo: 45ms"]
        a2["💾 Memória: 1000 alocações"]
        a3["💰 Custo: ALTO 🔴"]
        a4["for i in 0..1000 {<br/>  let copia = String::from(&texto[i..i+100])<br/>  // COPIA 100 bytes!<br/>}"]
    end
    
    subgraph Abordagem2["✅ ABORDAGEM 2: &str - Slice"]
        b1["⏱️ Tempo: 0.5ms"]
        b2["💾 Memória: 0 alocações"]
        b3["💰 Custo: ZERO 🟢"]
        b4["for i in 0..1000 {<br/>  let slice = &texto[i..i+100]<br/>  // Apenas ponteiro!<br/>}"]
    end
    
    Cenario --> Abordagem1
    Cenario --> Abordagem2
    
    Abordagem2 -.->|"90x mais rápido!"| Resultado["🏆 VENCEDOR"]
    
    style Abordagem1 fill:#FFCDD2,stroke:#C62828
    style Abordagem2 fill:#C8E6C9,stroke:#2E7D32
    style Resultado fill:#FFD54F,stroke:#F57F17
    style a3 fill:#F44336,color:#fff
    style b3 fill:#4CAF50,color:#fff
~~~

**Conclusão:** Slices são **90x mais rápidos** e usam **zero memória extra**!

---

## 💡 Demonstração e Modelagem

### Exemplo 1: String vs &str em Funções

**❌ Versão Inflexível (aceita apenas String):**

~~~rust {.line-numbers}
fn imprimir_saudacao(nome: String) {
    println!("Olá, {}!", nome);
}

fn main() {
    let nome = String::from("Alice");
    imprimir_saudacao(nome); // Move ownership!
    // println!("{}", nome); // ❌ ERRO! nome foi movido
    
    let literal = "Bob";
    // imprimir_saudacao(literal); // ❌ ERRO! &str não é String
}
~~~

**✅ Versão Flexível (aceita &str):**

~~~rust {.line-numbers}
fn imprimir_saudacao(nome: &str) {
    println!("Olá, {}!", nome);
}

fn main() {
    let nome = String::from("Alice");
    imprimir_saudacao(&nome); // Empresta! ✓
    println!("{}", nome); // ✓ Ainda posso usar!
    
    let literal = "Bob";
    imprimir_saudacao(literal); // ✓ Funciona direto!
    
    let slice = &nome[0..3];
    imprimir_saudacao(slice); // ✓ Slice também funciona!
}
~~~

**Lição:** `&str` como parâmetro aceita String, &str e slices!

---

### Exemplo 2: Slices de Vec

Slices não são só para Strings!

~~~rust {.line-numbers}
fn main() {
    let numeros = vec![10, 20, 30, 40, 50];
    
    // Slice de Vec<i32> = &[i32]
    let slice = &numeros[1..4]; // [20, 30, 40]
    
    println!("Slice: {:?}", slice);
    println!("Primeiro: {}", slice[0]); // 20
    println!("Tamanho: {}", slice.len()); // 3
    
    // Passar slice para função
    imprimir_slice(slice);
    imprimir_slice(&numeros[..]); // Vec inteiro como slice
}

fn imprimir_slice(dados: &[i32]) {
    for num in dados {
        print!("{} ", num);
    }
    println!();
}
~~~

**Saída:**
~~~
Slice: [20, 30, 40]
Primeiro: 20
Tamanho: 3
20 30 40 
10 20 30 40 50 
~~~

---

### Exemplo 3: Métodos Úteis de Slices

~~~rust {.line-numbers}
fn main() {
    let texto = "Rust é incrível!";
    
    // Métodos de &str
    println!("Tamanho: {}", texto.len()); // 17
    println!("Vazio? {}", texto.is_empty()); // false
    println!("Contém 'Rust'? {}", texto.contains("Rust")); // true
    println!("Começa com 'Rust'? {}", texto.starts_with("Rust")); // true
    println!("Termina com '!'? {}", texto.ends_with("!")); // true
    
    // Split (retorna iterador de slices!)
    for palavra in texto.split(' ') {
        println!("Palavra: {}", palavra);
    }
    
    // Trim (remove espaços)
    let com_espacos = "  Rust  ";
    println!("Trimmed: '{}'", com_espacos.trim()); // "Rust"
    
    // To uppercase/lowercase (retorna String)
    println!("Maiúsculas: {}", texto.to_uppercase());
}
~~~

**Saída:**
~~~
Tamanho: 17
Vazio? false
Contém 'Rust'? true
Começa com 'Rust'? true
Termina com '!'? true
Palavra: Rust
Palavra: é
Palavra: incrível!
Trimmed: 'Rust'
Maiúsculas: RUST É INCRÍVEL!
~~~

---

### Exemplo 4: Padrões Comuns com Ranges

~~~rust {.line-numbers}
fn main() {
    let dados = "ABCDEFGHIJ";
    
    // Primeiros 3 caracteres
    let inicio = &dados[..3];
    println!("Início: {}", inicio); // "ABC"
    
    // Últimos 3 caracteres
    let fim = &dados[dados.len()-3..];
    println!("Fim: {}", fim); // "HIJ"
    
    // Meio (sem início e fim)
    let meio = &dados[3..7];
    println!("Meio: {}", meio); // "DEFG"
    
    // Tudo
    let tudo = &dados[..];
    println!("Tudo: {}", tudo); // "ABCDEFGHIJ"
    
    // Usando variáveis
    let start = 2;
    let end = 5;
    let custom = &dados[start..end];
    println!("Custom: {}", custom); // "CDE"
}
~~~

---

### Exemplo 5: Slice Mutável

Você pode ter slices mutáveis também!

~~~rust {.line-numbers}
fn main() {
    let mut numeros = vec![1, 2, 3, 4, 5];
    
    // Slice mutável
    let slice = &mut numeros[1..4];
    
    // Modificar através do slice
    slice[0] = 20;
    slice[1] = 30;
    slice[2] = 40;
    
    println!("Números: {:?}", numeros); // [1, 20, 30, 40, 5]
}

fn zerar_slice(dados: &mut [i32]) {
    for num in dados {
        *num = 0;
    }
}
~~~

---

## 🎯 Prática Guiada: Parser de Texto com Slices

### 📝 Contexto do Exercício

Você vai criar um **parser eficiente** que processa texto sem fazer cópias desnecessárias. Imagine que você está construindo um analisador de logs ou um processador de configurações.

**Requisitos:**
1. Ler entrada de texto
2. Extrair primeira e última palavra
3. Dividir texto em palavras
4. Parsear formato "chave:valor"
5. Contar palavras
6. Tudo usando slices (zero cópias!)

---

### 🔧 Exercício Completo: Sistema de Parser

~~~rust {.line-numbers}
// ========================================
// PARSER DE TEXTO COM SLICES
// ========================================

fn main() {
    println!("=== PARSER DE TEXTO COM SLICES ===\n");
    
    // Texto de exemplo
    let texto = "Rust é uma linguagem de programação moderna";
    
    // 1. Primeira e última palavra
    println!("1. PRIMEIRA E ÚLTIMA PALAVRA:");
    if let Some(primeira) = primeira_palavra(texto) {
        println!("   Primeira: '{}'", primeira);
    }
    if let Some(ultima) = ultima_palavra(texto) {
        println!("   Última: '{}'", ultima);
    }
    println!();
    
    // 2. Contar palavras
    println!("2. CONTAGEM:");
    println!("   Total de palavras: {}", contar_palavras(texto));
    println!();
    
    // 3. Listar todas as palavras
    println!("3. TODAS AS PALAVRAS:");
    for (i, palavra) in extrair_palavras(texto).enumerate() {
        println!("   [{}] '{}'", i, palavra);
    }
    println!();
    
    // 4. Parser de configuração (formato chave:valor)
    println!("4. PARSER DE CONFIGURAÇÃO:");
    let configs = vec![
        "nome:Rust",
        "versao:1.75",
        "tipo:linguagem",
        "paradigma:multi",
    ];
    
    for config in configs {
        if let Some((chave, valor)) = parsear_config(config) {
            println!("   {} = {}", chave, valor);
        }
    }
    println!();
    
    // 5. Extrair substring segura
    println!("5. SUBSTRING SEGURA:");
    if let Some(sub) = substring_segura(texto, 0, 4) {
        println!("   [0..4]: '{}'", sub);
    }
    if let Some(sub) = substring_segura(texto, 5, 7) {
        println!("   [5..7]: '{}'", sub);
    }
    println!();
    
    // 6. Comparação de performance
    println!("6. COMPARAÇÃO DE PERFORMANCE:");
    comparar_performance();
}

// ========================================
// FUNÇÕES DO PARSER
// ========================================

/// Extrai a primeira palavra do texto
fn primeira_palavra(texto: &str) -> Option<&str> {
    texto.split_whitespace().next()
}

/// Extrai a última palavra do texto
fn ultima_palavra(texto: &str) -> Option<&str> {
    texto.split_whitespace().last()
}

/// Conta o número de palavras
fn contar_palavras(texto: &str) -> usize {
    texto.split_whitespace().count()
}

/// Retorna iterador de palavras (slices!)
fn extrair_palavras(texto: &str) -> impl Iterator<Item = &str> {
    texto.split_whitespace()
}

/// Parseia formato "chave:valor"
/// Retorna tuple de slices (zero cópias!)
fn parsear_config(linha: &str) -> Option<(&str, &str)> {
    // Encontrar posição do ':'
    let pos = linha.find(':')?;
    
    // Dividir em dois slices
    let chave = &linha[..pos];
    let valor = &linha[pos + 1..];
    
    Some((chave, valor))
}

/// Extrai substring com verificação de bounds
fn substring_segura(texto: &str, inicio: usize, fim: usize) -> Option<&str> {
    if fim <= texto.len() && inicio <= fim {
        Some(&texto[inicio..fim])
    } else {
        None
    }
}

/// Compara performance: String vs &str
fn comparar_performance() {
    use std::time::Instant;
    
    let texto = "Rust é incrível e muito performático!".repeat(1000);
    let iteracoes = 10000;
    
    // Teste 1: Usando String (com cópias)
    let inicio = Instant::now();
    for _ in 0..iteracoes {
        let _copia = String::from(&texto[0..20]);
    }
    let tempo_string = inicio.elapsed();
    
    // Teste 2: Usando slice (sem cópias)
    let inicio = Instant::now();
    for _ in 0..iteracoes {
        let _slice = &texto[0..20];
    }
    let tempo_slice = inicio.elapsed();
    
    println!("   String (com cópia): {:?}", tempo_string);
    println!("   Slice (sem cópia): {:?}", tempo_slice);
    println!("   Speedup: {:.2}x mais rápido!", 
             tempo_string.as_nanos() as f64 / tempo_slice.as_nanos() as f64);
}
~~~

---

### 📊 Saída Esperada

~~~
=== PARSER DE TEXTO COM SLICES ===

1. PRIMEIRA E ÚLTIMA PALAVRA:
   Primeira: 'Rust'
   Última: 'moderna'

2. CONTAGEM:
   Total de palavras: 7

3. TODAS AS PALAVRAS:
   [0] 'Rust'
   [1] 'é'
   [2] 'uma'
   [3] 'linguagem'
   [4] 'de'
   [5] 'programação'
   [6] 'moderna'

4. PARSER DE CONFIGURAÇÃO:
   nome = Rust
   versao = 1.75
   tipo = linguagem
   paradigma = multi

5. SUBSTRING SEGURA:
   [0..4]: 'Rust'
   [5..7]: 'é '

6. COMPARAÇÃO DE PERFORMANCE:
   String (com cópia): 2.5ms
   Slice (sem cópia): 25µs
   Speedup: 100.00x mais rápido!
~~~

---

### 🎓 Análise da Solução

~~~mermaid
graph TB
    subgraph Eficiencia["✅ Por que esta solução é eficiente?"]
        e1["1️⃣ Zero Cópias<br/>Todas funções retornam &str"]
        e2["2️⃣ Composabilidade<br/>Funções pequenas e reutilizáveis"]
        e3["3️⃣ Segurança<br/>Verificação de bounds"]
        e4["4️⃣ Performance<br/>100x mais rápido"]
        e5["5️⃣ Flexibilidade<br/>Aceita qualquer &str"]
    end
    
    subgraph Padroes["📐 Padrões Importantes"]
        p1["✅ BOM: fn processar(texto: &str) -> &str"]
        p2["❌ RUIM: fn processar(texto: &str) -> String"]
        p3["✅ BOM: Retorna Option para segurança"]
    end
    
    Eficiencia --> Padroes
    
    style e1 fill:#4CAF50,stroke:#2E7D32,color:#fff
    style e2 fill:#4CAF50,stroke:#2E7D32,color:#fff
    style e3 fill:#4CAF50,stroke:#2E7D32,color:#fff
    style e4 fill:#4CAF50,stroke:#2E7D32,color:#fff
    style e5 fill:#4CAF50,stroke:#2E7D32,color:#fff
    style p1 fill:#81C784,stroke:#2E7D32,color:#000
    style p2 fill:#EF5350,stroke:#C62828,color:#fff
    style p3 fill:#81C784,stroke:#2E7D32,color:#000
~~~

**Padrões de Código:**

~~~rust {.line-numbers}
// ✅ BOM: Retorna slice
fn processar(texto: &str) -> &str {
    &texto[0..5]
}

// ❌ RUIM: Copia desnecessariamente
fn processar_ruim(texto: &str) -> String {
    String::from(&texto[0..5])
}

// ✅ BOM: Retorna Option para segurança
fn extrair_seguro(texto: &str, pos: usize) -> Option<&str> {
    if pos < texto.len() {
        Some(&texto[pos..])
    } else {
        None
    }
}
~~~

---

### 🚀 Desafio Extra: Parser Avançado

Tente implementar estas funções adicionais:

~~~rust {.line-numbers}
/// Remove prefixo se existir
fn remover_prefixo<'a>(texto: &'a str, prefixo: &str) -> &'a str {
    // Seu código aqui
    todo!()
}

/// Remove sufixo se existir
fn remover_sufixo<'a>(texto: &'a str, sufixo: &str) -> &'a str {
    // Seu código aqui
    todo!()
}

/// Extrai texto entre delimitadores
fn extrair_entre<'a>(texto: &'a str, inicio: &str, fim: &str) -> Option<&'a str> {
    // Exemplo: extrair_entre("(hello)", "(", ")") -> Some("hello")
    // Seu código aqui
    todo!()
}

/// Divide em N partes iguais
fn dividir_em_partes(texto: &str, n: usize) -> Vec<&str> {
    // Seu código aqui
    todo!()
}
~~~

**Dica:** Use métodos como `strip_prefix()`, `strip_suffix()`, `find()` e ranges!

---

## 🔄 Feedback e Avaliação

### ✅ Checklist de Slices

Marque o que você já domina:

- [ ] Sei explicar o que é um slice
- [ ] Entendo a diferença entre String e &str
- [ ] Sei usar todos os tipos de ranges (.., ..=, etc)
- [ ] Consigo criar slices de String e Vec
- [ ] Sei quando usar &str vs String em funções
- [ ] Entendo que slices não copiam dados
- [ ] Consigo usar métodos de slices (split, trim, etc)
- [ ] Sei criar parsers eficientes com slices
- [ ] Entendo o conceito de "view" vs "ownership"
- [ ] Consigo usar slices mutáveis (&mut [T])

---

### 🧠 Quiz Rápido

~~~mermaid
graph TD
    Q1["❓ Questão 1:<br/>Qual é mais eficiente?"]
    Q1A["A: String::from(&texto[0..10])"]
    Q1B["B: &texto[0..10]"]
    
    Q2["❓ Questão 2:<br/>Qual função é mais flexível?"]
    Q2A["A: fn processar(texto: String)"]
    Q2B["B: fn processar(texto: &str)"]
    
    Q3["❓ Questão 3:<br/>O que imprime?<br/>let slice = &'Rust'[1..3]"]
    Q3R["Resposta: 'us'"]
    
    Q1 --> Q1A
    Q1 --> Q1B
    Q1B -.->|"✅ CORRETO"| R1["Slice não copia!"]
    
    Q2 --> Q2A
    Q2 --> Q2B
    Q2B -.->|"✅ CORRETO"| R2["Aceita String, &str e slices"]
    
    Q3 --> Q3R
    
    style Q1B fill:#4CAF50,color:#fff
    style Q2B fill:#4CAF50,color:#fff
    style R1 fill:#81C784,color:#000
    style R2 fill:#81C784,color:#000
    style Q3R fill:#FFD54F,color:#000
~~~

**Questão 4:** Qual é o tipo de retorno?
~~~rust {.line-numbers}
fn primeira_palavra(texto: &str) -> ??? {
    texto.split_whitespace().next()
}
~~~

<details>
<summary>Resposta</summary>
Option<&str> - pode não haver palavras!
</details>

---

**Questão 5:** Este código compila?
~~~rust {.line-numbers}
let mut texto = String::from("Rust");
let slice = &texto[..];
texto.push_str(" é legal");
println!("{}", slice);
~~~

<details>
<summary>Resposta</summary>
❌ NÃO! Você não pode modificar texto enquanto slice existe (borrow checker).
</details>

---

### 🎯 Exercícios de Otimização

**Exercício 1: Otimize este código**

~~~rust {.line-numbers}
// ❌ Versão ineficiente
fn contar_vogais(texto: &str) -> usize {
    let mut count = 0;
    let texto_copia = String::from(texto); // Cópia desnecessária!
    for c in texto_copia.chars() {
        if "aeiouAEIOU".contains(c) {
            count += 1;
        }
    }
    count
}
~~~

<details>
<summary>Solução Otimizada</summary>

~~~rust {.line-numbers}
// ✅ Versão eficiente
fn contar_vogais(texto: &str) -> usize {
    texto.chars()
        .filter(|c| "aeiouAEIOU".contains(*c))
        .count()
}
~~~
</details>

---

**Exercício 2: Implemente sem cópias**

~~~rust {.line-numbers}
/// Retorna as primeiras N palavras de um texto
/// Sem fazer cópias!
fn primeiras_n_palavras(texto: &str, n: usize) -> Vec<&str> {
    // Seu código aqui
    todo!()
}

// Teste
let texto = "Rust é uma linguagem incrível";
let palavras = primeiras_n_palavras(texto, 3);
assert_eq!(palavras, vec!["Rust", "é", "uma"]);
~~~

<details>
<summary>Solução</summary>

~~~rust {.line-numbers}
fn primeiras_n_palavras(texto: &str, n: usize) -> Vec<&str> {
    texto.split_whitespace()
        .take(n)
        .collect()
}
~~~
</details>

---

### 📈 Auto-Avaliação

~~~mermaid
graph LR
    N1["🌱 Nível 1<br/>Iniciante<br/>Entendo conceito básico"]
    N2["🌿 Nível 2<br/>Intermediário<br/>Uso slices em funções"]
    N3["🌳 Nível 3<br/>Avançado<br/>Otimizo com slices"]
    N4["🏆 Nível 4<br/>Expert<br/>Domino lifetimes"]
    
    N1 --> N2
    N2 --> N3
    N3 --> N4
    
    style N1 fill:#81C784,stroke:#2E7D32
    style N2 fill:#4CAF50,stroke:#2E7D32
    style N3 fill:#2E7D32,stroke:#1B5E20,color:#fff
    style N4 fill:#FFD54F,stroke:#F57F17
~~~

**Onde você se encaixa agora?** 🎯

---

## 🚀 Transferência e Aplicação

### 💼 Casos de Uso Reais

**1. Processamento de Logs**
~~~rust {.line-numbers}
fn parsear_log(linha: &str) -> Option<(&str, &str, &str)> {
    // Formato: "[TIMESTAMP] LEVEL: mensagem"
    let timestamp = &linha[1..linha.find(']')?];
    let resto = &linha[linha.find(']')? + 2..];
    let level = &resto[..resto.find(':')?];
    let mensagem = &resto[resto.find(':')? + 2..];
    
    Some((timestamp, level, mensagem))
}
~~~

**2. Validação de Entrada**
~~~rust {.line-numbers}
fn validar_email(email: &str) -> bool {
    email.contains('@') && 
    email.split('@').count() == 2 &&
    !email.starts_with('@') &&
    !email.ends_with('@')
}
~~~

**3. Parsing de CSV**
~~~rust {.line-numbers}
fn parsear_csv_linha(linha: &str) -> Vec<&str> {
    linha.split(',')
        .map(|campo| campo.trim())
        .collect()
}
~~~

---

### 🔮 Preparação para Lifetimes (Dia 19)

Você notou os `'a` no desafio extra? Esses são **lifetimes**!

~~~mermaid
graph LR
    A["Slices<br/>(Dia 18)"]
    B["Lifetimes<br/>(Dia 19)"]
    C["Por que slice<br/>retornado é válido?"]
    
    A -.->|"Próximo passo"| B
    A --> C
    C -.->|"Resposta"| B
    
    style A fill:#4CAF50,stroke:#2E7D32,color:#fff
    style B fill:#FF9800,stroke:#E65100,color:#fff
    style C fill:#2196F3,stroke:#1565C0,color:#fff
~~~

**Prévia:**
~~~rust {.line-numbers}
// Por que precisamos de 'a aqui?
fn remover_prefixo<'a>(texto: &'a str, prefixo: &str) -> &'a str {
    texto.strip_prefix(prefixo).unwrap_or(texto)
}
~~~

**Pergunta para pensar:**
> Como o compilador sabe que o slice retornado é válido?

**Resposta:** Lifetimes! Eles garantem que o slice não sobrevive aos dados originais.

**Amanhã você aprenderá:**
- O que são lifetimes
- Por que Rust precisa deles
- Como anotar lifetimes
- Regras de elision (quando não precisa anotar)

---

### 📚 Recursos Extras

**Documentação Oficial:**
- [The Rust Book - Slices](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [std::str documentation](https://doc.rust-lang.org/std/primitive.str.html)
- [std::slice documentation](https://doc.rust-lang.org/std/primitive.slice.html)

**Exercícios Práticos:**
- [Rustlings - Slices](https://github.com/rust-lang/rustlings)
- [Exercism - Rust Track](https://exercism.org/tracks/rust)

**Vídeos Recomendados:**
- "Rust Slices Explained" - Jon Gjengset
- "Zero Cost Abstractions" - Rust Conference

---

## 🎓 Resumo Final

### 🔑 Conceitos-Chave

~~~mermaid
mindmap
  root((🦀 SLICES))
    Conceito
      View eficiente
      Janela para dados
      Zero-cost abstraction
    Tipos
      &str - texto
      &[T] - arrays
    Ranges
      ..
      ..=
      a..
      ..b
      a..b
    Performance
      Sem cópias
      90x mais rápido
      Zero alocações
    Uso
      Parâmetros de função
      Parsers
      Substrings
~~~

### 💡 Regras de Ouro

✅ **Use &str em parâmetros de função**  
✅ **Use slices para evitar cópias**  
✅ **Prefira iteradores sobre slices**  
✅ **Verifique bounds com Option**  
✅ **Pense em "views" ao invés de "cópias"**

### 🎯 Próximos Passos

~~~mermaid
graph LR
    A["1️⃣ Complete<br/>exercícios"]
    B["2️⃣ Refatore<br/>código antigo"]
    C["3️⃣ Experimente<br/>ranges"]
    D["4️⃣ Leia sobre<br/>lifetimes"]
    E["5️⃣ Pratique<br/>parsing"]
    
    A --> B
    B --> C
    C --> D
    D --> E
    
    style A fill:#4CAF50,color:#fff
    style B fill:#2196F3,color:#fff
    style C fill:#FF9800,color:#fff
    style D fill:#9C27B0,color:#fff
    style E fill:#F44336,color:#fff
~~~

---

## 🎉 Parabéns!

Você agora domina **Slices**, uma das abstrações mais poderosas de Rust!

**Você aprendeu:**
- ✅ Conceito de slice como view
- ✅ Diferença entre String e &str
- ✅ Todos os tipos de ranges
- ✅ Como criar parsers eficientes
- ✅ Performance de zero-cost abstractions

**Próxima aula:** Lifetimes - garantindo segurança de memória em slices! 🦀

---

**"Slices são janelas mágicas: você vê tudo, sem copiar nada!"** 🪟✨

---

*Material criado com ❤️ para iniciantes em Rust*  
*Dia 18 de 30 - Jornada Rust Completa*