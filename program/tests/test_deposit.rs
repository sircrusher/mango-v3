#![cfg(feature = "test-bpf")]
// Tests related to depositing into mango group
use solana_program_test::*;

use mango::state::*;
use program_test::*;

mod program_test;

#[tokio::test]
async fn test_deposit_succeeds() {
    // Arrange
    let config = MangoProgramTestConfig::default();
    let mut test = MangoProgramTest::start_new(&config).await;

    let user_index = 0;
    let quote_index = config.num_mints - 1;
    let base_price = 10000;

    let (mango_group_pk, mango_group) = test.with_mango_group().await;
    let quote_mint = test.with_mint(quote_index as usize);

    let deposit_amount = (base_price * quote_mint.unit) as u64;
    let (mango_account_pk, _mango_account) =
        test.with_mango_account(&mango_group_pk, user_index).await;
    let user_token_account = test.with_user_token_account(user_index, quote_index as usize);
    let initial_balance = test.get_token_balance(user_token_account).await;

    // Act
    test.perform_deposit(
        &mango_group,
        &mango_group_pk,
        &mango_account_pk,
        user_index,
        quote_index as usize,
        deposit_amount,
    )
    .await;

    // Assert
    let post_balance = test.get_token_balance(user_token_account).await;
    assert_eq!(post_balance, initial_balance - deposit_amount);

    let (_root_bank_pk, root_bank) = test.with_root_bank(&mango_group, quote_index as usize).await;
    let (_node_bank_pk, node_bank) = test.with_node_bank(&root_bank, 0).await;
    let mango_vault_balance = test.get_token_balance(node_bank.vault).await;
    assert_eq!(mango_vault_balance, deposit_amount);
    let mango_account = test.load_account::<MangoAccount>(mango_account_pk).await;

    assert_eq!(mango_account.deposits[quote_index as usize], deposit_amount);
}
