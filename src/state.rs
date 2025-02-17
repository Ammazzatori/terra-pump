use cosmwasm_std::Storage;
use cw_storage_plus::Item;

pub const CW20_FACTORY_CODE_ID: Item<u64> = Item::new("cw20_factory_code_id");