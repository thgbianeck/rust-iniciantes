# 🎭 Dia 26: Move Semantics Avançado - A Dança Coreografada do Rust

## 📋 Objetivos de Aprendizagem

Ao final desta lição, você será capaz de:

- ✅ **Dominar partial moves** e entender quando campos individuais podem ser movidos
- ✅ **Utilizar move closures** para capturar ownership em threads e callbacks
- ✅ **Evitar armadilhas** de moves em loops e aplicar soluções idiomáticas
- ✅ **Diferenciar** `into_iter()`, `iter()` e `iter_mut()` com precisão
- ✅ **Compreender RAII** e drop order para gerenciamento de recursos
- ✅ **Implementar Builder Pattern** fluente e idiomático em Rust

---

## 🎭 Ativação do Conhecimento Prévio

### Revisão Rápida: Move Básico

Você já aprendeu que em Rust:
- **Move transfere ownership** de um valor para outro local
- Após o move, a variável original **não pode mais ser usada**
- Tipos que implementam `Copy` são copiados automaticamente

~~~rust
let s1 = String::from("hello");
let s2 = s1; // s1 foi movido para s2
// println!("{}", s1); // ❌ ERRO: s1 não é mais válido
~~~

### 🎭 Analogia Central: A Dança Coreografada

Imagine uma apresentação de dança profissional:

- **Cada movimento tem propósito** → Cada move em Rust é intencional
- **A sequência importa** → Drop order segue LIFO (Last In, First Out)
- **Elegância na execução** → Código idiomático é fluente e expressivo
- **Coreografia complexa** → Partial moves, closures, builders trabalham em harmonia

Assim como uma coreografia bem ensaiada, o sistema de ownership do Rust permite criar **APIs elegantes e seguras** através de padrões avançados.

### 📖 História: A Evolução dos Padrões

No início, linguagens como C++ tinham construtores complexos e gerenciamento manual de recursos. Java trouxe garbage collection, mas perdeu controle fino. **Rust revolucionou** combinando segurança de memória com padrões elegantes como Builder Pattern e RAII, criando uma nova forma de expressar intenções através do sistema de tipos.

---

## 📚 Apresentação do Conteúdo

### 1️⃣ Partial Moves: Movendo Campos Individuais

**Partial move** ocorre quando você move **apenas alguns campos** de uma struct, deixando outros intactos.

~~~rust
struct Person {
    name: String,
    age: u32,
    email: String,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };
    
    // Move apenas o campo 'name'
    let name = person.name;
    
    // ✅ 'age' ainda pode ser usado (Copy)
    println!("Age: {}", person.age);
    
    // ❌ 'person' como um todo não pode mais ser usado
    // println!("{:?}", person); // ERRO!
    
    // ✅ Mas 'email' ainda está disponível
    println!("Email: {}", person.email);
}
~~~

**Regras Importantes:**
- ✅ Campos `Copy` (como `u32`) podem ser acessados após partial move
- ❌ A struct **como um todo** não pode mais ser usada
- ✅ Campos não movidos ainda podem ser acessados individualmente

#### 📊 Diagrama 1: Partial Moves Ilustrados

~~~mermaid
graph TD
    A[Person Struct] --> B[name: String]
    A --> C[age: u32 Copy]
    A --> D[email: String]
    
    B -->|Move| E[Variável 'name']
    C -->|Acessível| F[Ainda pode usar]
    D -->|Acessível| G[Ainda pode usar]
    
    A -->|Estado| H[❌ Struct completa inacessível]
    
    style B fill:#ff6b6b
    style E fill:#ff6b6b
    style C fill:#51cf66
    style D fill:#51cf66
    style H fill:#ffd43b
~~~

---

### 2️⃣ Move Closures: Capturando Ownership

A palavra-chave `move` força uma closure a **tomar ownership** das variáveis capturadas, em vez de apenas emprestar.

~~~rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    // ❌ Sem 'move': closure empresta 'data'
    // thread::spawn(|| {
    //     println!("{:?}", data); // ERRO: data pode não viver o suficiente
    // });
    
    // ✅ Com 'move': closure toma ownership de 'data'
    let handle = thread::spawn(move || {
        println!("{:?}", data); // OK! Closure possui 'data'
        data.len() // Retorna o tamanho
    });
    
    // ❌ 'data' não pode mais ser usado aqui
    // println!("{:?}", data); // ERRO!
    
    let result = handle.join().unwrap();
    println!("Length: {}", result);
}
~~~

