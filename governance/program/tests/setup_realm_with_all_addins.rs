#![cfg(feature = "test-sbf")]

use solana_program::pubkey::Pubkey;
use solana_program_test::*;

mod program_test;

use program_test::*;

use crate::program_test::args::RealmSetupArgs;

#[tokio::test]
async fn test_create_realm_with_all_addins() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_with_all_addins().await;

    let mut realm_setup_args = RealmSetupArgs::default();

    realm_setup_args
        .community_token_config_args
        .voter_weight_addin = governance_test.voter_weight_addin_id;

    realm_setup_args
        .community_token_config_args
        .max_voter_weight_addin = governance_test.max_voter_weight_addin_id;

    // Act

    let realm_cookie = governance_test
        .with_realm_using_args(&realm_setup_args)
        .await;

    // Assert

    let realm_config_data = governance_test
        .get_realm_config_account(&realm_cookie.realm_config.address)
        .await;

    assert_eq!(realm_cookie.realm_config.account, realm_config_data);

    assert!(realm_config_data
        .community_token_config
        .voter_weight_addin
        .is_some());

    assert!(realm_config_data
        .community_token_config
        .max_voter_weight_addin
        .is_some());
}

#[tokio::test]
async fn test_set_all_addins_for_realm_without_addins() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_with_all_addins().await;

    let mut realm_setup_args = RealmSetupArgs::default();

    let mut realm_cookie = governance_test
        .with_realm_using_args(&realm_setup_args)
        .await;

    realm_setup_args
        .community_token_config_args
        .voter_weight_addin = governance_test.voter_weight_addin_id;

    realm_setup_args
        .community_token_config_args
        .max_voter_weight_addin = governance_test.max_voter_weight_addin_id;

    // Act

    governance_test
        .set_realm_config(&mut realm_cookie, &realm_setup_args)
        .await
        .unwrap();

    // Assert

    let realm_config_data = governance_test
        .get_realm_config_account(&realm_cookie.realm_config.address)
        .await;

    assert_eq!(realm_cookie.realm_config.account, realm_config_data);

    assert!(realm_config_data
        .community_token_config
        .voter_weight_addin
        .is_some());

    assert!(realm_config_data
        .community_token_config
        .max_voter_weight_addin
        .is_some());
}

#[tokio::test]
async fn test_set_all_addin_for_realm_without_council_and_addins() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_with_all_addins().await;

    let mut realm_setup_args = RealmSetupArgs::default();

    realm_setup_args.use_council_mint = false;

    let mut realm_cookie = governance_test
        .with_realm_using_args(&realm_setup_args)
        .await;

    realm_setup_args
        .community_token_config_args
        .voter_weight_addin = governance_test.voter_weight_addin_id;

    realm_setup_args
        .community_token_config_args
        .max_voter_weight_addin = governance_test.max_voter_weight_addin_id;

    // Act

    governance_test
        .set_realm_config(&mut realm_cookie, &realm_setup_args)
        .await
        .unwrap();

    // Assert

    let realm_config_data = governance_test
        .get_realm_config_account(&realm_cookie.realm_config.address)
        .await;

    assert_eq!(realm_cookie.realm_config.account, realm_config_data);

    assert!(realm_config_data
        .community_token_config
        .voter_weight_addin
        .is_some());

    assert!(realm_config_data
        .community_token_config
        .max_voter_weight_addin
        .is_some());
}

#[tokio::test]
async fn test_set_all_realm_addins_for_realm_with_all_addins() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_with_all_addins().await;

    let mut realm_cookie = governance_test.with_realm().await;

    let mut realm_setup_args = RealmSetupArgs::default();

    realm_setup_args
        .community_token_config_args
        .voter_weight_addin = governance_test.voter_weight_addin_id;

    realm_setup_args
        .community_token_config_args
        .max_voter_weight_addin = governance_test.max_voter_weight_addin_id;

    let max_community_voter_weight_addin_address = Pubkey::new_unique();

    realm_setup_args
        .community_token_config_args
        .max_voter_weight_addin = Some(max_community_voter_weight_addin_address);

    let community_voter_weight_addin_address = Pubkey::new_unique();
    realm_setup_args
        .community_token_config_args
        .voter_weight_addin = Some(community_voter_weight_addin_address);

    // Act

    governance_test
        .set_realm_config(&mut realm_cookie, &realm_setup_args)
        .await
        .unwrap();

    // Assert

    let realm_config_data = governance_test
        .get_realm_config_account(&realm_cookie.realm_config.address)
        .await;

    assert_eq!(realm_cookie.realm_config.account, realm_config_data);
    assert_eq!(
        realm_config_data
            .community_token_config
            .max_voter_weight_addin,
        Some(max_community_voter_weight_addin_address)
    );
    assert_eq!(
        realm_config_data.community_token_config.voter_weight_addin,
        Some(community_voter_weight_addin_address)
    );

    assert!(realm_config_data
        .community_token_config
        .voter_weight_addin
        .is_some());

    assert!(realm_config_data
        .community_token_config
        .max_voter_weight_addin
        .is_some());
}

#[tokio::test]
async fn test_set_realm_config_without_addins_for_realm_without_addins() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_with_all_addins().await;

    let mut realm_setup_args = RealmSetupArgs::default();

    let mut realm_cookie = governance_test
        .with_realm_using_args(&realm_setup_args)
        .await;

    realm_setup_args
        .community_token_config_args
        .max_voter_weight_addin = None;

    realm_setup_args
        .community_token_config_args
        .voter_weight_addin = None;

    // Act

    governance_test
        .set_realm_config(&mut realm_cookie, &realm_setup_args)
        .await
        .unwrap();

    // Assert

    let realm_config_data = governance_test
        .get_realm_config_account(&realm_cookie.realm_config.address)
        .await;

    assert!(realm_config_data
        .community_token_config
        .voter_weight_addin
        .is_none());

    assert!(realm_config_data
        .community_token_config
        .max_voter_weight_addin
        .is_none());
}

#[tokio::test]
async fn test_set_realm_config_without_any_addins_for_realm_with_existing_addins() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_with_all_addins().await;
    let mut realm_cookie = governance_test.with_realm().await;

    let realm_setup_args = RealmSetupArgs::default();

    // Act

    governance_test
        .set_realm_config(&mut realm_cookie, &realm_setup_args)
        .await
        .unwrap();

    // Assert

    let realm_config_data = governance_test
        .get_realm_config_account(&realm_cookie.realm_config.address)
        .await;

    assert!(realm_config_data
        .community_token_config
        .max_voter_weight_addin
        .is_none());

    assert!(realm_config_data
        .community_token_config
        .voter_weight_addin
        .is_none());
}
