# 📘 Dia 25: Pattern Matching Avançado em Rust

## 📋 Objetivos de Aprendizagem

Ao final desta lição, você será capaz de:

- ✅ **Dominar destructuring complexo** em tuplas, structs e enums aninhados
- ✅ **Usar patterns expressivamente** com @, guards, ranges e múltiplos padrões
- ✅ **Escrever código mais declarativo** e elegante com pattern matching
- ✅ **Distinguir patterns refutáveis e irrefutáveis** e aplicá-los corretamente
- ✅ **Refatorar código imperativo** para aproveitar o poder dos patterns

---

## 🎭 Ativação do Conhecimento Prévio

### Revisão Rápida: Match Básico

Você já conhece o `match` básico em Rust:

~~~rust
fn basic_match(number: i32) {
    match number {
        1 => println!("Um"),
        2 => println!("Dois"),
        _ => println!("Outro"),
    }
}
~~~

Mas o pattern matching em Rust vai **muito além** disso!

### 🔍 Analogia Central: "Impressão Digital de Dados"

Imagine que você é um detetive analisando impressões digitais:

- **Pattern** = O molde da impressão digital que você procura
- **Match** = O processo de encontrar correspondências exatas
- **Destructuring** = Extrair informações específicas (loops, verticilos) da impressão

Assim como uma impressão digital revela detalhes únicos, os patterns em Rust permitem:
1. **Descrever a forma exata** dos dados que você espera
2. **Encontrar correspondências** precisas
3. **Extrair informações** automaticamente durante a verificação

### 📖 História: O Poder do Reconhecimento de Padrões

Em 1901, Scotland Yard adotou o sistema de impressões digitais. Um detetive podia olhar para uma impressão e instantaneamente identificar características únicas - sem processos manuais tediosos.

Da mesma forma, pattern matching em Rust permite que você "olhe" para estruturas de dados complexas e instantaneamente:
- Identifique sua forma
- Extraia valores específicos
- Tome decisões baseadas em padrões

Isso transforma código verboso e imperativo em expressões declarativas elegantes.

---

## 📚 Apresentação do Conteúdo

### 1️⃣ Destructuring: Desempacotando Estruturas Complexas

#### Destructuring de Tuplas

~~~rust
fn analyze_point(point: (i32, i32, i32)) {
    match point {
        (0, 0, 0) => println!("Origem"),
        (x, 0, 0) => println!("No eixo X: {}", x),
        (0, y, 0) => println!("No eixo Y: {}", y),
        (0, 0, z) => println!("No eixo Z: {}", z),
        (x, y, z) => println!("Ponto 3D: ({}, {}, {})", x, y, z),
    }
}

// Tuplas aninhadas
fn nested_tuples(data: ((i32, i32), (i32, i32))) {
    match data {
        ((0, 0), (0, 0)) => println!("Dois pontos na origem"),
        ((x1, y1), (x2, y2)) => {
            println!("De ({}, {}) até ({}, {})", x1, y1, x2, y2);
        }
    }
}
~~~

#### Destructuring de Structs

~~~rust
struct User {
    name: String,
    age: u32,
    email: String,
}

fn greet_user(user: User) {
    match user {
        // Extrair campos específicos
        User { name, age: 18..=25, .. } => {
            println!("Olá jovem adulto {}!", name);
        }
        User { name, age, .. } if age < 18 => {
            println!("Olá menor de idade {}!", name);
        }
        User { name, .. } => {
            println!("Olá {}!", name);
        }
    }
}

// Structs aninhados
struct Address {
    city: String,
    country: String,
}

struct Person {
    name: String,
    address: Address,
}

fn check_location(person: Person) {
    match person {
        Person { 
            name, 
            address: Address { city, country: ref c } 
        } if c == "Brasil" => {
            println!("{} mora em {}, Brasil", name, city);
        }
        Person { name, address: Address { country, .. } } => {
            println!("{} mora em {}", name, country);
        }
    }
}
~~~

#### Destructuring de Enums Aninhados

~~~rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(u8, u8, u8),
    Hsv(u8, u8, u8),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Encerrando...");
        }
        Message::Move { x, y } => {
            println!("Movendo para ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("Texto: {}", text);
        }
        // Enum aninhado!
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Mudando para RGB({}, {}, {})", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Mudando para HSV({}, {}, {})", h, s, v);
        }
    }
}
~~~

---

### 2️⃣ @ Binding: Capturar e Testar Simultaneamente

O operador `@` permite **capturar um valor** enquanto **testa um padrão**.

~~~rust
fn categorize_age(age: u32) {
    match age {
        // Captura 'n' E testa se está no range
        n @ 0..=12 => println!("Criança de {} anos", n),
        n @ 13..=17 => println!("Adolescente de {} anos", n),
        n @ 18..=64 => println!("Adulto de {} anos", n),
        n @ 65.. => println!("Idoso de {} anos", n),
    }
}

// Exemplo mais complexo
enum Status {
    Active { id: u32 },
    Inactive { id: u32 },
    Pending { id: u32, priority: u8 },
}

fn handle_status(status: Status) {
    match status {
        // Captura o ID específico E testa o padrão
        Status::Active { id: id_val @ 1000..=9999 } => {
            println!("ID premium ativo: {}", id_val);
        }
        Status::Pending { id, priority: p @ 8..=10 } => {
            println!("Alta prioridade ({}) para ID {}", p, id);
        }
        Status::Active { id } => {
            println!("ID ativo: {}", id);
        }
        _ => println!("Outro status"),
    }
}
~~~

**Quando usar @:**
- Você precisa do valor completo E quer testar um padrão
- Evita repetição de lógica
- Torna o código mais expressivo

---

### 3️⃣ Guards: Condições Adicionais com `if`

Guards permitem adicionar **condições booleanas** aos patterns.

~~~rust
fn evaluate_number(num: i32) {
    match num {
        n if n < 0 => println!("Negativo: {}", n),
        n if n % 2 == 0 => println!("Par positivo: {}", n),
        n if n % 2 != 0 => println!("Ímpar positivo: {}", n),
        _ => unreachable!(),
    }
}

// Guards com destructuring
struct Point {
    x: i32,
    y: i32,
}

