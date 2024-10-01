use cvt::CVT_assume;
use soroban_sdk::{Address, Env};

use crate::Token;

// Sunbeam specs
#[no_mangle]
#[inline(never)]
fn sunbeam_init_balance_zero(e: Env, addr: Address) {
    CVT_assume(!e.storage().persistent().has(&addr));
    let balance = Token::balance(&e, addr);
    cvt::assert!(balance == 0);
}

#[no_mangle]
#[inline(never)]
fn sunbeam_transfer_is_correct(e: Env, to: Address, from: Address, amount: i64) {
    CVT_assume(
        e.storage().persistent().has(&from) && e.storage().persistent().has(&to) && to != from,
    );
    let balance_from_before = Token::balance(&e, from.clone());
    let balance_to_before = Token::balance(&e, to.clone());
    Token::transfer(&e, from.clone(), to.clone(), amount);
    let balance_from_after = Token::balance(&e, from.clone());
    let balance_to_after = Token::balance(&e, to.clone());
    cvt::assert!(
        (balance_to_after == balance_to_before + amount)
            && (balance_from_after == balance_from_before - amount)
    );
}

#[no_mangle]
#[inline(never)]
fn sunbeam_transfer_fails_if_low_balance(e: Env, to: Address, from: Address, amount: i64) {
    CVT_assume(
        e.storage().persistent().has(&from)
            && e.storage().persistent().has(&to)
            && to != from
            && Token::balance(&e, from.clone()) < amount,
    );
    Token::transfer(&e, from.clone(), to.clone(), amount);
    cvt::assert!(false); // should not reach and therefore rule must pass
}