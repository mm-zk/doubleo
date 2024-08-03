use std::collections::HashSet;

use std::str::FromStr;
use zksync_types::{transaction_request::CallRequest, Address};

#[derive(Clone)]
pub struct ContractWhitelist {
    whitelisted_contracts: HashSet<Address>,
}

impl ContractWhitelist {
    pub fn init(contract_whitelist: Vec<String>) -> Self {
        ContractWhitelist {
            whitelisted_contracts: HashSet::from_iter(
                contract_whitelist
                    .iter()
                    .map(|a| Address::from_str(a).unwrap()),
            ),
        }
    }
    pub fn allow_unauthorized_call(&self, req: &CallRequest) -> bool {
        if let Some(to) = req.to {
            // Contract must be on the whitelist
            self.whitelisted_contracts.contains(&to)
        } else {
            // Calls to 'null' address (eth contract creation) not allowed.
            false
        }
    }
}
