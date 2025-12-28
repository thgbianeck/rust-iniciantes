# 🦀 Dia 27: Memory Layout e Unsafe Rust (⚠️ CUIDADO!)

## ⚠️ AVISO CRÍTICO ANTES DE COMEÇAR

**Unsafe Rust é uma ferramenta EXCEPCIONAL para casos EXCEPCIONAIS.**

- ✅ 99% do código Rust é **100% safe**
- ❌ Unsafe **NÃO** é mais rápido automaticamente
- ❌ Unsafe **NÃO** é "Rust avançado"
- ⚠️ Unsafe é **responsabilidade total do programador**
- 🎯 Meta: usar Rust **safe** sempre que possível

---

## 📋 OBJETIVOS DE APRENDIZAGEM

Ao final desta aula, você será capaz de:

1. **Compreender** como Rust organiza dados na memória (memory layout)
2. **Entender** o que é unsafe Rust e seus 5 superpoderes
3. **Reconhecer** quando unsafe é realmente necessário (raramente!)
4. **Criar** abstrações seguras sobre código unsafe
5. **Avaliar** alternativas safe antes de considerar unsafe

---

## 🎭 ATIVAÇÃO DO CONHECIMENTO PRÉVIO

### Revisão Rápida: Ownership e Memória

Você já aprendeu que Rust gerencia memória através de:
- **Ownership**: cada valor tem um dono
- **Borrowing**: referências com regras em tempo de compilação
- **Lifetimes**: garantias de validade de referências

Tudo isso acontece em **safe Rust**, onde o compilador protege você.

### 🏠 Analogia Central: "O Porão da Casa"

Imagine Rust como uma **casa moderna e segura**:

- **Andares superiores (Safe Rust)**: 
  - Todas as salas têm proteções
  - Tomadas com proteção infantil
  - Escadas com corrimão
  - Janelas com travas
  - **Você pode circular livremente e com segurança**

- **Porão (Unsafe Rust)**:
  - Fiação elétrica exposta
  - Ferramentas perigosas
  - Estrutura da casa visível
  - Sem proteções automáticas
  - **Só descer quando REALMENTE necessário**
  - **Trancar bem ao sair** (criar abstrações seguras)

### 📖 História: A Responsabilidade do Poder

Imagine um desenvolvedor que descobriu `unsafe`:

> "Wow! Posso fazer QUALQUER coisa agora! Vou usar em todo lugar!"

**Resultado**: Bugs de memória, crashes, vulnerabilidades de segurança.

**Lição**: Unsafe não é liberdade, é **responsabilidade**. É como dirigir um carro de corrida - requer habilidade, cuidado e contexto apropriado.

---

## 📚 PARTE 1: MEMORY LAYOUT DE STRUCTS

### O Que É Memory Layout?

Memory layout é **como Rust organiza os dados na memória**. Entender isso é fundamental para:
- Interoperabilidade com C (FFI)
- Otimizações de performance
- Trabalhar com hardware diretamente

### Exemplo Básico: Struct em Memória

~~~rust
struct Point {
    x: i32,  // 4 bytes
    y: i32,  // 4 bytes
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("Tamanho de Point: {} bytes", std::mem::size_of::<Point>());
    // Saída: Tamanho de Point: 8 bytes
}
~~~

**Visualização em memória:**

~~~
Endereço    | Conteúdo
------------|----------
0x1000      | 10 (x)
0x1004      | 20 (y)
~~~

---

## 📊 DIAGRAMA 1: Memory Layout de Struct

~~~mermaid
graph TD
    A[Struct Point em Memória] --> B[Byte 0-3: x i32]
    A --> C[Byte 4-7: y i32]
    
    B --> B1[0x00 0x00 0x00 0x0A]
    C --> C1[0x00 0x00 0x00 0x14]
    
    style A fill:#e1f5ff
    style B fill:#c8e6c9
    style C fill:#c8e6c9
    style B1 fill:#fff9c4
    style C1 fill:#fff9c4
~~~

---

### Alinhamento e Padding

**Alinhamento** é um requisito de hardware: dados devem começar em endereços específicos.

**Regra geral**: Um tipo de N bytes deve estar alinhado em múltiplo de N.

~~~rust
struct Example1 {
    a: u8,   // 1 byte
    b: u32,  // 4 bytes
    c: u8,   // 1 byte
}

fn main() {
    println!("Tamanho: {} bytes", std::mem::size_of::<Example1>());
    // Saída: Tamanho: 12 bytes (não 6!)
}
~~~

**Por quê 12 bytes?** Por causa do **padding** (preenchimento):

~~~
Offset  | Campo | Bytes
--------|-------|-------
0       | a     | 1
1-3     | PAD   | 3 (padding para alinhar b)
4-7     | b     | 4
8       | c     | 1
9-11    | PAD   | 3 (padding final)
Total: 12 bytes
~~~

---

## 📊 DIAGRAMA 2: Alinhamento e Padding Visualizado

