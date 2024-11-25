#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol};

// shared state in contract
// env.storage().instance()
// Name
// Symbol
// Admin
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Name,
    Symbol,
    Admin,
}

// persistent state in contract specific to user
// env.storage().persistent()
// TokenOwned
// Seat
#[contracttype]
#[derive(Clone)]
pub enum UserDataKey {
    TokenOwner(u32), // takes in seat number, returns a token owner (address)
    Seat(Address),   // takes in address, returns a seat number
}

#[contract]
pub struct TourNFTContract;

#[contractimpl]
impl TourNFTContract {
    // initialize
    pub fn initialize(env: Env, name: String, symbol: Symbol, admin: Address) {
        // making sure that this contract doesn't have admin
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("This contract already has an admin");
        }

        // store
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    // mint
    pub fn mint(env: Env, to: Address, seat_num: u32) {
        if env
            .storage()
            .persistent()
            .has(&UserDataKey::TokenOwner(seat_num))
        {
            panic!("This NFT already has an owner");
        }

        env.storage()
            .persistent()
            .set(&UserDataKey::TokenOwner(seat_num), &to);

        env.storage()
            .persistent()
            .set(&UserDataKey::Seat(to.clone()), &seat_num);
    }

    // owner of
    pub fn owner_of(env: Env, seat_num: u32) -> Address {
        let owner: Address = env
            .storage()
            .persistent()
            .get(&UserDataKey::TokenOwner(seat_num))
            .unwrap();

        owner
    }

    // transfer
    pub fn transfer(env: Env, from: Address, to: Address, seat_num: u32) {
        from.require_auth();

        // check to see if the receiver has NFT token
        if env
            .storage()
            .persistent()
            .has(&UserDataKey::Seat(to.clone()))
        {
            panic!("This receiver already has NFT, rejected");
        }

        // check to see the sender has the token
        if env
            .storage()
            .persistent()
            .has(&UserDataKey::Seat(from.clone()))
        {
            env.storage().persistent().remove(&UserDataKey::Seat(from));
        } else {
            panic!("Sender does not own this NFT");
        }

        env.storage()
            .persistent()
            .set(&UserDataKey::TokenOwner(seat_num), &to);
        env.storage()
            .persistent()
            .set(&UserDataKey::Seat(to.clone()), &seat_num);
    }
}