fn analyze_point_advanced(point: Point) {
    match point {
        Point { x, y } if x == y => {
            println!("Diagonal principal: ({}, {})", x, y);
        }
        Point { x, y } if x == -y => {
            println!("Diagonal secundária: ({}, {})", x, y);
        }
        Point { x: 0, y } | Point { x: _, y } if y == 0 => {
            println!("Sobre um eixo");
        }
        Point { x, y } if x > 0 && y > 0 => {
            println!("Quadrante 1");
        }
        _ => println!("Outro quadrante"),
    }
}

// Guards com enums
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

fn warn_temperature(temp: Temperature) {
    match temp {
        Temperature::Celsius(t) if t > 40.0 => {
            println!("⚠️ PERIGO: {}°C é extremamente quente!", t);
        }
        Temperature::Celsius(t) if t < 0.0 => {
            println!("❄️ Congelante: {}°C", t);
        }
        Temperature::Fahrenheit(t) if t > 104.0 => {
            println!("⚠️ PERIGO: {}°F é extremamente quente!", t);
        }
        Temperature::Celsius(t) => println!("{}°C - OK", t),
        Temperature::Fahrenheit(t) => println!("{}°F - OK", t),
    }
}
~~~

---

### 4️⃣ Ranges em Patterns

Rust permite usar **ranges** diretamente em patterns.

~~~rust
fn classify_char(c: char) {
    match c {
        'a'..='z' => println!("Letra minúscula"),
        'A'..='Z' => println!("Letra maiúscula"),
        '0'..='9' => println!("Dígito"),
        _ => println!("Outro caractere"),
    }
}

fn http_status(code: u16) {
    match code {
        200 => println!("✅ OK"),
        201..=299 => println!("✅ Sucesso"),
        300..=399 => println!("↪️ Redirecionamento"),
        400 => println!("❌ Bad Request"),
        401 => println!("🔒 Não autorizado"),
        404 => println!("🔍 Não encontrado"),
        402..=499 => println!("❌ Erro do cliente"),
        500..=599 => println!("💥 Erro do servidor"),
        _ => println!("❓ Código desconhecido"),
    }
}

// Ranges com @
fn categorize_score(score: u32) {
    match score {
        s @ 90..=100 => println!("Excelente! Nota: {}", s),
        s @ 70..=89 => println!("Bom! Nota: {}", s),
        s @ 50..=69 => println!("Regular. Nota: {}", s),
        s @ 0..=49 => println!("Insuficiente. Nota: {}", s),
        _ => println!("Nota inválida"),
    }
}
~~~

---

### 5️⃣ Underscore `_`: Ignorando Partes

Use `_` para ignorar valores que você não precisa.

~~~rust
// Ignorar valores em tuplas
fn process_tuple(data: (i32, i32, i32, i32)) {
    match data {
        (first, _, _, last) => {
            println!("Primeiro: {}, Último: {}", first, last);
        }
    }
}

// Ignorar campos em structs
struct Config {
    host: String,
    port: u16,
    timeout: u32,
    retries: u32,
}

fn connect(config: Config) {
    match config {
        Config { host, port, .. } => {
            println!("Conectando a {}:{}", host, port);
            // Ignoramos timeout e retries
        }
    }
}

// Ignorar variantes de enum
enum Event {
    Click { x: i32, y: i32 },
    KeyPress(char),
    Scroll(i32),
}

fn handle_event(event: Event) {
    match event {
        Event::Click { x, y } => {
            println!("Click em ({}, {})", x, y);
        }
        _ => {
            // Ignoramos KeyPress e Scroll
        }
    }
}

// Ignorar com nome (mais expressivo)
fn process_result(result: Result<String, String>) {
    match result {
        Ok(value) => println!("Sucesso: {}", value),
        Err(_error) => println!("Erro ocorreu"),
        // _error indica que ignoramos propositalmente
    }
}
~~~

---

### 6️⃣ Pipe `|`: Múltiplos Patterns

Use `|` para combinar múltiplos patterns em um único arm.

~~~rust
fn is_weekend(day: &str) {
    match day {
        "sábado" | "domingo" => println!("🎉 Final de semana!"),
        "segunda" | "terça" | "quarta" | "quinta" | "sexta" => {
            println!("📅 Dia útil");
        }
        _ => println!("Dia inválido"),
    }
}

// Com números
fn classify_digit(n: u32) {
    match n {
        0 => println!("Zero"),
        1 | 3 | 5 | 7 | 9 => println!("Ímpar"),
        2 | 4 | 6 | 8 => println!("Par"),
        _ => println!("Não é um dígito"),
    }
}

// Combinando com destructuring
enum Action {
    Move { x: i32, y: i32 },
    Stop,
    Pause,
}

fn handle_action(action: Action) {
    match action {
        Action::Stop | Action::Pause => {
            println!("Parando movimento");
        }
        Action::Move { x, y } => {
            println!("Movendo para ({}, {})", x, y);
        }
    }
}

// Combinando ranges e múltiplos patterns
fn classify_grade(grade: char) {
    match grade {
        'A' | 'B' => println!("Aprovado com distinção"),
        'C' | 'D' => println!("Aprovado"),
        'E' | 'F' => println!("Reprovado"),
        _ => println!("Nota inválida"),
    }
}
~~~

---

### 7️⃣ Ref e Mut em Patterns

Use `ref` e `ref mut` para criar referências em patterns.

~~~rust
fn analyze_string(s: String) {
    match s {
        // 'ref' cria uma referência, não move o valor
        ref text if text.len() > 10 => {
            println!("Texto longo: {}", text);
            // 's' ainda é válido aqui!
        }
        ref text => {
            println!("Texto curto: {}", text);
        }
    }
    // 's' foi movido, não pode ser usado aqui
}

// Com mut
fn modify_value(mut num: i32) {
    match num {
        ref mut n if *n < 0 => {
            *n = -*n; // Torna positivo
            println!("Convertido para positivo: {}", n);
        }
        ref mut n => {
            *n += 10;
            println!("Incrementado: {}", n);
        }
    }
}

// Exemplo prático
struct Buffer {
    data: Vec<u8>,
}