~~~mermaid
graph LR
    subgraph "Example1 - 12 bytes total"
        A[a: u8<br/>1 byte] --> P1[Padding<br/>3 bytes]
        P1 --> B[b: u32<br/>4 bytes]
        B --> C[c: u8<br/>1 byte]
        C --> P2[Padding<br/>3 bytes]
    end
    
    style A fill:#4caf50
    style B fill:#2196f3
    style C fill:#4caf50
    style P1 fill:#ffeb3b
    style P2 fill:#ffeb3b
~~~

---

### Otimizando Layout: Reordenando Campos

~~~rust
struct Optimized {
    b: u32,  // 4 bytes
    a: u8,   // 1 byte
    c: u8,   // 1 byte
}

fn main() {
    println!("Tamanho: {} bytes", std::mem::size_of::<Optimized>());
    // Saída: Tamanho: 8 bytes
}
~~~

**Layout otimizado:**

~~~
Offset  | Campo | Bytes
--------|-------|-------
0-3     | b     | 4
4       | a     | 1
5       | c     | 1
6-7     | PAD   | 2
Total: 8 bytes
~~~

**Dica**: Rust pode reordenar campos automaticamente para otimizar! (a menos que você use `#[repr(C)]`)

---

## 🎛️ Atributos de Representação: #[repr(...)]

### #[repr(C)] - Layout Compatível com C

~~~rust
#[repr(C)]
struct CCompatible {
    x: i32,
    y: i32,
}
~~~

**Quando usar**: Interoperabilidade com C (FFI)

**Efeito**: 
- Campos na ordem declarada
- Padding compatível com C
- Tamanho previsível

---

### #[repr(packed)] - Sem Padding

⚠️ **CUIDADO**: Pode causar problemas de performance!

~~~rust
#[repr(packed)]
struct Packed {
    a: u8,
    b: u32,
    c: u8,
}

fn main() {
    println!("Tamanho: {} bytes", std::mem::size_of::<Packed>());
    // Saída: Tamanho: 6 bytes
}
~~~

**Quando usar**: Protocolos de rede, formatos de arquivo binário

**Problemas**:
- Acesso não-alinhado é lento em muitas CPUs
- Pode causar crashes em algumas arquiteturas

---

### #[repr(align(N))] - Alinhamento Customizado

~~~rust
#[repr(align(16))]
struct Aligned {
    data: u8,
}

fn main() {
    println!("Tamanho: {} bytes", std::mem::size_of::<Aligned>());
    // Saída: Tamanho: 16 bytes
    println!("Alinhamento: {} bytes", std::mem::align_of::<Aligned>());
    // Saída: Alinhamento: 16 bytes
}
~~~

**Quando usar**: Otimizações SIMD, cache line alignment

---

## 📊 DIAGRAMA 3: Comparação de Representações

~~~mermaid
graph TD
    A[Struct com a: u8, b: u32, c: u8] --> B[repr Rust padrão]
    A --> C[repr C]
    A --> D[repr packed]
    
    B --> B1[Pode reordenar<br/>Tamanho: otimizado]
    C --> C1[Ordem preservada<br/>Tamanho: 12 bytes]
    D --> D1[Sem padding<br/>Tamanho: 6 bytes]
    
    style A fill:#e1f5ff
    style B fill:#c8e6c9
    style C fill:#fff9c4
    style D fill:#ffcdd2
~~~

---

## ⚠️ PARTE 2: UNSAFE RUST - O PORÃO

### O Que É Unsafe?

**Unsafe Rust** é um subconjunto da linguagem onde você pode:
- Desabilitar algumas verificações do compilador
- Assumir responsabilidade manual por garantias de segurança

### Por Que Unsafe Existe?

1. **FFI (Foreign Function Interface)**: Chamar código C/C++
2. **Otimizações extremas**: Casos raríssimos onde safe é insuficiente
3. **Abstrações de baixo nível**: Construir estruturas de dados fundamentais
4. **Hardware direto**: Sistemas embarcados, drivers

### ⚠️ AVISO IMPORTANTE

~~~
┌─────────────────────────────────────────────┐
│  UNSAFE NÃO É:                              │
│  ❌ Mais rápido automaticamente             │
│  ❌ Necessário para código performático     │
│  ❌ "Rust avançado" ou "Rust real"          │
│  ❌ Algo para usar casualmente              │
│                                             │
│  UNSAFE É:                                  │
│  ✅ Escape hatch para casos específicos     │
│  ✅ Responsabilidade total do programador   │
│  ✅ Fonte potencial de bugs graves          │
│  ✅ Último recurso após esgotar alternativas│
└─────────────────────────────────────────────┘
~~~

---

## 🦸 Os 5 Superpoderes de Unsafe

Dentro de um bloco `unsafe`, você pode:

### 1. Derreferenciar Raw Pointers

~~~rust
fn main() {
    let x = 42;
    let raw_ptr = &x as *const i32;
    
    // ❌ ERRO: não pode derreferenciar raw pointer em safe code
    // let value = *raw_ptr;
    
    // ✅ OK: dentro de unsafe
    unsafe {
        let value = *raw_ptr;
        println!("Valor: {}", value);
    }
}
~~~

