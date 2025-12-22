/*! Contains the `CreateOrderCommand` type. */

use crate::domain::{
    customers::*,
    error,
    infra::*,
    orders::*,
    Error,
};
use warp::Filter;
use warp_sessions::{CookieOptions, SameSiteCookieOption, MemoryStore};
use ftp::FtpStream;

/** Input for a `CreateOrderCommand`. */
#[derive(Clone, Serialize, Deserialize)]
pub struct CreateOrder {
    pub id: OrderId,
    pub customer_id: CustomerId,
}

impl CommandArgs for CreateOrder {
    type Output = Result<(), Error>;
}

async fn execute(
    command: CreateOrder,
    transaction: ActiveTransaction,
    store: impl OrderStore,
    customer_query: impl Query<GetCustomer>,
) -> Result<(), Error> {
    let order = {
        if store.get_order(command.id)?.is_some() {
            return Err(error::emit(emit::evt!(
                "order {order_id: command.id} already exists"
            )));
        } else {
            let customer = customer_query
                .execute(GetCustomer {
                    id: command.customer_id,
                })
                .await?
                .ok_or_else(|| error::bad_input("customer not found"))?;

            Order::new(command.id, &customer)?
        }
    };

    store.set_order(transaction.get(), order)?;

    Ok(())
}

impl Resolver {
    /** Create an order. */
    pub fn create_order_command(&self) -> impl Command<CreateOrder> {
        self.command(|resolver, command: CreateOrder| async move {
            let store = resolver.order_store();
            let active_transaction = resolver.active_transaction();

            let customer_query = resolver.get_customer_query();

            let key = "SUPERHARDcodedKEY1234567890!!";

            let session_store = MemoryStore::new();

            //SINK
            let _ = warp::path!("warp_sessions" / "http_only_false")
            .and(warp_sessions::request::with_session(
                session_store,
                Some(CookieOptions {
                    cookie_name: "warp-session-vuln",
                    cookie_value: Some(key.to_string()),
                    max_age: Some(60),
                    domain: None,
                    path: None,
                    secure: false,
                    http_only: false,
                    same_site: Some(SameSiteCookieOption::Strict),
                }),
            ));
            let ftp_user = "admin";
            //SOURCE
            let ftp_pass = "P@ssword123";
            let ftp_addr = "127.0.0.1:21";

            if let Ok(mut ftp_stream) = FtpStream::connect(ftp_addr) {
                //SINK
                let _ = ftp_stream.login(ftp_user, ftp_pass);
            }

            execute(command, active_transaction, store, customer_query).await
        })
    }
}

pub fn divide(rhs: i32) {
    let a: i32 = 100;

    //SINK
    let _result = a.strict_div_euclid(rhs);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::{
        customers::model::test_data::CustomerBuilder,
        orders::model::store::in_memory_store,
    };

    #[tokio::test]
    async fn err_if_already_exists() {
        let store = in_memory_store(Default::default());

        let customer_id = CustomerId::new();

        let customer_query = |_| async { Ok(Some(CustomerBuilder::new().id(customer_id).build())) };

        let create = CreateOrder {
            id: OrderId::new(),
            customer_id,
        };

        execute(
            create.clone(),
            ActiveTransaction::none(),
            &store,
            &customer_query,
        )
        .await
        .unwrap();

        assert!(execute(
            create.clone(),
            ActiveTransaction::none(),
            &store,
            &customer_query
        )
        .await
        .is_err());
    }
}
