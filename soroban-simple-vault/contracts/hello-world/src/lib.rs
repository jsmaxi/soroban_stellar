#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, log, symbol_short, Address, Env, Map, Symbol,
};

const VAULTDATA: Symbol = symbol_short!("vaultdata");

const INIT: Symbol = symbol_short!("init");

#[contract]
pub struct Vault;

#[derive(Clone, Debug)]
#[contracttype]
pub struct VaultData {
    pub total_assets: i128,           // Total underlying assets held in the vault
    pub total_shares: i128,           // Total shares issued by the vault
    pub balances: Map<Address, i128>, // Share balances for users
}

// Public Interface
pub trait VaultInterface {
    fn initialize(env: Env) -> bool;
    fn deposit(env: Env, user: Address, amount: i128) -> i128;
    fn withdraw(env: Env, user: Address, shares: i128) -> i128;
    fn total_assets(env: Env) -> i128;
    fn total_shares(env: Env) -> i128;
    fn balance_of(env: Env, user: Address) -> i128;
    fn price_per_share(env: Env) -> i128;
}

#[contractimpl]
impl VaultInterface for Vault {
    // Initialize the vault once with zero assets and shares
    fn initialize(env: Env) -> bool {
        if env.storage().instance().has(&INIT) {
            panic!("Already initialized");
        }

        log!(&env, "Initializing");

        let time = env.ledger().timestamp();
        log!(&env, "Time: {}", time);

        let vault_data = VaultData {
            total_assets: 0,
            total_shares: 0,
            balances: Map::new(&env),
        };

        log!(&env, "Total Assets: {}", vault_data.total_assets);
        log!(&env, "Total Shares: {}", vault_data.total_shares);
        log!(&env, "Balances: {:?}", vault_data.balances);

        env.storage().instance().set(&INIT, &true);

        log!(&env, "Storing...");

        env.storage().persistent().set(&VAULTDATA, &vault_data);

        log!(&env, "Initialized");

        true
    }

    // Deposit underlying assets and receive shares
    fn deposit(env: Env, user: Address, amount: i128) -> i128 {
        if !env.storage().instance().get(&INIT).unwrap_or(false) {
            panic!("Not initialized");
        }

        if amount <= 0 {
            panic!("Deposit amount must be greater than zero");
        }

        log!(&env, "Depositing");

        let mut data: VaultData = env
            .storage()
            .persistent()
            .get(&VAULTDATA)
            .unwrap_or(VaultData {
                total_assets: 0,
                total_shares: 0,
                balances: Map::new(&env),
            });

        let shares_to_mint = if data.total_assets > 0 && data.total_shares > 0 {
            (amount * data.total_shares) / data.total_assets
        } else {
            amount // Initial deposit: 1:1 ratio
        };

        log!(&env, "Shares to mint: {}", shares_to_mint);

        // Update the vault state
        data.total_assets += amount;
        data.total_shares += shares_to_mint;

        let user_balance = data.balances.get(user.clone()).unwrap_or(0);

        data.balances.set(user, user_balance + shares_to_mint);

        env.storage().persistent().set(&VAULTDATA, &data);

        log!(&env, "Deposited");

        shares_to_mint
    }

    // Redeem shares for underlying assets
    fn withdraw(env: Env, user: Address, shares: i128) -> i128 {
        if !env.storage().instance().get(&INIT).unwrap_or(false) {
            panic!("Not initialized");
        }

        if shares <= 0 {
            panic!("Withdraw amount must be greater than zero");
        }

        log!(&env, "Withdrawing");

        let mut data: VaultData = env
            .storage()
            .persistent()
            .get(&VAULTDATA)
            .unwrap_or(VaultData {
                total_assets: 0,
                total_shares: 0,
                balances: Map::new(&env),
            });

        let user_balance = data.balances.get(user.clone()).unwrap_or(0);

        if user_balance < shares {
            panic!("Insufficient share balance");
        }

        // Calculate assets to return
        let assets_to_return = (shares * data.total_assets) / data.total_shares;

        log!(&env, "Assets to return: {}", assets_to_return);

        // Update the vault state
        data.total_assets -= assets_to_return;
        data.total_shares -= shares;

        data.balances.set(user, user_balance - shares);

        env.storage().persistent().set(&VAULTDATA, &data);

        log!(&env, "Withdrawn");

        assets_to_return
    }

    // View: Total assets held by the vault
    fn total_assets(env: Env) -> i128 {
        let data: VaultData = env
            .storage()
            .persistent()
            .get(&VAULTDATA)
            .unwrap_or(VaultData {
                total_assets: 0,
                total_shares: 0,
                balances: Map::new(&env),
            });

        data.total_assets
    }

    // View: Total shares issued by the vault
    fn total_shares(env: Env) -> i128 {
        let data: VaultData = env
            .storage()
            .persistent()
            .get(&VAULTDATA)
            .unwrap_or(VaultData {
                total_assets: 0,
                total_shares: 0,
                balances: Map::new(&env),
            });

        data.total_shares
    }

    // View: Balance of shares for a specific user
    fn balance_of(env: Env, user: Address) -> i128 {
        let data: VaultData = env
            .storage()
            .persistent()
            .get(&VAULTDATA)
            .unwrap_or(VaultData {
                total_assets: 0,
                total_shares: 0,
                balances: Map::new(&env),
            });

        data.balances.get(user).unwrap_or(0)
    }

    // View: Value of 1 share in terms of the underlying asset
    fn price_per_share(env: Env) -> i128 {
        let data: VaultData = env
            .storage()
            .persistent()
            .get(&VAULTDATA)
            .unwrap_or(VaultData {
                total_assets: 0,
                total_shares: 0,
                balances: Map::new(&env),
            });

        if data.total_shares == 0 {
            0 // Avoid division by zero
        } else {
            data.total_assets / data.total_shares
        }
    }
}
