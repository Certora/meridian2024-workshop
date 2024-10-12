#![no_std]

use cvt_soroban_macros::{declare_rules, rule};
use soroban_sdk::{Address, Env};

use crate::Token;
use cvt::*;
use cvt_soroban::{cvt_cex_print_i64, CVT_calltrace_print_c_i64, is_auth};

// Sunbeam specs

#[rule]
fn sanity(e: Env, addr: Address) {
    let balance = Token::balance(&e, addr);
    cvt::satisfy!(true);
}

#[rule]
fn init_balance(e: Env, addr: Address) {
    // Your property here
}


#[rule]
fn transfer_is_correct(e: Env, to: Address, from: Address, amount: i64) {
    // Your property here
}

#[rule]
fn transfer_no_effect_on_other(e: Env, amount: i64, from: Address, to: Address, other: Address) {
    // Your property here
}


#[rule]
fn transfer_fails_if_low_balance(e: Env, to: Address, from: Address, amount: i64) {
    // Your property here
}