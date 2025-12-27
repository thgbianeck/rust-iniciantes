# 📦 Dia 20: Box - Seu Primeiro Smart Pointer em Rust

## 📋 Objetivos de Aprendizagem

Ao final desta lição, você será capaz de:

✅ **Entender** o que é Box e quando usá-lo  
✅ **Criar** estruturas de dados recursivas com Box  
✅ **Compreender** os fundamentos de smart pointers em Rust  

---

## 🎭 Ativação do Conhecimento Prévio

### Revisão Rápida: Ownership e Heap

Você já aprendeu que:
- **Stack**: memória rápida, tamanho fixo conhecido em tempo de compilação
- **Heap**: memória flexível, tamanho pode ser dinâmico
- **Ownership**: cada valor tem um único dono

### 🎁 Analogia Central: "A Caixa de Transporte"

Imagine que você precisa enviar um objeto valioso:

**Sem Box (direto na stack):**
- Como carregar um piano no bolso ❌
- Tamanho precisa ser conhecido
- Limitado ao espaço da stack

**Com Box (na heap):**
- Como uma caixa de transporte especial 📦
- Você carrega apenas o "endereço da caixa" (ponteiro)
- A caixa real (dados) fica no depósito (heap)
- **Ownership único**: só você tem a chave da caixa
- Quando você descarta a chave, a caixa é automaticamente destruída

### 📖 História: O Problema das Estruturas Recursivas

Imagine que você quer criar uma lista encadeada:

~~~rust {.line-numbers}
// Tentativa ingênua (NÃO COMPILA!)
struct Node {
    value: i32,
    next: Node,  // ❌ Tamanho infinito!
}
~~~

**O problema:** O compilador precisa saber o tamanho de `Node`. Mas `Node` contém outro `Node`, que contém outro `Node`... infinito! 🔄

**A solução:** Box! Ele tem tamanho fixo (apenas um ponteiro), mas aponta para dados na heap.

---

## 📚 Apresentação do Conteúdo

### O que é Box?

`Box<T>` é um **smart pointer** que:
- 📍 Aloca dados do tipo `T` na **heap**
- 🔑 Mantém **ownership único** dos dados
- 📏 Tem tamanho **fixo e conhecido** (tamanho de um ponteiro)
- 🗑️ **Libera automaticamente** a memória quando sai de escopo (RAII)

### Quando Usar Box?

| Situação | Por que Box? | Exemplo |
|----------|--------------|---------|
| **Tamanho desconhecido** | Tipo recursivo precisa de tamanho fixo | Linked List, Árvore |
| **Dados grandes** | Evitar cópias caras na stack | Struct com arrays grandes |
| **Trait objects** | Polimorfismo dinâmico | `Box<dyn Trait>` |
| **Ownership explícito na heap** | Controle fino de alocação | Cache, pools |

---

## 🎨 Diagramas Visuais

### Diagrama 1: Memória - T vs Box<T>

~~~mermaid
graph TD
    subgraph "Stack - Valor Direto"
        A[variável: i32<br/>valor: 42]
    end
    
    subgraph "Stack + Heap - Box"
        B[variável: Box&lt;i32&gt;<br/>ponteiro: 0x1234]
        C[Heap 0x1234<br/>valor: 42]
        B -->|aponta para| C
    end
    
    style A fill:#a8dadc
    style B fill:#457b9d
    style C fill:#f1faee
~~~

**Explicação:**
- **Esquerda:** Valor direto na stack (tamanho conhecido)
- **Direita:** Box na stack (ponteiro fixo) + dados na heap (tamanho flexível)

---

### Diagrama 2: Por que Tipos Recursivos Precisam de Box

