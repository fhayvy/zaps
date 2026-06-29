#![no_std]
#![allow(dead_code, unused_variables, unused_imports, unexpected_cfgs)]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, String};

#[contract]
pub struct UserRegistryContract;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    User(Address),     // Maps Address -> Username (String)
    Username(String),  // Maps Username (String) -> Address
    Avatar(Address),   // Maps Address -> Avatar URI (String)
}

#[contractimpl]
impl UserRegistryContract {
    /// Register a username mapping to the sender's address
    pub fn register_user(env: Env, user: Address, username: String) {
        // TODO: Implement SC-002 (Validate username rules: length 3-15, alphanumeric, lowercase)
        user.require_auth();

        let username_key = DataKey::Username(username.clone());
        let user_key = DataKey::User(user.clone());

        // Check if username is already taken (uniqueness validation)
        if env.storage().persistent().has(&username_key) {
            panic!("username already taken");
        }

        // Store the mappings
        env.storage().persistent().set(&user_key, &username);
        env.storage().persistent().set(&username_key, &user);
    }

    /// Retrieve the Address associated with a username
    pub fn get_address(env: Env, username: String) -> Address {
        let username_key = DataKey::Username(username);
        env.storage()
            .persistent()
            .get(&username_key)
            .unwrap_or_else(|| panic!("username not found"))
    }

    /// Retrieve the username associated with an Address
    pub fn get_username(env: Env, user: Address) -> String {
        let user_key = DataKey::User(user);
        env.storage()
            .persistent()
            .get(&user_key)
            .unwrap_or_else(|| panic!("address not registered"))
    }

    /// Update user profile metadata (e.g. avatar URI)
    pub fn update_profile(env: Env, user: Address, avatar_uri: String) {
        user.require_auth();
        env.storage()
            .persistent()
            .set(&DataKey::Avatar(user), &avatar_uri);
    }

    /// Retrieve the avatar URI associated with an Address
    pub fn get_avatar(env: Env, user: Address) -> String {
        env.storage()
            .persistent()
            .get(&DataKey::Avatar(user))
            .unwrap_or_else(|| String::from_str(&env, ""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_register_and_update_profile() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, UserRegistryContract);
        let client = UserRegistryContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        let username = String::from_str(&env, "ebube");

        // Register user
        client.register_user(&user, &username);
        assert_eq!(client.get_address(&username), user);
        assert_eq!(client.get_username(&user), username);

        // Update profile
        let avatar_uri = String::from_str(&env, "https://example.com/avatar.png");
        client.update_profile(&user, &avatar_uri);

        assert_eq!(client.get_avatar(&user), avatar_uri);
    }

    #[test]
    #[ignore]
    fn test_update_profile_fails_without_auth() {
        let env = Env::default();
        // Do NOT mock all auths here

        let contract_id = env.register_contract(None, UserRegistryContract);
        let client = UserRegistryContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        let avatar_uri = String::from_str(&env, "https://example.com/avatar.png");

        let res = client.try_update_profile(&user, &avatar_uri);
        assert!(res.is_err());
    }
}
