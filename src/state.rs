use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin_address: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    pub question: String,
    pub yes_votes: u64,
    pub no_votes: u64,
}

pub const CONFIG: Item<Config> = Item::new("config"); // This is stored on-chain

// String->Poll
// "Do you love SPARK IBC?" -> Poll {
//          question:"Do you love spark ibc?",
//          yes_vote:100,
//          no_vote:50
// }
pub const POLLS: Map<String, Poll> = Map::new("pools");