fn process_buffer(buffer: Buffer) {
    match buffer {
        Buffer { ref data } if data.len() > 1024 => {
            println!("Buffer grande: {} bytes", data.len());
            // buffer não foi movido
        }
        Buffer { data } => {
            println!("Buffer pequeno: {} bytes", data.len());
            // buffer foi movido aqui
        }
    }
}
~~~

**Quando usar ref:**
- Você quer uma referência, não ownership
- Precisa usar o valor depois do match
- Evitar clones desnecessários

---

### 8️⃣ Patterns Irrefutáveis vs Refutáveis

**Irrefutável**: Pattern que sempre corresponde (não pode falhar)
**Refutável**: Pattern que pode não corresponder

~~~rust
// IRREFUTÁVEL - sempre funciona
let x = 5; // pattern 'x' sempre corresponde
let (a, b) = (1, 2); // sempre funciona com tupla de 2 elementos

// REFUTÁVEL - pode falhar
let some_value = Some(5);
// if let aceita patterns refutáveis
if let Some(x) = some_value {
    println!("Valor: {}", x);
}

// ERRO: let não aceita patterns refutáveis
// let Some(x) = some_value; // ❌ Não compila!

// while let com pattern refutável
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);
}
~~~

**Contextos que aceitam apenas irrefutáveis:**
- `let` statements
- Parâmetros de função
- `for` loops

**Contextos que aceitam refutáveis:**
- `match` arms
- `if let`
- `while let`

~~~rust
// Função: apenas irrefutável
fn print_coordinates((x, y): (i32, i32)) {
    println!("({}, {})", x, y);
}

// For: apenas irrefutável
let pairs = vec![(1, 2), (3, 4)];
for (x, y) in pairs {
    println!("{}, {}", x, y);
}

// Match: aceita refutável
fn process_option(opt: Option<i32>) {
    match opt {
        Some(x) => println!("Valor: {}", x),
        None => println!("Nenhum valor"),
    }
}
~~~

---

## 🎨 Diagramas Mermaid

### Diagrama 1: Hierarquia de Patterns
~~~

~~~mermaid
graph TD
    A[Patterns em Rust] --> B[Literais]
    A --> C[Variáveis]
    A --> D[Wildcards]
    A --> E[Estruturados]
    
    B --> B1["1, 'a', true"]
    
    C --> C1[x, name, value]
    
    D --> D1[_ ignora tudo]
    D --> D2[.. ignora resto]
    
    E --> E1[Tuplas]
    E --> E2[Structs]
    E --> E3[Enums]
    E --> E4[Referencias]
    
    E1 --> E1A["(x, y, z)"]
    E2 --> E2A["Point { x, y }"]
    E3 --> E3A["Some(x)"]
    E4 --> E4A["ref x, ref mut y"]
    
    style A fill:#ff6b6b
    style E fill:#4ecdc4
    style E1 fill:#45b7d1
    style E2 fill:#45b7d1
    style E3 fill:#45b7d1
    style E4 fill:#45b7d1
~~~

### Diagrama 2: Fluxograma de Destructuring Aninhado
~~~

~~~mermaid
flowchart TD
    A[Valor Complexo] --> B{Tipo?}
    
    B -->|Tupla| C[Extrair elementos]
    B -->|Struct| D[Extrair campos]
    B -->|Enum| E[Identificar variante]
    
    C --> F{Elemento é<br/>aninhado?}
    D --> F
    E --> F
    
    F -->|Sim| G[Aplicar pattern<br/>recursivamente]
    F -->|Não| H[Capturar valor]
    
    G --> I[Continuar destructuring]
    I --> H
    
    H --> J[Valores extraídos<br/>disponíveis]
    
    style A fill:#ff6b6b
    style J fill:#51cf66
    style G fill:#ffd43b
~~~

### Diagrama 3: @ Binding em Ação
~~~

~~~mermaid
sequenceDiagram
    participant V as Valor
    participant P as Pattern
    participant T as Teste
    participant C as Captura
    
    V->>P: age = 15
    P->>T: Testa: 13..=17?
    T-->>P: ✅ Corresponde
    P->>C: Captura em 'n'
    C-->>V: n = 15 disponível
    
    Note over V,C: @ permite testar E capturar<br/>em uma única operação
    
    rect rgb(200, 240, 200)
        Note right of C: n @ 13..=17<br/>Testa o range<br/>Captura o valor
    end
~~~

### Diagrama 4: Comparação - Com e Sem Guards
~~~

~~~mermaid
graph LR
    subgraph "Sem Guards - Verboso"
        A1[match x] --> B1{Pattern 1}
        B1 --> C1[if condition]
        C1 -->|true| D1[Action 1]
        C1 -->|false| E1[if condition 2]
        E1 --> F1[Action 2]
    end
    
    subgraph "Com Guards - Elegante"
        A2[match x] --> B2["Pattern 1 if cond"]
        A2 --> C2["Pattern 2 if cond2"]
        B2 --> D2[Action 1]
        C2 --> E2[Action 2]
    end
    
    style A2 fill:#51cf66
    style B2 fill:#51cf66
    style C2 fill:#51cf66
~~~

### Diagrama 5: Galeria de Patterns Úteis
~~~

~~~mermaid
mindmap
  root((Patterns<br/>Úteis))
    Ranges
      1..=10
      'a'..='z'
      0..100
    Múltiplos
      x | y | z
      Some(1 | 2 | 3)
    Ignorar
      _ wildcard
      .. resto
      _unused
    Captura
      @ binding
      ref pattern
      ref mut
    Combinados
      @ com ranges
      | com guards
      nested destructuring
    Guards
      if condition
      if x > 0
      if let Some
~~~

---

## 📊 Tabela Comparativa de Patterns

| Pattern | Sintaxe | Uso | Exemplo |
|---------|---------|-----|---------|
| **Literal** | `1`, `'a'`, `true` | Valores exatos | `match x { 1 => ... }` |
| **Variável** | `x`, `name` | Captura qualquer valor | `match x { n => ... }` |
| **Wildcard** | `_` | Ignora valor | `match x { _ => ... }` |
| **Range** | `1..=5` | Intervalo inclusivo | `match x { 1..=5 => ... }` |
| **Múltiplos** | `1 \| 2 \| 3` | Vários patterns | `match x { 1\|2\|3 => ... }` |
| **@ Binding** | `n @ 1..=5` | Captura + testa | `match x { n@1..=5 => ... }` |
| **Guard** | `x if x > 0` | Condição extra | `match x { n if n>0 => ... }` |
| **Ref** | `ref x` | Referência | `match s { ref r => ... }` |
| **Tupla** | `(x, y, z)` | Destructure tupla | `match p { (x,y) => ... }` |
| **Struct** | `Point { x, y }` | Destructure struct | `match p { Point{x,..} => }` |
| **Enum** | `Some(x)` | Destructure enum | `match o { Some(x) => ... }` |

