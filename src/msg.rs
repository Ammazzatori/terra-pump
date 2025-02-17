use cosmwasm_std::Uint128;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub cw20_factory_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateCurve { name: String, symbol: String, supply: Uint128 },
    Buy { curve_id: String, amount: Uint128 },
    Sell { curve_id: String, amount: Uint128 },
}