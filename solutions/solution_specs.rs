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
    // Reachability check: is this satisfy reachable?
    certora::satisfy!(true);
}

// Exercise 1
#[rule]
fn init_balance(e: Env, addr: Address) {
    // precondition macro
    certora::require!(!e.storage().persistent().has(&addr), "address must not exists");
    let balance = Token::balance(&e, addr);
    // use this macro to see additional information in the calltrace
    certora_print_i64!("value of balance is:", balance); 
    // postcondition macro
    certora::assert!(balance == 0);
}


// Exercise 2
#[rule]
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
    certora::assert!(
        (balance_to_after == balance_to_before + amount)
            && (balance_from_after == balance_from_before - amount)
    );
}

// Exercise 2
#[rule]
fn transfer_no_effect_on_other(e: Env, amount: i64, from: Address, to: Address, other: Address) {
    require!(to != other && from != other, "addresses are all different");
    let balance_other_before = Token::balance(&e, other.clone());
    Token::transfer(&e, from.clone(), to.clone(), amount);
    let balance_other_after = Token::balance(&e, other.clone());
    certora::assert!(balance_other_after == balance_other_before);
}

// Exercise 3
#[rule]
fn transfer_fails_if_low_balance(e: Env, to: Address, from: Address, amount: i64) {
    require!(
        e.storage().persistent().has(&from)
            && e.storage().persistent().has(&to)
            && to != from
            && Token::balance(&e, from.clone()) < amount,
        "addresses exist and different, and balance < amount"
    );
    Token::transfer(&e, from.clone(), to.clone(), amount);
    // should not reach and therefore rule must pass
    certora::assert!(false);
}

// Exercise 4
#[rule]
fn burn_is_correct(e: Env, from: Address, amount: i64) {
    let balance_before = Token::balance(&e, from.clone());
    Token::burn(&e, from.clone(), amount);
    let balance_after = Token::balance(&e, from.clone());
    certora::assert!(balance_after == balance_before - amount);
}

// Exercise 4
#[rule]
fn mint_is_authorized(e: Env, to: Address, amount: i64) {
    let admin = e
            .storage()
            .persistent()
            .get::<_, Address>(&"ADMIN")
            .unwrap();
    require!(is_auth(admin), "admin is authorized");
    Token::mint(&e, to, amount);
    certora::satisfy!(true);
}

// Exercise 4
#[rule]
fn mint_not_authorized_fails(e: Env, to: Address, amount: i64) {
    let admin = e
            .storage()
            .persistent()
            .get::<_, Address>(&"ADMIN")
            .unwrap();
    require!(!is_auth(admin), "admin is not authorized");
    Token::mint(&e, to, amount);
    certora::assert!(false); // should pass
}

// Exercise 4
#[rule]
fn mint_is_correct(e: Env, to: Address, amount: i64) {
    let balance_before = Token::balance(&e, to.clone());
    Token::mint(&e, to.clone(), amount);
    let balance_after = Token::balance(&e, to.clone());
    certora::assert!(balance_after == balance_before + amount);
}