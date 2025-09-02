use anyhow::Result;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer};
use spl_associated_token_account::{
    get_associated_token_address, get_associated_token_address_with_program_id,
};
use spl_token::ID as TOKEN_PROGRAM_ID;
use spl_token_2022::ID as TOKEN_2022_PROGRAM_ID;

/// Enum to select the SPL token program version
#[derive(Clone)]
pub enum TokenProgram {
    /// Standard SPL token
    Legacy,
    /// SPL Token 2022
    Token2022,
}

impl TokenProgram {
    /// Returns the `Pubkey` of the token program
    ///
    /// # Example
    /// ```
    /// let prog = TokenProgram::Token2022;
    /// println!("Program ID: {}", prog.program_id());
    /// ```
    pub fn program_id(&self) -> Pubkey {
        match self {
            TokenProgram::Legacy => TOKEN_PROGRAM_ID,
            TokenProgram::Token2022 => TOKEN_2022_PROGRAM_ID,
        }
    }

    /// Returns the associated token account (ATA) for a given owner and mint
    ///
    /// # Arguments
    /// * `owner` - Pubkey of the account owner
    /// * `mint` - Pubkey of the token mint
    ///
    /// # Example
    /// ```
    /// let ata = prog.ata(&owner_pubkey, &mint_pubkey);
    /// ```
    pub fn ata(&self, owner: &Pubkey, mint: &Pubkey) -> Pubkey {
        match self {
            TokenProgram::Legacy => get_associated_token_address(owner, mint),
            TokenProgram::Token2022 => {
                get_associated_token_address_with_program_id(owner, mint, &TOKEN_2022_PROGRAM_ID)
            }
        }
    }

    /// Creates a transfer instruction for SPL tokens
    ///
    /// # Arguments
    /// * `from` - ATA of the sender
    /// * `to` - ATA of the receiver
    /// * `authority` - Keypair of the account authority
    /// * `amount` - Token amount in the smallest unit
    /// * `decimals` - Mint decimals
    /// * `mint` - Mint Pubkey (required for Token2022)
    ///
    /// # Example
    /// ```
    /// let ix = prog.transfer_ix(&ata_from, &ata_to, &payer, 1_000_000, 6, &mint_pubkey)?;
    /// ```
    pub fn transfer_ix(
        &self,
        from: &Pubkey,
        to: &Pubkey,
        authority: &Keypair,
        amount: u64,
        decimals: u8,
        mint: &Pubkey,
    ) -> Result<Instruction> {
        match self {
            TokenProgram::Legacy => Ok(spl_token::instruction::transfer(
                &TOKEN_PROGRAM_ID,
                from,
                to,
                &authority.pubkey(),
                &[&authority.pubkey()],
                amount,
            )?),
            TokenProgram::Token2022 => Ok(spl_token_2022::instruction::transfer_checked(
                &TOKEN_2022_PROGRAM_ID,
                from,
                mint,
                to,
                &authority.pubkey(),
                &[&authority.pubkey()],
                amount,
                decimals,
            )?),
        }
    }

    /// Creates an instruction to close an associated token account (ATA)
    ///
    /// # Arguments
    /// * `account` - ATA to close
    /// * `destination` - Pubkey that receives remaining SOL
    /// * `authority` - Keypair authority of the account
    ///
    /// # Example
    /// ```
    /// let close_ix = prog.close_ix(&ata, &owner, &payer)?;
    /// ```
    pub fn close_ix(
        &self,
        account: &Pubkey,
        destination: &Pubkey,
        authority: &Keypair,
    ) -> Result<Instruction> {
        match self {
            TokenProgram::Legacy => Ok(spl_token::instruction::close_account(
                &TOKEN_PROGRAM_ID,
                account,
                destination,
                &authority.pubkey(),
                &[&authority.pubkey()],
            )?),
            TokenProgram::Token2022 => Ok(spl_token_2022::instruction::close_account(
                &TOKEN_2022_PROGRAM_ID,
                account,
                destination,
                &authority.pubkey(),
                &[&authority.pubkey()],
            )?),
        }
    }
}
