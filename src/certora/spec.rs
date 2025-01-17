#![no_std]

use certora_soroban_macros::{declare_rules, rule};
use soroban_sdk::{Address, Env};

use crate::Token;
use certora::*;
use certora_soroban::{certora_print_i64, CERTORA_calltrace_print_c_i64, is_auth};

// Sunbeam specs

// Exercise 0
#[rule]
fn sanity(e: Env, addr: Address) {
    let balance = Token::balance(&e, addr);
    certora::satisfy!(true);
}

// Exercise 1
#[rule]
fn init_balance(e: Env, addr: Address) {
    // Your property here
}

// Exercise 2
#[rule]
fn transfer_is_correct(e: Env, to: Address, from: Address, amount: i64) {
    // Your property here
}

// Exercise 2
#[rule]
fn transfer_no_effect_on_other(e: Env, amount: i64, from: Address, to: Address, other: Address) {
    // Your property here
}


// Exercise 3
#[rule]
fn transfer_fails_if_low_balance(e: Env, to: Address, from: Address, amount: i64) {
    // Your property here
}

// Exercise 4
// Add your rules for `mint` and `burn` here!
