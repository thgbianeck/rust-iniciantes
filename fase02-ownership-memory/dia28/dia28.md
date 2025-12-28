# 🎉 DIA 28: PROJETO INTEGRADOR - BIBLIOTECA DE ESTRUTURAS DE DADOS

## 🏆 CULMINÂNCIA DA FASE 2: OWNERSHIP & SMART POINTERS

---

## 🎭 MOTIVAÇÃO E CONTEXTO

### 🌟 Sua Jornada Até Aqui

Parabéns por chegar ao **Dia 28**! Você percorreu uma jornada incrível:

- **Dias 15-21**: Dominou ownership, borrowing e lifetimes
- **Dias 22-25**: Conquistou smart pointers (Box, Rc, Arc, RefCell)
- **Dias 26-27**: Praticou patterns avançados e integração

**Hoje é o dia de integrar tudo isso em um projeto real e profissional!**

### 🎯 A Analogia: Construindo Suas Próprias Ferramentas

Imagine um carpinteiro que aprendeu a usar martelo, serrote e plaina. Agora, ele vai **construir sua própria caixa de ferramentas personalizada**. Você aprendeu os conceitos fundamentais de Rust - agora vai criar estruturas de dados reutilizáveis que outros desenvolvedores (incluindo você mesmo) poderão usar em projetos futuros.

### 💡 Por Que Este Projeto É Importante?

1. **Integração Real**: Você verá como ownership, borrowing e smart pointers trabalham juntos
2. **Portfolio**: Código profissional para mostrar seu domínio de Rust
3. **Fundação Sólida**: Base para a Fase 3 (Traits e Genéricos)
4. **Confiança**: Prova concreta de que você domina os conceitos mais desafiadores de Rust

---

## 📋 OBJETIVOS DE APRENDIZAGEM

Ao completar este projeto, você será capaz de:

✅ **Integrar** ownership, borrowing e lifetimes em código real  
✅ **Aplicar** smart pointers apropriadamente (Box, Rc, RefCell)  
✅ **Criar** APIs públicas seguras e ergonômicas  
✅ **Implementar** estruturas de dados clássicas em Rust  
✅ **Escrever** testes extensivos e documentação profissional  
✅ **Demonstrar** domínio completo da Fase 2  

---

## 📚 ESPECIFICAÇÃO DO PROJETO

### 🎯 Objetivo Geral

Criar uma **biblioteca Rust** com 3 estruturas de dados, demonstrando domínio de:
- Ownership e move semantics
- Borrowing (imutável e mutável)
- Lifetimes
- Smart pointers (Box, Rc, RefCell)
- API design
- Testing e documentação

### 📦 Escopo: Escolha 3 de 5 Estruturas

| Estrutura | Dificuldade | Smart Pointer | Conceitos Principais |
|-----------|-------------|---------------|---------------------|
| **1. Stack<T>** | ⭐⭐ Fácil | Vec (ownership) | LIFO, push/pop, borrowing |
| **2. Queue<T>** | ⭐⭐ Fácil | VecDeque | FIFO, enqueue/dequeue |
| **3. LinkedList<T>** | ⭐⭐⭐ Médio | Box<Node> | Recursão, ownership chain |
| **4. BinarySearchTree<T>** | ⭐⭐⭐⭐ Difícil | Box<Node> | Árvore, recursão, ordenação |
| **5. Graph<T>** | ⭐⭐⭐⭐⭐ Avançado | Rc/RefCell | Ciclos, shared ownership |

**💡 Recomendação para Iniciantes:**
- **Opção 1**: Stack + Queue + LinkedList (progressão natural)
- **Opção 2**: Stack + Queue + BinarySearchTree (desafio maior)

---

## 🏗️ ESTRUTURA DO PROJETO

~~~
data_structures_lib/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs              # Módulo principal
│   ├── stack.rs            # Implementação Stack<T>
│   ├── queue.rs            # Implementação Queue<T>
│   └── linked_list.rs      # Implementação LinkedList<T>
├── tests/
│   └── integration_tests.rs
└── examples/
    └── usage.rs
~~~

---

## 🎯 REQUISITOS TÉCNICOS

### ✅ Funcionalidades Obrigatórias

Para cada estrutura de dados:

1. **Métodos CRUD Completos**
   - Inserção (push, enqueue, insert)
   - Remoção (pop, dequeue, remove)
   - Consulta (peek, front, search)
   - Utilitários (is_empty, len, clear)

2. **Ownership Correto**
   - Move semantics quando apropriado
   - Borrowing imutável para leitura
   - Borrowing mutável para modificação

3. **Smart Pointers Apropriados**
   - Vec/VecDeque para coleções simples
   - Box para estruturas recursivas
   - Rc/RefCell para compartilhamento (Graph)

4. **Testes Unitários**
   - Cobertura > 80%
   - Casos de borda
   - Testes de ownership

