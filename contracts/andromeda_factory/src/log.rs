use std::fmt;

use cosmwasm_std::HumanAddr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintLog {
    pub token_id: i64,
    pub owner: HumanAddr,
}

impl fmt::Display for MintLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{token_id: {}, owner: {}}}", self.token_id, self.owner)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TransferLog {
    pub token_id: i64,
    pub from: HumanAddr,
    pub to: HumanAddr,
}
impl fmt::Display for TransferLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{token_id: {}, from: {}, to: {}}}", self.token_id, self.from, self.to)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BurnLog {
    pub token_id: i64,
    pub burner: HumanAddr,
}

impl fmt::Display for BurnLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{token_id: {}, burner: {}}}", self.token_id, self.burner)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ArchiveLog {
    pub token_id: i64,
    pub archiver: HumanAddr,
}

impl fmt::Display for ArchiveLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{token_id: {}, archiver: {}}}", self.token_id, self.archiver)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WhitelistLog {
    pub address: HumanAddr,
    pub whitelister: HumanAddr,
    pub whitelisted: bool,
}

impl fmt::Display for WhitelistLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{address: {}, whitelister: {}, whitelisted: {}}}", self.address, self.whitelister, self.whitelisted)
    }
}