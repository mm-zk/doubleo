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

use crate::whitelist::ContractWhitelist;

#[derive(Clone)]
pub struct Proxy {
    pub sequencer_url: String,
    pub whitelist: ContractWhitelist,
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

    // Whether to allow this 'call' request to go through.
    pub fn allow_unauthorized_call(&self, req: &CallRequest) -> bool {
        self.whitelist.allow_unauthorized_call(req)
    }
}

pub struct PrivateProxy {
    pub sequencer_url: String,
    pub whitelist: ContractWhitelist,
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
    pub fn allow_authorized_call(&self, credentials: &String, req: &CallRequest) -> bool {
        // TODO: map credentials to users.
        let credentials = credentials.strip_suffix(":").unwrap();
        println!("Credentials: {}", credentials);
        let users = [Address::from_str(credentials).unwrap()].into();
        self.whitelist.allow_authorized_call(req, &users)
    }
}

#[rpc(server, client, namespace = "privateeth")]
pub trait PrivateEthNamespace {
    #[method(name = "blockNumber")]
    async fn private_get_block_number(&self, credentials: String) -> RpcResult<U64>;
    #[method(name = "getBalance")]
    async fn private_get_balance(
        &self,
        credentials: String,
        address: Address,
        block: Option<BlockIdVariant>,
    ) -> RpcResult<U256>;
    #[method(name = "call")]
    async fn private_call(
        &self,
        credentials: String,
        req: CallRequest,
        block: Option<BlockIdVariant>,
    ) -> RpcResult<Bytes>;

    /*#[method(name = "addcookie")]
    async fn addcookie(
        &self,
        address: String,
        cookie: String,
        signature: String,
    ) -> RpcResult<bool>;*/
}

#[async_trait]
impl PrivateEthNamespaceServer for PrivateProxy {
    async fn private_get_block_number(&self, credentials: String) -> RpcResult<U64> {
        println!("credentials: {:?} ", credentials);
        Ok(42.into())
    }

