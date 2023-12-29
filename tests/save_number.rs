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
    let program_id =
        dbg!(Pubkey::from_str("EEokyqATvFYrpNEWQgqjQjntf6mkjr5rn6osS8aEdT6H").unwrap());
    let answer: u64 = 42;

    let mut test_env = ProgramTest::new("saveit", program_id, processor!(process_instruction));
    let pubkey = Pubkey::new_unique();
    test_env.add_account(
        pubkey,
        Account::new(u64::default(), size_of::<u64>(), &program_id),
    );

    let (mut banks_client, payer, recent_blockhash) = test_env.start().await;
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            program_id,
            &answer,
            vec![AccountMeta::new(pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer], recent_blockhash);
    banks_client.send_transaction(transaction).await.unwrap();
}
