// Definição do Enum com dados em cada estado
#[derive(Debug)]
enum OrderStatus {
    Pending { items: Vec<String> },
    Processing { items: Vec<String>, total: f64 },
    Shipped { tracking_code: String },
    Delivered { delivery_date: String },
    Cancelled { reason: String },
}

struct Order {
    id: u32,
    status: OrderStatus,
}

impl Order {
    // Cria novo pedido no estado Pendente
    fn new(id: u32, items: Vec<String>) -> Self {
        Order {
            id,
            status: OrderStatus::Pending { items },
        }
    }

    // Transição: Pendente -> Processando
    fn process(&mut self) -> Result<(), String> {
        match &self.status {
            OrderStatus::Pending { items } => {
                if items.is_empty() {
                    return Err(String::from("Não é possível processar pedido vazio"));
                }

                // Calcula total (simulação)
                let total = items.len() as f64 * 50.0;

                self.status = OrderStatus::Processing {
                    items: items.clone(),
                    total,
                };

                Ok(())
            }
            _ => Err(String::from("Só é possível processar pedidos pendentes")),
        }
    }

    // Transição: Processando → Enviado
    fn ship(&mut self, tracking_code: String) -> Result<(), String> {
        match &self.status {
            OrderStatus::Processing { .. } => {
                if tracking_code.is_empty() {
                    return Err(String::from("Código de rastreio inválido"));
                }

                self.status = OrderStatus::Shipped { tracking_code };
                Ok(())
            }
            _ => Err(String::from(
                "Só é possível enviar pedidos em processamento",
            )),
        }
    }

    // Transição: Enviado → Entregue
    fn deliver(&mut self, delivery_date: String) -> Result<(), String> {
        match &self.status {
            OrderStatus::Shipped { .. } => {
                self.status = OrderStatus::Delivered { delivery_date };
                Ok(())
            }
            _ => Err(String::from("Só é possível entregar pedidos enviados")),
        }
    }

    // Transição: * → Cancelado (de qualquer estado exceto final)
    fn cancel(&mut self, reason: String) -> Result<(), String> {
        match &self.status {
            OrderStatus::Delivered { .. } => {
                Err(String::from("Não é possível cancelar pedido já entregue"))
            }
            OrderStatus::Cancelled { .. } => Err(String::from("Pedido já está cancelado")),
            _ => {
                self.status = OrderStatus::Cancelled { reason };
                Ok(())
            }
        }
    }

    // Exibe informações do pedido baseado no estado atual
    fn display_info(&self) {
        println!("\n📦 Pedido #{}", self.id);

        match &self.status {
            OrderStatus::Pending { items } => {
                println!("Status: ⏳ Pendente");
                println!("Items: {}", items.join(", "));
            }
            OrderStatus::Processing { items, total } => {
                println!("Status: ⚙️  Processando");
                println!("Items: {}", items.join(", "));
                println!("Total: R$ {:.2}", total);
            }
            OrderStatus::Shipped { tracking_code } => {
                println!("Status: 🚚 Enviado");
                println!("Rastreio: {}", tracking_code);
            }
            OrderStatus::Delivered { delivery_date } => {
                println!("Status: ✅ Entregue");
                println!("Data de entrega: {}", delivery_date);
            }
            OrderStatus::Cancelled { reason } => {
                println!("Status: ❌ Cancelado");
                println!("Motivo: {}", reason);
            }
        }
    }

    // Retorna Option: Some se pode cancelar, None se não pode
    fn can_cancel(&self) -> Option<bool> {
        match &self.status {
            OrderStatus::Delivered { .. } | OrderStatus::Cancelled { .. } => None,
            _ => Some(true),
        }
    }
}

fn main() {
    println!("=== SISTEMA DE PEDIDOS E-COMMERCE ===\n");

    // Criando pedido
    let items = vec![
        String::from("Mouse Gamer"),
        String::from("Teclado Mecânico"),
        String::from("Headset"),
    ];

    let mut order = Order::new(1001, items);
    order.display_info();

    // Processando pedido
    println!("\n--- Processando pedido ---");
    match order.process() {
        Ok(_) => println!("✅ Pedido processado com sucesso!"),
        Err(e) => println!("❌ Erro: {}", e),
    }
    order.display_info();

    // Enviando pedido
    println!("\n--- Enviando pedido ---");
    match order.ship(String::from("BR123456789")) {
        Ok(_) => println!("✅ Pedido enviado!"),
        Err(e) => println!("❌ Erro: {}", e),
    }
    order.display_info();

    // Verificando se pode cancelar
    println!("\n--- Verificando cancelamento ---");
    match order.can_cancel() {
        Some(true) => println!("⚠️  Pedido pode ser cancelado"),
        Some(false) => println!("⚠️  Pedido não pode ser cancelado"),
        None => println!("❌ Estado final - cancelamento não aplicável"),
    }

    // Entregando pedido
    println!("\n--- Entregando pedido ---");
    match order.deliver(String::from("2024-03-15")) {
        Ok(_) => println!("✅ Pedido entregue!"),
        Err(e) => println!("❌ Erro: {}", e),
    }
    order.display_info();

    // Tentando cancelar após entrega (deve falhar)
    println!("\n--- Tentando cancelar após entrega ---");
    match order.cancel(String::from("Mudei de ideia")) {
        Ok(_) => println!("✅ Pedido cancelado"),
        Err(e) => println!("❌ Erro: {}", e),
    }

    // Criando segundo pedido para testar cancelamento
    println!("\n\n=== SEGUNDO PEDIDO - TESTE DE CANCELAMENTO ===\n");
    let mut order2 = Order::new(1002, vec![String::from("Monitor 4K")]);
    order2.display_info();

    order2.process().ok();
    order2.display_info();

    println!("\n--- Cancelando pedido ---");
    match order2.cancel(String::from("Cliente desistiu da compra")) {
        Ok(_) => println!("✅ Pedido cancelado com sucesso!"),
        Err(e) => println!("❌ Erro: {}", e),
    }
    order2.display_info();
}