5. **Documentação**
   - Doc comments (///)
   - Exemplos de uso
   - Complexidade temporal

### 🎁 Funcionalidades Bonus (Opcional)

- Implementar `IntoIterator` e `Iterator`
- Implementar `Display` e `Debug`
- Benchmarks de performance
- Métodos funcionais (map, filter, fold)

---

## 🗺️ GUIA DE IMPLEMENTAÇÃO (ETAPAS)

### ETAPA 1: Planejamento e Setup (20-30 min)

#### 1.1 Criar Projeto

~~~bash
cargo new data_structures_lib --lib
cd data_structures_lib
~~~

#### 1.2 Configurar Cargo.toml

~~~toml
[package]
name = "data_structures_lib"
version = "0.1.0"
edition = "2021"
authors = ["Seu Nome <seu@email.com>"]
description = "Biblioteca de estruturas de dados em Rust"

[dependencies]

[dev-dependencies]
# Para testes mais avançados (opcional)
~~~

#### 1.3 Escolher Suas 3 Estruturas

**Minha escolha:** (anote aqui)
- [ ] Estrutura 1: _______________
- [ ] Estrutura 2: _______________
- [ ] Estrutura 3: _______________

#### 1.4 Planejar Interface Pública

Para cada estrutura, defina:
- Quais métodos públicos?
- Quais tipos genéricos?
- Quais traits implementar?

---

### ETAPA 2: Implementar Stack<T> (40-60 min)

#### 📐 Diagrama de Memória: Stack

~~~
Stack<i32>
┌─────────────────┐
│ items: Vec<i32> │──→ [10, 20, 30] ← topo
└─────────────────┘
                      ↑
                      pop() remove daqui
                      push() adiciona aqui
~~~

#### 📝 Especificação

**Arquivo:** `src/stack.rs`

**Struct:**
~~~rust {.line-numbers}
pub struct Stack<T> {
    items: Vec<T>,
}
~~~

**Métodos Obrigatórios:**
- `new() -> Self` - Cria stack vazia
- `push(&mut self, item: T)` - Adiciona no topo
- `pop(&mut self) -> Option<T>` - Remove do topo
- `peek(&self) -> Option<&T>` - Consulta topo (sem remover)
- `is_empty(&self) -> bool` - Verifica se vazia
- `len(&self) -> usize` - Retorna tamanho

#### 💻 Template Inicial

~~~rust {.line-numbers}
/// Pilha LIFO (Last In, First Out) genérica
/// 
/// # Exemplos
/// 
/// ~~~
/// use data_structures_lib::Stack;
/// 
/// let mut stack = Stack::new();
/// stack.push(1);
/// stack.push(2);
/// assert_eq!(stack.pop(), Some(2));
/// ~~~
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Cria uma nova stack vazia
    /// 
    /// # Complexidade
    /// O(1)
    pub fn new() -> Self {
        // TODO: implementar
        todo!()
    }

    /// Adiciona um item no topo da pilha
    /// 
    /// # Argumentos
    /// * `item` - O item a ser adicionado (ownership é transferido)
    /// 
    /// # Complexidade
    /// O(1) amortizado
    pub fn push(&mut self, item: T) {
        // TODO: implementar
        todo!()
    }

    /// Remove e retorna o item do topo
    /// 
    /// # Retorno
    /// `Some(item)` se a pilha não está vazia, `None` caso contrário
    /// 
    /// # Complexidade
    /// O(1)
    pub fn pop(&mut self) -> Option<T> {
        // TODO: implementar
        todo!()
    }

    /// Retorna uma referência ao item do topo sem removê-lo
    /// 
    /// # Complexidade
    /// O(1)
    pub fn peek(&self) -> Option<&T> {
        // TODO: implementar
        todo!()
    }

    /// Verifica se a pilha está vazia
    pub fn is_empty(&self) -> bool {
        // TODO: implementar
        todo!()
    }

    /// Retorna o número de itens na pilha
    pub fn len(&self) -> usize {
        // TODO: implementar
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_push_and_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        
        stack.push(10);
        assert_eq!(stack.peek(), Some(&10));
        assert_eq!(stack.len(), 1); // peek não remove
    }

    // TODO: adicionar mais testes
}
~~~

#### 🎯 Checklist de Implementação

- [ ] Implementar `new()`
- [ ] Implementar `push()`
- [ ] Implementar `pop()`
- [ ] Implementar `peek()`
- [ ] Implementar `is_empty()` e `len()`
- [ ] Todos os testes passando
- [ ] Documentação completa
- [ ] Testar com diferentes tipos (String, struct customizada)

#### 💡 Dicas de Implementação

**Dica 1: Ownership em push()**
~~~rust {.line-numbers}
pub fn push(&mut self, item: T) {
    self.items.push(item); // item é movido para dentro do Vec
}
~~~

**Dica 2: Borrowing em peek()**
~~~rust {.line-numbers}
pub fn peek(&self) -> Option<&T> {
    self.items.last() // retorna Option<&T>, não move
}
~~~

**Dica 3: Move em pop()**
~~~rust {.line-numbers}
pub fn pop(&mut self) -> Option<T> {
    self.items.pop() // remove e retorna ownership
}
~~~

---

### ETAPA 3: Implementar Queue<T> (40-60 min)

#### 📐 Diagrama de Memória: Queue

~~~
Queue<i32>
┌──────────────────────┐
│ items: VecDeque<i32> │──→ [10, 20, 30]
└──────────────────────┘     ↑          ↑
                             │          │
                          dequeue()  enqueue()
                          (front)    (back)
~~~

#### 📝 Especificação

**Arquivo:** `src/queue.rs`