~~~mermaid
graph LR
    subgraph "❌ Sem Box - Tamanho Infinito"
        A1[Node<br/>value: i32<br/>next: Node]
        A2[Node<br/>value: i32<br/>next: Node]
        A3[Node<br/>value: i32<br/>next: ...]
        A1 -.contém.- A2
        A2 -.contém.- A3
        A3 -.contém.- A1
    end
    
    subgraph "✅ Com Box - Tamanho Fixo"
        B1[Node<br/>value: i32<br/>next: Box]
        B2[Node na Heap<br/>value: i32<br/>next: Box]
        B3[Node na Heap<br/>value: i32<br/>next: None]
        B1 -->|ponteiro| B2
        B2 -->|ponteiro| B3
    end
    
    style A1 fill:#e63946
    style B1 fill:#2a9d8f
    style B2 fill:#2a9d8f
    style B3 fill:#2a9d8f
~~~

---

### Diagrama 3: Trait Deref - Box se Comporta como T

~~~mermaid
classDiagram
    class Deref {
        <<trait>>
        +deref(&self) &T
    }
    
    class Box~T~ {
        -ptr: *mut T
        +new(value: T) Box~T~
        +deref(&self) &T
    }
    
    class T {
        +metodo1()
        +metodo2()
    }
    
    Deref <|.. Box~T~ : implementa
    Box~T~ --> T : aponta para
    
    note for Box~T~ "Deref coercion permite usar\nBox&lt;T&gt; como se fosse &T"
~~~

**Deref Coercion em ação:**
~~~rust {.line-numbers}
let boxed = Box::new(String::from("Rust"));
// Você pode chamar métodos de String diretamente!
println!("{}", boxed.len()); // len() é de String, não de Box
~~~

---

### Diagrama 4: Fluxograma - Quando Usar Box?

~~~mermaid
flowchart TD
    Start([Preciso alocar dados?])
    Start --> Q1{Tipo recursivo?}
    
    Q1 -->|Sim| UseBox1[✅ Use Box]
    Q1 -->|Não| Q2{Dados muito grandes<br/>para stack?}
    
    Q2 -->|Sim| UseBox2[✅ Use Box]
    Q2 -->|Não| Q3{Precisa de trait object<br/>dinâmico?}
    
    Q3 -->|Sim| UseBox3[✅ Use Box&lt;dyn Trait&gt;]
    Q3 -->|Não| Q4{Múltiplos donos?}
    
    Q4 -->|Sim| UseRc[❌ Use Rc/Arc<br/>Dia 21]
    Q4 -->|Não| Q5{Referência temporária<br/>é suficiente?}
    
    Q5 -->|Sim| UseRef[❌ Use &T]
    Q5 -->|Não| UseBox4[✅ Use Box]
    
    style UseBox1 fill:#2a9d8f
    style UseBox2 fill:#2a9d8f
    style UseBox3 fill:#2a9d8f
    style UseBox4 fill:#2a9d8f
    style UseRc fill:#e76f51
    style UseRef fill:#e76f51
~~~

---

### Diagrama 5: Sequência - Drop Automático (RAII)

~~~mermaid
sequenceDiagram
    participant Código
    participant Stack
    participant Heap
    
    Código->>Stack: let b = Box::new(42)
    Stack->>Heap: Aloca memória
    Heap-->>Stack: Retorna ponteiro
    Stack-->>Código: b criado
    
    Note over Código,Heap: b está em uso...
    
    Código->>Stack: } // b sai de escopo
    Stack->>Stack: Drop trait chamado
    Stack->>Heap: Libera memória
    Heap-->>Stack: Memória liberada
    
    Note over Código,Heap: Sem memory leak! 🎉
~~~

---

### Diagrama 6: Comparação - Stack vs Heap com Box

~~~mermaid
graph TB
    subgraph "Stack (rápida, limitada)"
        S1[x: i32 = 10]
        S2[y: i32 = 20]
        S3[box_ptr: Box&lt;i32&gt;]
    end
    
    subgraph "Heap (flexível, mais lenta)"
        H1[42]
        H2[Grande Struct<br/>1000 campos...]
        H3[Node → Node → Node]
    end
    
    S3 -.->|ponteiro| H1
    
    style S1 fill:#457b9d
    style S2 fill:#457b9d
    style S3 fill:#457b9d
    style H1 fill:#f1faee
    style H2 fill:#f1faee
    style H3 fill:#f1faee