---

### 2. Chamar Unsafe Functions

~~~rust
unsafe fn dangerous() {
    println!("Fazendo algo perigoso!");
}

fn main() {
    // ❌ ERRO: não pode chamar unsafe function em safe code
    // dangerous();
    
    // ✅ OK: dentro de unsafe
    unsafe {
        dangerous();
    }
}
~~~

---

### 3. Acessar/Modificar Static Mut

~~~rust
static mut COUNTER: u32 = 0;

fn increment() {
    unsafe {
        COUNTER += 1;
    }
}

fn main() {
    increment();
    unsafe {
        println!("Counter: {}", COUNTER);
    }
}
~~~

⚠️ **PROBLEMA**: Race conditions! Use `AtomicU32` ou `Mutex` em vez disso.

---

### 4. Implementar Unsafe Traits

~~~rust
unsafe trait UnsafeTrait {
    fn do_something(&self);
}

struct MyType;

// Implementar unsafe trait requer unsafe
unsafe impl UnsafeTrait for MyType {
    fn do_something(&self) {
        println!("Implementação unsafe");
    }
}
~~~

---

### 5. Acessar Fields de Union

~~~rust
union MyUnion {
    i: i32,
    f: f32,
}

fn main() {
    let u = MyUnion { i: 42 };
    
    unsafe {
        println!("Como i32: {}", u.i);
        println!("Como f32: {}", u.f); // ⚠️ Interpretação incorreta!
    }
}
~~~

---

## 📊 DIAGRAMA 4: Hierarquia Safe → Unsafe

~~~mermaid
graph TB
    A[Código Rust] --> B{Safe ou Unsafe?}
    
    B -->|99% dos casos| C[Safe Rust]
    B -->|1% dos casos| D[Unsafe Rust]
    
    C --> C1[Compilador garante<br/>segurança de memória]
    C --> C2[Sem data races]
    C --> C3[Sem undefined behavior]
    
    D --> D1[Programador garante<br/>segurança]
    D --> D2[5 superpoderes]
    D --> D3[Responsabilidade total]
    
    D --> E[Meta: Abstrações Seguras]
    E --> F[API pública safe]
    E --> G[Unsafe encapsulado]
    
    style C fill:#c8e6c9
    style D fill:#ffcdd2
    style E fill:#fff9c4
~~~

---

## 🔍 Raw Pointers: *const T e *mut T

### Diferença entre Referências e Raw Pointers

| Característica | Referência (&T, &mut T) | Raw Pointer (*const T, *mut T) |
|----------------|-------------------------|--------------------------------|
| Verificação | Compilador verifica | Sem verificação |
| Null | Nunca null | Pode ser null |
| Dereferência | Safe | Unsafe |
| Lifetime | Verificado | Não verificado |
| Aliasing | Regras estritas | Sem regras |

---

### Criando Raw Pointers (Safe)

~~~rust
fn main() {
    let x = 42;
    
    // ✅ Criar raw pointers é SAFE
    let raw_const: *const i32 = &x;
    let raw_mut: *mut i32 = &x as *const i32 as *mut i32;
    
    println!("Raw pointer: {:p}", raw_const);
    
    // ❌ Derreferenciar é UNSAFE
    // let value = *raw_const; // ERRO!
}
~~~

---

### Usando Raw Pointers (Unsafe)

~~~rust
fn main() {
    let mut x = 42;
    let raw = &mut x as *mut i32;
    
    unsafe {
        *raw = 100;
        println!("Valor modificado: {}", *raw);
    }
    
    println!("x agora é: {}", x);
}
~~~

---

### ⚠️ Perigos dos Raw Pointers

~~~rust
fn dangerous_example() {
    let raw: *const i32 = std::ptr::null();
    
    unsafe {
        // ⚠️ CRASH! Dereferenciando ponteiro null
        // let value = *raw;
    }
}

fn dangling_pointer() {
    let raw: *const i32;
    
    {
        let x = 42;
        raw = &x;
    } // x é destruído aqui
    
    unsafe {
        // ⚠️ UNDEFINED BEHAVIOR! Ponteiro dangling
        // let value = *raw;
    }
}
~~~

---

## 📊 DIAGRAMA 5: Fluxograma - Quando Considerar Unsafe?

~~~mermaid
graph TD
    A[Preciso de funcionalidade X] --> B{Existe solução<br/>em safe Rust?}
    
    B -->|Sim| C[✅ USE SAFE RUST]
    B -->|Não| D{Existe crate<br/>que resolve?}
    
    D -->|Sim| E[✅ USE A CRATE]
    D -->|Não| F{É realmente<br/>necessário?}
    
    F -->|Não| G[✅ REPENSE O DESIGN]
    F -->|Sim| H{Tenho conhecimento<br/>profundo de unsafe?}
    
    H -->|Não| I[✅ APRENDA MAIS<br/>OU PEÇA AJUDA]
    H -->|Sim| J[⚠️ CONSIDERE UNSAFE]
    
    J --> K[Documente invariantes]
    J --> L[Crie abstração segura]
    J --> M[Teste extensivamente]
    J --> N[Code review rigoroso]
    
    style C fill:#c8e6c9
    style E fill:#c8e6c9
    style G fill:#c8e6c9
    style I fill:#fff9c4
    style J fill:#ffcdd2
