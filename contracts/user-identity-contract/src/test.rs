#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_register_user_success() {
    let env = Env::default();
    let contract_id = env.register_contract(None, UserIdentityContract);
    let client = UserIdentityContractClient::new(&env, &contract_id);

    // Mock the test environment
    env.mock_all_auths();

    let user_addr = Address::generate(&env);
    let username = String::from_str(&env, "admin_user");
    let role = String::from_str(&env, "admin");
    let profile_uri = String::from_str(&env, "https://example.com/admin");

    // Register user
    client.register(&user_addr, &username, &role, &profile_uri);

    // Verify user is registered
    assert!(client.is_registered(&user_addr));

    // Verify user data
    let user = client.get_user(&user_addr);
    assert_eq!(user.address, user_addr);
    assert_eq!(user.role, role);
}

#[test]
fn test_register_duplicate_user() {
    let env = Env::default();
    let contract_id = env.register_contract(None, UserIdentityContract);
    let client = UserIdentityContractClient::new(&env, &contract_id);

    env.mock_all_auths();

    let user_addr = Address::generate(&env);
    let username = String::from_str(&env, "test_user");
    let role = String::from_str(&env, "user");
    let profile_uri = String::from_str(&env, "https://example.com/user");

    // Register user first time
    client.register(&user_addr, &username, &role, &profile_uri);

    // Try to register the same user again
    let result = client.try_register(&user_addr, &username, &role, &profile_uri);
    assert_eq!(result, Err(Ok(Error::AlreadyRegistered)));
}

#[test]
fn test_get_user_not_found() {
    let env = Env::default();
    let contract_id = env.register_contract(None, UserIdentityContract);
    let client = UserIdentityContractClient::new(&env, &contract_id);

    let user_addr = Address::generate(&env);

    // Try to get user that doesn't exist
    let result = client.try_get_user(&user_addr);
    assert_eq!(result, Err(Ok(Error::UserNotFound)));
}

#[test]
fn test_is_registered_false() {
    let env = Env::default();
    let contract_id = env.register_contract(None, UserIdentityContract);
    let client = UserIdentityContractClient::new(&env, &contract_id);

    let user_addr = Address::generate(&env);

    // Check if unregistered user is registered
    assert!(!client.is_registered(&user_addr));
}

#[test]
fn test_multiple_users_with_different_roles() {
    let env = Env::default();
    let contract_id = env.register_contract(None, UserIdentityContract);
    let client = UserIdentityContractClient::new(&env, &contract_id);

    env.mock_all_auths();

    // Register multiple users with different roles
    let admin_addr = Address::generate(&env);
    let admin_username = String::from_str(&env, "admin_user");
    let admin_role = String::from_str(&env, "admin");
    let admin_profile = String::from_str(&env, "https://example.com/admin");

    let moderator_addr = Address::generate(&env);
    let moderator_username = String::from_str(&env, "mod_user");
    let moderator_role = String::from_str(&env, "moderator");
    let moderator_profile = String::from_str(&env, "https://example.com/mod");

    let user_addr = Address::generate(&env);
    let user_username = String::from_str(&env, "regular_user");
    let user_role = String::from_str(&env, "user");
    let user_profile = String::from_str(&env, "https://example.com/user");

    // Register all users
    client.register(&admin_addr, &admin_username, &admin_role, &admin_profile);
    client.register(&moderator_addr, &moderator_username, &moderator_role, &moderator_profile);
    client.register(&user_addr, &user_username, &user_role, &user_profile);

    // Verify all are registered
    assert!(client.is_registered(&admin_addr));
    assert!(client.is_registered(&moderator_addr));
    assert!(client.is_registered(&user_addr));

    // Verify correct roles
    let admin = client.get_user(&admin_addr);
    assert_eq!(admin.role, admin_role);

    let moderator = client.get_user(&moderator_addr);
    assert_eq!(moderator.role, moderator_role);

    let user = client.get_user(&user_addr);
    assert_eq!(user.role, user_role);
}

#[test]
fn test_register_requires_auth() {
    let env = Env::default();
    let contract_id = env.register_contract(None, UserIdentityContract);
    let client = UserIdentityContractClient::new(&env, &contract_id);

    let user_addr = Address::generate(&env);
    let username = String::from_str(&env, "test_user");
    let role = String::from_str(&env, "user");
    let profile_uri = String::from_str(&env, "https://example.com/user");

    // Mock authentication
    env.mock_all_auths();

    client.register(&user_addr, &username, &role, &profile_uri);

    // Verify auth was required by checking that auth was recorded
    let auths = env.auths();
    assert!(!auths.is_empty());
    assert_eq!(auths.len(), 1);
    assert_eq!(auths[0].0, user_addr);
}

#[test]
fn test_register_with_empty_role() {
    let env = Env::default();
    let contract_id = env.register_contract(None, UserIdentityContract);
    let client = UserIdentityContractClient::new(&env, &contract_id);

    env.mock_all_auths();

    let user_addr = Address::generate(&env);
    let username = String::from_str(&env, "empty_role_user");
    let empty_role = String::from_str(&env, "");
    let profile_uri = String::from_str(&env, "https://example.com/user");

    // Register user with empty role (should succeed as validation is up to the caller)
    client.register(&user_addr, &username, &empty_role, &profile_uri);

    // Verify user is registered with empty role
    let user = client.get_user(&user_addr);
    assert_eq!(user.role, empty_role);
}

#[test]
fn test_register_with_long_role_name() {
    let env = Env::default();
    let contract_id = env.register_contract(None, UserIdentityContract);
    let client = UserIdentityContractClient::new(&env, &contract_id);

    env.mock_all_auths();

    let user_addr = Address::generate(&env);
    let username = String::from_str(&env, "long_role_user");
    let long_role = String::from_str(&env, "super_administrator_with_full_permissions");
    let profile_uri = String::from_str(&env, "https://example.com/admin");

    // Register user with long role name
    client.register(&user_addr, &username, &long_role, &profile_uri);

    // Verify user data
    let user = client.get_user(&user_addr);
    assert_eq!(user.role, long_role);
}