~~~

---

## 💡 Demonstração e Modelagem

### Problema: Tipo Recursivo Sem Box (ERRO!)

~~~rust {.line-numbers}
// ❌ ISSO NÃO COMPILA!
struct ListNode {
    value: i32,
    next: ListNode,  // Erro: tamanho infinito!
}

// Erro do compilador:
// error[E0072]: recursive type `ListNode` has infinite size
//  --> src/main.rs:2:1
//   |
// 2 | struct ListNode {
//   | ^^^^^^^^^^^^^^^ recursive type has infinite size
// 3 |     value: i32,
// 4 |     next: ListNode,
//   |           -------- recursive without indirection
~~~

---

### Solução: Com Box (FUNCIONA!)

~~~rust {.line-numbers}
// ✅ ISSO COMPILA!
struct ListNode {
    value: i32,
    next: Option<Box<ListNode>>,  // Tamanho fixo!
}

fn main() {
    // Criando uma lista: 1 -> 2 -> 3
    let list = ListNode {
        value: 1,
        next: Some(Box::new(ListNode {
            value: 2,
            next: Some(Box::new(ListNode {
                value: 3,
                next: None,  // Fim da lista
            })),
        })),
    };
    
    println!("Primeiro valor: {}", list.value);
}
~~~

**Por que funciona?**
- `Box<ListNode>` tem tamanho fixo (8 bytes em 64-bit)
- `Option<Box<ListNode>>` também tem tamanho fixo
- O compilador consegue calcular o tamanho total de `ListNode`

---

### Deref Coercion em Ação

~~~rust {.line-numbers}
fn main() {
    let boxed_num = Box::new(42);
    
    // Deref automático: Box<i32> -> &i32
    print_number(&boxed_num);  // Funciona!
    
    // Você também pode fazer manualmente:
    print_number(&*boxed_num);
}

fn print_number(n: &i32) {
    println!("Número: {}", n);
}
~~~

---

### Padrões Comuns com Box

#### 1. Dados Grandes na Heap

~~~rust {.line-numbers}
struct LargeData {
    buffer: [u8; 1_000_000],  // 1 MB!
}

fn main() {
    // ❌ Ruim: copia 1 MB na stack
    // let data = LargeData { buffer: [0; 1_000_000] };
    
    // ✅ Bom: apenas ponteiro na stack
    let data = Box::new(LargeData { buffer: [0; 1_000_000] });
    
    // Transferir ownership é barato (move apenas o ponteiro)
    let moved_data = data;
}
~~~

#### 2. Trait Objects

~~~rust {.line-numbers}
trait Animal {
    fn make_sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) { println!("Woof!"); }
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) { println!("Meow!"); }
}

fn main() {
    // Polimorfismo dinâmico
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];
    
    for animal in animals {
        animal.make_sound();
    }
}
~~~

---

## 🎯 Prática Guiada: Linked List Simples

### 📝 Contexto do Exercício

Vamos implementar uma **Singly Linked List** (lista encadeada simples), uma das estruturas de dados mais clássicas da computação. Esta é a aplicação perfeita para entender Box!

**O que você vai aprender:**
- Por que Box é **essencial** para estruturas recursivas
- Como manipular ownership com Box
- Padrões de código com `Option<Box<T>>`

---

### 🏗️ Estrutura da Lista

~~~mermaid
graph LR
    Head[LinkedList<br/>head: Option] -->|Some| N1[Box → Node<br/>value: 10<br/>next: Some]
    N1 -->|Some| N2[Box → Node<br/>value: 20<br/>next: Some]
    N2 -->|Some| N3[Box → Node<br/>value: 30<br/>next: None]
    
    style Head fill:#457b9d,color:#fff
    style N1 fill:#2a9d8f,color:#fff
    style N2 fill:#2a9d8f,color:#fff
    style N3 fill:#2a9d8f,color:#fff