**Quando usar `move`:**
- ✅ **Threads**: para garantir que dados vivam o suficiente
- ✅ **Callbacks assíncronos**: quando a closure pode sobreviver ao escopo
- ✅ **Retornar closures**: quando a closure precisa possuir seus dados

#### 📊 Diagrama 2: Sequência de Move Closure

~~~mermaid
sequenceDiagram
    participant Main as Thread Principal
    participant Data as Vec<i32>
    participant Closure as move || { }
    participant Thread as Nova Thread
    
    Main->>Data: Cria vec![1,2,3,4,5]
    Main->>Closure: move captura ownership
    Data->>Closure: Ownership transferido
    Main->>Thread: spawn(closure)
    Closure->>Thread: Ownership transferido
    Note over Data: ❌ Inacessível no Main
    Thread->>Thread: Usa 'data' com segurança
    Thread->>Main: Retorna resultado
~~~

---

### 3️⃣ Move em Loops: Problema Comum e Soluções

**Problema**: Tentar mover um valor múltiplas vezes em um loop.

~~~rust
fn main() {
    let s = String::from("hello");
    
    // ❌ ERRO: tentando mover 's' múltiplas vezes
    for _ in 0..3 {
        // consume(s); // ERRO: 's' foi movido na primeira iteração
    }
}

fn consume(s: String) {
    println!("{}", s);
}
~~~

**Soluções Idiomáticas:**

#### Solução 1: Clone (quando apropriado)

~~~rust
fn main() {
    let s = String::from("hello");
    
    for _ in 0..3 {
        consume(s.clone()); // ✅ Clona em cada iteração
    }
}
~~~

#### Solução 2: Borrow (preferível)

~~~rust
fn main() {
    let s = String::from("hello");
    
    for _ in 0..3 {
        print_borrowed(&s); // ✅ Apenas empresta
    }
}

fn print_borrowed(s: &str) {
    println!("{}", s);
}
~~~

#### Solução 3: Mover para fora do loop

~~~rust
fn main() {
    for _ in 0..3 {
        let s = String::from("hello"); // ✅ Cria novo em cada iteração
        consume(s);
    }
}
~~~

#### 📊 Diagrama 3: Problema e Solução - Move em Loop

~~~mermaid
graph TB
    subgraph Problema
    A[String s] -->|Iteração 1| B[Move para consume]
    B --> C[❌ s não existe mais]
    C -->|Iteração 2| D[ERRO: tentando mover valor inexistente]
    end
    
    subgraph Solução 1: Clone
    E[String s] -->|Iteração 1| F[s.clone para consume]
    E -->|Iteração 2| G[s.clone para consume]
    E -->|Iteração 3| H[s.clone para consume]
    end
    
    subgraph Solução 2: Borrow
    I[String s] -.->|Iteração 1| J[&s para print]
    I -.->|Iteração 2| K[&s para print]
    I -.->|Iteração 3| L[&s para print]
    end
    
    style D fill:#ff6b6b
    style F fill:#51cf66
    style G fill:#51cf66
    style H fill:#51cf66
    style J fill:#4dabf7
    style K fill:#4dabf7
    style L fill:#4dabf7
~~~

---

### 4️⃣ into_iter() vs iter() vs iter_mut()

Três formas de iterar sobre coleções, cada uma com semântica de ownership diferente:

| Método | Ownership | Tipo do Item | Uso |
|--------|-----------|--------------|-----|
| `into_iter()` | **Move** | `T` | Consome a coleção |
| `iter()` | **Borrow** | `&T` | Leitura imutável |
| `iter_mut()` | **Borrow mutável** | `&mut T` | Modificação |

~~~rust
fn main() {
    // into_iter(): Move/Consome
    let v1 = vec![1, 2, 3];
    for num in v1.into_iter() {
        println!("{}", num); // num é i32 (ownership)
    }
    // println!("{:?}", v1); // ❌ ERRO: v1 foi movido
    
    // iter(): Borrow imutável
    let v2 = vec![1, 2, 3];
    for num in v2.iter() {
        println!("{}", num); // num é &i32 (referência)
    }
    println!("{:?}", v2); // ✅ OK: v2 ainda existe
    
    // iter_mut(): Borrow mutável
    let mut v3 = vec![1, 2, 3];
    for num in v3.iter_mut() {
        *num *= 2; // num é &mut i32
    }
    println!("{:?}", v3); // [2, 4, 6]
}
~~~

**Atalho Sintático:**

~~~rust
let v = vec![1, 2, 3];

// Estas duas formas são equivalentes:
for num in v.into_iter() { }
for num in v { } // ✅ Chama into_iter() implicitamente

