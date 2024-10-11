use soroban_sdk::{Address, Env};

use crate::Token;
use cvt::*;
use cvt_soroban::CVT_calltrace_print_c_i64;

// Sunbeam specs
// #[rule]
#[inline(never)]
#[no_mangle]
fn init_balance_zero(e: Env, addr: Address) {
    require!(!e.storage().persistent().has(&addr), "address exists");
    let balance = Token::balance(&e, addr);
    cvt::assert!(balance == 0);
}

// #[rule]
#[inline(never)]
#[no_mangle]
fn transfer_is_correct(e: Env, to: Address, from: Address, amount: i64) {
    require!(
        e.storage().persistent().has(&from) && e.storage().persistent().has(&to) && to != from,
        "addresses exist and different"
    );
    let balance_from_before = Token::balance(&e, from.clone());
    let balance_to_before = Token::balance(&e, to.clone());
    Token::transfer(&e, from.clone(), to.clone(), amount);
    let balance_from_after = Token::balance(&e, from.clone());
    let balance_to_after = Token::balance(&e, to.clone());
    cvt_soroban::cvt_cex_print_i64!(balance_from_before);
    cvt_soroban::cvt_cex_print_i64!(balance_to_before);
    cvt_soroban::cvt_cex_print_i64!(balance_from_after);
    cvt_soroban::cvt_cex_print_i64!(balance_to_after);
    cvt::assert!(
        (balance_to_after == balance_to_before + amount)
            && (balance_from_after == balance_from_before - amount)
    );
}

// #[rule]
#[inline(never)]
#[no_mangle]
fn transfer_fails_if_low_balance(e: Env, to: Address, from: Address, amount: i64) {
    require!(
        e.storage().persistent().has(&from)
            && e.storage().persistent().has(&to)
            && to != from
            && Token::balance(&e, from.clone()) < amount,
        "addresses exist and different, and balance < amount"
    );
    Token::transfer(&e, from.clone(), to.clone(), amount);
    cvt::assert!(false); // should not reach and therefore rule must pass
}

// #[rule]
#[inline(never)]
#[no_mangle]
fn transfer_no_effect_on_other(e: Env, amount: i64, from: Address, to: Address, other: Address) {
    require!(to != other && from != other, "addresses are all different");
    let balance_other_before = Token::balance(&e, other.clone());
    Token::transfer(&e, from.clone(), to.clone(), amount);
    let balance_other_after = Token::balance(&e, other.clone());
    cvt::assert!(balance_other_after == balance_other_before);
}

// declare_rules! {transfer_no_effect_on_other, transfer_fails_if_low_balance, transfer_is_correct, init_balance_zero
// }