~~~

---

### 📦 Código Completo com Comentários

~~~rust {.line-numbers}
// Estrutura do nó individual
// Box é NECESSÁRIO aqui para evitar tamanho infinito
struct Node {
    value: i32,
    next: Option<Box<Node>>,  // None = fim da lista
}

// Estrutura da lista encadeada
pub struct LinkedList {
    head: Option<Box<Node>>,  // None = lista vazia
}

impl LinkedList {
    /// Cria uma nova lista vazia
    pub fn new() -> Self {
        LinkedList { head: None }
    }
    
    /// Adiciona um elemento no início da lista
    /// Complexidade: O(1)
    pub fn push_front(&mut self, value: i32) {
        // Cria um novo nó
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),  // take() move o head antigo
        });
        
        // O novo nó se torna o head
        self.head = Some(new_node);
    }
    
    /// Remove e retorna o primeiro elemento
    /// Complexidade: O(1)
    pub fn pop_front(&mut self) -> Option<i32> {
        // take() move o head para fora, deixando None no lugar
        self.head.take().map(|node| {
            // Extrai o valor e o próximo nó do Box
            self.head = node.next;  // Atualiza head
            node.value              // Retorna o valor
        })
    }
    
    /// Retorna o número de elementos na lista
    /// Complexidade: O(n)
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        
        // Percorre a lista contando nós
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        
        count
    }
    
    /// Verifica se a lista está vazia
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    
    /// Imprime todos os elementos
    pub fn print(&self) {
        let mut current = &self.head;
        print!("Lista: ");
        
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }
        
        println!("None");
    }
}

// Drop é implementado automaticamente!
// Quando LinkedList sai de escopo, todos os Boxes são liberados
// recursivamente

fn main() {
    let mut list = LinkedList::new();
    
    println!("=== Testando LinkedList ===\n");
    
    // Adiciona elementos
    println!("Adicionando 10, 20, 30...");
    list.push_front(10);
    list.push_front(20);
    list.push_front(30);
    list.print();
    println!("Tamanho: {}\n", list.len());
    
    // Remove elementos
    println!("Removendo elemento...");
    if let Some(value) = list.pop_front() {
        println!("Removido: {}", value);
    }
    list.print();
    println!("Tamanho: {}\n", list.len());
    
    // Remove mais elementos
    println!("Removendo todos os elementos:");
    while let Some(value) = list.pop_front() {
        println!("  Removido: {}", value);
    }
    list.print();
    println!("Lista vazia? {}", list.is_empty());
}
~~~

---

### 🔍 Análise Passo a Passo: push_front

Vamos entender como `push_front(20)` funciona quando a lista já tem `[10]`:

~~~mermaid
sequenceDiagram
    participant Código
    participant LinkedList
    participant Stack
    participant Heap
    
    Note over Código,Heap: Estado inicial: head -> Box(Node{10, None})
    
    Código->>LinkedList: push_front(20)
    LinkedList->>LinkedList: self.head.take()
    Note over LinkedList: head agora é None<br/>Retorna Some(Box(Node{10, None}))
    
    LinkedList->>Stack: Cria new_node
    Stack->>Heap: Aloca Box(Node{20, ...})
    Note over Heap: Node { value: 20,<br/>next: Some(Box(Node{10, None})) }
    
    LinkedList->>LinkedList: self.head = Some(new_node)
    
    Note over Código,Heap: Estado final: head -> Box(Node{20, Some(Box(Node{10, None}))})
~~~

**Pontos-chave:**
1. `take()` **move** o head antigo, deixando `None` no lugar
2. O novo nó **aponta** para o head antigo (ownership transferido)
3. O novo nó se torna o novo head

---

### 🧩 Por que Box é Necessário Aqui?

~~~rust {.line-numbers}
// ❌ SEM BOX - NÃO COMPILA
struct Node {
    value: i32,
    next: Option<Node>,  // Tamanho infinito!
}
// Erro: recursive type has infinite size

