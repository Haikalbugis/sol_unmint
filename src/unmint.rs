use std::{str::FromStr, sync::Arc};

use crate::token_program::TokenProgram;
use anyhow::{Ok, Result, anyhow};
use solana_sdk::{instruction::Instruction, signature::Signature, transaction::Transaction};

use {
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair, signer::Signer,
    },
};

#[derive(Clone)]
pub struct Unmint {
    client: Arc<RpcClient>,
    token_program: TokenProgram,
}

/// Create a new instance of Unmint
///
/// # Arguments
///
/// * `rpc_url` - Solana's RPC URL, e.g., "https://api.mainnet-beta.solana.com"
///
/// # Example
/// ```
/// let unmint = gamba::Unmint::new("https://api.mainnet-beta.solana.com");
// ```
impl Unmint {
    pub fn new(rpc_url: &str, token_program: TokenProgram) -> Self {
        let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
        Self {
            client: client.into(),
            token_program,
        }
    }

    fn close_token_account_instruction(
        &self,
        from_base58_string: &str,
        token_mint_address: &str,
        address_reedem_sol: Option<&Pubkey>,
    ) -> Result<Instruction> {
        let from_keypair = Keypair::from_base58_string(from_base58_string);
        let token_mint_pubkey = Pubkey::from_str(token_mint_address)?;
        let address = *address_reedem_sol.unwrap_or(&from_keypair.pubkey());

        let ata_sender = self
            .token_program
            .ata(&from_keypair.pubkey(), &token_mint_pubkey);

        let instraction = self
            .token_program
            .close_ix(&ata_sender, &address, &from_keypair)?;

        Ok(instraction)
    }

    fn send_max_token_instruction(
        &self,
        from_base58_string: &str,
        to_base58_string: &str,
        token_mint_address: &str,
    ) -> Result<Instruction> {
        let from_keypair = Keypair::from_base58_string(from_base58_string);
        let to_keypair = Keypair::from_base58_string(to_base58_string);
        let token_mint_pubkey = Pubkey::from_str(token_mint_address)?;

        let ata_sender = self
            .token_program
            .ata(&from_keypair.pubkey(), &token_mint_pubkey);

        let ata_destinaton = self
            .token_program
            .ata(&to_keypair.pubkey(), &token_mint_pubkey);

        let balances = self
            .client
            .get_token_account_balance(&ata_sender)
            .map_err(|e| {
                if let solana_client::client_error::ClientErrorKind::RpcError(
                    solana_client::rpc_request::RpcError::RpcResponseError { message, .. },
                ) = e.kind()
                {
                    if message.contains("could not find account") {
                        return anyhow!("token account not found");
                    }
                }
                anyhow!("rpc error: {:?}", e)
            })?;

        let instraction = self.token_program.transfer_ix(
            &ata_sender,
            &ata_destinaton,
            &from_keypair,
            balances.amount.parse::<u64>()?,
            balances.decimals,
            &token_mint_pubkey,
        )?;

        Ok(instraction)
    }

    fn send_token_instruction(
        &self,
        from_base58_string: &str,
        to_pubkey: &Pubkey,
        token_mint_address: &str,
        amount: f64,
    ) -> Result<Instruction> {
        let from_keypair = Keypair::from_base58_string(from_base58_string);
        let token_mint_pubkey = Pubkey::from_str(token_mint_address)?;

        let ata_sender = self
            .token_program
            .ata(&from_keypair.pubkey(), &token_mint_pubkey);

        let ata_destinaton = self.token_program.ata(&to_pubkey, &token_mint_pubkey);

        let balances = self
            .client
            .get_token_account_balance(&ata_sender)
            .map_err(|e| {
                if let solana_client::client_error::ClientErrorKind::RpcError(
                    solana_client::rpc_request::RpcError::RpcResponseError { message, .. },
                ) = e.kind()
                {
                    if message.contains("could not find account") {
                        return anyhow!("token account not found");
                    }
                }
                anyhow!("rpc error: {:?}", e)
            })?;

        let decimal: u8 = balances.decimals;
        let amount_to_send = (amount * 10u64.pow(decimal.into()) as f64) as u64;

        let instraction = self.token_program.transfer_ix(
            &ata_sender,
            &ata_destinaton,
            &from_keypair,
            amount_to_send,
            balances.decimals,
            &token_mint_pubkey,
        )?;

        Ok(instraction)
    }

