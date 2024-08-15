use std::collections::HashMap;

use std::str::FromStr;
use zksync_types::{transaction_request::CallRequest, Address};

use crate::WhitelistEntry;

#[derive(Clone)]
pub struct ContractWhitelist {
    whitelisted_contracts: HashMap<Address, WhitelistEntry>,
}

impl ContractWhitelist {
    pub fn init(contract_whitelist: Vec<WhitelistEntry>) -> Self {
        ContractWhitelist {
            whitelisted_contracts: HashMap::from_iter(
                contract_whitelist
                    .iter()
                    .map(|a| (Address::from_str(&a.address).unwrap(), a.clone())),
            ),
        }
    }
    pub fn allow_unauthorized_call(&self, req: &CallRequest) -> bool {
        if let Some(to) = req.to {
            // Contract must be on the whitelist
            match self.whitelisted_contracts.get(&to) {
                Some(whitelist_entry) => {
                    if whitelist_entry.fully_whitelisted {
                        true
                    } else {
                        let selector = req.data.as_ref().map(|input| hex::encode(&input.0[..4]));
                        match selector {
                            Some(selector) => {
                                println!("looking at selector {}", selector);
                                match &whitelist_entry.methods {
                                    Some(methods) => match &methods.unrestricted {
                                        Some(unrestricted) => unrestricted.contains(&selector),
                                        None => false,
                                    },
                                    None => false,
                                }
                            }
                            None => false,
                        }
                    }
                }
                None => false,
            }
        } else {
            // Calls to 'null' address (eth contract creation) not allowed.
            false
        }
    }
}
