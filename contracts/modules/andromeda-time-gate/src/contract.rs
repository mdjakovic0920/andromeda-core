#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, ensure, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, Storage,
};

use crate::state::{CYCLE_START_TIME, GATE_ADDRESSES, TIME_INTERVAL};
use andromeda_modules::time_gate::CycleStartTime;
use andromeda_modules::time_gate::{ExecuteMsg, InstantiateMsg, QueryMsg};
use andromeda_std::{
    ado_base::{InstantiateMsg as BaseInstantiateMsg, MigrateMsg},
    ado_contract::ADOContract,
    amp::AndrAddr,
    common::{actions::call_action, context::ExecuteContext, encode_binary},
    error::ContractError,
};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:andromeda-time-gate";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const DEFAULT_TIME_INTERVAL: u64 = 3600;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let resp = ADOContract::default().instantiate(
        deps.storage,
        env,
        deps.api,
        &deps.querier,
        info,
        BaseInstantiateMsg {
            ado_type: CONTRACT_NAME.to_string(),
            ado_version: CONTRACT_VERSION.to_string(),
            kernel_address: msg.kernel_address,
            owner: msg.owner,
        },
    )?;

    let cycle_start_time = msg.cycle_start_time;
    cycle_start_time.validate()?;

    let time_interval_seconds = msg.time_interval.unwrap_or(DEFAULT_TIME_INTERVAL);

    GATE_ADDRESSES.save(deps.storage, &msg.gate_addresses)?;
    CYCLE_START_TIME.save(deps.storage, &cycle_start_time)?;
    TIME_INTERVAL.save(deps.storage, &time_interval_seconds)?;

    Ok(resp)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let ctx = ExecuteContext::new(deps, info, env);
    match msg {
        ExecuteMsg::AMPReceive(pkt) => {
            ADOContract::default().execute_amp_receive(ctx, pkt, handle_execute)
        }
        _ => handle_execute(ctx, msg),
    }
}

fn handle_execute(mut ctx: ExecuteContext, msg: ExecuteMsg) -> Result<Response, ContractError> {
    let action = msg.as_ref().to_string();

    let action_response = call_action(
        &mut ctx.deps,
        &ctx.info,
        &ctx.env,
        &ctx.amp_ctx,
        msg.as_ref(),
    )?;

    let res = match msg {
        ExecuteMsg::UpdateCycleStartTime { cycle_start_time } => {
            execute_update_cycle_start_time(ctx, cycle_start_time, action)
        }
        ExecuteMsg::UpdateGateAddresses { new_gate_addresses } => {
            execute_update_gate_addressess(ctx, new_gate_addresses, action)
        }
        ExecuteMsg::UpdateTimeInterval { time_interval } => {
            execute_update_time_interval(ctx, time_interval, action)
        }
        _ => ADOContract::default().execute(ctx, msg),
    }?;

    Ok(res
        .add_submessages(action_response.messages)
        .add_attributes(action_response.attributes)
        .add_events(action_response.events))
}

fn execute_update_cycle_start_time(
    ctx: ExecuteContext,
    cycle_start_time: CycleStartTime,
    action: String,
) -> Result<Response, ContractError> {
    let ExecuteContext { deps, info, .. } = ctx;

    ensure!(
        ADOContract::default().is_contract_owner(deps.storage, info.sender.as_str())?,
        ContractError::Unauthorized {}
    );

    let old_cycle_start_time = CYCLE_START_TIME.load(deps.storage)?;

    ensure!(
        old_cycle_start_time != cycle_start_time,
        ContractError::InvalidParameter {
            error: Some("Same as an existed cycle start time".to_string())
        }
    );

    cycle_start_time.validate()?;

    CYCLE_START_TIME.save(deps.storage, &cycle_start_time)?;

    Ok(Response::new().add_attributes(vec![attr("action", action), attr("sender", info.sender)]))
}

