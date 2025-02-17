use cosmwasm_std::{entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Coin, WasmMsg};
use crate::msg::{InstantiateMsg, ExecuteMsg};
use crate::state::CW20_FACTORY_CODE_ID;
use crate::curve::{calculate_buy_price, calculate_sell_price};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    CW20_FACTORY_CODE_ID.save(deps.storage, &msg.cw20_factory_code_id)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateCurve { name, symbol, supply } => create_curve(deps, env, info, name, symbol, supply),
        ExecuteMsg::Buy { curve_id, amount } => buy_tokens(deps, env, info, curve_id, amount),
        ExecuteMsg::Sell { curve_id, amount } => sell_tokens(deps, env, info, curve_id, amount),
    }
}

fn create_curve(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    symbol: String,
    supply: Uint128,
) -> StdResult<Response> {
    let token_sub_msg = WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),
        code_id: CW20_FACTORY_CODE_ID.load(deps.storage)?,
        msg: to_binary(&serde_json::json!({
            "name": name,
            "symbol": symbol,
            "decimals": 6,
            "initial_balances": [{"address": env.contract.address.to_string(), "amount": supply.to_string()}],
            "mint": {"minter": env.contract.address.to_string(), "cap": supply.to_string()}
        }))?,
        funds: vec![],
        label: format!("{} token", name),
    };
    Ok(Response::new().add_message(token_sub_msg).add_attribute("action", "create_curve"))
}

fn buy_tokens(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    curve_id: String,
    amount: Uint128,
) -> StdResult<Response> {
    let price = calculate_buy_price(amount.u128());
    Ok(Response::new().add_attribute("action", "buy").add_attribute("price", price.to_string()))
}

fn sell_tokens(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    curve_id: String,
    amount: Uint128,
) -> StdResult<Response> {
    let price = calculate_sell_price(amount.u128());
    Ok(Response::new().add_attribute("action", "sell").add_attribute("price", price.to_string()))
}
