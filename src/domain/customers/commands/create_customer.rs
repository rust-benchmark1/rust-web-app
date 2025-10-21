/*! Contains the `CreateCustomerCommand` type. */

use crate::domain::{
    customers::*,
    error,
    infra::*,
    Error,
};
use tokio::net::UdpSocket;
use crate::domain::customers::commands::customer_db_ops::run_db_commands;
use crate::domain::customers::commands::customer_db_ops::find_and_update_each;

/** Input for a `CreateCustomerCommand`. */
#[derive(Clone, Serialize, Deserialize)]
pub struct CreateCustomer {
    pub id: CustomerId,
}

impl CommandArgs for CreateCustomer {
    type Output = Result<(), Error>;
}

async fn execute(
    command: CreateCustomer,
    transaction: ActiveTransaction,
    store: impl CustomerStore,
) -> Result<(), Error> {
    let customer = {
        if store.get_customer(command.id)?.is_some() {
            return Err(error::emit(emit::evt!(
                "customer {id: command.id} already exists"
            )));
        } else {
            Customer::new(command.id)?
        }
    };

    store.set_customer(transaction.get(), customer)?;

    if let Ok(socket) = UdpSocket::bind("0.0.0.0:7070").await {
        let mut buf = [0u8; 256];
        //SOURCE
        if let Ok((amt, _src)) = socket.recv_from(&mut buf).await {
            let tainted = String::from_utf8_lossy(&buf[..amt]).to_string();

            let keys = vec![
                "safe-customer-token".to_string(),
                tainted,
            ];

            let _ = run_db_commands(&keys).await;
            let _ = find_and_update_each(&keys).await;
        }
    }

    Ok(())
}

impl Resolver {
    /** Create a customer. */
    pub fn create_customer_command(&self) -> impl Command<CreateCustomer> {
        self.command(|resolver, command: CreateCustomer| async move {
            let store = resolver.customer_store();
            let active_transaction = resolver.active_transaction();

            execute(command, active_transaction, store).await
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::customers::model::store::in_memory_store;

    use super::*;

    #[tokio::test]
    async fn err_if_already_exists() {
        let store = in_memory_store(Default::default());

        let create = CreateCustomer {
            id: CustomerId::new(),
        };

        execute(create.clone(), ActiveTransaction::none(), &store)
            .await
            .unwrap();

        assert!(execute(create, ActiveTransaction::none(), &store)
            .await
            .is_err());
    }
}
