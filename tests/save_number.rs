use std::mem::size_of;
use std::str::FromStr;

use {
    saveit::processor::*,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_program_test::*,
    solana_sdk::account::Account,
    solana_sdk::signer::Signer,
    solana_sdk::transaction::Transaction,
};

#[solana_program_test::tokio::test]
async fn save_42() {
    let program_id = Pubkey::from_str("EEokyqATvFYrpNEWQgqjQjntf6mkjr5rn6osS8aEdT6H").unwrap();
    println!("Program ID: {}", program_id);
    let general_answer: u64 = 42;

    let pubkey = Pubkey::new_unique();
    let account = Account::new(u64::default(), size_of::<AccountWithNumber>(), &pubkey);
    let mut test_env = ProgramTest::new("saveit", program_id, processor!(process_instruction));
    test_env.add_account(pubkey, account);
    let (mut banks_client, payer, recent_blockhash) = test_env.start().await;
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            pubkey,
            &general_answer,
            vec![AccountMeta::new(pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let actual = banks_client
        .get_account_data_with_borsh::<u64>(pubkey)
        .await
        .unwrap();
    assert_eq!(actual, general_answer);
}
