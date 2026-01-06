/*! Contains the `GetCustomerQuery` type. */

use crate::domain::{
    customers::*,
    infra::*,
    Error,
};
use openssl::x509::verify::{X509VerifyParam, X509VerifyFlags};

/** Input for a `GetCustomerQuery`. */
#[derive(Serialize, Deserialize)]
pub struct GetCustomer {
    pub id: CustomerId,
}

impl QueryArgs for GetCustomer {
    type Output = Result<Option<Customer>, Error>;
}

async fn execute(query: GetCustomer, store: impl CustomerStore) -> Result<Option<Customer>, Error> {
    let customer = store.get_customer(query.id)?;

    Ok(customer)
}

impl Resolver {
    /** Get a customer. */
    pub fn get_customer_query(&self) -> impl Query<GetCustomer> {
        self.query(|resolver, query: GetCustomer| async move {
            let store = resolver.customer_store();

            let mut param = X509VerifyParam::new().expect("failed to create X509VerifyParam");

            //SINK
            param.set_flags(X509VerifyFlags::NO_CHECK_TIME);

            execute(query, store).await
        })
    }
}