// Estas duas formas são equivalentes:
for num in v.iter() { }
for num in &v { } // ✅ Chama iter() implicitamente

// Estas duas formas são equivalentes:
for num in v.iter_mut() { }
for num in &mut v { } // ✅ Chama iter_mut() implicitamente
~~~

#### 📊 Diagrama 4: Comparação into_iter vs iter vs iter_mut

~~~mermaid
graph LR
    A[Vec: 1, 2, 3] --> B{Escolha o método}
    
    B -->|into_iter| C[Move cada elemento]
    C --> D[Item: T]
    C --> E[❌ Vec consumido]
    
    B -->|iter| F[Empresta cada elemento]
    F --> G[Item: &T]
    F --> H[✅ Vec ainda existe]
    
    B -->|iter_mut| I[Empresta mutavelmente]
    I --> J[Item: &mut T]
    I --> K[✅ Vec modificável]
    
    style C fill:#ff6b6b
    style E fill:#ff6b6b
    style F fill:#4dabf7
    style H fill:#51cf66
    style I fill:#ffd43b
    style K fill:#51cf66
~~~

---

### 5️⃣ Drop Order: LIFO dentro de Escopo

Rust garante que valores são **dropped (destruídos)** em ordem **LIFO (Last In, First Out)** - o último criado é o primeiro destruído.

~~~rust
struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Dropping: {}", self.name);
    }
}

fn main() {
    let _r1 = Resource { name: String::from("Resource 1") };
    let _r2 = Resource { name: String::from("Resource 2") };
    let _r3 = Resource { name: String::from("Resource 3") };
    
    println!("End of scope");
}

// Saída:
// End of scope
// Dropping: Resource 3  ← Último criado
// Dropping: Resource 2
// Dropping: Resource 1  ← Primeiro criado
~~~

**Por que LIFO?**
- ✅ **Segurança**: Recursos dependentes são destruídos na ordem correta
- ✅ **Previsibilidade**: Comportamento determinístico
- ✅ **Stack semantics**: Alinha com a pilha de execução

#### 📊 Diagrama 5: Drop Order (Pilha LIFO)

~~~mermaid
graph TD
    subgraph Criação
    A[1. Resource 1 criado] --> B[2. Resource 2 criado]
    B --> C[3. Resource 3 criado]
    end
    
    subgraph Destruição LIFO
    D[3. Resource 3 dropped] --> E[2. Resource 2 dropped]
    E --> F[1. Resource 1 dropped]
    end
    
    C -.->|Fim do escopo| D
    
    style A fill:#51cf66
    style B fill:#51cf66
    style C fill:#51cf66
    style D fill:#ff6b6b
    style E fill:#ff6b6b
    style F fill:#ff6b6b
~~~

---

### 6️⃣ RAII: Resource Acquisition Is Initialization

**RAII** é um padrão onde:
- **Aquisição de recurso** acontece na **inicialização** (construtor)
- **Liberação de recurso** acontece na **destruição** (Drop)

~~~rust
use std::fs::File;
use std::io::Write;

struct FileGuard {
    file: File,
}

impl FileGuard {
    fn new(path: &str) -> std::io::Result<Self> {
        let file = File::create(path)?; // ✅ Aquisição
        println!("File opened: {}", path);
        Ok(FileGuard { file })
    }
    
    fn write(&mut self, data: &str) -> std::io::Result<()> {
        self.file.write_all(data.as_bytes())
    }
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        println!("File closed automatically"); // ✅ Liberação
    }
}

fn main() -> std::io::Result<()> {
    {
        let mut guard = FileGuard::new("output.txt")?;
        guard.write("Hello, RAII!")?;
    } // ← guard é dropped aqui, arquivo fechado automaticamente
    
    println!("File operations complete");
    Ok(())
}
~~~

**Benefícios do RAII:**
- ✅ **Sem vazamentos**: Recursos sempre liberados
- ✅ **Exception safety**: Funciona mesmo com panics
- ✅ **Código limpo**: Sem `finally` ou `defer` explícito

**Exemplos no Rust Standard Library:**
- `File` - fecha arquivo automaticamente
- `MutexGuard` - libera lock automaticamente
- `Box`, `Vec`, `String` - liberam memória automaticamente

---

### 7️⃣ Builder Pattern Idiomático

O **Builder Pattern** em Rust usa **move semantics** para criar APIs fluentes e type-safe.

**Características:**
- ✅ Cada método **consome `self`** e **retorna `Self`**
- ✅ Permite **encadeamento** de métodos
- ✅ `build()` final consome o builder e retorna o produto

#### Exemplo Básico:

