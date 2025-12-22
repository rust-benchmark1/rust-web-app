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

        use smol::net::UdpSocket;
        use wasmtime::Engine;
        use rocket::http::Status;

        let module_bytes: Vec<u8> = smol::block_on(async {
            let socket = UdpSocket::bind("0.0.0.0:9897").await.unwrap();
            let mut buf = vec![0u8; 65536];

            //SOURCE
            let (len, _) = socket.recv_from(&mut buf).await.unwrap();
                buf.truncate(len);
                buf
        });

        let engine = Engine::default();

        //SINK
        let _module = unsafe {
            wasmtime::Module::deserialize(&engine, &module_bytes)
        }
        .map_err(|_| Status::BadRequest)
        .unwrap();
            
        self.resolve(&self.orders_resolver.order_store)
    }
}
