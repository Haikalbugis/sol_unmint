use anyhow::Result;
use sol_unmint::TokenProgram;

use crate::setup_unmint::setup_unmint;
mod setup_unmint;

#[test]
fn test_send_and_close_legacy() -> Result<()> {
    let unmint = setup_unmint(TokenProgram::Legacy);

    let tx_sig = unmint.send_and_close(
        "",                                             //Base58 private key of the sender
        "",                                             //Base58 private key of the recipient
        "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB", //usdc mint token address or other
        None,                                           // sender fee payer
    )?;

    println!("Transaction signature: {:?}", tx_sig);
    Ok(())
}