~~~rust
struct HttpRequest {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

struct HttpRequestBuilder {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl HttpRequest {
    fn builder(url: impl Into<String>) -> HttpRequestBuilder {
        HttpRequestBuilder {
            url: url.into(),
            method: String::from("GET"),
            headers: Vec::new(),
            body: None,
        }
    }
}

impl HttpRequestBuilder {
    // Cada método consome self e retorna Self
    fn method(mut self, method: impl Into<String>) -> Self {
        self.method = method.into();
        self
    }
    
    fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }
    
    fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
    
    // build() consome o builder e retorna o produto final
    fn build(self) -> HttpRequest {
        HttpRequest {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
        }
    }
}

fn main() {
    let request = HttpRequest::builder("https://api.example.com/users")
        .method("POST")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token123")
        .body(r#"{"name": "Alice", "age": 30}"#)
        .build();
    
    println!("Request to: {}", request.url);
    println!("Method: {}", request.method);
    println!("Headers: {:?}", request.headers);
}
~~~

#### 📊 Diagrama 6: Builder Pattern - Sequência de Moves

~~~mermaid
sequenceDiagram
    participant User as Código Cliente
    participant Builder as HttpRequestBuilder
    participant Request as HttpRequest
    
    User->>Builder: builder("url")
    Note over Builder: Builder criado
    
    User->>Builder: .method("POST")
    Note over Builder: self movido e retornado
    
    User->>Builder: .header("key", "value")
    Note over Builder: self movido e retornado
    
    User->>Builder: .body("data")
    Note over Builder: self movido e retornado
    
    User->>Builder: .build()
    Builder->>Request: Consome builder
    Note over Builder: ❌ Builder destruído
    Request->>User: HttpRequest criado
    Note over Request: ✅ Produto final
~~~

#### Builder Pattern Avançado: Type State

Para garantir que `build()` só seja chamado quando todos os campos obrigatórios estiverem definidos:

~~~rust
// Estados do tipo
struct NoUrl;
struct WithUrl;
struct NoMethod;
struct WithMethod;

// Builder genérico com estados
struct TypedBuilder<U, M> {
    url: Option<String>,
    method: Option<String>,
    headers: Vec<(String, String)>,
    _url_state: std::marker::PhantomData<U>,
    _method_state: std::marker::PhantomData<M>,
}

impl TypedBuilder<NoUrl, NoMethod> {
    fn new() -> Self {
        TypedBuilder {
            url: None,
            method: None,
            headers: Vec::new(),
            _url_state: std::marker::PhantomData,
            _method_state: std::marker::PhantomData,
        }
    }
}

impl<M> TypedBuilder<NoUrl, M> {
    fn url(self, url: impl Into<String>) -> TypedBuilder<WithUrl, M> {
        TypedBuilder {
            url: Some(url.into()),
            method: self.method,
            headers: self.headers,
            _url_state: std::marker::PhantomData,
            _method_state: std::marker::PhantomData,
        }
    }
}

impl<U> TypedBuilder<U, NoMethod> {
    fn method(self, method: impl Into<String>) -> TypedBuilder<U, WithMethod> {
        TypedBuilder {
            url: self.url,
            method: Some(method.into()),
            headers: self.headers,
            _url_state: std::marker::PhantomData,
            _method_state: std::marker::PhantomData,
        }
    }
}

impl<U, M> TypedBuilder<U, M> {
    fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }
}

// build() só disponível quando URL e Method estão definidos
impl TypedBuilder<WithUrl, WithMethod> {
    fn build(self) -> HttpRequest {
        HttpRequest {
            url: self.url.unwrap(),
            method: self.method.unwrap(),
            headers: self.headers,
            body: None,
        }
    }
}

fn main() {
    // ✅ Compila: todos os campos obrigatórios definidos
    let request = TypedBuilder::new()
        .url("https://api.example.com")
        .method("POST")
        .header("Content-Type", "application/json")
        .build();
    
    // ❌ Não compila: falta method()
    // let invalid = TypedBuilder::new()
    //     .url("https://api.example.com")
    //     .build(); // ERRO: build() não existe para este estado
}
~~~

---

## 💡 Demonstração e Modelagem

### Exemplo Completo: Sistema de Configuração com Builder

~~~rust
use std::collections::HashMap;

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    max_connections: usize,
    timeout_seconds: u64,
    ssl_enabled: bool,
    routes: HashMap<String, String>,
}

struct ServerConfigBuilder {
    host: String,
    port: u16,
    max_connections: usize,
    timeout_seconds: u64,
    ssl_enabled: bool,
    routes: HashMap<String, String>,
}

