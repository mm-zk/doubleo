use std::str::FromStr;
use zksync_types::{
    api::{
        BlockId, BlockIdVariant, BlockNumber, Transaction, TransactionReceipt, TransactionVariant,
    },
    transaction_request::CallRequest,
    url::SensitiveUrl,
    web3::{Bytes, FeeHistory, Index, SyncState},
    Address, H256, U256, U64,
};
use zksync_web3_decl::jsonrpsee::proc_macros::rpc;
use zksync_web3_decl::{
    client::{Client, L2},
    jsonrpsee::{
        core::{async_trait, RpcResult},
        types::{error::ErrorCode, ErrorObject},
    },
    namespaces::{EthNamespaceClient, EthNamespaceServer},
    types::{Block, Filter, FilterChanges, Log},
};

use zksync_web3_decl::*;

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

pub struct PrivateProxy {
    pub sequencer_url: String,
}

impl PrivateProxy {
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

#[rpc(server, client, namespace = "privateeth")]
pub trait PrivateEthNamespace {
    #[method(name = "blockNumber")]
    async fn private_get_block_number(&self, username: String, password: String) -> RpcResult<U64>;
    #[method(name = "getBalance")]
    async fn private_get_balance(
        &self,
        username: String,
        password: String,
        address: Address,
        block: Option<BlockIdVariant>,
    ) -> RpcResult<U256>;
}

#[async_trait]
impl PrivateEthNamespaceServer for PrivateProxy {
    async fn private_get_block_number(&self, username: String, password: String) -> RpcResult<U64> {
        println!("username: {:?} password: {:?}", username, password);
        Ok(42.into())
    }

    async fn private_get_balance(
        &self,
        username: String,
        password: String,
        address: Address,
        block: Option<BlockIdVariant>,
    ) -> RpcResult<U256> {
        let client = self.create_client();
        client
            .get_balance(address, block)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }
}

#[async_trait]
impl EthNamespaceServer for Proxy {
    async fn get_block_number(&self) -> RpcResult<U64> {
        let client = self.create_client();
        client
            .get_block_number()
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn chain_id<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<U64>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn call<'life0, 'async_trait>(
        &'life0 self,
        req: CallRequest,
        block: Option<BlockIdVariant>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Bytes>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn estimate_gas<'life0, 'async_trait>(
        &'life0 self,
        req: CallRequest,
        _block: Option<BlockNumber>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<U256>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn gas_price<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<U256>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn new_filter<'life0, 'async_trait>(
        &'life0 self,
        filter: Filter,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<U256>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn new_block_filter<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<U256>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn uninstall_filter<'life0, 'async_trait>(
        &'life0 self,
        idx: U256,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<bool>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn new_pending_transaction_filter<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<U256>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_logs<'life0, 'async_trait>(
        &'life0 self,
        filter: Filter,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Vec<Log>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_filter_logs<'life0, 'async_trait>(
        &'life0 self,
        filter_index: U256,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<FilterChanges>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_filter_changes<'life0, 'async_trait>(
        &'life0 self,
        filter_index: U256,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<FilterChanges>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    async fn get_balance(
        &self,
        address: Address,
        block: Option<BlockIdVariant>,
    ) -> RpcResult<U256> {
        Err(ErrorObject::from(ErrorCode::ServerError(403)))
        /*let client = self.create_client();
        client
            .get_balance(address, block)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())*/
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_block_by_number<'life0, 'async_trait>(
        &'life0 self,
        block_number: BlockNumber,
        full_transactions: bool,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Option<Block<TransactionVariant>>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_block_by_hash<'life0, 'async_trait>(
        &'life0 self,
        hash: H256,
        full_transactions: bool,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Option<Block<TransactionVariant>>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_block_transaction_count_by_number<'life0, 'async_trait>(
        &'life0 self,
        block_number: BlockNumber,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Option<U256>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_block_receipts<'life0, 'async_trait>(
        &'life0 self,
        block_id: BlockId,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Option<Vec<TransactionReceipt>>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_block_transaction_count_by_hash<'life0, 'async_trait>(
        &'life0 self,
        block_hash: H256,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Option<U256>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_code<'life0, 'async_trait>(
        &'life0 self,
        address: Address,
        block: Option<BlockIdVariant>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Bytes>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_storage_at<'life0, 'async_trait>(
        &'life0 self,
        address: Address,
        idx: U256,
        block: Option<BlockIdVariant>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<H256>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_transaction_count<'life0, 'async_trait>(
        &'life0 self,
        address: Address,
        block: Option<BlockIdVariant>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<U256>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_transaction_by_hash<'life0, 'async_trait>(
        &'life0 self,
        hash: H256,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Option<Transaction>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_transaction_by_block_hash_and_index<'life0, 'async_trait>(
        &'life0 self,
        block_hash: H256,
        index: Index,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Option<Transaction>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_transaction_by_block_number_and_index<'life0, 'async_trait>(
        &'life0 self,
        block_number: BlockNumber,
        index: Index,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Option<Transaction>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_transaction_receipt<'life0, 'async_trait>(
        &'life0 self,
        hash: H256,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Option<TransactionReceipt>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn protocol_version<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<String>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn send_raw_transaction<'life0, 'async_trait>(
        &'life0 self,
        tx_bytes: Bytes,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<H256>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn syncing<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<SyncState>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn accounts<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Vec<Address>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn coinbase<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Address>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn compilers<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Vec<String>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn hashrate<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<U256>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_uncle_count_by_block_hash<'life0, 'async_trait>(
        &'life0 self,
        hash: H256,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Option<U256>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get_uncle_count_by_block_number<'life0, 'async_trait>(
        &'life0 self,
        number: BlockNumber,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Option<U256>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn mining<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<bool>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn fee_history<'life0, 'async_trait>(
        &'life0 self,
        block_count: U64,
        newest_block: BlockNumber,
        reward_percentiles: Vec<f32>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<FeeHistory>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
}