---

## 💡 Demonstração e Modelagem

### Evolução: Do Simples ao Complexo

#### Nível 1: Match Básico
~~~rust
fn level1_basic(x: i32) {
    match x {
        1 => println!("Um"),
        2 => println!("Dois"),
        _ => println!("Outro"),
    }
}
~~~

#### Nível 2: Com Ranges
~~~rust
fn level2_ranges(x: i32) {
    match x {
        1..=10 => println!("Entre 1 e 10"),
        11..=20 => println!("Entre 11 e 20"),
        _ => println!("Fora do intervalo"),
    }
}
~~~

#### Nível 3: Com @ Binding
~~~rust
fn level3_binding(x: i32) {
    match x {
        n @ 1..=10 => println!("{} está entre 1 e 10", n),
        n @ 11..=20 => println!("{} está entre 11 e 20", n),
        n => println!("{} está fora do intervalo", n),
    }
}
~~~

#### Nível 4: Com Guards
~~~rust
fn level4_guards(x: i32) {
    match x {
        n @ 1..=10 if n % 2 == 0 => println!("{} é par entre 1-10", n),
        n @ 1..=10 => println!("{} é ímpar entre 1-10", n),
        n @ 11..=20 if n % 2 == 0 => println!("{} é par entre 11-20", n),
        n @ 11..=20 => println!("{} é ímpar entre 11-20", n),
        n => println!("{} está fora", n),
    }
}
~~~

#### Nível 5: Estruturas Complexas
~~~rust
enum Data {
    Number(i32),
    Range { start: i32, end: i32 },
    List(Vec<i32>),
}

fn level5_complex(data: Data) {
    match data {
        Data::Number(n @ 1..=10) if n % 2 == 0 => {
            println!("Número par entre 1-10: {}", n);
        }
        Data::Range { start: s @ 0..=10, end: e } if e - s <= 5 => {
            println!("Range pequeno: {}..{}", s, e);
        }
        Data::List(ref list) if list.len() > 5 => {
            println!("Lista grande com {} elementos", list.len());
        }
        Data::Number(n) => println!("Número: {}", n),
        Data::Range { start, end } => println!("Range: {}..{}", start, end),
        Data::List(list) => println!("Lista: {:?}", list),
    }
}
~~~

---

### Refatoração: Imperativo → Declarativo

#### ❌ Antes: Código Imperativo (Verboso)
~~~rust
fn check_status_imperative(code: u16, message: &str) {
    if code >= 200 && code < 300 {
        if code == 200 {
            println!("OK: {}", message);
        } else if code == 201 {
            println!("Criado: {}", message);
        } else {
            println!("Sucesso ({}): {}", code, message);
        }
    } else if code >= 400 && code < 500 {
        if code == 404 {
            println!("Não encontrado: {}", message);
        } else if code == 401 {
            println!("Não autorizado: {}", message);
        } else {
            println!("Erro do cliente ({}): {}", code, message);
        }
    } else if code >= 500 {
        println!("Erro do servidor ({}): {}", code, message);
    } else {
        println!("Status desconhecido ({}): {}", code, message);
    }
}
~~~

#### ✅ Depois: Pattern Matching (Elegante)
~~~rust
fn check_status_declarative(code: u16, message: &str) {
    match code {
        200 => println!("OK: {}", message),
        201 => println!("Criado: {}", message),
        202..=299 => println!("Sucesso ({}): {}", code, message),
        
        404 => println!("Não encontrado: {}", message),
        401 => println!("Não autorizado: {}", message),
        400..=499 => println!("Erro do cliente ({}): {}", code, message),
        
        500..=599 => println!("Erro do servidor ({}): {}", code, message),
        
        _ => println!("Status desconhecido ({}): {}", code, message),
    }
}
~~~

**Benefícios:**
- ✅ Mais legível e declarativo
- ✅ Exhaustividade garantida pelo compilador
- ✅ Menos aninhamento
- ✅ Intenção clara

---

### Padrões Idiomáticos em Rust

#### 1. Option Handling
~~~rust
// ❌ Verboso
fn get_first_imperative(list: Vec<i32>) -> i32 {
    if list.is_empty() {
        0
    } else {
        list[0]
    }
}

// ✅ Idiomático
fn get_first_idiomatic(list: Vec<i32>) -> i32 {
    match list.first() {
        Some(&first) => first,
        None => 0,
    }
}

// ✅ Ainda melhor
fn get_first_best(list: Vec<i32>) -> i32 {
    list.first().copied().unwrap_or(0)
}
~~~

#### 2. Result Processing
~~~rust
fn process_file(path: &str) -> Result<String, String> {
    // Simulação
    if path.ends_with(".txt") {
        Ok(format!("Conteúdo de {}", path))
    } else {
        Err("Formato inválido".to_string())
    }
}

// Pattern matching expressivo
fn handle_file(path: &str) {
    match process_file(path) {
        Ok(content) if content.len() > 100 => {
            println!("Arquivo grande processado");
        }
        Ok(content) => {
            println!("Conteúdo: {}", content);
        }
        Err(e) => {
            eprintln!("Erro: {}", e);
        }
    }
}
~~~

#### 3. State Machines
~~~rust
enum State {
    Idle,
    Running { progress: u8 },
    Paused { progress: u8 },
    Completed,
    Failed { error: String },
}

fn transition(state: State, action: &str) -> State {
    match (state, action) {
        (State::Idle, "start") => State::Running { progress: 0 },
        
        (State::Running { progress: p }, "pause") => {
            State::Paused { progress: p }
        }
        
        (State::Running { progress: p }, "update") if p < 100 => {
            State::Running { progress: p + 10 }
        }
        
        (State::Running { progress: 100 }, _) => State::Completed,
        
        (State::Paused { progress: p }, "resume") => {
            State::Running { progress: p }
        }
        
        (_, "cancel") => State::Failed { 
            error: "Cancelado pelo usuário".to_string() 
        },
        
        (current, _) => current, // Mantém estado atual
    }
}
~~~

