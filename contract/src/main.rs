#![no_std]
#![no_main]

mod detail;
mod address;
mod error;
extern crate alloc;
use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, Key, EntryPoint, EntryPoints, contracts::NamedKeys, CLType, CLValue, EntryPointAccess, EntryPointType, URef};

#[no_mangle]
pub extern "C" fn set_immediate_caller(){
    let immediate_caller: address::Address = address::Address::from(runtime::get_caller());
}

#[no_mangle]
pub extern "C" fn call(){
    let entry_points: EntryPoints = 
    {
        let mut entry_points = EntryPoints::new();
        entry_points.add_entry_point(EntryPoint::new(
            "stores_immediate_caller",
            vec![],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        ));
        entry_points
    };
    let mut named_keys = NamedKeys::new();
    let immediate_caller: URef = storage::new_uref("immediate_caller");
    named_keys.insert(String::from("immediate_caller"), immediate_caller.into());

    storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(String::from("Caller_Stack_Contract")),
        Some(String::from("Caller_Stack_URef")),
    );
}