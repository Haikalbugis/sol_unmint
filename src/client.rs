use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{signature::Keypair, signer::Signer};
use spl_token::{
    ID as TOKEN_PROGRAM_ID,
    instruction::{close_account, transfer_checked},
    solana_program::{instruction::Instruction, pubkey::Pubkey},
};

pub struct Web3Client {
    pub client: RpcClient,
}

impl Web3Client {
    pub fn new(rpc_url: String) -> Self {
        let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

        Self { client }
    }

    // pub fn instruction_close_token_account(
    //     &self,
    //     authority_keypair: &Keypair,
    //     ata_sender: &Pubkey,
    //     destination_sol_pubkey_unmint: &Pubkey,
    // ) -> anyhow::Result<Instruction> {
    //     let close_ix = close_account(
    //         &TOKEN_PROGRAM_ID,
    //         ata_sender,
    //         destination_sol_pubkey_unmint,
    //         &authority_keypair.pubkey(),
    //         &[&authority_keypair.pubkey()],
    //     )?;

    //     Ok(close_ix)
    // }

    // pub fn instruction_send_token(
    //     &self,
    //     authority_keypair: &Keypair,
    //     sender_ata: &Pubkey,
    //     token_mint: &Pubkey,
    //     recipient_ata: &Pubkey,
    //     amount: u64,
    //     decimals: u8,
    // ) -> anyhow::Result<Instruction> {
    //     let ix = transfer_checked(
    //         &spl_token::ID,
    //         &sender_ata,
    //         token_mint,
    //         &recipient_ata,
    //         &authority_keypair.pubkey(),
    //         &[],
    //         amount,
    //         decimals,
    //     )?;

    //     Ok(ix)
    // }

    // pub fn sign_and_confirm_tx(
    //     &self,
    //     instructions: Vec<Instruction>,
    //     authority_keypair: &Keypair,
    // ) -> anyhow::Result<Signature> {
    //     let mut transaction =
    //         Transaction::new_with_payer(&instructions, Some(&authority_keypair.pubkey()));
    //     transaction.sign(
    //         &[&authority_keypair],
    //         self.client
    //             .get_latest_blockhash()
    //             .expect("error get last block"),
    //     );

    //     let confirm = self.client.send_and_confirm_transaction(&transaction)?;

    //     Ok(confirm)
    // }

    // pub fn instruction_send_sol(&self, from_pubkey: &Pubkey, to_pubkey: &Pubkey, amount: f64) {
    //     let amout_send = amount * LAMPORTS_PER_SOL;
    //     let transfer_ix = transfer(&from_pubkey, &to_pubkey, amout_send);
    // }
}