---

## 🎯 Prática Guiada: Parser de Comandos CLI

### 📝 Contexto do Exercício

Você está desenvolvendo uma ferramenta CLI (Command Line Interface) para gerenciar usuários. O sistema precisa processar comandos complexos como:

- `add user admin password123` - Adicionar usuário admin
- `delete id:42` - Deletar usuário com ID 42
- `list --limit 10` - Listar 10 usuários
- `update id:15 role:moderator` - Atualizar papel do usuário
- `search name:João age:25..35` - Buscar por nome e faixa etária

**Objetivo**: Criar um parser elegante usando pattern matching avançado.

---

### 🏗️ Estruturas de Dados

~~~rust
#[derive(Debug, PartialEq)]
enum Command {
    Add { 
        username: String, 
        role: Role, 
        password: String 
    },
    Delete { 
        id: u32 
    },
    List { 
        limit: Option<u32> 
    },
    Update { 
        id: u32, 
        role: Option<Role>,
        password: Option<String>,
    },
    Search { 
        name: Option<String>,
        age_range: Option<(u32, u32)>,
        role: Option<Role>,
    },
    Help,
    Exit,
}

#[derive(Debug, PartialEq, Clone)]
enum Role {
    Admin,
    Moderator,
    User,
}

#[derive(Debug)]
enum ParseError {
    InvalidCommand,
    MissingArgument(String),
    InvalidId,
    InvalidRole,
    InvalidRange,
}
~~~

---

### 🔨 Implementação do Parser

~~~rust
fn parse_command(input: &str) -> Result<Command, ParseError> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    match parts.as_slice() {
        // Comando: help
        ["help"] => Ok(Command::Help),
        
        // Comando: exit
        ["exit"] | ["quit"] => Ok(Command::Exit),
        
        // Comando: add user <role> <password>
        ["add", "user", role, password] => {
            let parsed_role = parse_role(role)?;
            Ok(Command::Add {
                username: format!("user_{}", rand::random::<u16>()),
                role: parsed_role,
                password: password.to_string(),
            })
        }
        
        // Comando: delete id:123
        ["delete", id_str] if id_str.starts_with("id:") => {
            let id = parse_id(id_str)?;
            Ok(Command::Delete { id })
        }
        
        // Comando: list
        ["list"] => Ok(Command::List { limit: None }),
        
        // Comando: list --limit 10
        ["list", "--limit", limit_str] => {
            match limit_str.parse::<u32>() {
                Ok(n @ 1..=100) => Ok(Command::List { limit: Some(n) }),
                _ => Err(ParseError::InvalidCommand),
            }
        }
        
        // Comando: update id:15 role:moderator
        ["update", id_str, updates @ ..] if id_str.starts_with("id:") => {
            let id = parse_id(id_str)?;
            let (role, password) = parse_updates(updates)?;
            Ok(Command::Update { id, role, password })
        }
        
        // Comando: search name:João age:25..35 role:admin
        ["search", filters @ ..] if !filters.is_empty() => {
            let (name, age_range, role) = parse_filters(filters)?;
            Ok(Command::Search { name, age_range, role })
        }
        
        // Comando inválido
        _ => Err(ParseError::InvalidCommand),
    }
}

// Funções auxiliares com pattern matching

fn parse_role(role_str: &str) -> Result<Role, ParseError> {
    match role_str.to_lowercase().as_str() {
        "admin" | "administrator" => Ok(Role::Admin),
        "mod" | "moderator" => Ok(Role::Moderator),
        "user" | "member" => Ok(Role::User),
        _ => Err(ParseError::InvalidRole),
    }
}

fn parse_id(id_str: &str) -> Result<u32, ParseError> {
    match id_str.strip_prefix("id:") {
        Some(num_str) => num_str.parse().map_err(|_| ParseError::InvalidId),
        None => Err(ParseError::InvalidId),
    }
}

fn parse_updates(updates: &[&str]) -> Result<(Option<Role>, Option<String>), ParseError> {
    let mut role = None;
    let mut password = None;
    
    for update in updates {
        match update.split_once(':') {
            Some(("role", r)) => role = Some(parse_role(r)?),
            Some(("password", p)) => password = Some(p.to_string()),
            _ => return Err(ParseError::InvalidCommand),
        }
    }
    
    Ok((role, password))
}

fn parse_filters(filters: &[&str]) -> Result<(
    Option<String>, 
    Option<(u32, u32)>, 
    Option<Role>
), ParseError> {
    let mut name = None;
    let mut age_range = None;
    let mut role = None;
    
    for filter in filters {
        match filter.split_once(':') {
            Some(("name", n)) => name = Some(n.to_string()),
            
            Some(("age", range_str)) => {
                // Pattern matching para range: "25..35"
                match range_str.split_once("..") {
                    Some((start, end)) => {
                        let s = start.parse().map_err(|_| ParseError::InvalidRange)?;
                        let e = end.parse().map_err(|_| ParseError::InvalidRange)?;
                        age_range = Some((s, e));
                    }
                    None => return Err(ParseError::InvalidRange),
                }
            }
            
            Some(("role", r)) => role = Some(parse_role(r)?),
            
            _ => return Err(ParseError::InvalidCommand),
        }
    }
    
    Ok((name, age_range, role))
}
~~~

---

### 🎬 Executando Comandos