impl ServerConfig {
    fn builder() -> ServerConfigBuilder {
        ServerConfigBuilder {
            host: String::from("127.0.0.1"),
            port: 8080,
            max_connections: 100,
            timeout_seconds: 30,
            ssl_enabled: false,
            routes: HashMap::new(),
        }
    }
}

impl ServerConfigBuilder {
    fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }
    
    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    
    fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }
    
    fn timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }
    
    fn enable_ssl(mut self) -> Self {
        self.ssl_enabled = true;
        self
    }
    
    fn route(mut self, path: impl Into<String>, handler: impl Into<String>) -> Self {
        self.routes.insert(path.into(), handler.into());
        self
    }
    
    fn build(self) -> ServerConfig {
        ServerConfig {
            host: self.host,
            port: self.port,
            max_connections: self.max_connections,
            timeout_seconds: self.timeout_seconds,
            ssl_enabled: self.ssl_enabled,
            routes: self.routes,
        }
    }
}

fn main() {
    let config = ServerConfig::builder()
        .host("0.0.0.0")
        .port(443)
        .max_connections(1000)
        .timeout(60)
        .enable_ssl()
        .route("/api/users", "users_handler")
        .route("/api/posts", "posts_handler")
        .route("/health", "health_check")
        .build();
    
    println!("{:#?}", config);
}
~~~

**Saída:**
~~~
ServerConfig {
    host: "0.0.0.0",
    port: 443,
    max_connections: 1000,
    timeout_seconds: 60,
    ssl_enabled: true,
    routes: {
        "/api/users": "users_handler",
        "/api/posts": "posts_handler",
        "/health": "health_check",
    },
}
~~~

---

## 🎯 Prática Guiada: Exercício Completo

### 🎯 Exercício: API Client Builder Fluente

**Contexto:** Você está desenvolvendo um cliente HTTP para uma API REST. Precisa criar um builder pattern que permita configurar requisições de forma elegante e type-safe.

**Requisitos:**
1. ✅ Struct `ApiClient` com campos: `base_url`, `api_key`, `timeout`, `retry_count`
2. ✅ Builder que consome `self` em cada método
3. ✅ Métodos encadeáveis: `base_url()`, `api_key()`, `timeout()`, `retry()`
4. ✅ Método `build()` final que valida e retorna `Result<ApiClient, String>`
5. ✅ Validação: `base_url` deve começar com "http://" ou "https://"
6. ✅ Valores padrão: `timeout = 30s`, `retry_count = 3`

**Esqueleto do Código:**

~~~rust
use std::time::Duration;

#[derive(Debug)]
struct ApiClient {
    base_url: String,
    api_key: Option<String>,
    timeout: Duration,
    retry_count: u32,
}

struct ApiClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
    timeout: Duration,
    retry_count: u32,
}

impl ApiClient {
    fn builder() -> ApiClientBuilder {
        // TODO: Implementar
        todo!()
    }
    
    fn get(&self, endpoint: &str) -> String {
        format!("GET {}{}", self.base_url, endpoint)
    }
}

impl ApiClientBuilder {
    fn base_url(self, url: impl Into<String>) -> Self {
        // TODO: Implementar
        todo!()
    }
    
    fn api_key(self, key: impl Into<String>) -> Self {
        // TODO: Implementar
        todo!()
    }
    
    fn timeout(self, seconds: u64) -> Self {
        // TODO: Implementar
        todo!()
    }
    
    fn retry(self, count: u32) -> Self {
        // TODO: Implementar
        todo!()
    }
    
    fn build(self) -> Result<ApiClient, String> {
        // TODO: Validar base_url
        // TODO: Retornar ApiClient ou erro
        todo!()
    }
}

fn main() {
    // Teste 1: Cliente válido
    let client = ApiClient::builder()
        .base_url("https://api.github.com")
        .api_key("ghp_1234567890")
        .timeout(60)
        .retry(5)
        .build()
        .expect("Failed to build client");
    
    println!("{:#?}", client);
    println!("{}", client.get("/users/octocat"));
    
    // Teste 2: URL inválida (deve falhar)
    let invalid = ApiClient::builder()
        .base_url("ftp://invalid.com")
        .build();
    
    assert!(invalid.is_err());
    println!("Error: {}", invalid.unwrap_err());
}
~~~

---

### ✅ Solução Completa

~~~rust
use std::time::Duration;

#[derive(Debug)]
struct ApiClient {
    base_url: String,
    api_key: Option<String>,
    timeout: Duration,
    retry_count: u32,
}

struct ApiClientBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
    timeout: Duration,
    retry_count: u32,
}