**Struct:**
~~~rust {.line-numbers}
use std::collections::VecDeque;

pub struct Queue<T> {
    items: VecDeque<T>,
}
~~~

**Métodos Obrigatórios:**
- `new() -> Self` - Cria fila vazia
- `enqueue(&mut self, item: T)` - Adiciona no final
- `dequeue(&mut self) -> Option<T>` - Remove do início
- `front(&self) -> Option<&T>` - Consulta início
- `is_empty(&self) -> bool` - Verifica se vazia
- `len(&self) -> usize` - Retorna tamanho

#### 💻 Template Inicial

~~~rust {.line-numbers}
use std::collections::VecDeque;

/// Fila FIFO (First In, First Out) genérica
/// 
/// # Exemplos
/// 
/// ~~~
/// use data_structures_lib::Queue;
/// 
/// let mut queue = Queue::new();
/// queue.enqueue(1);
/// queue.enqueue(2);
/// assert_eq!(queue.dequeue(), Some(1));
/// ~~~
pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    /// Cria uma nova fila vazia
    pub fn new() -> Self {
        // TODO: implementar
        todo!()
    }

    /// Adiciona um item no final da fila
    /// 
    /// # Complexidade
    /// O(1) amortizado
    pub fn enqueue(&mut self, item: T) {
        // TODO: implementar
        todo!()
    }

    /// Remove e retorna o item do início da fila
    /// 
    /// # Complexidade
    /// O(1)
    pub fn dequeue(&mut self) -> Option<T> {
        // TODO: implementar
        todo!()
    }

    /// Retorna uma referência ao item do início sem removê-lo
    pub fn front(&self) -> Option<&T> {
        // TODO: implementar
        todo!()
    }

    /// Verifica se a fila está vazia
    pub fn is_empty(&self) -> bool {
        // TODO: implementar
        todo!()
    }

    /// Retorna o número de itens na fila
    pub fn len(&self) -> usize {
        // TODO: implementar
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_queue_is_empty() {
        let queue: Queue<i32> = Queue::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_enqueue_and_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        
        assert_eq!(queue.dequeue(), Some(1)); // FIFO!
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_front() {
        let mut queue = Queue::new();
        queue.enqueue(100);
        queue.enqueue(200);
        
        assert_eq!(queue.front(), Some(&100));
        assert_eq!(queue.len(), 2); // front não remove
    }

    // TODO: adicionar mais testes
}
~~~

#### 🎯 Checklist de Implementação

- [ ] Implementar `new()`
- [ ] Implementar `enqueue()`
- [ ] Implementar `dequeue()`
- [ ] Implementar `front()`
- [ ] Implementar `is_empty()` e `len()`
- [ ] Todos os testes passando
- [ ] Documentação completa
- [ ] Testar ordem FIFO corretamente

---

### ETAPA 4: Implementar LinkedList<T> (60-90 min)

#### 📐 Diagrama de Memória: LinkedList

~~~
LinkedList<i32>
┌──────────────────────┐
│ head: Option<Box<..>>│──→ Box<Node>
└──────────────────────┘      ┌─────────┐
                              │ data: 10│
                              │ next: ──┼──→ Box<Node>
                              └─────────┘      ┌─────────┐
                                               │ data: 20│
                                               │ next: ──┼──→ Box<Node>
                                               └─────────┘      ┌─────────┐
                                                                │ data: 30│
                                                                │ next: ∅ │
                                                                └─────────┘
~~~

#### 📝 Especificação

**Arquivo:** `src/linked_list.rs`

**Structs:**
~~~rust {.line-numbers}
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}
~~~

**Métodos Obrigatórios:**
- `new() -> Self` - Cria lista vazia
- `push_front(&mut self, data: T)` - Adiciona no início
- `push_back(&mut self, data: T)` - Adiciona no final
- `pop_front(&mut self) -> Option<T>` - Remove do início
- `is_empty(&self) -> bool` - Verifica se vazia
- `len(&self) -> usize` - Retorna tamanho

#### 💻 Template Inicial

~~~rust {.line-numbers}
/// Nó interno da lista ligada
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
        }
    }
}

/// Lista ligada simples (singly linked list)
/// 
/// # Exemplos
/// 
/// ~~~
/// use data_structures_lib::LinkedList;
/// 
/// let mut list = LinkedList::new();
/// list.push_front(1);
/// list.push_front(2);
/// assert_eq!(list.pop_front(), Some(2));
/// ~~~
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T> {
    /// Cria uma nova lista vazia
    pub fn new() -> Self {
        // TODO: implementar
        todo!()
    }

    /// Adiciona um elemento no início da lista
    /// 
    /// # Complexidade
    /// O(1)
    pub fn push_front(&mut self, data: T) {
        // TODO: implementar
        // Dica: criar novo nó, fazer next apontar para head atual,
        // depois atualizar head
        todo!()
    }

    /// Remove e retorna o elemento do início da lista
    /// 
    /// # Complexidade
    /// O(1)
    pub fn pop_front(&mut self) -> Option<T> {
        // TODO: implementar
        // Dica: usar take() para pegar ownership do head,
        // depois atualizar head para next
        todo!()
    }

    /// Adiciona um elemento no final da lista
    /// 
    /// # Complexidade
    /// O(n) - precisa percorrer até o final
    pub fn push_back(&mut self, data: T) {
        // TODO: implementar (mais desafiador!)
        // Dica: se head é None, é como push_front
        // Senão, precisa percorrer até o último nó
        todo!()
    }

    /// Verifica se a lista está vazia
    pub fn is_empty(&self) -> bool {
        // TODO: implementar
        todo!()
    }

    /// Retorna o número de elementos na lista
    pub fn len(&self) -> usize {
        // TODO: implementar
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list_is_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_push_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(3)); // último inserido
    }

    #[test]
    fn test_pop_front_until_empty() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_push_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        
        assert_eq!(list.pop_front(), Some(1)); // primeiro inserido
    }

    // TODO: adicionar mais testes
}
~~~