~~~rust
fn execute_command(cmd: Command) {
    match cmd {
        Command::Add { username, role, password } => {
            println!("✅ Usuário '{}' criado com papel {:?}", username, role);
            println!("   Senha: {}", "*".repeat(password.len()));
        }
        
        Command::Delete { id } => {
            println!("🗑️  Usuário ID {} deletado", id);
        }
        
        Command::List { limit: Some(n @ 1..=10) } => {
            println!("📋 Listando {} usuários (limite pequeno)", n);
        }
        
        Command::List { limit: Some(n) } => {
            println!("📋 Listando {} usuários", n);
        }
        
        Command::List { limit: None } => {
            println!("📋 Listando todos os usuários");
        }
        
        Command::Update { id, role: Some(r), password: Some(p) } => {
            println!("🔄 Usuário {} atualizado: papel={:?}, senha alterada", id, r);
        }
        
        Command::Update { id, role: Some(r), password: None } => {
            println!("🔄 Usuário {} atualizado: papel={:?}", id, r);
        }
        
        Command::Update { id, role: None, password: Some(_) } => {
            println!("🔄 Usuário {} atualizado: senha alterada", id);
        }
        
        Command::Update { id, .. } => {
            println!("⚠️  Nenhuma atualização especificada para usuário {}", id);
        }
        
        Command::Search { 
            name: Some(n), 
            age_range: Some((start, end)), 
            role: Some(r) 
        } => {
            println!("🔍 Buscando: nome='{}', idade={}-{}, papel={:?}", 
                     n, start, end, r);
        }
        
        Command::Search { name: Some(n), age_range: Some((s, e)), .. } => {
            println!("🔍 Buscando: nome='{}', idade={}-{}", n, s, e);
        }
        
        Command::Search { name: Some(n), .. } => {
            println!("🔍 Buscando por nome: '{}'", n);
        }
        
        Command::Search { age_range: Some((s, e)), .. } => {
            println!("🔍 Buscando por idade: {}-{}", s, e);
        }
        
        Command::Search { role: Some(r), .. } => {
            println!("🔍 Buscando por papel: {:?}", r);
        }
        
        Command::Search { .. } => {
            println!("⚠️  Nenhum filtro especificado");
        }
        
        Command::Help => {
            print_help();
        }
        
        Command::Exit => {
            println!("👋 Até logo!");
        }
    }
}

fn print_help() {
    println!("📖 Comandos disponíveis:");
    println!("  add user <role> <password>       - Adicionar usuário");
    println!("  delete id:<id>                   - Deletar usuário");
    println!("  list [--limit <n>]               - Listar usuários");
    println!("  update id:<id> [role:<r>] [password:<p>] - Atualizar");
    println!("  search [name:<n>] [age:<s>..<e>] [role:<r>] - Buscar");
    println!("  help                             - Mostrar ajuda");
    println!("  exit                             - Sair");
}
~~~

---

### 🧪 Testando o Sistema

~~~rust
fn main() {
    let test_commands = vec![
        "help",
        "add user admin secret123",
        "delete id:42",
        "list",
        "list --limit 5",
        "list --limit 50",
        "update id:15 role:moderator",
        "update id:20 password:newpass",
        "update id:25 role:admin password:secure",
        "search name:João",
        "search age:25..35",
        "search name:Maria age:20..30 role:user",
        "exit",
    ];
    
    println!("🚀 Testando Parser de Comandos CLI\n");
    println!("{}", "=".repeat(60));
    
    for (i, cmd_str) in test_commands.iter().enumerate() {
        println!("\n[Teste {}] Input: \"{}\"", i + 1, cmd_str);
        
        match parse_command(cmd_str) {
            Ok(cmd) => {
                println!("✅ Parsed: {:?}", cmd);
                execute_command(cmd);
            }
            Err(e) => {
                println!("❌ Erro: {:?}", e);
            }
        }
        
        println!("{}", "-".repeat(60));
    }
}
~~~

---

### 📊 Comparação: Imperativo vs Declarativo

#### ❌ Abordagem Imperativa (Sem Pattern Matching)

~~~rust
fn parse_command_imperative(input: &str) -> Result<Command, ParseError> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    if parts.is_empty() {
        return Err(ParseError::InvalidCommand);
    }
    
    let command = parts[0];
    
    if command == "help" {
        return Ok(Command::Help);
    }
    
    if command == "exit" || command == "quit" {
        return Ok(Command::Exit);
    }
    
    if command == "add" {
        if parts.len() < 4 {
            return Err(ParseError::MissingArgument("role ou password".to_string()));
        }
        if parts[1] != "user" {
            return Err(ParseError::InvalidCommand);
        }
        let role = parse_role(parts[2])?;
        let password = parts[3].to_string();
        return Ok(Command::Add {
            username: format!("user_{}", rand::random::<u16>()),
            role,
            password,
        });
    }
    
    if command == "delete" {
        if parts.len() < 2 {
            return Err(ParseError::MissingArgument("id".to_string()));
        }
        if !parts[1].starts_with("id:") {
            return Err(ParseError::InvalidId);
        }
        let id = parse_id(parts[1])?;
        return Ok(Command::Delete { id });
    }
    
    // ... e assim por diante (muito verboso!)
    
    Err(ParseError::InvalidCommand)
}
~~~

**Problemas:**
- ❌ Muito verboso e repetitivo
- ❌ Difícil de manter
- ❌ Propenso a erros
- ❌ Não aproveita o sistema de tipos
- ❌ Lógica espalhada e aninhada

#### ✅ Abordagem Declarativa (Com Pattern Matching)

A versão com pattern matching que implementamos acima é:
- ✅ Concisa e expressiva
- ✅ Fácil de entender e manter
- ✅ Type-safe e exhaustiva
- ✅ Aproveita o poder do compilador
- ✅ Lógica clara e organizada

---

## 🔄 Feedback e Avaliação

### ✅ Checklist de Patterns

Marque o que você já domina:

- [ ] Consigo fazer destructuring de tuplas simples
- [ ] Consigo fazer destructuring de structs
- [ ] Consigo fazer destructuring de enums
- [ ] Entendo destructuring aninhado
- [ ] Sei usar @ binding para capturar valores
- [ ] Sei combinar @ com ranges
- [ ] Sei usar guards (if) em match arms
- [ ] Consigo usar ranges em patterns (1..=10)
- [ ] Sei usar _ para ignorar valores
- [ ] Sei usar .. para ignorar múltiplos campos
- [ ] Sei usar | para múltiplos patterns
- [ ] Entendo quando usar ref e ref mut
- [ ] Distingo patterns refutáveis de irrefutáveis
- [ ] Sei onde cada tipo de pattern pode ser usado
- [ ] Consigo refatorar if/else para match elegante

---

### 🧩 Quiz de Correspondência

**Pergunta 1**: Qual pattern corresponde a números pares entre 10 e 20?