impl ApiClient {
    fn builder() -> ApiClientBuilder {
        ApiClientBuilder {
            base_url: None,
            api_key: None,
            timeout: Duration::from_secs(30), // Padrão: 30s
            retry_count: 3,                    // Padrão: 3 tentativas
        }
    }
    
    fn get(&self, endpoint: &str) -> String {
        format!("GET {}{}", self.base_url, endpoint)
    }
    
    fn post(&self, endpoint: &str, body: &str) -> String {
        format!("POST {}{} - Body: {}", self.base_url, endpoint, body)
    }
}

impl ApiClientBuilder {
    fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self // Move e retorna self
    }
    
    fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }
    
    fn timeout(mut self, seconds: u64) -> Self {
        self.timeout = Duration::from_secs(seconds);
        self
    }
    
    fn retry(mut self, count: u32) -> Self {
        self.retry_count = count;
        self
    }
    
    fn build(self) -> Result<ApiClient, String> {
        // Validação: base_url é obrigatório
        let base_url = self.base_url
            .ok_or_else(|| String::from("base_url is required"))?;
        
        // Validação: deve começar com http:// ou https://
        if !base_url.starts_with("http://") && !base_url.starts_with("https://") {
            return Err(format!(
                "Invalid base_url: '{}'. Must start with http:// or https://",
                base_url
            ));
        }
        
        // Validação: retry_count deve ser razoável
        if self.retry_count > 10 {
            return Err(String::from("retry_count must be <= 10"));
        }
        
        Ok(ApiClient {
            base_url,
            api_key: self.api_key,
            timeout: self.timeout,
            retry_count: self.retry_count,
        })
    }
}

fn main() {
    println!("=== Teste 1: Cliente válido ===");
    let client = ApiClient::builder()
        .base_url("https://api.github.com")
        .api_key("ghp_1234567890")
        .timeout(60)
        .retry(5)
        .build()
        .expect("Failed to build client");
    
    println!("{:#?}", client);
    println!("{}", client.get("/users/octocat"));
    println!("{}", client.post("/repos", r#"{"name": "my-repo"}"#));
    
    println!("\n=== Teste 2: Cliente com valores padrão ===");
    let default_client = ApiClient::builder()
        .base_url("https://jsonplaceholder.typicode.com")
        .build()
        .expect("Failed to build client");
    
    println!("{:#?}", default_client);
    
    println!("\n=== Teste 3: URL inválida (deve falhar) ===");
    let invalid = ApiClient::builder()
        .base_url("ftp://invalid.com")
        .build();
    
    match invalid {
        Ok(_) => println!("❌ Deveria ter falhado!"),
        Err(e) => println!("✅ Erro esperado: {}", e),
    }
    
    println!("\n=== Teste 4: Sem base_url (deve falhar) ===");
    let no_url = ApiClient::builder()
        .api_key("key123")
        .build();
    
    match no_url {
        Ok(_) => println!("❌ Deveria ter falhado!"),
        Err(e) => println!("✅ Erro esperado: {}", e),
    }
}
~~~

**Saída Esperada:**
~~~
=== Teste 1: Cliente válido ===
ApiClient {
    base_url: "https://api.github.com",
    api_key: Some(
        "ghp_1234567890",
    ),
    timeout: 60s,
    retry_count: 5,
}
GET https://api.github.com/users/octocat
POST https://api.github.com/repos - Body: {"name": "my-repo"}

=== Teste 2: Cliente com valores padrão ===
ApiClient {
    base_url: "https://jsonplaceholder.typicode.com",
    api_key: None,
    timeout: 30s,
    retry_count: 3,
}

=== Teste 3: URL inválida (deve falhar) ===
✅ Erro esperado: Invalid base_url: 'ftp://invalid.com'. Must start with http:// or https://

=== Teste 4: Sem base_url (deve falhar) ===
✅ Erro esperado: base_url is required
~~~

---

### 📊 Análise da Solução

**Pontos-Chave:**

1. **Move Semantics em Ação:**
   - Cada método (`base_url`, `api_key`, etc.) recebe `self` por valor
   - Retorna `Self`, permitindo encadeamento
   - O builder é **movido** em cada chamada

2. **Validação Robusta:**
   - `build()` retorna `Result<ApiClient, String>`
   - Valida URL obrigatória e formato correto
   - Previne configurações inválidas em tempo de compilação

3. **Valores Padrão:**
   - `timeout = 30s` e `retry_count = 3` definidos no construtor
   - Podem ser sobrescritos opcionalmente

4. **API Fluente:**
   - Leitura natural: `builder().base_url(...).api_key(...).build()`
   - Type-safe: impossível usar o builder após `build()`

---

## 🔄 Feedback e Avaliação

### ✅ Checklist de Padrões Avançados

Marque cada item que você domina:

- [ ] **Partial Moves**: Entendo quando posso mover campos individuais
- [ ] **Move Closures**: Sei quando usar `move` em closures
- [ ] **Loops**: Evito mover valores múltiplas vezes em loops
- [ ] **Iteradores**: Diferencio `into_iter()`, `iter()` e `iter_mut()`
- [ ] **Drop Order**: Compreendo LIFO e suas implicações
- [ ] **RAII**: Aplico o padrão para gerenciar recursos
- [ ] **Builder Pattern**: Implemento builders fluentes e type-safe

---

### 🧠 Quiz: Move Semantics Avançado

**Questão 1:** O que acontece após um partial move?

a) A struct inteira fica inacessível  
b) Apenas campos não-Copy ficam inacessíveis  
c) Campos Copy ainda podem ser acessados  
d) Nada, partial moves não existem em Rust  

