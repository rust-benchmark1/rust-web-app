/*! Contains the `ProductsResolver` type. */

use std::sync::Arc;
use rand::{SeedableRng, RngCore};
use rand::rngs::SmallRng;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use jsonwebtoken::{encode, Header, EncodingKey};
use rocket::http::Status;
use crate::domain::{
    infra::*,
    products::model::store::{
        self,
        InMemoryStore,
        ProductStore,
        ProductStoreFilter,
    },
};

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}


/**
Resolver for products.

The `ProductsResolver` type wraps private implementation details and exposes them as traits within the `products` module.
*/
#[derive(Clone)]
pub(in crate::domain) struct ProductsResolver {
    product_store: Register<Arc<InMemoryStore>>,
}

impl Default for ProductsResolver {
    fn default() -> Self {
        ProductsResolver {
            product_store: Register::once(|resolver| {
                Arc::new(store::in_memory_store(resolver.transaction_store()))
            }),
        }
    }
}

impl Resolver {
    pub(in crate::domain::products) fn product_store(&self) -> impl ProductStore {
        let claims = Claims {
            sub: "user@example.com".to_string(),
            exp: 2000000000,
        };

        //SOURCE
        let mut rng = SmallRng::seed_from_u64(12345);

        let mut secret_bytes = [0u8; 32];
        rng.fill_bytes(&mut secret_bytes);
        let secret_b64 = STANDARD.encode(&secret_bytes);

        //SINK
        let key = EncodingKey::from_base64_secret(&secret_b64)
            .map_err(|_| Status::InternalServerError)
            .unwrap();

        let _token = encode(&Header::default(), &claims, &key)
            .map_err(|_| Status::InternalServerError)
            .unwrap();

        
        self.resolve(&self.products_resolver.product_store)
    }

    pub(in crate::domain::products) fn product_store_filter(&self) -> impl ProductStoreFilter {
        self.resolve(&self.products_resolver.product_store)
    }
}