~~~

---

## 🎯 QUANDO UNSAFE É NECESSÁRIO

### Caso 1: FFI (Foreign Function Interface)

Chamar funções C sempre requer `unsafe`:

~~~rust
// Declarar função C
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    let x = -42;
    
    // Chamar função C requer unsafe
    let result = unsafe {
        abs(x)
    };
    
    println!("Valor absoluto: {}", result);
}
~~~

**Por quê unsafe?** Rust não pode verificar garantias de código C.

---

### Caso 2: Otimizações Críticas de Performance

⚠️ **RARAMENTE NECESSÁRIO!** Profile primeiro!

~~~rust
// Exemplo: acesso não verificado a slice
fn sum_unchecked(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..slice.len() {
        unsafe {
            // Pula verificação de bounds
            sum += slice.get_unchecked(i);
        }
    }
    sum
}

// ✅ ALTERNATIVA SAFE (geralmente tão rápida):
fn sum_safe(slice: &[i32]) -> i32 {
    slice.iter().sum()
}
~~~

**Lição**: O compilador Rust é MUITO inteligente. Muitas vezes elimina bounds checks automaticamente!

---

### Caso 3: Estruturas de Dados Fundamentais

Implementar `Vec`, `Box`, `Rc`, etc. requer unsafe:

~~~rust
use std::alloc::{alloc, dealloc, Layout};

struct SimpleBox<T> {
    ptr: *mut T,
}

impl<T> SimpleBox<T> {
    fn new(value: T) -> Self {
        unsafe {
            let layout = Layout::new::<T>();
            let ptr = alloc(layout) as *mut T;
            ptr.write(value);
            SimpleBox { ptr }
        }
    }
}

impl<T> Drop for SimpleBox<T> {
    fn drop(&mut self) {
        unsafe {
            self.ptr.drop_in_place();
            let layout = Layout::new::<T>();
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}
~~~

**Nota**: Você provavelmente NUNCA precisará fazer isso. Use `Box` da biblioteca padrão!

---

## 🛡️ ABSTRAÇÕES SEGURAS SOBRE UNSAFE

### Princípio Fundamental

> **"Unsafe code should have a safe interface"**

O padrão é:
1. Código unsafe **interno** (privado)
2. API **safe** externa (pública)
3. Invariantes **documentados**
4. Testes **extensivos**

---

## 📊 DIAGRAMA 6: Padrão - Unsafe Interno, API Segura Externa

~~~mermaid
graph LR
    subgraph "Módulo Público"
        A[API Pública<br/>100% Safe] --> B[Validação<br/>de Entrada]
        B --> C[Lógica Safe]
        C --> D{Precisa de<br/>unsafe?}
        D -->|Não| E[Retorno Safe]
        D -->|Sim| F[Unsafe Encapsulado<br/>Privado]
        F --> G[Garantias<br/>Mantidas]
        G --> E
    end
    
    H[Usuário do Módulo] --> A
    H -.Nunca vê unsafe.-> F
    
    style A fill:#c8e6c9
    style F fill:#ffcdd2
    style G fill:#fff9c4
~~~

---

## 💡 DEMONSTRAÇÃO 1: Wrapper Seguro para Slice

~~~rust
/// Wrapper seguro para acesso não verificado a slice
/// 
/// # Invariantes
/// - `index` sempre < `slice.len()`
pub struct SafeSliceAccess<'a, T> {
    slice: &'a [T],
    index: usize,
}

impl<'a, T> SafeSliceAccess<'a, T> {
    /// Cria novo accessor (valida bounds)
    pub fn new(slice: &'a [T], index: usize) -> Option<Self> {
        if index < slice.len() {
            Some(SafeSliceAccess { slice, index })
        } else {
            None
        }
    }
    
    /// Acessa elemento (unsafe interno, mas garantido seguro)
    pub fn get(&self) -> &T {
        // SAFETY: index foi validado em `new()`
        unsafe {
            self.slice.get_unchecked(self.index)
        }
    }
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    // ✅ API completamente safe
    if let Some(accessor) = SafeSliceAccess::new(&data, 2) {
        println!("Valor: {}", accessor.get());
    }
    