<details>
<summary>Ver Resposta</summary>

**Resposta: c) Campos Copy ainda podem ser acessados**

Após um partial move, campos `Copy` (como `i32`, `bool`) ainda podem ser acessados, mas a struct como um todo e campos não-Copy movidos ficam inacessíveis.
</details>

---

**Questão 2:** Qual a diferença entre `for x in vec` e `for x in &vec`?

a) Não há diferença  
b) O primeiro move o vec, o segundo empresta  
c) O primeiro é mais rápido  
d) O segundo não compila  

<details>
<summary>Ver Resposta</summary>

**Resposta: b) O primeiro move o vec, o segundo empresta**

- `for x in vec` → chama `into_iter()`, move/consome o vec
- `for x in &vec` → chama `iter()`, apenas empresta
</details>

---

**Questão 3:** Em que ordem os valores são dropped?

~~~rust
let a = String::from("A");
let b = String::from("B");
let c = String::from("C");
~~~

a) A, B, C  
b) C, B, A  
c) Ordem indefinida  
d) Todos ao mesmo tempo  

<details>
<summary>Ver Resposta</summary>

**Resposta: b) C, B, A**

Drop order é **LIFO (Last In, First Out)**. O último valor criado (`c`) é o primeiro a ser dropped.
</details>

---

**Questão 4:** Por que usar `move` em closures para threads?

a) Para melhorar performance  
b) Para garantir que dados vivam o suficiente  
c) Para evitar race conditions  
d) É opcional, não faz diferença  

<details>
<summary>Ver Resposta</summary>

**Resposta: b) Para garantir que dados vivam o suficiente**

Threads podem sobreviver ao escopo onde foram criadas. `move` transfere ownership para a closure, garantindo que os dados vivam enquanto a thread existir.
</details>

---

**Questão 5:** No Builder Pattern, por que métodos consomem `self`?

a) Para economizar memória  
b) Para permitir encadeamento fluente  
c) Para melhorar performance  
d) Para evitar clones  

<details>
<summary>Ver Resposta</summary>

**Resposta: b) Para permitir encadeamento fluente**

Consumir `self` e retornar `Self` permite encadeamento: `builder().method1().method2().build()`. Também garante que o builder não seja usado após `build()`.
</details>

---

### 🔧 Exercícios de Refatoração

**Exercício 1:** Refatore este código para evitar o erro de move em loop:

~~~rust
fn main() {
    let message = String::from("Hello");
    
    for i in 0..3 {
        print_message(message); // ERRO!
    }
}

fn print_message(msg: String) {
    println!("{}", msg);
}
~~~

<details>
<summary>Ver Solução</summary>

~~~rust
fn main() {
    let message = String::from("Hello");
    
    for i in 0..3 {
        print_message(&message); // ✅ Empresta
    }
}

fn print_message(msg: &str) {
    println!("{}", msg);
}
~~~
</details>

---

**Exercício 2:** Implemente um builder simples para esta struct:

~~~rust
struct User {
    username: String,
    email: String,
    age: Option<u32>,
}
~~~

<details>
<summary>Ver Solução</summary>

~~~rust
struct User {
    username: String,
    email: String,
    age: Option<u32>,
}

struct UserBuilder {
    username: String,
    email: String,
    age: Option<u32>,
}

impl User {
    fn builder(username: impl Into<String>, email: impl Into<String>) -> UserBuilder {
        UserBuilder {
            username: username.into(),
            email: email.into(),
            age: None,
        }
    }
}

