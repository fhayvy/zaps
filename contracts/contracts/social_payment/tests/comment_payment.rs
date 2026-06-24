use soroban_sdk::{testutils::Env as _, Address, Env, String, Symbol};
use zaps_social_payment::SocialPaymentContract;

#[test]
fn comment_payment_accepts_valid_comment() {
    let env = Env::default();
    let sender = Address::random(&env);
    env.mock_auth(&sender);

    let tx_id = Symbol::new(&env, "payment-123");
    let comment = String::from_slice(&env, "This is a valid comment.");

    SocialPaymentContract.comment_payment(env, sender, tx_id, comment);
}

#[test]
#[should_panic(expected = "comment exceeds maximum length")]
fn comment_payment_rejects_overlong_comment() {
    let env = Env::default();
    let sender = Address::random(&env);
    env.mock_auth(&sender);

    let tx_id = Symbol::new(&env, "payment-456");
    let long_text = "x".repeat(121);
    let comment = String::from_slice(&env, &long_text);

    SocialPaymentContract.comment_payment(env, sender, tx_id, comment);
}
