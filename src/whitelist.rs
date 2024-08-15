use std::collections::HashSet;
use std::{collections::HashMap, ops::Add};

use std::str::FromStr;
use zksync_types::{protocol_upgrade::Call, transaction_request::CallRequest, Address};

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

    fn get_selector(req: &CallRequest) -> Option<String> {
        req.data.as_ref().map(|input| hex::encode(&input.0[..4]))
    }

    fn get_first_user(req: &CallRequest) -> Option<Address> {
        if req.data.is_none() {
            return None;
        }
        let input = req.data.as_ref().unwrap();

        for i in 4..16 {
            if input.0[i] != 0 {
                return None;
            }
        }

        let first_param = &input.0[16..36];

        Some(Address::from_slice(first_param))
    }

    pub fn allow_unauthorized_call(&self, req: &CallRequest) -> bool {
        if let Some(to) = req.to {
            // Contract must be on the whitelist
            match self.whitelisted_contracts.get(&to) {
                Some(whitelist_entry) => {
                    if whitelist_entry.fully_whitelisted {
                        true
                    } else {
                        let selector = ContractWhitelist::get_selector(req);
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

    pub fn allow_authorized_call(&self, req: &CallRequest, users: &HashSet<Address>) -> bool {
        if self.allow_unauthorized_call(req) {
            return true;
        }

        if let Some(req_user) = ContractWhitelist::get_first_user(req) {
            if !users.contains(&req_user) {
                println!("User {} not in hashset", req_user);
                return false;
            }

            if let Some(to) = req.to {
                if let Some(whitelist) = self.whitelisted_contracts.get(&to) {
                    if let Some(selector) = ContractWhitelist::get_selector(req) {
                        if let Some(methods) = &whitelist.methods {
                            if let Some(authorizations) = &methods.requires_authorization {
                                // We need more verification here..
                                return authorizations.contains(&selector);
                            }
                        }
                    }
                }
            }
        }
        return false;
    }
}