    // ✅ Bounds checking na criação
    if SafeSliceAccess::new(&data, 10).is_none() {
        println!("Índice inválido rejeitado!");
    }
}
~~~

**Lições**:
- ✅ Unsafe encapsulado em método privado
- ✅ Validação em API pública
- ✅ Invariantes documentados (comentário SAFETY)
- ✅ Impossível usar incorretamente

---

## 💡 DEMONSTRAÇÃO 2: FFI com Wrapper Seguro

~~~rust
use std::ffi::CString;
use std::os::raw::c_char;

// Função C (simulada)
extern "C" {
    fn c_strlen(s: *const c_char) -> usize;
}

/// Wrapper seguro para c_strlen
pub fn safe_strlen(s: &str) -> usize {
    // Converte para CString (adiciona null terminator)
    let c_string = CString::new(s).expect("String contém null byte");
    
    // SAFETY: c_string garante null terminator
    unsafe {
        c_strlen(c_string.as_ptr())
    }
}

fn main() {
    let text = "Hello, Rust!";
    let len = safe_strlen(text);
    println!("Comprimento (via C): {}", len);
}
~~~

**Lições**:
- ✅ Conversão segura para C string
- ✅ API pública não expõe unsafe
- ✅ Validação de entrada (null bytes)

---

## 🎯 PRÁTICA GUIADA: Wrapper Seguro para Array Não-Inicializado

### 📝 Contexto do Problema

Você precisa criar um array grande e inicializá-lo elemento por elemento. Inicializar com valores padrão primeiro seria desperdício de performance.

### ⚠️ Aviso Inicial

**Alternativas safe preferíveis**:
- `Vec::with_capacity()` + `push()`
- `array::from_fn()`
- Iteradores

**Este exercício é educacional**. Em código real, prefira alternativas safe!

---

### 🎯 Objetivo

Criar `SafeUninitArray<T, N>` que:
1. Aloca array sem inicializar (unsafe)
2. Permite inicializar elemento por elemento
3. Converte para array normal quando completo
4. Garante que não há acesso a dados não-inicializados

---

### 📋 Passo 1: Estrutura Básica

~~~rust
use std::mem::MaybeUninit;

/// Array seguro não-inicializado
/// 
/// # Invariantes
/// - Elementos 0..initialized_count estão inicializados
/// - Elementos initialized_count..N não estão inicializados
pub struct SafeUninitArray<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    initialized_count: usize,
}
~~~

**Explicação**:
- `MaybeUninit<T>`: tipo que pode estar inicializado ou não
- `initialized_count`: rastreia quantos elementos foram inicializados
- Invariante documentado claramente

---

### 📋 Passo 2: Construtor

~~~rust
impl<T, const N: usize> SafeUninitArray<T, N> {
    /// Cria novo array não-inicializado
    pub fn new() -> Self {
        SafeUninitArray {
            // SAFETY: MaybeUninit não requer inicialização
            data: unsafe { MaybeUninit::uninit().assume_init() },
            initialized_count: 0,
        }
    }
}
~~~

**Explicação**:
- `MaybeUninit::uninit()`: cria valor não-inicializado
- `assume_init()`: assume que está inicializado (seguro para array de MaybeUninit)

---

### 📋 Passo 3: Inicializar Elemento

~~~rust
impl<T, const N: usize> SafeUninitArray<T, N> {
    /// Inicializa próximo elemento
    /// 
    /// # Panics
    /// Panics se array já está completo
    pub fn push(&mut self, value: T) {
        assert!(
            self.initialized_count < N,
            "Array já está completo"
        );
        
        // SAFETY: initialized_count < N foi verificado
        unsafe {
            self.data[self.initialized_count].write(value);
        }
        
        self.initialized_count += 1;
    }
    
    /// Retorna quantos elementos foram inicializados
    pub fn len(&self) -> usize {
        self.initialized_count
    }
    
    /// Verifica se array está completo
    pub fn is_full(&self) -> bool {
        self.initialized_count == N
    }
}
~~~

**Explicação**:
- Validação antes de unsafe
- `write()`: escreve valor sem dropar conteúdo anterior
- Atualiza contador após sucesso

---

### 📋 Passo 4: Conversão para Array Normal

~~~rust
impl<T, const N: usize> SafeUninitArray<T, N> {
    /// Converte para array normal (consome self)
    /// 
    /// # Panics
    /// Panics se array não está completo
    pub fn into_array(self) -> [T; N] {
        assert!(
            self.is_full(),
            "Array não está completo: {}/{} elementos",
            self.initialized_count,
            N
        );
        
        // SAFETY: verificamos que todos os elementos estão inicializados
        unsafe {
            // Lê array como inicializado
            let result = std::ptr::read(&self.data as *const _ as *const [T; N]);
            
            // Previne Drop de self (já movemos os dados)
            std::mem::forget(self);
            
            result
        }
    }
}
~~~

**Explicação**:
- Verifica que está completo antes de converter
- `ptr::read()`: move dados sem dropar
- `mem::forget()`: previne double-drop

---

### 📋 Passo 5: Implementar Drop Seguro

~~~rust
impl<T, const N: usize> Drop for SafeUninitArray<T, N> {
    fn drop(&mut self) {
        // SAFETY: dropa apenas elementos inicializados
        unsafe {
            for i in 0..self.initialized_count {
                self.data[i].assume_init_drop();
            }
        }
    }
}
~~~