#### 💡 Dicas de Implementação

**Dica 1: push_front() - Ownership com Box**
~~~rust {.line-numbers}
pub fn push_front(&mut self, data: T) {
    let mut new_node = Box::new(Node::new(data));
    new_node.next = self.head.take(); // take() pega ownership do Option
    self.head = Some(new_node);
    self.size += 1;
}
~~~

**Dica 2: pop_front() - Pattern Matching**
~~~rust {.line-numbers}
pub fn pop_front(&mut self) -> Option<T> {
    self.head.take().map(|node| {
        self.head = node.next;
        self.size -= 1;
        node.data
    })
}
~~~

**Dica 3: push_back() - Borrowing Mutável Recursivo**
~~~rust {.line-numbers}
pub fn push_back(&mut self, data: T) {
    let new_node = Box::new(Node::new(data));
    
    match &mut self.head {
        None => {
            self.head = Some(new_node);
        }
        Some(head) => {
            // Percorrer até o último nó
            let mut current = head;
            while let Some(ref mut next_node) = current.next {
                current = next_node;
            }
            current.next = Some(new_node);
        }
    }
    self.size += 1;
}
~~~

#### 🎯 Checklist de Implementação

- [ ] Implementar `Node<T>`
- [ ] Implementar `new()`
- [ ] Implementar `push_front()`
- [ ] Implementar `pop_front()`
- [ ] Implementar `push_back()` (desafio!)
- [ ] Implementar `is_empty()` e `len()`
- [ ] Todos os testes passando
- [ ] Entender ownership de Box
- [ ] Documentação completa

---

### ETAPA 5: Integração no lib.rs (15-20 min)

#### 📝 Arquivo: `src/lib.rs`

~~~rust {.line-numbers}
//! # Data Structures Library
//! 
//! Biblioteca de estruturas de dados implementadas em Rust,
//! demonstrando ownership, borrowing, lifetimes e smart pointers.
//! 
//! ## Estruturas Disponíveis
//! 
//! - [`Stack`]: Pilha LIFO
//! - [`Queue`]: Fila FIFO
//! - [`LinkedList`]: Lista ligada simples
//! 
//! ## Exemplo de Uso
//! 
//! ~~~
//! use data_structures_lib::{Stack, Queue, LinkedList};
//! 
//! // Stack
//! let mut stack = Stack::new();
//! stack.push(1);
//! stack.push(2);
//! assert_eq!(stack.pop(), Some(2));
//! 
//! // Queue
//! let mut queue = Queue::new();
//! queue.enqueue(1);
//! queue.enqueue(2);
//! assert_eq!(queue.dequeue(), Some(1));
//! 
//! // LinkedList
//! let mut list = LinkedList::new();
//! list.push_front(1);
//! list.push_back(2);
//! ~~~

mod stack;
mod queue;
mod linked_list;

pub use stack::Stack;
pub use queue::Queue;
pub use linked_list::LinkedList;
~~~

---

### ETAPA 6: Testes de Integração (20-30 min)

#### 📝 Arquivo: `tests/integration_tests.rs`

~~~rust {.line-numbers}
use data_structures_lib::{Stack, Queue, LinkedList};

#[test]
fn test_stack_with_strings() {
    let mut stack = Stack::new();
    stack.push(String::from("primeiro"));
    stack.push(String::from("segundo"));
    stack.push(String::from("terceiro"));
    
    assert_eq!(stack.pop(), Some(String::from("terceiro")));
    assert_eq!(stack.len(), 2);
}

#[test]
fn test_queue_with_custom_struct() {
    #[derive(Debug, PartialEq)]
    struct Task {
        id: u32,
        name: String,
    }
    
    let mut queue = Queue::new();
    queue.enqueue(Task { id: 1, name: String::from("Task 1") });
    queue.enqueue(Task { id: 2, name: String::from("Task 2") });
    
    let first = queue.dequeue().unwrap();
    assert_eq!(first.id, 1);
}

#[test]
fn test_linked_list_ownership() {
    let mut list = LinkedList::new();
    
    // Testar que ownership é transferido corretamente
    let data = String::from("owned data");
    list.push_front(data);
    // data não pode mais ser usado aqui (foi movido)
    
    let retrieved = list.pop_front().unwrap();
    assert_eq!(retrieved, "owned data");
}

