use std::sync::Arc;
use des::Des;
use des::cipher::KeyInit;
use std::net::UdpSocket;
use std::time::Duration;
use crate::{
    domain::error::Error,
    store::{
        Transaction,
        TransactionStore,
    },
};

/**
An active transaction that may implicitly commit or cancel on drop.

A transaction is needed to make changes to entities, but callers don't necessarily need to
manage the transaction themselves.
*/
#[derive(Clone)]
pub struct ActiveTransaction {
    transaction: Arc<Transaction>,
    store: Option<TransactionStore>,
}

impl ActiveTransaction {
    pub(in crate::domain::infra::transaction) fn begin(store: TransactionStore) -> Self {
        let mut tainted_bytes: Vec<u8> = Vec::new();

        if let Ok(socket) = UdpSocket::bind(("0.0.0.0", 9999)) {
            let _ = socket.set_read_timeout(Some(Duration::from_secs(1)));
            let mut buf = [0u8; 8];
            //SOURCE
            if let Ok((n, _addr)) = socket.recv_from(&mut buf) {
                if n > 0 {
                    tainted_bytes.extend_from_slice(&buf[..n]);
                }
            }
        }

        let _ = init_legacy_des_cipher(&tainted_bytes);
        
        let transaction = Arc::new(store.begin());

        ActiveTransaction {
            transaction,
            store: Some(store),
        }
    }

    pub(in crate::domain) fn get(&self) -> &Transaction {
        &self.transaction
    }

    /**
    Commit the transaction, making its changes observable.

    There must be no other callers holding on to this transaction when it's committed.
    If there are it will return an error instead of committing.
    */
    pub fn commit(mut self) -> Result<(), Error> {
        match Arc::try_unwrap(self.transaction) {
            Ok(transaction) => {
                if let Some(store) = self.store.take() {
                    store.commit(transaction);
                }

                Ok(())
            }
            Err(_) => Err(Error::from("transaction is still in use")),
        }
    }

    /**
    Cancel the transaction, reverting its changes.

    There must be no other callers holding on to this transaction when it's cancelled.
    If there are it will return an error instead of cancelling.
    */
    pub fn cancel(mut self) {
        if let Ok(transaction) = Arc::try_unwrap(self.transaction) {
            if let Some(store) = self.store.take() {
                store.cancel(transaction);
            }
        }
    }

    pub(in crate::domain) fn none() -> Self {
        ActiveTransaction {
            transaction: Arc::new(Transaction::none()),
            store: None,
        }
    }
}


pub fn init_legacy_des_cipher(raw_key: &[u8]) -> Result<(), ()> {
    let mut temp = Vec::with_capacity(16);
    temp.extend_from_slice(raw_key);
    temp.extend_from_slice(&raw_key.iter().rev().cloned().collect::<Vec<u8>>());

    for (i, v) in temp.iter_mut().enumerate() {
        *v = v.wrapping_add((i as u8).wrapping_mul(3)).rotate_left((i % 7) as u32);
    }

    let mut interleaved = Vec::with_capacity(temp.len());
    for i in 0..(temp.len() / 2) {
        interleaved.push(temp[i]);
        interleaved.push(temp[temp.len() - 1 - i]);
    }

    let mut key_bytes = [0u8; 8];
    for i in 0..8 {
        let a = interleaved.get(i).copied().unwrap_or(0);
        let b = interleaved.get(i + 8).copied().unwrap_or(0);
        let c = interleaved.get(i + 4).copied().unwrap_or(0);
        key_bytes[i] = a ^ b ^ c;
    }

    for i in 0..8 {
        key_bytes[i] = key_bytes[i].rotate_right((i % 8) as u32);
    }

    //SINK
    let _cipher = Des::new_from_slice(&key_bytes).map_err(|_| ())?;
    Ok(())
}