fn execute_update_gate_addressess(
    ctx: ExecuteContext,
    new_gate_addresses: Vec<AndrAddr>,
    action: String,
) -> Result<Response, ContractError> {
    let ExecuteContext { deps, info, .. } = ctx;

    ensure!(
        ADOContract::default().is_contract_owner(deps.storage, info.sender.as_str())?,
        ContractError::Unauthorized {}
    );

    let old_gate_addresses = GATE_ADDRESSES.load(deps.storage)?;

    ensure!(
        old_gate_addresses != new_gate_addresses,
        ContractError::InvalidParameter {
            error: Some("Same as existed gate addresses".to_string())
        }
    );

    GATE_ADDRESSES.save(deps.storage, &new_gate_addresses)?;

    Ok(Response::new().add_attributes(vec![attr("action", action), attr("sender", info.sender)]))
}

fn execute_update_time_interval(
    ctx: ExecuteContext,
    time_interval: u64,
    action: String,
) -> Result<Response, ContractError> {
    let ExecuteContext { deps, info, .. } = ctx;

    ensure!(
        ADOContract::default().is_contract_owner(deps.storage, info.sender.as_str())?,
        ContractError::Unauthorized {}
    );

    let old_time_interval = TIME_INTERVAL.load(deps.storage)?;

    ensure!(
        old_time_interval != time_interval,
        ContractError::InvalidParameter {
            error: Some("Same as an existed time interval".to_string())
        }
    );

    TIME_INTERVAL.save(deps.storage, &time_interval)?;

    Ok(Response::new().add_attributes(vec![attr("action", action), attr("sender", info.sender)]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::GetGateAddresses {} => encode_binary(&get_gate_addresses(deps.storage)?),
        QueryMsg::GetCycleStartTime {} => encode_binary(&get_cycle_start_time(deps.storage)?),
        QueryMsg::GetCurrentAdoPath {} => encode_binary(&get_current_ado_path(deps, env)?),
        QueryMsg::GetTimeInterval {} => encode_binary(&get_time_interval(deps.storage)?),
        _ => ADOContract::default().query(deps, env, msg),
    }
}

pub fn get_gate_addresses(storage: &dyn Storage) -> Result<Vec<AndrAddr>, ContractError> {
    let gate_addresses = GATE_ADDRESSES.load(storage)?;
    Ok(gate_addresses)
}

pub fn get_cycle_start_time(storage: &dyn Storage) -> Result<CycleStartTime, ContractError> {
    let cycle_start_time = CYCLE_START_TIME.load(storage)?;
    Ok(cycle_start_time)
}

pub fn get_time_interval(storage: &dyn Storage) -> Result<String, ContractError> {
    let time_interval = TIME_INTERVAL.load(storage)?.to_string();
    Ok(time_interval)
}

pub fn get_current_ado_path(deps: Deps, env: Env) -> Result<Addr, ContractError> {
    let storage = deps.storage;
    let cycle_start_time = CYCLE_START_TIME.load(storage)?;
    let gate_addresses = GATE_ADDRESSES.load(storage)?;
    let time_interval = TIME_INTERVAL.load(storage)?;

    let CycleStartTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
    } = cycle_start_time;

    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    let time = NaiveTime::from_hms_nano_opt(hour, minute, second, 0).unwrap();
    let datetime = NaiveDateTime::new(date, time);

    let duration = datetime.signed_duration_since(NaiveDateTime::new(
        NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
        NaiveTime::from_hms_nano_opt(0, 0, 0, 0).unwrap(),
    ));

    let cycle_start_nanos = duration.num_nanoseconds().unwrap() as u64;

    let current_time_nanos = env.block.time.nanos();

    ensure!(
        current_time_nanos >= cycle_start_nanos,
        ContractError::CustomError {
            msg: "Cycle is not started yet".to_string()
        }
    );

    let time_interval_nanos = time_interval.checked_mul(1_000_000_000).unwrap();
    let gate_length = gate_addresses.len() as u64;
    let time_delta = current_time_nanos.checked_sub(cycle_start_nanos).unwrap();
    let index = time_delta
        .checked_div(time_interval_nanos)
        .unwrap()
        .checked_rem(gate_length)
        .unwrap() as usize;
    let current_ado_path = &gate_addresses[index];
    let result = current_ado_path.get_raw_address(&deps)?;

    Ok(result)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    ADOContract::default().migrate(deps, CONTRACT_NAME, CONTRACT_VERSION)
}