#[test]
fn test_all_structures_together() {
    // Simular um sistema de processamento de tarefas
    let mut pending = Queue::new();
    let mut processing = Stack::new();
    let mut completed = LinkedList::new();
    
    // Adicionar tarefas pendentes
    pending.enqueue(1);
    pending.enqueue(2);
    pending.enqueue(3);
    
    // Processar (mover para stack)
    while let Some(task) = pending.dequeue() {
        processing.push(task);
    }
    
    // Completar (mover para lista)
    while let Some(task) = processing.pop() {
        completed.push_back(task);
    }
    
    assert_eq!(completed.len(), 3);
}
~~~

---

### ETAPA 7: Exemplos de Uso (15-20 min)

#### 📝 Arquivo: `examples/usage.rs`

~~~rust {.line-numbers}
use data_structures_lib::{Stack, Queue, LinkedList};

fn main() {
    println!("=== Demonstração da Biblioteca de Estruturas de Dados ===\n");
    
    demo_stack();
    demo_queue();
    demo_linked_list();
}

fn demo_stack() {
    println!("📚 STACK (Pilha LIFO)");
    println!("----------------------");
    
    let mut stack = Stack::new();
    
    println!("Empilhando: 10, 20, 30");
    stack.push(10);
    stack.push(20);
    stack.push(30);
    
    println!("Topo da pilha: {:?}", stack.peek());
    println!("Tamanho: {}", stack.len());
    
    println!("\nDesempilhando:");
    while let Some(value) = stack.pop() {
        println!("  -> {}", value);
    }
    
    println!("Pilha vazia? {}\n", stack.is_empty());
}

fn demo_queue() {
    println!("🎫 QUEUE (Fila FIFO)");
    println!("--------------------");
    
    let mut queue = Queue::new();
    
    println!("Enfileirando: A, B, C");
    queue.enqueue("A");
    queue.enqueue("B");
    queue.enqueue("C");
    
    println!("Início da fila: {:?}", queue.front());
    println!("Tamanho: {}", queue.len());
    
    println!("\nDesenfileirando:");
    while let Some(value) = queue.dequeue() {
        println!("  -> {}", value);
    }
    
    println!("Fila vazia? {}\n", queue.is_empty());
}

fn demo_linked_list() {
    println!("🔗 LINKED LIST (Lista Ligada)");
    println!("------------------------------");
    
    let mut list = LinkedList::new();
    
    println!("Adicionando no início: 3, 2, 1");
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    
    println!("Adicionando no final: 4, 5");
    list.push_back(4);
    list.push_back(5);
    
    println!("Tamanho: {}", list.len());
    
    println!("\nRemovendo do início:");
    for _ in 0..3 {
        if let Some(value) = list.pop_front() {
            println!("  -> {}", value);
        }
    }
    
    println!("Tamanho restante: {}\n", list.len());
}
~~~

**Executar exemplo:**
~~~bash
cargo run --example usage
~~~

---

### ETAPA 8: README e Documentação (20-30 min)

#### 📝 Arquivo: `README.md`

~~~markdown
# 📚 Data Structures Library

Biblioteca de estruturas de dados implementadas em Rust, criada como projeto integrador da Fase 2 do aprendizado de Rust.

## 🎯 Objetivo

Demonstrar domínio de:
- ✅ Ownership e move semantics
- ✅ Borrowing (imutável e mutável)
- ✅ Lifetimes
- ✅ Smart pointers (Box, Rc, RefCell)
- ✅ API design segura e ergonômica
- ✅ Testing e documentação profissional

## 📦 Estruturas Implementadas

### 1. Stack<T> - Pilha LIFO

Pilha (Last In, First Out) genérica implementada com `Vec<T>`.

**Métodos:**
- `new()` - Cria pilha vazia
- `push(item)` - Adiciona no topo
- `pop()` - Remove do topo
- `peek()` - Consulta topo
- `is_empty()` - Verifica se vazia
- `len()` - Retorna tamanho

**Exemplo:**
~~~rust {.line-numbers}
use data_structures_lib::Stack;

let mut stack = Stack::new();
stack.push(1);
stack.push(2);
assert_eq!(stack.pop(), Some(2));
~~~

### 2. Queue<T> - Fila FIFO

Fila (First In, First Out) genérica implementada com `VecDeque<T>`.

**Métodos:**
- `new()` - Cria fila vazia
- `enqueue(item)` - Adiciona no final
- `dequeue()` - Remove do início
- `front()` - Consulta início
- `is_empty()` - Verifica se vazia
- `len()` - Retorna tamanho

**Exemplo:**
~~~rust {.line-numbers}
use data_structures_lib::Queue;

let mut queue = Queue::new();
queue.enqueue(1);
queue.enqueue(2);
assert_eq!(queue.dequeue(), Some(1));
~~~

### 3. LinkedList<T> - Lista Ligada

Lista ligada simples implementada com `Box<Node<T>>`.

**Métodos:**
- `new()` - Cria lista vazia
- `push_front(data)` - Adiciona no início
- `push_back(data)` - Adiciona no final
- `pop_front()` - Remove do início
- `is_empty()` - Verifica se vazia
- `len()` - Retorna tamanho

**Exemplo:**
~~~rust {.line-numbers}
use data_structures_lib::LinkedList;

let mut list = LinkedList::new();
list.push_front(1);
list.push_back(2);
assert_eq!(list.pop_front(), Some(1));
~~~

## 🚀 Como Usar

### Adicionar ao seu projeto

~~~toml
[dependencies]
data_structures_lib = { path = "../data_structures_lib" }
~~~