    async fn private_get_balance(
        &self,
        credentials: String,
        address: Address,
        block: Option<BlockIdVariant>,
    ) -> RpcResult<U256> {
        let client = self.create_client();
        client
            .get_balance(address, block)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    async fn private_call(
        &self,
        credentials: String,
        req: CallRequest,
        block: Option<BlockIdVariant>,
    ) -> RpcResult<Bytes> {
        if !self.allow_authorized_call(&credentials, &req) {
            return Err(ErrorObject::from(ErrorCode::ServerError(403)));
        }
        let client = self.create_client();
        client
            .call(req, block)
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

    async fn chain_id(&self) -> RpcResult<U64> {
        let client = self.create_client();
        client
            .chain_id()
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    async fn call(&self, req: CallRequest, block: Option<BlockIdVariant>) -> RpcResult<Bytes> {
        if !self.allow_unauthorized_call(&req) {
            return Err(ErrorObject::from(ErrorCode::ServerError(403)));
        }
        let client = self.create_client();
        client
            .call(req, block)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    async fn estimate_gas(&self, req: CallRequest, block: Option<BlockNumber>) -> RpcResult<U256> {
        if !self.allow_unauthorized_call(&req) {
            return Err(ErrorObject::from(ErrorCode::ServerError(403)));
        }
        let client = self.create_client();
        client
            .estimate_gas(req, block)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    async fn gas_price(&self) -> RpcResult<U256> {
        let client = self.create_client();
        client
            .gas_price()
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    async fn new_filter(&self, _filter: Filter) -> RpcResult<U256> {
        Err(ErrorObject::from(ErrorCode::ServerError(403)))
    }

    async fn new_block_filter(&self) -> RpcResult<U256> {
        Err(ErrorObject::from(ErrorCode::ServerError(403)))
    }

    async fn uninstall_filter(&self, _idx: U256) -> RpcResult<bool> {
        Err(ErrorObject::from(ErrorCode::ServerError(403)))
    }

    async fn new_pending_transaction_filter(&self) -> RpcResult<U256> {
        Err(ErrorObject::from(ErrorCode::ServerError(403)))
    }

    async fn get_logs(&self, _filter: Filter) -> RpcResult<Vec<Log>> {
        Err(ErrorObject::from(ErrorCode::ServerError(403)))
    }

    async fn get_filter_logs(&self, _filter_index: U256) -> RpcResult<FilterChanges> {
        Err(ErrorObject::from(ErrorCode::ServerError(403)))
    }
    async fn get_filter_changes(&self, _filter_index: U256) -> RpcResult<FilterChanges> {
        Err(ErrorObject::from(ErrorCode::ServerError(403)))
    }

    async fn get_balance(
        &self,
        _address: Address,
        _block: Option<BlockIdVariant>,
    ) -> RpcResult<U256> {
        Err(ErrorObject::from(ErrorCode::ServerError(403)))
    }

    async fn get_block_by_number(
        &self,
        block_number: BlockNumber,
        full_transactions: bool,
    ) -> RpcResult<Option<Block<TransactionVariant>>> {
        if full_transactions {
            return Err(ErrorObject::from(ErrorCode::ServerError(403)));
        }
        let client = self.create_client();
        let mut result = client
            .get_block_by_number(block_number, full_transactions)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned());

        // Filter out transactions
        if let Ok(result_details) = &mut result {
            if let Some(block) = result_details {
                block.transactions.clear();
            }
        }
        result
    }

    async fn get_block_by_hash(
        &self,
        hash: H256,
        full_transactions: bool,
    ) -> RpcResult<Option<Block<TransactionVariant>>> {
        if full_transactions {
            return Err(ErrorObject::from(ErrorCode::ServerError(403)));
        }
        let client = self.create_client();
        let mut result = client
            .get_block_by_hash(hash, full_transactions)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned());

        // Filter out transactions
        if let Ok(result_details) = &mut result {
            if let Some(block) = result_details {
                block.transactions.clear();
            }
        }
        result
    }

    async fn get_block_transaction_count_by_number(
        &self,
        block_number: BlockNumber,
    ) -> RpcResult<Option<U256>> {
        let client = self.create_client();
        client
            .get_block_transaction_count_by_number(block_number)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    async fn get_block_receipts(
        &self,
        block_id: BlockId,
    ) -> RpcResult<Option<Vec<TransactionReceipt>>> {
        Err(ErrorObject::from(ErrorCode::ServerError(403)))
    }

    async fn get_block_transaction_count_by_hash(
        &self,
        block_hash: H256,
    ) -> RpcResult<Option<U256>> {
        let client = self.create_client();
        client
            .get_block_transaction_count_by_hash(block_hash)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    async fn get_code(&self, address: Address, block: Option<BlockIdVariant>) -> RpcResult<Bytes> {
        let client = self.create_client();
        client
            .get_code(address, block)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    // Storage is hard one.. for now not allowed.
    async fn get_storage_at(
        &self,
        _address: Address,
        _idx: U256,
        _block: Option<BlockIdVariant>,
    ) -> RpcResult<H256> {
        Err(ErrorObject::from(ErrorCode::ServerError(403)))
    }

    // Transaction count is ok.
    async fn get_transaction_count(
        &self,
        address: Address,
        block: Option<BlockIdVariant>,
    ) -> RpcResult<U256> {
        let client = self.create_client();
        client
            .get_transaction_count(address, block)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    async fn get_transaction_by_hash(&self, hash: H256) -> RpcResult<Option<Transaction>> {
        let client = self.create_client();
        client
            .get_transaction_by_hash(hash)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    // Listing block transactions is not allowed.
    async fn get_transaction_by_block_hash_and_index(
        &self,
        _block_hash: H256,
        _index: Index,
    ) -> RpcResult<Option<Transaction>> {
        Err(ErrorObject::from(ErrorCode::ServerError(403)))
    }

    // Listing block transactions is not allowed
    async fn get_transaction_by_block_number_and_index(
        &self,
        _block_number: BlockNumber,
        _index: Index,
    ) -> RpcResult<Option<Transaction>> {
        Err(ErrorObject::from(ErrorCode::ServerError(403)))
    }

    // Tx receipt is allowed - if you know the hash.
    async fn get_transaction_receipt(&self, hash: H256) -> RpcResult<Option<TransactionReceipt>> {
        let client = self.create_client();
        client
            .get_transaction_receipt(hash)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    async fn protocol_version(&self) -> RpcResult<String> {
        let client = self.create_client();
        client
            .protocol_version()
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    // Sending raw transactions is allowed.
    async fn send_raw_transaction(&self, tx_bytes: Bytes) -> RpcResult<H256> {
        let client = self.create_client();
        client
            .send_raw_transaction(tx_bytes)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    async fn syncing(&self) -> RpcResult<SyncState> {
        let client = self.create_client();
        client
            .syncing()
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    async fn accounts(&self) -> RpcResult<Vec<Address>> {
        Err(ErrorObject::from(ErrorCode::ServerError(403)))
    }

    async fn coinbase(&self) -> RpcResult<Address> {
        let client = self.create_client();
        client
            .coinbase()
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    async fn compilers(&self) -> RpcResult<Vec<String>> {
        let client = self.create_client();
        client
            .compilers()
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    async fn hashrate(&self) -> RpcResult<U256> {
        let client = self.create_client();
        client
            .hashrate()
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    async fn get_uncle_count_by_block_hash(&self, hash: H256) -> RpcResult<Option<U256>> {
        let client = self.create_client();
        client
            .get_uncle_count_by_block_hash(hash)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    async fn get_uncle_count_by_block_number(
        &self,
        number: BlockNumber,
    ) -> RpcResult<Option<U256>> {
        let client = self.create_client();
        client
            .get_uncle_count_by_block_number(number)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }

    async fn mining(&self) -> RpcResult<bool> {
        Err(ErrorObject::from(ErrorCode::ServerError(403)))
    }

    async fn fee_history(
        &self,
        block_count: U64,
        newest_block: BlockNumber,
        reward_percentiles: Vec<f32>,
    ) -> RpcResult<FeeHistory> {
        let client = self.create_client();
        client
            .fee_history(block_count, newest_block, reward_percentiles)
            .await
            .map_err(|_| ErrorObject::from(ErrorCode::InternalError).into_owned())
    }
}
