## Mutant 1.

The bug is that the value read when the `addr` key is not present in storage is `1` but it should be `0`.
```
pub fn read_balance(e: &Env, addr: &Address) -> i64 {
    e.storage().persistent().get(&addr).unwrap_or(1)
}
```

## Mutant 2.

The bug is that `transfer` calls `spend_balance` twice instead of calling it once followed by a `receive_balance`.

```
pub fn transfer(e: &Env, from: Address, to: Address, amount: i64) {
    from.require_auth();
    check_nonnegative_amount(amount);
    spend_balance(&e, from.clone(), amount);
    spend_balance(&e, to.clone(), amount);
}
```

## Mutant 3.
The bug is that `transfer` calls `spend_balance(&e, from.clone(), amount + 1);`
instead of `spend_balance(&e, from.clone(), amount);`.

```
pub fn transfer(e: &Env, from: Address, to: Address, amount: i64) {
    from.require_auth();
    check_nonnegative_amount(amount);
    spend_balance(&e, from.clone(), amount + 1);
    receive_balance(&e, to.clone(), amount);
}
```