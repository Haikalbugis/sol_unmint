use anyhow::Result;
use sol_unmint::TokenProgram;

use crate::setup_unmint::setup_unmint;
mod setup_unmint;

#[test]
fn test_send_and_close_token2022() -> Result<()> {
    let unmint = setup_unmint(TokenProgram::Token2022);

    let tx_sig = unmint.send_and_close(
        "",                                            //Base58 private key of the sender
        "",                                            //Base58 private key of the recipient
        "pumpCmXqMfrsAkQ5r49WcJnRayYRqmXz6ae8H7H9Dfn", //usdc mint token address or other
        None,                                          // sender fee payer
    )?;

    println!("Transaction signature: {:?}", tx_sig);
    Ok(())
}