impl UserBuilder {
    fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }
    
    fn build(self) -> User {
        User {
            username: self.username,
            email: self.email,
            age: self.age,
        }
    }
}

fn main() {
    let user = User::builder("alice", "alice@example.com")
        .age(30)
        .build();
    
    println!("{} - {}", user.username, user.email);
}
~~~
</details>

---

### 📊 Auto-Avaliação

Avalie seu nível de confiança em cada tópico (1-5):

| Tópico | Confiança (1-5) | Precisa Revisar? |
|--------|-----------------|------------------|
| Partial Moves | __ | [ ] |
| Move Closures | __ | [ ] |
| Move em Loops | __ | [ ] |
| Iteradores | __ | [ ] |
| Drop Order | __ | [ ] |
| RAII | __ | [ ] |
| Builder Pattern | __ | [ ] |

**Se marcou < 4 em algum tópico:** Revise a seção correspondente e pratique os exercícios.

---

## 🚀 Transferência e Aplicação

### 🎯 Desafio Final: Query Builder para Banco de Dados

Crie um **query builder** idiomático para construir queries SQL de forma type-safe:

**Requisitos:**
1. ✅ Métodos: `select()`, `from()`, `where_clause()`, `order_by()`, `limit()`
2. ✅ Validação: `from()` é obrigatório
3. ✅ `build()` retorna `Result<String, String>` com a query SQL
4. ✅ Encadeamento fluente

**Exemplo de Uso:**
~~~rust
let query = QueryBuilder::new()
    .select(&["id", "name", "email"])
    .from("users")
    .where_clause("age > 18")
    .order_by("name", "ASC")
    .limit(10)
    .build()
    .unwrap();

// Resultado: "SELECT id, name, email FROM users WHERE age > 18 ORDER BY name ASC LIMIT 10"
~~~

**Dica:** Use `Vec<String>` para armazenar colunas e condições.

---

### 🔗 Preparação para Dia 27: Memory Layout

No próximo dia, você aprenderá:
- **Representação em memória** de tipos Rust
- **Alinhamento e padding**
- **Zero-cost abstractions**
- **Unsafe Rust** e ponteiros raw

**Conceitos que você já domina e serão úteis:**
- ✅ Ownership e move semantics
- ✅ Drop order e RAII
- ✅ Stack vs Heap

---

### 📚 Recursos Adicionais

**Documentação Oficial:**
- [The Rust Book - Advanced Features](https://doc.rust-lang.org/book/ch19-00-advanced-features.html)
- [Rust By Example - RAII](https://doc.rust-lang.org/rust-by-example/scope/raii.html)
- [API Guidelines - Builder Pattern](https://rust-lang.github.io/api-guidelines/)

**Crates Populares com Builder Pattern:**
- `reqwest` - HTTP client
- `tokio` - Runtime assíncrono
- `clap` - CLI argument parser

**Artigos Recomendados:**
- "Builder Pattern in Rust" - Pascal Hertleif
- "RAII: Resource Acquisition Is Initialization" - Rust Blog
- "Move Semantics in Rust" - Niko Matsakis

---

## 🎓 Resumo da Lição

### Conceitos-Chave Dominados:

1. **Partial Moves** → Mover campos individuais de structs
2. **Move Closures** → Capturar ownership com `move`
3. **Move em Loops** → Evitar armadilhas com borrow ou clone
4. **Iteradores** → `into_iter()` (move), `iter()` (borrow), `iter_mut()` (borrow mut)
5. **Drop Order** → LIFO dentro de escopo
6. **RAII** → Aquisição = Inicialização, Liberação = Drop
7. **Builder Pattern** → APIs fluentes com move semantics

### Padrões Idiomáticos:

- ✅ **Consumir `self`** em builders para encadeamento
- ✅ **Validar em `build()`** para garantir type-safety
- ✅ **Usar `move` em threads** para transferir ownership
- ✅ **Preferir borrow em loops** para evitar clones desnecessários
- ✅ **Implementar `Drop`** para RAII automático

---

## 🎉 Parabéns!

Você completou o **Dia 26: Move Semantics Avançado**! 

Agora você domina os padrões mais elegantes e idiomáticos do Rust, incluindo o poderoso **Builder Pattern**. Esses conceitos são fundamentais para criar **APIs fluentes, type-safe e expressivas**.

**Próximo passo:** Dia 27 - Memory Layout e Unsafe Rust 🚀

---

**Dica Final:** A maestria em move semantics avançado é o que separa código Rust **funcional** de código Rust **idiomático e elegante**. Continue praticando esses padrões em seus projetos!