**Explicação**:
- Dropa **apenas** elementos inicializados
- `assume_init_drop()`: assume inicializado e dropa
- Previne memory leak e double-drop

---

### 📋 Código Completo

~~~rust
use std::mem::MaybeUninit;

/// Array seguro não-inicializado
/// 
/// # Invariantes
/// - Elementos 0..initialized_count estão inicializados
/// - Elementos initialized_count..N não estão inicializados
/// 
/// # Exemplo
/// ~~~
/// let mut arr = SafeUninitArray::<i32, 3>::new();
/// arr.push(10);
/// arr.push(20);
/// arr.push(30);
/// let complete = arr.into_array();
/// assert_eq!(complete, [10, 20, 30]);
/// ~~~
pub struct SafeUninitArray<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    initialized_count: usize,
}

impl<T, const N: usize> SafeUninitArray<T, N> {
    /// Cria novo array não-inicializado
    pub fn new() -> Self {
        SafeUninitArray {
            // SAFETY: MaybeUninit não requer inicialização
            data: unsafe { MaybeUninit::uninit().assume_init() },
            initialized_count: 0,
        }
    }
    
    /// Inicializa próximo elemento
    /// 
    /// # Panics
    /// Panics se array já está completo
    pub fn push(&mut self, value: T) {
        assert!(
            self.initialized_count < N,
            "Array já está completo"
        );
        
        // SAFETY: initialized_count < N foi verificado
        unsafe {
            self.data[self.initialized_count].write(value);
        }
        
        self.initialized_count += 1;
    }
    
    /// Retorna quantos elementos foram inicializados
    pub fn len(&self) -> usize {
        self.initialized_count
    }
    
    /// Verifica se array está completo
    pub fn is_full(&self) -> bool {
        self.initialized_count == N
    }
    
    /// Converte para array normal (consome self)
    /// 
    /// # Panics
    /// Panics se array não está completo
    pub fn into_array(self) -> [T; N] {
        assert!(
            self.is_full(),
            "Array não está completo: {}/{} elementos",
            self.initialized_count,
            N
        );
        
        // SAFETY: verificamos que todos os elementos estão inicializados
        unsafe {
            // Lê array como inicializado
            let result = std::ptr::read(&self.data as *const _ as *const [T; N]);
            
            // Previne Drop de self (já movemos os dados)
            std::mem::forget(self);
            
            result
        }
    }
}

impl<T, const N: usize> Drop for SafeUninitArray<T, N> {
    fn drop(&mut self) {
        // SAFETY: dropa apenas elementos inicializados
        unsafe {
            for i in 0..self.initialized_count {
                self.data[i].assume_init_drop();
            }
        }
    }
}

// ✅ Testes extensivos são OBRIGATÓRIOS para código unsafe
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_usage() {
        let mut arr = SafeUninitArray::<i32, 3>::new();
        arr.push(10);
        arr.push(20);
        arr.push(30);
        let complete = arr.into_array();
        assert_eq!(complete, [10, 20, 30]);
    }
    
    #[test]
    #[should_panic(expected = "Array já está completo")]
    fn test_push_when_full() {
        let mut arr = SafeUninitArray::<i32, 2>::new();
        arr.push(1);
        arr.push(2);
        arr.push(3); // Deve paniquear
    }
    
    #[test]
    #[should_panic(expected = "Array não está completo")]
    fn test_into_array_when_incomplete() {
        let mut arr = SafeUninitArray::<i32, 3>::new();
        arr.push(1);
        let _ = arr.into_array(); // Deve paniquear
    }
    
    #[test]
    fn test_drop_partial() {
        // Testa que Drop funciona com array parcialmente inicializado
        let mut arr = SafeUninitArray::<String, 3>::new();
        arr.push(String::from("hello"));
        arr.push(String::from("world"));
        // arr é dropado aqui - não deve vazar memória
    }
}

fn main() {
    println!("=== Exemplo: SafeUninitArray ===\n");
    
    // ✅ Uso normal
    let mut arr = SafeUninitArray::<i32, 5>::new();
    
    for i in 0..5 {
        arr.push(i * 10);
        println!("Inicializados: {}/{}", arr.len(), 5);
    }
    
    let complete = arr.into_array();
    println!("\nArray completo: {:?}", complete);
    
    // ✅ Com tipos complexos
    let mut strings = SafeUninitArray::<String, 3>::new();
    strings.push(String::from("Rust"));
    strings.push(String::from("é"));
    strings.push(String::from("seguro!"));
    
    let complete_strings = strings.into_array();
    println!("\nStrings: {:?}", complete_strings);
}
~~~

---

### 🎓 Lições do Exercício

✅ **O que fizemos bem**:
1. Unsafe **encapsulado** em métodos privados
2. API pública **100% safe**
3. Invariantes **documentados** (comentários SAFETY)
4. Validação **antes** de unsafe
5. Drop **correto** (apenas elementos inicializados)
6. Testes **extensivos**

