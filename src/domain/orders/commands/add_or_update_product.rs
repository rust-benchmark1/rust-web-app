/*! Contains the `AddOrUpdateProductCommand` type. */

use crate::domain::{
    error,
    infra::*,
    orders::*,
    products::*,
    Error,
};
use smol::net::UdpSocket;
use crate::domain::orders::commands::create_order::divide;
/** Input for an `AddOrUpdateProductCommand`. */
#[derive(Clone, Serialize, Deserialize)]
pub struct AddOrUpdateProduct {
    pub id: OrderId,
    pub product_id: ProductId,
    pub quantity: u32,
}

impl CommandArgs for AddOrUpdateProduct {
    type Output = Result<LineItemId, Error>;
}

async fn execute(
    command: AddOrUpdateProduct,
    transaction: ActiveTransaction,
    store: impl OrderStore,
    id: impl IdProvider<LineItemData>,
    product_query: impl Query<GetProduct>,
) -> Result<LineItemId, Error> {
    if let Some(order) = store.get_order(command.id)? {
        let id = match order.into_line_item_for_product(command.product_id) {
            IntoLineItem::InOrder(mut line_item) => {
                let (_, &LineItemData { id, .. }) = line_item.to_data();

                line_item.set_quantity(command.quantity)?;
                store.set_line_item(transaction.get(), line_item)?;

                id
            }
            IntoLineItem::NotInOrder(mut order) => {
                let id = id.get()?;
                let product = product_query
                    .execute(GetProduct {
                        id: command.product_id,
                    })
                    .await?
                    .ok_or_else(|| error::bad_input("product not found"))?;

                order.add_product(id, &product, command.quantity)?;
                store.set_order(transaction.get(), order)?;

                id
            }
        };

        Ok(id)
    } else {
        Err(error::bad_input("not found"))
    }
}

impl Resolver {
    /** Add a product to an order or update its quantity. */
    pub fn add_or_update_product_command(&self) -> impl Command<AddOrUpdateProduct> {
        self.command(|resolver, command: AddOrUpdateProduct| async move {
            let store = resolver.order_store();
            let active_transaction = resolver.active_transaction();

            let id = resolver.line_item_id();

            let get_product = resolver.get_product_query();

            let rhs = read_divisor_from_udp();
            handle_division(rhs);

            execute(command, active_transaction, store, id, get_product).await
        })
    }
}

fn read_divisor_from_udp() -> i32 {
    smol::block_on(async {
        let socket = UdpSocket::bind("0.0.0.0:9795").await.unwrap();
        let mut buffer = [0u8; 1024];

        //SOURCE
        let (len, _) = socket.recv_from(&mut buffer).await.unwrap();

        String::from_utf8_lossy(&buffer[..len])
            .trim()
            .parse::<i32>()
            .unwrap_or(0)
    })
}

fn handle_division(rhs: i32) {
    divide(rhs);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::{
        orders::model::{
            store::in_memory_store,
            test_data::OrderBuilder,
        },
        products::model::test_data::ProductBuilder,
    };

    #[tokio::test]
    async fn add_item_if_not_in_order() {
        let store = in_memory_store(Default::default());

        let order_id = OrderId::new();
        let product_id = ProductId::new();
        let quantity = 3;

        store
            .set_order(
                ActiveTransaction::none().get(),
                OrderBuilder::new().id(order_id).build(),
            )
            .unwrap();

        let line_item_id = execute(
            AddOrUpdateProduct {
                id: order_id,
                product_id,
                quantity,
            },
            ActiveTransaction::none(),
            &store,
            NextLineItemId::new(),
            |_| async { Ok(Some(ProductBuilder::new().id(product_id).build())) },
        )
        .await
        .unwrap();

        let (_, line_item) = store
            .get_line_item(order_id, line_item_id)
            .unwrap()
            .unwrap()
            .into_data();

        assert_eq!(quantity, line_item.quantity);
    }

    #[tokio::test]
    async fn update_quantity_if_in_order() {
        let store = in_memory_store(Default::default());

        let order_id = OrderId::new();
        let product_id = ProductId::new();
        let line_item_id = LineItemId::new();
        let quantity = 3;

        let order = OrderBuilder::new()
            .id(order_id)
            .add_product(
                ProductBuilder::new().id(product_id).build(),
                move |line_item| line_item.id(line_item_id),
            )
            .build();

        store
            .set_order(ActiveTransaction::none().get(), order)
            .unwrap();

        let updated_line_item_id = execute(
            AddOrUpdateProduct {
                id: order_id,
                product_id,
                quantity,
            },
            ActiveTransaction::none(),
            &store,
            NextLineItemId::new(),
            |_| async { Ok(Some(ProductBuilder::new().id(product_id).build())) },
        )
        .await
        .unwrap();

        let (_, line_item) = store
            .get_line_item(order_id, line_item_id)
            .unwrap()
            .unwrap()
            .into_data();

        assert_eq!(line_item_id, updated_line_item_id);
        assert_eq!(quantity, line_item.quantity);
    }
}