    /// Sends all tokens from `from` to `to` and closes the ATA.
    ///
    // # Arguments
    /// * `from_base58_string` - Base58 private key of the sender
    /// * `to_base58_string` - Base58 private key of the recipient
    /// * `token_mint_address` - Token mint address
    /// * `fee_payer_base58_string` - Optional, who pays the fee
    ///
    /// # Returns
    /// * `Signature` of the transaction
    pub fn send_and_close(
        &self,
        from_base58_string: &str,
        to_base58_string: &str,
        token_mint_address: &str,
        fee_payer_base58_string: Option<&str>,
    ) -> Result<Signature> {
        let from_keypair = Keypair::from_base58_string(from_base58_string);

        let fee_payer: Keypair = fee_payer_base58_string
            .map(|s| Keypair::from_base58_string(s))
            .unwrap_or_else(|| Keypair::from_base58_string(from_base58_string));

        let send_token_instruction = self.send_max_token_instruction(
            from_base58_string,
            to_base58_string,
            token_mint_address,
        )?;

        let close_token_account_instruction = self.close_token_account_instruction(
            from_base58_string,
            token_mint_address,
            Some(&fee_payer.pubkey()),
        )?;

        let instructions = vec![send_token_instruction, close_token_account_instruction];

        let mut transaction = Transaction::new_with_payer(&instructions, Some(&fee_payer.pubkey()));

        let mut signers: Vec<&Keypair> = vec![&from_keypair];
        if fee_payer.pubkey() != from_keypair.pubkey() {
            signers.push(&fee_payer);
        }

        transaction.sign(&signers, self.client.get_latest_blockhash()?);

        let confirm = self.client.send_and_confirm_transaction(&transaction)?;

        Ok(confirm)
    }

    /// Sends a specified amount of SPL token from one account to another.
    ///
    /// # Arguments
    ///
    /// * `from_base58_string` - The sender's private key in Base58 string format.
    /// * `to_address` - The recipient's public key (Base58 string).
    /// * `token_mint_address` - The SPL token mint address (Base58 string).
    /// * `amount` - The amount of tokens to send as a floating-point number.
    /// * `fee_payer_base58_string` - Optional: the Base58 private key of the fee payer.
    ///    If `None`, the sender will pay the transaction fee.
    ///
    /// # Returns
    /// * `Result<Signature>` - Returns the transaction signature if successful.
    pub fn send_token(
        &self,
        from_base58_string: &str,
        to_address: &str,
        token_mint_address: &str,
        amount: f64,
        fee_payer_base58_string: Option<&str>,
    ) -> Result<Signature> {
        let from_keypair = Keypair::from_base58_string(from_base58_string);
        let to_pubkey = Pubkey::from_str(to_address)?;

        let fee_payer: Keypair = fee_payer_base58_string
            .map(|s| Keypair::from_base58_string(s))
            .unwrap_or_else(|| Keypair::from_base58_string(from_base58_string));

        let send_token_instruction = self.send_token_instruction(
            from_base58_string,
            &to_pubkey,
            token_mint_address,
            amount,
        )?;

        let instructions = vec![send_token_instruction];

        let mut transaction = Transaction::new_with_payer(&instructions, Some(&fee_payer.pubkey()));

        let mut signers: Vec<&Keypair> = vec![&from_keypair];
        if fee_payer.pubkey() != from_keypair.pubkey() {
            signers.push(&fee_payer);
        }

        transaction.sign(&signers, self.client.get_latest_blockhash()?);

        let confirm = self.client.send_and_confirm_transaction(&transaction)?;

        Ok(confirm)
    }
}