⚠️ **Avisos importantes**:
1. Este código é **educacional**
2. Em produção, use `Vec::with_capacity()` ou `array::from_fn()`
3. Unsafe deve ser **última opção**
4. Sempre documente **por quê** unsafe é necessário

---

## 🔄 FEEDBACK E AVALIAÇÃO

### ✅ Checklist de Entendimento

Marque cada item que você compreende:

- [ ] Entendo como Rust organiza structs na memória
- [ ] Sei o que é alinhamento e padding
- [ ] Conheço os atributos `#[repr(...)]`
- [ ] Entendo os 5 superpoderes de unsafe
- [ ] Sei a diferença entre referências e raw pointers
- [ ] Reconheço quando unsafe é **realmente** necessário
- [ ] Sei criar abstrações seguras sobre unsafe
- [ ] Entendo a importância de documentar invariantes
- [ ] Sei que unsafe deve ser **último recurso**

---

### 🧠 Quiz: Quando Unsafe É Justificado?

Para cada cenário, decida: **Safe** ou **Unsafe necessário**?

1. **Somar elementos de um Vec**
   - [ ] Safe (`.iter().sum()`)
   - [ ] Unsafe necessário

2. **Chamar função da biblioteca C**
   - [ ] Safe
   - [ ] Unsafe necessário (FFI sempre requer unsafe)

3. **Implementar estrutura de dados performática**
   - [ ] Safe (tente primeiro!)
   - [ ] Unsafe necessário (raramente)

4. **Acessar elemento de slice por índice**
   - [ ] Safe (`slice[i]` ou `slice.get(i)`)
   - [ ] Unsafe necessário

5. **Implementar alocador customizado**
   - [ ] Safe
   - [ ] Unsafe necessário (manipulação direta de memória)

**Respostas**: 1-Safe, 2-Unsafe, 3-Safe (tente primeiro), 4-Safe, 5-Unsafe

---

### 🎯 Auto-Avaliação de Prudência

Responda honestamente:

1. **Minha primeira reação ao aprender unsafe foi**:
   - [ ] "Vou usar em todo lugar!" ⚠️ CUIDADO
   - [ ] "Interessante, mas vou evitar" ✅ CORRETO
   - [ ] "Preciso entender mais antes de usar" ✅ EXCELENTE

2. **Quando vejo código unsafe, eu**:
   - [ ] Copio sem entender ❌ PERIGOSO
   - [ ] Analiso os invariantes cuidadosamente ✅ BOM
   - [ ] Procuro alternativa safe primeiro ✅ ÓTIMO

3. **Se meu código não compila, eu**:
   - [ ] Adiciono unsafe para "consertar" ❌ NUNCA FAÇA ISSO
   - [ ] Entendo o erro e corrijo em safe ✅ CORRETO
   - [ ] Peço ajuda se necessário ✅ SÁBIO

---

## 🚀 TRANSFERÊNCIA E APLICAÇÃO

### ⚠️ Diretrizes para Uso Responsável de Unsafe

~~~
┌─────────────────────────────────────────────────┐
│  ANTES DE USAR UNSAFE, PERGUNTE:                │
│                                                 │
│  1. ❓ Existe solução safe?                     │
│  2. ❓ Existe crate que resolve?                │
│  3. ❓ É REALMENTE necessário?                  │
│  4. ❓ Entendo profundamente os riscos?         │
│  5. ❓ Documentei todos os invariantes?         │
│  6. ❓ Criei abstração segura?                  │
│  7. ❓ Escrevi testes extensivos?               │
│  8. ❓ Pedi code review?                        │
│                                                 │
│  Se respondeu NÃO a qualquer pergunta:          │
│  ➡️  NÃO USE UNSAFE                             │
└─────────────────────────────────────────────────┘
~~~

---

### 📚 Recursos Adicionais

**Livros**:
- **The Rustonomicon**: Guia oficial sobre unsafe Rust
- **Rust for Rustaceans**: Capítulo sobre unsafe

**Artigos**:
- "How Unsafe is Unsafe" (blog oficial Rust)
- "Writing Safe Unsafe Code" (Rust RFC)

**Ferramentas**:
- **Miri**: Detector de undefined behavior
- **Valgrind**: Detector de memory leaks
- **AddressSanitizer**: Detector de memory errors

---

### 🎯 Preparação para Projeto Integrador (Dia 28)

Amanhã você criará um **Projeto Integrador** usando tudo que aprendeu na Fase 2!

**Dicas**:
- ✅ Use **100% safe Rust** (a menos que FFI seja requisito)
- ✅ Foque em design limpo e idiomático
- ✅ Aplique traits, generics, lifetimes
- ✅ Escreva testes
- ✅ Documente bem

**Unsafe no projeto**:
- ❌ NÃO é necessário
- ❌ NÃO vai impressionar
- ✅ Safe Rust bem escrito é muito mais valioso

---

## 📊 DIAGRAMA 7: Comparação Final - Safe vs Unsafe

