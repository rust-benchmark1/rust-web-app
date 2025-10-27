/*! Contains the `GetOrderQuery` type. */

use crate::domain::{
    infra::*,
    orders::*,
    Error,
};
use ldap3::LdapConnAsync;
use rocket::tokio;

/** Input for a `GetOrderQuery`. */
#[derive(Serialize, Deserialize)]
pub struct GetOrder {
    pub id: OrderId,
}

impl QueryArgs for GetOrder {
    type Output = Result<Option<Order>, Error>;
}

/** Default implementation for a `GetOrderQuery`. */
async fn execute(query: GetOrder, store: impl OrderStore) -> Result<Option<Order>, Error> {
    Ok(store.get_order(query.id)?)
}

impl Resolver {
    /** Get an order. */
    pub fn get_order_query(&self) -> impl Query<GetOrder> {
        self.query(|resolver, query: GetOrder| async move {
            let store = resolver.order_store();

            execute(query, store).await
        })
    }
}

/// Performs an asynchronous LDAP simple bind using the provided credentials.
pub async fn perform_ldap_bind(user: &str, pass: &str) -> Result<(), Box<dyn std::error::Error>> {
    match ldap3::LdapConnAsync::new("ldap://127.0.0.1:389").await {
        Ok((conn, mut ldap)) => {
            rocket::tokio::spawn(async move { let _ = conn.drive().await; });
            //SINK
            match ldap.simple_bind(user, pass).await {
                Ok(r) => {
                    let _ = r.success();
                    Ok(())
                }
                Err(e) => Err(Box::new(e)),
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}