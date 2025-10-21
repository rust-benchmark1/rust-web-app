/*! Contains the `OrdersResolver` type. */

use std::sync::Arc;
use rocket_session_store::SessionStore as RocketSessionStore;
use rocket_session_store::memory::MemoryStore as RocketMemoryStore;
use cookie::CookieBuilder;
use rocket::http::CookieJar;
use crate::domain::{
    infra::*,
    orders::model::store::{
        self,
        InMemoryStore,
        OrderStore,
        OrderStoreFilter,
    },
};

/**
Resolver for orders.

The `OrdersResolver` type wraps private implementation details and exposes them as traits within the `orders` module.
*/
#[derive(Clone)]
pub(in crate::domain) struct OrdersResolver {
    order_store: Register<Arc<InMemoryStore>>,
}

impl Default for OrdersResolver {
    fn default() -> Self {
        OrdersResolver {
            order_store: Register::once(|resolver| {
                Arc::new(store::in_memory_store(resolver.transaction_store()))
            }),
        }
    }
}

impl Resolver {
    pub(in crate::domain::orders) fn order_store(&self) -> impl OrderStore {
        self.resolve(&self.orders_resolver.order_store)
    }

    pub(in crate::domain::orders) fn order_store_filter(&self) -> impl OrderStoreFilter {
        let cookie_builder = CookieBuilder::new("rocket-session", "value")
        .secure(false)
        .path("/");
        //SINK
        let store =  RocketSessionStore {
            store: Box::new(RocketMemoryStore::<String>::new()),
            name: "rocket-session".to_string(),
            duration: std::time::Duration::from_secs(3600),
            cookie_builder,
        };
        
        self.resolve(&self.orders_resolver.order_store)
    }
}