A) `n @ 10..=20`  
B) `n @ 10..=20 if n % 2 == 0`  
C) `10 | 12 | 14 | 16 | 18 | 20`  
D) `n if n >= 10 && n <= 20 && n % 2 == 0`

<details>
<summary>Resposta</summary>

**B** é a melhor opção - usa @ para capturar, range para limitar, e guard para testar paridade.

C também funciona mas não é escalável.
D funciona mas não usa o poder dos patterns.
</details>

---

**Pergunta 2**: O que este pattern faz?

~~~rust
match point {
    Point { x: 0, y: 0 } => println!("Origem"),
    Point { x, y: 0 } => println!("Eixo X: {}", x),
    Point { x: 0, y } => println!("Eixo Y: {}", y),
    Point { x, y } => println!("Ponto: ({}, {})", x, y),
}
~~~

A) Verifica se o ponto está em um quadrante específico  
B) Identifica se o ponto está na origem ou em um dos eixos  
C) Calcula a distância do ponto até a origem  
D) Transforma coordenadas cartesianas em polares

<details>
<summary>Resposta</summary>

**B** - O pattern identifica casos especiais: origem (0,0), pontos no eixo X (y=0), pontos no eixo Y (x=0), e pontos gerais.
</details>

---

**Pergunta 3**: Qual é o erro neste código?

~~~rust
let some_value = Some(5);
let Some(x) = some_value;
println!("{}", x);
~~~

A) Some não pode ser usado em let  
B) Pattern refutável em contexto irrefutável  
C) some_value precisa ser mut  
D) Não há erro

<details>
<summary>Resposta</summary>

**B** - `let` aceita apenas patterns irrefutáveis. `Some(x)` é refutável (pode ser None). Use `if let` ou `match`.
</details>

---

### 🔧 Exercício de Refatoração

Refatore este código usando pattern matching:

~~~rust
fn classify_temperature(temp: f64, unit: &str) {
    if unit == "C" {
        if temp < 0.0 {
            println!("Congelante");
        } else if temp >= 0.0 && temp < 15.0 {
            println!("Frio");
        } else if temp >= 15.0 && temp < 25.0 {
            println!("Agradável");
        } else if temp >= 25.0 && temp < 35.0 {
            println!("Quente");
        } else {
            println!("Muito quente");
        }
    } else if unit == "F" {
        if temp < 32.0 {
            println!("Congelante");
        } else if temp >= 32.0 && temp < 59.0 {
            println!("Frio");
        } else if temp >= 59.0 && temp < 77.0 {
            println!("Agradável");
        } else if temp >= 77.0 && temp < 95.0 {
            println!("Quente");
        } else {
            println!("Muito quente");
        }
    } else {
        println!("Unidade inválida");
    }
}
~~~

<details>
<summary>Solução Refatorada</summary>

~~~rust
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

fn classify_temperature(temp: Temperature) {
    match temp {
        Temperature::Celsius(t) if t < 0.0 => {
            println!("Congelante");
        }
        Temperature::Celsius(t @ 0.0..15.0) => {
            println!("Frio ({}°C)", t);
        }
        Temperature::Celsius(t @ 15.0..25.0) => {
            println!("Agradável ({}°C)", t);
        }
        Temperature::Celsius(t @ 25.0..35.0) => {
            println!("Quente ({}°C)", t);
        }
        Temperature::Celsius(t) => {
            println!("Muito quente ({}°C)", t);
        }
        
        Temperature::Fahrenheit(t) if t < 32.0 => {
            println!("Congelante");
        }
        Temperature::Fahrenheit(t @ 32.0..59.0) => {
            println!("Frio ({}°F)", t);
        }
        Temperature::Fahrenheit(t @ 59.0..77.0) => {
            println!("Agradável ({}°F)", t);
        }
        Temperature::Fahrenheit(t @ 77.0..95.0) => {
            println!("Quente ({}°F)", t);
        }
        Temperature::Fahrenheit(t) => {
            println!("Muito quente ({}°F)", t);
        }
    }
}
~~~

**Melhorias:**
- ✅ Usa enum para representar unidades (type-safe)
- ✅ Ranges com @ binding para capturar valores
- ✅ Guards para condições especiais
- ✅ Exhaustividade garantida pelo compilador
- ✅ Mais legível e manutenível
</details>

---

### 📝 Auto-Avaliação

Responda honestamente:

1. **Compreensão** (1-5): Quanto você entendeu dos conceitos?
2. **Confiança** (1-5): Quão confiante você está para usar patterns?
3. **Aplicação** (1-5): Consegue aplicar em projetos reais?

**Se pontuou < 4 em qualquer área:**
- Revise os diagramas
- Refaça o exercício CLI
- Pratique os exemplos de refatoração
- Consulte a documentação oficial

---

## 🚀 Transferência e Aplicação

### 🎯 Desafio Final: State Machine com Patterns

Implemente uma máquina de estados para um sistema de pedidos (orders):

~~~rust
#[derive(Debug)]
enum OrderState {
    Created { id: u32, items: Vec<String> },
    PaymentPending { id: u32, amount: f64 },
    PaymentConfirmed { id: u32, amount: f64 },
    Shipped { id: u32, tracking: String },
    Delivered { id: u32, date: String },
    Cancelled { id: u32, reason: String },
}

#[derive(Debug)]
enum Action {
    ConfirmPayment(f64),
    Ship(String),
    Deliver,
    Cancel(String),
}

// TODO: Implemente esta função usando pattern matching avançado
fn transition_order(state: OrderState, action: Action) -> Result<OrderState, String> {
    // Sua implementação aqui
    // Use:
    // - Destructuring de enums
    // - Guards para validações
    // - @ binding quando necessário
    // - Múltiplos patterns com |
    todo!()
}
~~~

<details>
<summary>💡 Dica</summary>

Use match com tupla `(state, action)` para combinar estado atual e ação. Valide transições válidas e retorne erros para transições inválidas.
</details>

<details>
<summary>✅ Solução Completa</summary>