// ✅ COM BOX - COMPILA
struct Node {
    value: i32,
    next: Option<Box<Node>>,  // Tamanho fixo!
}
// Tamanho de Node = sizeof(i32) + sizeof(Option<Box<Node>>)
//                 = 4 bytes + 8 bytes = 12 bytes (em 64-bit)
~~~

**Explicação:**
- `Box<Node>` é apenas um ponteiro (8 bytes em 64-bit)
- `Option<Box<Node>>` usa null pointer optimization (também 8 bytes)
- O compilador consegue calcular o tamanho total!

---

### 🚀 Extensões Opcionais (Desafios)

Tente implementar estas funções adicionais:

#### 1. push_back (adicionar no final)
~~~rust {.line-numbers}
pub fn push_back(&mut self, value: i32) {
    // Dica: percorra até o último nó
    // Cuidado com ownership!
}
~~~

#### 2. remove (remover valor específico)
~~~rust {.line-numbers}
pub fn remove(&mut self, value: i32) -> bool {
    // Dica: você precisa de dois ponteiros
    // (current e previous)
}
~~~

#### 3. reverse (inverter a lista)
~~~rust {.line-numbers}
pub fn reverse(&mut self) {
    // Dica: use três ponteiros
    // (prev, current, next)
}
~~~

---

## 🔄 Feedback e Avaliação

### ✅ Checklist de Conceitos

Marque o que você entendeu:

- [ ] Box aloca dados na heap
- [ ] Box tem ownership único
- [ ] Box tem tamanho fixo (ponteiro)
- [ ] Box implementa Drop (RAII)
- [ ] Box resolve tipos recursivos
- [ ] Deref coercion permite usar Box como &T
- [ ] `Option<Box<T>>` é comum em estruturas de dados
- [ ] `take()` move valores de Option

---

### 🧠 Quiz Rápido

**1. Qual é o tamanho de `Box<String>` em um sistema 64-bit?**
- a) Depende do tamanho da String
- b) 8 bytes (tamanho de um ponteiro)
- c) 24 bytes (tamanho de String)
- d) Infinito

<details>
<summary>Resposta</summary>

**b) 8 bytes**

Box sempre tem o tamanho de um ponteiro, independentemente do tipo T que ele contém.
</details>

---

**2. Por que este código não compila?**
~~~rust {.line-numbers}
struct Node {
    value: i32,
    next: Node,
}
~~~

- a) Node não implementa Copy
- b) Tipo recursivo tem tamanho infinito
- c) next deveria ser &Node
- d) value deveria ser String

<details>
<summary>Resposta</summary>

**b) Tipo recursivo tem tamanho infinito**

O compilador não consegue calcular o tamanho de Node porque ele contém outro Node diretamente, criando recursão infinita.
</details>

---

**3. O que acontece quando Box sai de escopo?**
~~~rust {.line-numbers}
{
    let b = Box::new(42);
} // <- aqui
~~~

- a) Nada, você precisa chamar `free()`
- b) Memory leak
- c) Drop é chamado automaticamente, liberando a memória
- d) Panic

<details>
<summary>Resposta</summary>

**c) Drop é chamado automaticamente, liberando a memória**

Box implementa o trait Drop, que é chamado automaticamente quando o valor sai de escopo (RAII).
</details>

---

**4. Quando você deve usar Box?**

Marque todas as corretas:
- [ ] a) Tipo recursivo (lista, árvore)
- [ ] b) Transferir dados grandes sem cópia
- [ ] c) Quando precisa de múltiplos donos
- [ ] d) Trait objects (`Box<dyn Trait>`)

<details>
<summary>Resposta</summary>

**a, b, d estão corretas**

- ✅ a) Box resolve tipos recursivos
- ✅ b) Box evita cópias caras na stack
- ❌ c) Para múltiplos donos, use Rc/Arc (Dia 21)
- ✅ d) Box é usado para trait objects dinâmicos
</details>