### Executar testes

~~~bash
cargo test
~~~

### Executar exemplo

~~~bash
cargo run --example usage
~~~

### Gerar documentação

~~~bash
cargo doc --open
~~~

## 📊 Complexidade Temporal

| Estrutura | Operação | Complexidade |
|-----------|----------|--------------|
| Stack | push | O(1) amortizado |
| Stack | pop | O(1) |
| Stack | peek | O(1) |
| Queue | enqueue | O(1) amortizado |
| Queue | dequeue | O(1) |
| Queue | front | O(1) |
| LinkedList | push_front | O(1) |
| LinkedList | pop_front | O(1) |
| LinkedList | push_back | O(n) |

## 🧪 Cobertura de Testes

- ✅ Testes unitários para cada estrutura
- ✅ Testes de integração
- ✅ Testes com tipos customizados
- ✅ Testes de ownership e borrowing
- ✅ Cobertura > 80%

## 🏆 Conceitos de Rust Demonstrados

### Ownership
- Transferência de ownership em `push()`, `enqueue()`
- Retorno de ownership em `pop()`, `dequeue()`

### Borrowing
- Borrowing imutável em `peek()`, `front()`
- Borrowing mutável em métodos de modificação

### Smart Pointers
- `Box<T>` para alocação heap em LinkedList
- `Option<T>` para valores opcionais
- `Vec<T>` e `VecDeque<T>` para coleções

### Generics
- Todas as estruturas são genéricas sobre `T`
- Funcionam com qualquer tipo que implemente traits necessários

## 📝 Licença

MIT

## 👤 Autor

[Seu Nome] - Projeto Integrador Fase 2 Rust
~~~

---

## 📐 DIAGRAMAS MERMAID

### Diagrama 1: Arquitetura da Biblioteca

~~~mermaid
graph TB
    A[data_structures_lib] --> B[Stack<T>]
    A --> C[Queue<T>]
    A --> D[LinkedList<T>]
    
    B --> B1[Vec<T>]
    C --> C1[VecDeque<T>]
    D --> D1[Box Node T]
    
    B1 --> E[Ownership]
    C1 --> E
    D1 --> E
    
    B --> F[Public API]
    C --> F
    D --> F
    
    F --> G[User Code]
    
    style A fill:#4CAF50
    style B fill:#2196F3
    style C fill:#2196F3
    style D fill:#2196F3
    style E fill:#FF9800
    style F fill:#9C27B0
~~~

### Diagrama 2: Diagrama de Classes UML

~~~mermaid
classDiagram
    class Stack~T~ {
        -Vec~T~ items
        +new() Stack~T~
        +push(item: T)
        +pop() Option~T~
        +peek() Option~ref T~
        +is_empty() bool
        +len() usize
    }
    
    class Queue~T~ {
        -VecDeque~T~ items
        +new() Queue~T~
        +enqueue(item: T)
        +dequeue() Option~T~
        +front() Option~ref T~
        +is_empty() bool
        +len() usize
    }
    
    class LinkedList~T~ {
        -Option~Box~Node~T~~~ head
        -usize size
        +new() LinkedList~T~
        +push_front(data: T)
        +push_back(data: T)
        +pop_front() Option~T~
        +is_empty() bool
        +len() usize
    }
    
    class Node~T~ {
        -T data
        -Option~Box~Node~T~~~ next
        +new(data: T) Node~T~
    }
    
    LinkedList~T~ --> Node~T~ : contains
~~~

### Diagrama 3: Fluxo de Ownership - Stack::push()

~~~mermaid
sequenceDiagram
    participant User
    participant Stack
    participant Vec
    
    User->>Stack: push(item: T)
    Note over User,Stack: Ownership de 'item' é transferido
    Stack->>Vec: vec.push(item)
    Note over Stack,Vec: Vec toma ownership
    Vec-->>Stack: ()
    Stack-->>User: ()
    Note over User: 'item' não pode mais ser usado
~~~

### Diagrama 4: Fluxo de Borrowing - Stack::peek()

~~~mermaid
sequenceDiagram
    participant User
    participant Stack
    participant Vec
    
    User->>Stack: peek() -> Option<&T>
    Note over User,Stack: Pede referência emprestada
    Stack->>Vec: vec.last() -> Option<&T>
    Note over Stack,Vec: Vec empresta referência
    Vec-->>Stack: Some(&item)
    Stack-->>User: Some(&item)
    Note over User: User pode ler, mas não modificar
    Note over User: 'item' ainda pertence ao Stack
~~~

### Diagrama 5: Memória - LinkedList com 3 Nós

~~~mermaid
graph LR
    A[LinkedList] -->|head: Some| B[Box Node 1]
    B -->|data: 10| B
    B -->|next: Some| C[Box Node 2]
    C -->|data: 20| C
    C -->|next: Some| D[Box Node 3]
    D -->|data: 30| D
    D -->|next: None| E[∅]
    
    style A fill:#4CAF50
    style B fill:#2196F3
    style C fill:#2196F3
    style D fill:#2196F3
    style E fill:#f44336
~~~

---

## ✅ CHECKLIST FINAL DE QUALIDADE

### Funcionalidades

