#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String, Symbol};

// Data keys
const NAME: Symbol = symbol_short!("name");
const SYMBOL: Symbol = symbol_short!("symbol");
const DECIMALS: Symbol = symbol_short!("decimals");
const TOTAL_SUPPLY: Symbol = symbol_short!("supply");
const BALANCE: Symbol = symbol_short!("balance");
const ALLOWANCE: Symbol = symbol_short!("allowance");

// Contract structure
#[contract]
pub struct Token;

// Contract implementation
#[contractimpl]
impl Token {
    // Helper to decrease balance
    fn spend_balance(env: &Env, owner: &Address, amount: u128) {
        let current_balance = Self::balance_of(env.clone(), owner.clone());

        if current_balance < amount {
            panic!("Insufficient balance");
        }

        let bal: u128 = current_balance - amount;
        env.storage().persistent().set(&(BALANCE, owner), &bal);
    }

    // Helper to increase balance
    fn increase_balance(env: &Env, owner: &Address, amount: u128) {
        let current_balance = Self::balance_of(env.clone(), owner.clone());

        let bal: u128 = current_balance + amount;
        env.storage().persistent().set(&(BALANCE, owner), &bal);
    }

    // Initialize the token with name, symbol, decimals, and initial supply
    pub fn initialize(
        env: Env,
        name: String,
        symbol: Symbol,
        decimals: u128,
        initial_supply: u128,
        owner: Address,
    ) -> bool {
        // Ensure initialization only happens once
        if env.storage().persistent().has(&NAME) {
            panic!("Token is already initialized");
        }

        env.storage().persistent().set(&NAME, &name);
        env.storage().persistent().set(&SYMBOL, &symbol);
        env.storage().persistent().set(&DECIMALS, &decimals);
        env.storage()
            .persistent()
            .set(&TOTAL_SUPPLY, &initial_supply);

        // Assign the entire initial supply to the owner's balance
        env.storage()
            .persistent()
            .set(&(BALANCE, owner.clone()), &initial_supply);

        true
    }

    // Get the token name
    pub fn name(env: Env) -> String {
        env.storage()
            .persistent()
            .get(&NAME)
            .unwrap_or_else(|| String::from_str(&env, "undefined"))
    }

    // Get the token symbol
    pub fn symbol(env: Env) -> Symbol {
        env.storage()
            .persistent()
            .get(&SYMBOL)
            .unwrap_or(symbol_short!("undefined"))
    }

    // Get the token decimals
    pub fn decimals(env: Env) -> u128 {
        env.storage().persistent().get(&DECIMALS).unwrap_or(0)
    }

    // Get the total supply of the token
    pub fn total_supply(env: Env) -> u128 {
        env.storage().persistent().get(&TOTAL_SUPPLY).unwrap_or(0)
    }

    // Get the balance of a given address
    pub fn balance_of(env: Env, owner: Address) -> u128 {
        env.storage()
            .persistent()
            .get(&(BALANCE, owner))
            .unwrap_or(0)
    }

    // Transfer tokens from the sender to a recipient
    pub fn transfer(env: Env, from: Address, to: Address, amount: u128) -> bool {
        from.require_auth();

        Self::spend_balance(&env, &from.clone(), amount);
        Self::increase_balance(&env, &to, amount);

        // Publish an event about the transfer
        env.events()
            .publish((symbol_short!("trnsf"), from, to), amount);

        true
    }

    // Approve an allowance for a spender
    pub fn approve(env: Env, owner: Address, spender: Address, amount: u128) -> bool {
        owner.require_auth();

        env.storage()
            .persistent()
            .set(&(ALLOWANCE, owner.clone(), spender.clone()), &amount);

        // Publish an event about the approval
        env.events()
            .publish((symbol_short!("apprv"), owner, spender), amount);

        true
    }

    // Get the allowance between an owner and a spender
    pub fn allowance(env: Env, owner: Address, spender: Address) -> u128 {
        env.storage()
            .persistent()
            .get(&(ALLOWANCE, owner, spender))
            .unwrap_or(0)
    }

    // Transfer tokens on behalf of an owner
    pub fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: u128,
    ) -> bool {
        spender.require_auth();

        let current_allowance = Self::allowance(env.clone(), from.clone(), spender.clone());

        if current_allowance < amount {
            panic!("Allowance exceeded");
        }

        let bal: u128 = current_allowance - amount;

        // Deduct the allowance and transfer tokens
        env.storage()
            .persistent()
            .set(&(ALLOWANCE, from.clone(), spender.clone()), &bal);

        Self::spend_balance(&env, &from, amount);
        Self::increase_balance(&env, &to, amount);

        // Publish an event about the transfer
        env.events()
            .publish((symbol_short!("trnsf_frm"), spender, from, to), amount);

        true
    }

    // View: get current contract address
    pub fn get_contract_address(env: &Env) -> Address {
        env.current_contract_address()
    }
}