~~~rust
fn transition_order(state: OrderState, action: Action) -> Result<OrderState, String> {
    match (state, action) {
        // Created → PaymentPending
        (OrderState::Created { id, items }, Action::ConfirmPayment(amount)) 
            if amount > 0.0 && !items.is_empty() => 
        {
            Ok(OrderState::PaymentPending { id, amount })
        }
        
        // PaymentPending → PaymentConfirmed
        (OrderState::PaymentPending { id, amount: a1 }, Action::ConfirmPayment(a2))
            if (a1 - a2).abs() < 0.01 => // Valores iguais (float comparison)
        {
            Ok(OrderState::PaymentConfirmed { id, amount: a1 })
        }
        
        // PaymentConfirmed → Shipped
        (OrderState::PaymentConfirmed { id, .. }, Action::Ship(tracking))
            if !tracking.is_empty() =>
        {
            Ok(OrderState::Shipped { id, tracking })
        }
        
        // Shipped → Delivered
        (OrderState::Shipped { id, .. }, Action::Deliver) => {
            let date = "2024-01-15".to_string(); // Simulação
            Ok(OrderState::Delivered { id, date })
        }
        
        // Qualquer estado → Cancelled (exceto Delivered)
        (OrderState::Created { id, .. }, Action::Cancel(reason))
        | (OrderState::PaymentPending { id, .. }, Action::Cancel(reason))
        | (OrderState::PaymentConfirmed { id, .. }, Action::Cancel(reason))
        | (OrderState::Shipped { id, .. }, Action::Cancel(reason))
            if !reason.is_empty() =>
        {
            Ok(OrderState::Cancelled { id, reason })
        }
        
        // Delivered não pode ser cancelado
        (OrderState::Delivered { id, .. }, Action::Cancel(_)) => {
            Err(format!("Pedido {} já foi entregue, não pode ser cancelado", id))
        }
        
        // Transições inválidas
        (state, action) => {
            Err(format!(
                "Transição inválida de {:?} com ação {:?}",
                state, action
            ))
        }
    }
}

// Teste
fn test_state_machine() {
    let mut state = OrderState::Created {
        id: 1,
        items: vec!["Item A".to_string(), "Item B".to_string()],
    };
    
    println!("Estado inicial: {:?}\n", state);
    
    // Confirmar pagamento
    state = transition_order(state, Action::ConfirmPayment(99.90))
        .expect("Falha ao confirmar pagamento");
    println!("Após confirmar pagamento: {:?}\n", state);
    
    // Enviar
    state = transition_order(state, Action::Ship("BR123456789".to_string()))
        .expect("Falha ao enviar");
    println!("Após envio: {:?}\n", state);
    
    // Entregar
    state = transition_order(state, Action::Deliver)
        .expect("Falha ao entregar");
    println!("Após entrega: {:?}\n", state);
    
    // Tentar cancelar (deve falhar)
    match transition_order(state, Action::Cancel("Desistência".to_string())) {
        Ok(_) => println!("❌ Não deveria permitir cancelamento"),
        Err(e) => println!("✅ Erro esperado: {}", e),
    }
}
~~~
</details>

---

### 🔗 Preparação para o Dia 26: Move Semantics Avançado

Pattern matching se conecta diretamente com ownership:

~~~rust
// Patterns podem MOVER ou EMPRESTAR valores

fn example_ownership(opt: Option<String>) {
    match opt {
        // Move: 's' toma ownership
        Some(s) => println!("Movido: {}", s),
        None => println!("Nada"),
    }
    // opt não pode mais ser usado!
}

fn example_borrowing(opt: &Option<String>) {
    match opt {
        // Borrow: 's' é uma referência
        Some(s) => println!("Emprestado: {}", s),
        None => println!("Nada"),
    }
    // opt ainda pode ser usado!
}

fn example_ref_pattern(opt: Option<String>) {
    match opt {
        // ref cria referência sem mover
        Some(ref s) => println!("Referência: {}", s),
        None => println!("Nada"),
    }
    // opt ainda pode ser usado!
}
~~~

**No Dia 26**, você aprenderá:
- Como patterns interagem com ownership
- Move semantics em destructuring
- Quando usar ref vs ref mut
- Patterns e lifetimes

---

### 📚 Recursos Extras

#### Documentação Oficial
- [The Rust Book - Chapter 18: Patterns](https://doc.rust-lang.org/book/ch18-00-patterns.html)
- [Rust by Example - Match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)
- [Rust Reference - Patterns](https://doc.rust-lang.org/reference/patterns.html)

#### Exercícios Práticos
- [Rustlings - Pattern Matching](https://github.com/rust-lang/rustlings)
- [Exercism - Rust Track](https://exercism.org/tracks/rust)

#### Artigos Avançados
- "Pattern Matching in Rust" - Blog oficial
- "Exhaustiveness Checking" - Rust RFC
- "Refutable vs Irrefutable Patterns" - Rust Nomicon

---

## 🎓 Resumo da Lição

### 🔑 Conceitos-Chave

1. **Destructuring** permite desempacotar estruturas complexas
2. **@ binding** captura valores enquanto testa patterns
3. **Guards** adicionam condições booleanas aos patterns
4. **Ranges** tornam patterns mais expressivos
5. **Múltiplos patterns** com `|` reduzem repetição
6. **ref/ref mut** controlam ownership em patterns
7. **Exhaustividade** garante que todos os casos são tratados

### 💎 Padrões de Elegância

- ✅ Prefira `match` a múltiplos `if/else`
- ✅ Use @ quando precisar do valor E testar um padrão
- ✅ Combine ranges com guards para validações complexas
- ✅ Aproveite destructuring para código declarativo
- ✅ Deixe o compilador garantir exhaustividade

### 🚦 Próximos Passos

1. ✅ Complete o desafio da state machine
2. ✅ Refatore código existente para usar patterns
3. ✅ Pratique com exercícios do Rustlings
4. ✅ Prepare-se para Move Semantics Avançado (Dia 26)

---

## 🎉 Parabéns!

Você dominou **Pattern Matching Avançado** em Rust! 

Agora você pode:
- 🎯 Escrever código mais expressivo e elegante
- 🔍 Desempacotar estruturas complexas com facilidade
- 🛡️ Aproveitar a segurança do compilador
- 🚀 Criar parsers e state machines sofisticados

**Continue praticando e nos vemos no Dia 26!** 🦀✨

---

*"Pattern matching is not just about control flow - it's about expressing intent clearly and letting the compiler ensure correctness."* - The Rust Community