- [ ] **Stack<T>**
  - [ ] Todos os métodos implementados
  - [ ] Testes passando (>80% cobertura)
  - [ ] Documentação completa
  
- [ ] **Queue<T>**
  - [ ] Todos os métodos implementados
  - [ ] Testes passando (>80% cobertura)
  - [ ] Documentação completa
  
- [ ] **LinkedList<T>**
  - [ ] Todos os métodos implementados
  - [ ] Testes passando (>80% cobertura)
  - [ ] Documentação completa

### Qualidade de Código

- [ ] `cargo build` sem warnings
- [ ] `cargo test` todos os testes passando
- [ ] `cargo clippy` sem sugestões
- [ ] `cargo fmt` código formatado
- [ ] `cargo doc` documentação gerada

### Conceitos de Rust

- [ ] Ownership usado corretamente
- [ ] Borrowing imutável e mutável apropriados
- [ ] Smart pointers aplicados corretamente
- [ ] Sem uso de `unsafe`
- [ ] Sem `clone()` desnecessário

### Documentação

- [ ] Doc comments em todas as structs públicas
- [ ] Doc comments em todos os métodos públicos
- [ ] Exemplos de uso funcionais
- [ ] README completo
- [ ] Complexidade temporal documentada

### Testes

- [ ] Testes unitários por estrutura
- [ ] Testes de integração
- [ ] Testes com tipos customizados
- [ ] Testes de casos de borda
- [ ] Cobertura > 80%

---

## 🎓 AUTO-AVALIAÇÃO

### Conceitos Dominados

Avalie seu domínio (1-5 estrelas):

**Ownership:**
- [ ] ⭐ Iniciante - Ainda confuso
- [ ] ⭐⭐ Básico - Entendo o conceito
- [ ] ⭐⭐⭐ Intermediário - Uso corretamente
- [ ] ⭐⭐⭐⭐ Avançado - Domino bem
- [ ] ⭐⭐⭐⭐⭐ Expert - Posso ensinar

**Borrowing:**
- [ ] ⭐ Iniciante
- [ ] ⭐⭐ Básico
- [ ] ⭐⭐⭐ Intermediário
- [ ] ⭐⭐⭐⭐ Avançado
- [ ] ⭐⭐⭐⭐⭐ Expert

**Smart Pointers:**
- [ ] ⭐ Iniciante
- [ ] ⭐⭐ Básico
- [ ] ⭐⭐⭐ Intermediário
- [ ] ⭐⭐⭐⭐ Avançado
- [ ] ⭐⭐⭐⭐⭐ Expert

**API Design:**
- [ ] ⭐ Iniciante
- [ ] ⭐⭐ Básico
- [ ] ⭐⭐⭐ Intermediário
- [ ] ⭐⭐⭐⭐ Avançado
- [ ] ⭐⭐⭐⭐⭐ Expert

### Reflexão

**O que aprendi neste projeto?**
_[Escreva aqui suas reflexões]_

**Quais foram os maiores desafios?**
_[Escreva aqui]_

**O que faria diferente?**
_[Escreva aqui]_

**Próximos passos:**
_[Escreva aqui]_

---

## 🎉 CELEBRAÇÃO DA FASE 2

### 🏆 Conquistas Desbloqueadas

Parabéns! Você completou a Fase 2 e desbloqueou:

#### 🥇 Ownership Master
**Descrição:** Dominou ownership e move semantics em Rust  
**Evidência:** Implementou estruturas de dados com transferência correta de ownership

#### 🔐 Borrow Checker Ally
**Descrição:** Trabalha em harmonia com o borrow checker  
**Evidência:** Usou borrowing imutável e mutável apropriadamente em toda a biblioteca

#### 📦 Smart Pointer Specialist
**Descrição:** Sabe quando e como usar Box, Rc, Arc e RefCell  
**Evidência:** Aplicou Box em LinkedList para estruturas recursivas

#### 🏗️ Data Structure Architect
**Descrição:** Projetou e implementou estruturas de dados profissionais  
**Evidência:** Biblioteca completa com 3 estruturas, testes e documentação

---

## 📊 RETROSPECTIVA DA FASE 2

### Jornada de 14 Dias

**Dias 15-17: Fundamentos de Ownership**
- ✅ Move semantics
- ✅ Copy vs Clone
- ✅ Stack vs Heap

**Dias 18-20: Borrowing e Referências**
- ✅ Referências imutáveis
- ✅ Referências mutáveis
- ✅ Regras do borrow checker

**Dias 21-22: Lifetimes**
- ✅ Anotações de lifetime
- ✅ Lifetime elision
- ✅ Structs com lifetimes

**Dias 23-25: Smart Pointers**
- ✅ Box<T> para heap allocation
- ✅ Rc<T> para shared ownership
- ✅ Arc<T> para concorrência
- ✅ RefCell<T> para interior mutability

**Dias 26-27: Patterns Avançados**
- ✅ Combinação de smart pointers
- ✅ Padrões de design em Rust
- ✅ Preparação para projeto final

**Dia 28: Projeto Integrador** ← VOCÊ ESTÁ AQUI! 🎯
- ✅ Integração de todos os conceitos
- ✅ Projeto profissional completo
- ✅ Portfolio piece

### Estatísticas da Sua Jornada

