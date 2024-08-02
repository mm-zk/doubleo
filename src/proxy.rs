use crate::eth::EthNamespaceT;

use eyre::Context;
use jsonrpc_core::{BoxFuture, Result};
use std::{future::Future, pin::Pin, str::FromStr};
use tokio::runtime::Builder;
use zksync_types::{
    api::{BlockIdVariant, BlockNumber, Transaction, TransactionReceipt, TransactionVariant},
    transaction_request::CallRequest,
    url::SensitiveUrl,
    web3::{Bytes, FeeHistory, Index, SyncState},
    Address, H256, U256, U64,
};
use zksync_web3_decl::{
    client::{Client, L2},
    namespaces::EthNamespaceClient,
    types::{Block, Filter, FilterChanges, Log},
};

pub(crate) trait IntoBoxedFuture: Sized + Send + 'static {
    fn into_boxed_future(self) -> Pin<Box<dyn Future<Output = Self> + Send>> {
        Box::pin(async { self })
    }
}

impl<T, U> IntoBoxedFuture for std::result::Result<T, U>
where
    T: Send + 'static,
    U: Send + 'static,
{
}

#[derive(Clone)]
pub struct Proxy {
    pub sequencer_url: String,
}

impl Proxy {
    pub fn create_client(&self) -> Client<L2> {
        let url = SensitiveUrl::from_str(&self.sequencer_url)
            .unwrap_or_else(|_| panic!("Unable to parse client URL: {}", &self.sequencer_url));
        Client::http(url)
            .unwrap_or_else(|_| {
                panic!("Unable to create a client for fork: {}", self.sequencer_url)
            })
            .build()
    }
}

pub fn block_on<F: Future + Send + 'static>(future: F) -> F::Output
where
    F::Output: Send,
{
    std::thread::spawn(move || {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime creation failed");
        runtime.block_on(future)
    })
    .join()
    .unwrap()
}

impl EthNamespaceT for Proxy {
    fn get_block_number(&self) -> BoxFuture<Result<U64>> {
        let client = self.create_client();
        block_on(async move { client.get_block_number().await })
            .map_err(|_| jsonrpc_core::Error::internal_error())
            .into_boxed_future()
    }

    fn chain_id(&self) -> BoxFuture<Result<U64>> {
        todo!()
    }

    fn call(&self, req: CallRequest, block: Option<BlockIdVariant>) -> BoxFuture<Result<Bytes>> {
        todo!()
    }

    fn estimate_gas(
        &self,
        req: CallRequest,
        _block: Option<BlockNumber>,
    ) -> BoxFuture<Result<U256>> {
        todo!()
    }

    fn gas_price(&self) -> BoxFuture<Result<U256>> {
        todo!()
    }

    fn new_filter(&self, filter: Filter) -> BoxFuture<Result<U256>> {
        todo!()
    }

    fn new_block_filter(&self) -> BoxFuture<Result<U256>> {
        todo!()
    }

    fn uninstall_filter(&self, idx: U256) -> BoxFuture<Result<bool>> {
        todo!()
    }

    fn new_pending_transaction_filter(&self) -> BoxFuture<Result<U256>> {
        todo!()
    }

    fn get_logs(&self, filter: Filter) -> BoxFuture<Result<Vec<Log>>> {
        todo!()
    }

    fn get_filter_logs(&self, filter_index: U256) -> BoxFuture<Result<FilterChanges>> {
        todo!()
    }

    fn get_filter_changes(&self, filter_index: U256) -> BoxFuture<Result<FilterChanges>> {
        todo!()
    }

    fn get_balance(
        &self,
        address: Address,
        block: Option<BlockIdVariant>,
    ) -> BoxFuture<Result<U256>> {
        todo!()
    }

    fn get_block_by_number(
        &self,
        block_number: BlockNumber,
        full_transactions: bool,
    ) -> BoxFuture<Result<Option<Block<TransactionVariant>>>> {
        todo!()
    }

    fn get_block_by_hash(
        &self,
        hash: H256,
        full_transactions: bool,
    ) -> BoxFuture<Result<Option<Block<TransactionVariant>>>> {
        todo!()
    }

    fn get_block_transaction_count_by_number(
        &self,
        block_number: BlockNumber,
    ) -> BoxFuture<Result<Option<U256>>> {
        todo!()
    }

    fn get_block_transaction_count_by_hash(
        &self,
        block_hash: H256,
    ) -> BoxFuture<Result<Option<U256>>> {
        todo!()
    }

    fn get_code(
        &self,
        address: Address,
        block: Option<BlockIdVariant>,
    ) -> BoxFuture<Result<Bytes>> {
        todo!()
    }

    fn get_storage(
        &self,
        address: Address,
        idx: U256,
        block: Option<BlockIdVariant>,
    ) -> BoxFuture<Result<H256>> {
        todo!()
    }

    fn get_transaction_count(
        &self,
        address: Address,
        block: Option<BlockIdVariant>,
    ) -> BoxFuture<Result<U256>> {
        todo!()
    }

    fn get_transaction_by_hash(&self, hash: H256) -> BoxFuture<Result<Option<Transaction>>> {
        todo!()
    }

    fn get_transaction_by_block_hash_and_index(
        &self,
        block_hash: H256,
        index: Index,
    ) -> BoxFuture<Result<Option<Transaction>>> {
        todo!()
    }

    fn get_transaction_by_block_number_and_index(
        &self,
        block_number: BlockNumber,
        index: Index,
    ) -> BoxFuture<Result<Option<Transaction>>> {
        todo!()
    }

    fn get_transaction_receipt(&self, hash: H256) -> BoxFuture<Result<Option<TransactionReceipt>>> {
        todo!()
    }

    fn protocol_version(&self) -> BoxFuture<Result<String>> {
        todo!()
    }

    fn send_raw_transaction(&self, tx_bytes: Bytes) -> BoxFuture<Result<H256>> {
        todo!()
    }

    fn syncing(&self) -> BoxFuture<Result<SyncState>> {
        todo!()
    }

    fn accounts(&self) -> BoxFuture<Result<Vec<Address>>> {
        todo!()
    }

    fn coinbase(&self) -> BoxFuture<Result<Address>> {
        todo!()
    }

    fn compilers(&self) -> BoxFuture<Result<Vec<String>>> {
        todo!()
    }

    fn hashrate(&self) -> BoxFuture<Result<U256>> {
        todo!()
    }

    fn get_uncle_count_by_block_hash(&self, hash: H256) -> BoxFuture<Result<Option<U256>>> {
        todo!()
    }

    fn get_uncle_count_by_block_number(
        &self,
        number: BlockNumber,
    ) -> BoxFuture<Result<Option<U256>>> {
        todo!()
    }

    fn mining(&self) -> BoxFuture<Result<bool>> {
        todo!()
    }

    fn fee_history(
        &self,
        block_count: U64,
        newest_block: BlockNumber,
        reward_percentiles: Vec<f32>,
    ) -> BoxFuture<Result<FeeHistory>> {
        todo!()
    }
}