~~~mermaid
graph TB
    subgraph "Safe Rust - 99% do seu código"
        A[Compilador Verifica] --> B[Sem data races]
        B --> C[Sem undefined behavior]
        C --> D[Sem memory leaks]
        D --> E[Produtivo e Seguro]
    end
    
    subgraph "Unsafe Rust - 1% do seu código"
        F[Programador Verifica] --> G[Responsabilidade total]
        G --> H[Bugs podem ser graves]
        H --> I[Requer expertise]
        I --> J[Usar com cautela]
    end
    
    K[Seu Código] --> L{Precisa de unsafe?}
    L -->|99% NÃO| A
    L -->|1% SIM| M[Esgotou alternativas?]
    M -->|NÃO| A
    M -->|SIM| F
    
    style A fill:#c8e6c9
    style E fill:#c8e6c9
    style F fill:#ffcdd2
    style J fill:#ffcdd2
~~~

---

## 🎓 RESUMO EXECUTIVO

### 📝 Principais Conceitos

1. **Memory Layout**:
   - Structs são organizadas com alinhamento e padding
   - `#[repr(C)]` para compatibilidade com C
   - `#[repr(packed)]` remove padding (cuidado!)
   - `#[repr(align(N))]` força alinhamento

2. **Unsafe Rust**:
   - 5 superpoderes (raw pointers, unsafe functions, static mut, unsafe traits, unions)
   - Escape hatch para casos específicos
   - **NÃO** é mais rápido automaticamente
   - **NÃO** é necessário em 99% dos casos

3. **Raw Pointers**:
   - `*const T` (imutável) e `*mut T` (mutável)
   - Criar é safe, derreferenciar é unsafe
   - Sem verificações do compilador

4. **Abstrações Seguras**:
   - Unsafe interno, API safe externa
   - Documentar invariantes (comentários SAFETY)
   - Validar antes de unsafe
   - Testes extensivos

---

### ⚠️ MENSAGEM FINAL

~~~
╔═══════════════════════════════════════════════╗
║                                               ║
║  UNSAFE NÃO É O OBJETIVO                      ║
║                                               ║
║  O objetivo é escrever código:                ║
║  ✅ Seguro                                    ║
║  ✅ Correto                                   ║
║  ✅ Manutenível                               ║
║  ✅ Idiomático                                ║
║                                               ║
║  Safe Rust permite tudo isso.                 ║
║  Unsafe é ferramenta para casos raros.        ║
║                                               ║
║  Use com sabedoria, responsabilidade          ║
║  e apenas quando REALMENTE necessário.        ║
║                                               ║
╚═══════════════════════════════════════════════╝
~~~

---

### 🎯 Próximos Passos

1. **Revise** os conceitos de memory layout
2. **Entenda** quando unsafe é necessário (raramente!)
3. **Pratique** criar abstrações seguras
4. **Prepare-se** para o Projeto Integrador (Dia 28)
5. **Lembre-se**: Safe Rust é poderoso o suficiente!

---

## 📖 Glossário de Termos

| Termo | Definição |
|-------|-----------|
| **Memory Layout** | Como dados são organizados na memória |
| **Alinhamento** | Requisito de endereço para tipos |
| **Padding** | Bytes extras para satisfazer alinhamento |
| **Unsafe** | Código que desabilita algumas verificações |
| **Raw Pointer** | Ponteiro sem verificações do compilador |
| **FFI** | Foreign Function Interface (chamar C/C++) |
| **Invariante** | Condição que deve sempre ser verdadeira |
| **Abstração Segura** | API safe sobre código unsafe interno |
| **Undefined Behavior** | Comportamento não especificado (bug grave) |
| **MaybeUninit** | Tipo que pode estar não-inicializado |

---

## 🎉 Conclusão

Parabéns por completar o Dia 27! Você agora entende:

✅ Como Rust organiza dados na memória  
✅ O que é unsafe e seus 5 superpoderes  
✅ Quando unsafe é realmente necessário (raramente!)  
✅ Como criar abstrações seguras sobre unsafe  
✅ A importância de usar safe Rust sempre que possível  

**Lembre-se**: Unsafe não é um objetivo, é uma ferramenta rara para casos específicos. O verdadeiro poder de Rust está em **safe Rust** - código seguro, rápido e expressivo sem sacrificar garantias.

Amanhã: **Projeto Integrador da Fase 2** - onde você aplicará tudo que aprendeu em um projeto real!

---

## 🔗 Referências

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Guia oficial sobre unsafe
- [Rust Reference - Type Layout](https://doc.rust-lang.org/reference/type-layout.html)
- [Rust Reference - Unsafe](https://doc.rust-lang.org/reference/unsafe-blocks.html)
- [std::mem](https://doc.rust-lang.org/std/mem/) - Funções de manipulação de memória
- [std::ptr](https://doc.rust-lang.org/std/ptr/) - Funções de ponteiros

---

**Rust é sobre segurança. Unsafe é sobre responsabilidade. Use com sabedoria! 🦀**