---

### 🔬 Exercícios de Diagnóstico

#### Exercício 1: Identifique o Problema

~~~rust {.line-numbers}
struct TreeNode {
    value: i32,
    left: TreeNode,   // Linha A
    right: TreeNode,  // Linha B
}
~~~

**Pergunta:** O que está errado? Como corrigir?

<details>
<summary>Solução</summary>

**Problema:** Tipo recursivo sem indireção (tamanho infinito)

**Correção:**
~~~rust {.line-numbers}
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}
~~~
</details>

---

#### Exercício 2: Complete o Código

~~~rust {.line-numbers}
fn main() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    
    // Complete: remova todos os elementos e imprima
    // Resultado esperado: 3, 2, 1
    
    // SEU CÓDIGO AQUI
}
~~~

<details>
<summary>Solução</summary>

~~~rust {.line-numbers}
while let Some(value) = list.pop_front() {
    println!("{}", value);
}
~~~
</details>

---

### 📊 Auto-Avaliação

Avalie seu entendimento (1-5):

| Conceito | 1 (não entendi) | 2 | 3 | 4 | 5 (domino) |
|----------|----------------|---|---|---|------------|
| O que é Box | ☐ | ☐ | ☐ | ☐ | ☐ |
| Quando usar Box | ☐ | ☐ | ☐ | ☐ | ☐ |
| Tipos recursivos | ☐ | ☐ | ☐ | ☐ | ☐ |
| Deref coercion | ☐ | ☐ | ☐ | ☐ | ☐ |
| Drop automático | ☐ | ☐ | ☐ | ☐ | ☐ |
| Implementar LinkedList | ☐ | ☐ | ☐ | ☐ | ☐ |

**Se você marcou 3 ou menos em algum item:** Revise a seção correspondente!

---

## 🚀 Transferência e Aplicação

### 🎯 Desafio Final: Binary Tree

Agora que você domina LinkedList, implemente uma **árvore binária de busca**!

~~~rust {.line-numbers}
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

pub struct BinaryTree {
    root: Option<Box<TreeNode>>,
}

impl BinaryTree {
    pub fn new() -> Self {
        // TODO: implementar
    }
    
    pub fn insert(&mut self, value: i32) {
        // TODO: inserir mantendo ordem BST
        // Regra: menores à esquerda, maiores à direita
    }
    
    pub fn contains(&self, value: i32) -> bool {
        // TODO: buscar valor na árvore
    }
    
    pub fn inorder_print(&self) {
        // TODO: imprimir em ordem (esquerda, raiz, direita)
    }
}
~~~

**Dicas:**
- Use recursão para `insert` e `contains`
- `inorder_print` deve visitar: esquerda → raiz → direita
- Lembre-se de usar `as_ref()` e `as_mut()` com Option

<details>
<summary>Solução Completa</summary>

~~~rust {.line-numbers}
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

pub struct BinaryTree {
    root: Option<Box<TreeNode>>,
}

impl BinaryTree {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }
    
    pub fn insert(&mut self, value: i32) {
        self.root = Self::insert_recursive(self.root.take(), value);
    }
    
    fn insert_recursive(node: Option<Box<TreeNode>>, value: i32) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode {
                value,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                if value < n.value {
                    n.left = Self::insert_recursive(n.left.take(), value);
                } else if value > n.value {
                    n.right = Self::insert_recursive(n.right.take(), value);
                }
                // Se value == n.value, não insere (sem duplicatas)
                Some(n)
            }
        }
    }
    
    pub fn contains(&self, value: i32) -> bool {
        Self::contains_recursive(&self.root, value)
    }
    
    fn contains_recursive(node: &Option<Box<TreeNode>>, value: i32) -> bool {
        match node {
            None => false,
            Some(n) => {
                if value == n.value {
                    true
                } else if value < n.value {
                    Self::contains_recursive(&n.left, value)
                } else {
                    Self::contains_recursive(&n.right, value)
                }
            }
        }
    }
    
    pub fn inorder_print(&self) {
        Self::inorder_recursive(&self.root);
        println!();
    }
    
    fn inorder_recursive(node: &Option<Box<TreeNode>>) {
        if let Some(n) = node {
            Self::inorder_recursive(&n.left);
            print!("{} ", n.value);
            Self::inorder_recursive(&n.right);
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new();
    
    // Inserindo valores
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);
    
    // Imprimindo em ordem (deve sair ordenado)
    print!("Árvore em ordem: ");
    tree.inorder_print();  // 1 3 5 7 9
    
    // Buscando valores
    println!("Contém 7? {}", tree.contains(7));   // true
    println!("Contém 4? {}", tree.contains(4));   // false
}
~~~
</details>

