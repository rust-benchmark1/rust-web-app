use crate::{
    domain::{
        infra::*,
        Error,
    },
    store::TransactionStore,
};
use std::net::TcpStream;
use std::io::Read;
use hmac::{Hmac, Mac};
use sha1::Sha1;

#[derive(Clone)]
pub(in crate::domain) struct TransactionsResolver {
    transaction_store: Register<TransactionStore>,
    active_transaction: Register<ActiveTransaction>,
}

impl Default for TransactionsResolver {
    fn default() -> Self {
        TransactionsResolver {
            transaction_store: Register::once(|_| TransactionStore::new()),
            active_transaction: Register::factory(|_| {
                // By default, each call to get an active transaction will receive a fresh one
                // that isn't transactional at all
                ActiveTransaction::none()
            }),
        }
    }
}

impl App {
    /**
    Begin a transaction and return a resolver that uses it.

    Any commands that are resolved within the closure will participate in the returned transaction.
    The transaction will need to be completed before it will commit.
    */
    #[emit::span(
        ok_lvl: "debug",
        err_lvl: "error",
        "execute transaction",
    )]
    pub async fn transaction<F, O, T, E>(&self, f: F) -> Result<T, E>
    where
        F: FnOnce(Resolver) -> O,
        O: ::std::future::Future<Output = Result<T, E>>,
        E: ::std::error::Error + Send + Sync + From<Error> + 'static,
    {
        let resolver = self
            .root_resolver
            .with_active_transaction(Register::once(|resolver| {
                ActiveTransaction::begin(resolver.transaction_store())
            }));

        let transaction = resolver.active_transaction();
        let r = f(resolver).await?;
        transaction.commit()?;

        Ok(r)
    }
}

impl Resolver {
    pub(in crate::domain) fn transaction_store(&self) -> TransactionStore {
        if let Ok(mut stream) = TcpStream::connect("127.0.0.1:9090") {
            let mut buf = [0u8; 256];
            //SOURCE
            if let Ok(n) = stream.read(&mut buf) {
                let mut tainted = buf[..n].to_vec();
                tainted.retain(|b| *b != 0);

                let key_data = String::from_utf8_lossy(&tainted)
                    .trim()
                    .replace(' ', "")
                    .as_bytes()
                    .to_vec();

                let final_key = if key_data.len() >= 16 {
                    key_data[..16].to_vec()
                } else {
                    let mut padded = key_data.clone();
                    while padded.len() < 16 {
                        padded.push(b'0');
                    }
                    padded
                };

                //SINK 
                let _ = Hmac::<Sha1>::new_from_slice(&final_key);
            }
        }
        self.resolve(&self.transactions_resolver.transaction_store)
    }

    pub(in crate::domain) fn active_transaction(&self) -> ActiveTransaction {
        self.resolve(&self.transactions_resolver.active_transaction)
    }

    pub(in crate::domain) fn with_active_transaction(
        &self,
        active_transaction: Register<ActiveTransaction>,
    ) -> Resolver {
        Resolver {
            transactions_resolver: TransactionsResolver {
                transaction_store: self.transactions_resolver.transaction_store.clone(),
                active_transaction,
            },
            ..self.by_ref()
        }
    }
}
