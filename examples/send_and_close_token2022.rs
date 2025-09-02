use anyhow::Result;
use sol_unmint::{TokenProgram, Unmint};

fn main() -> Result<()> {
    let unmint = Unmint::new(
        "https://api.mainnet-beta.solana.com",
        TokenProgram::Token2022,
    );

    let tx_sig = unmint.send_and_close(
        "",                                            //Base58 private key of the sender
        "",                                            //Base58 private key of the recipient
        "pumpCmXqMfrsAkQ5r49WcJnRayYRqmXz6ae8H7H9Dfn", //usdc mint token address or other
        None,                                          // sender fee payer
    )?;

    println!("Transaction signature: {:?}", tx_sig);
    Ok(())
}