---

### 🔮 Preparação para o Dia 21: Rc e Arc

Você aprendeu que **Box tem ownership único**. Mas e se você precisar de **múltiplos donos**?

~~~rust {.line-numbers}
// Problema: como compartilhar um nó entre duas listas?
let node = Box::new(Node { value: 42, next: None });
let list1 = LinkedList { head: Some(node) };
let list2 = LinkedList { head: Some(node) };  // ❌ Erro! node já foi movido
~~~

**No próximo dia você aprenderá:**
- 📚 **Rc<T>**: Reference Counted (múltiplos donos, single-threaded)
- 🔒 **Arc<T>**: Atomic Reference Counted (múltiplos donos, thread-safe)
- 🔄 **Weak<T>**: Referências fracas para evitar ciclos

---

### 📚 Recursos Complementares

#### Documentação Oficial
- [std::boxed::Box](https://doc.rust-lang.org/std/boxed/struct.Box.html)
- [The Rust Book - Box<T>](https://doc.rust-lang.org/book/ch15-01-box.html)

#### Artigos Recomendados
- "Understanding Smart Pointers in Rust" - Rust Blog
- "When to Use Box, Rc, or Arc" - Rust Patterns

#### Vídeos
- "Rust Smart Pointers Explained" - Let's Get Rusty (YouTube)
- "Box and Heap Allocation" - Jon Gjengset (YouTube)

#### Exercícios Práticos
- [Rustlings](https://github.com/rust-lang/rustlings) - Seção "smart_pointers"
- [Exercism Rust Track](https://exercism.org/tracks/rust) - "Simple Linked List"

---

## 🎓 Resumo Final

### O que você aprendeu hoje:

✅ **Box<T>** é um smart pointer que aloca na heap  
✅ Box tem **ownership único** e tamanho **fixo**  
✅ Box é **essencial** para tipos recursivos  
✅ **Deref coercion** permite usar Box como &T  
✅ **Drop automático** (RAII) previne memory leaks  
✅ Implementou uma **LinkedList** funcional  

### Conceitos-chave:

| Conceito | Descrição |
|----------|-----------|
| `Box::new(value)` | Aloca `value` na heap |
| `Option<Box<T>>` | Padrão para estruturas recursivas |
| `take()` | Move valor de Option, deixando None |
| Deref | Box se comporta como &T |
| Drop | Liberação automática de memória |

### Padrão Mental:

~~~
Preciso de estrutura recursiva? → Use Box
Dados muito grandes? → Use Box
Ownership único + heap? → Use Box
Múltiplos donos? → Aguarde Rc/Arc (Dia 21)
~~~

---

## 🎉 Parabéns!

Você completou o **Dia 20** e agora entende o primeiro smart pointer de Rust! Box é a fundação para entender Rc, Arc, RefCell e outros smart pointers mais avançados.

**Próximo passo:** Pratique implementando a Binary Tree e prepare-se para aprender sobre **shared ownership** com Rc e Arc!

---

**Dúvidas? Pontos para revisar?**
- Revise os diagramas
- Execute o código da LinkedList
- Tente o desafio da Binary Tree
- Compare Box com ownership direto

**Continue praticando! 🦀**