- **Dias de estudo:** 14
- **Conceitos dominados:** 15+
- **Linhas de código escritas:** ~500+ (neste projeto)
- **Testes criados:** 15+
- **Estruturas de dados implementadas:** 3
- **Nível de confiança:** 📈 ALTO!

---

## 🚀 PREPARAÇÃO PARA FASE 3

### O Que Vem a Seguir?

**Fase 3: Traits e Genéricos (Dias 29-42)**

Você está PRONTO para:
- ✅ Definir traits customizados
- ✅ Implementar traits da standard library
- ✅ Trait bounds e where clauses
- ✅ Associated types
- ✅ Genéricos avançados
- ✅ Polimorfismo em Rust

### Como Este Projeto Te Preparou

1. **Você já usou genéricos** em Stack<T>, Queue<T>, LinkedList<T>
2. **Você já trabalhou com traits** implicitamente (Debug, PartialEq)
3. **Você entende ownership** - essencial para trait objects
4. **Você sabe projetar APIs** - base para trait design

### Próximo Desafio

Na Fase 3, você vai:
- Adicionar traits customizados às suas estruturas
- Implementar `Iterator` para suas coleções
- Criar traits para comportamentos compartilhados
- Usar trait objects para polimorfismo

---

## 💬 MENSAGEM FINAL

### 🎊 Parabéns, Rustacean!

Você completou um dos marcos mais importantes da sua jornada em Rust. A Fase 2 é conhecida por ser **a mais desafiadora** para iniciantes, porque ownership e borrowing são conceitos únicos de Rust.

**Mas você conseguiu!** 🎉

Você não apenas aprendeu os conceitos - você os **aplicou** em um projeto real. Você criou uma biblioteca que:
- ✅ Compila sem erros
- ✅ Passa em todos os testes
- ✅ Tem documentação profissional
- ✅ Demonstra domínio de Rust

### 🌟 Você Agora É Capaz De:

1. **Entender** mensagens de erro do borrow checker
2. **Projetar** estruturas de dados seguras
3. **Escolher** entre ownership, borrowing e smart pointers
4. **Criar** APIs públicas ergonômicas
5. **Escrever** código Rust idiomático

### 📚 Continue Construindo

Este projeto é apenas o começo. Ideias para expandir:

1. **Adicionar mais estruturas:**
   - BinarySearchTree
   - Graph
   - HashMap customizado

2. **Implementar traits:**
   - Iterator e IntoIterator
   - Display e Debug
   - From e Into

3. **Adicionar funcionalidades:**
   - Métodos funcionais (map, filter, fold)
   - Serialização (serde)
   - Benchmarks de performance

4. **Publicar:**
   - Criar repositório no GitHub
   - Publicar no crates.io
   - Compartilhar com a comunidade

### 🎯 Lembre-se

> "O borrow checker não é seu inimigo - é seu mentor."

Cada erro que você resolveu te tornou um programador melhor. Cada conceito que você dominou te deu superpoderes que outras linguagens não oferecem.

**Você está pronto para a Fase 3!** 🚀

---

## 📞 RECURSOS ADICIONAIS

### Documentação Oficial
- [The Rust Book - Chapter 4: Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [The Rust Book - Chapter 15: Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Rust By Example - Box, Rc](https://doc.rust-lang.org/rust-by-example/std/box.html)

### Ferramentas
- `cargo doc --open` - Gerar e visualizar documentação
- `cargo test -- --nocapture` - Ver prints durante testes
- `cargo clippy` - Linter para código idiomático
- `cargo fmt` - Formatador automático

### Comunidade
- [Rust Users Forum](https://users.rust-lang.org/)
- [Rust Discord](https://discord.gg/rust-lang)
- [r/rust](https://reddit.com/r/rust)

---

## ✨ CÓDIGO DE REFERÊNCIA COMPLETO

### Stack<T> - Implementação Completa

~~~rust {.line-numbers}
/// Pilha LIFO (Last In, First Out) genérica
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
~~~

### Queue<T> - Implementação Completa

~~~rust {.line-numbers}
use std::collections::VecDeque;

/// Fila FIFO (First In, First Out) genérica
pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn front(&self) -> Option<&T> {
        self.items.front()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
~~~

### LinkedList<T> - Implementação Completa

~~~rust {.line-numbers}
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

/// Lista ligada simples
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let mut new_node = Box::new(Node::new(data));
        new_node.next = self.head.take();
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.data
        })
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));
        
        match &mut self.head {
            None => {
                self.head = Some(new_node);
            }
            Some(head) => {
                let mut current = head;
                while let Some(ref mut next_node) = current.next {
                    current = next_node;
                }
                current.next = Some(new_node);
            }
        }
        self.size += 1;
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.size
    }
}
~~~

---

## 🎊 VOCÊ CONSEGUIU!

**Agora é hora de:**

1. ✅ Executar `cargo test` e ver tudo verde
2. ✅ Executar `cargo run --example usage` e ver sua biblioteca em ação
3. ✅ Executar `cargo doc --open` e admirar sua documentação
4. ✅ Fazer commit no Git e celebrar!
5. ✅ Descansar e se preparar para a Fase 3

**Você é oficialmente um Ownership Master! 🏆**

---

**Próximo passo:** Fase 3 - Traits e Genéricos (Dia 29)

**Até lá, celebre sua conquista! 🎉🦀**