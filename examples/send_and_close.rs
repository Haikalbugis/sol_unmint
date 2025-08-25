use anyhow::Result;
use sol_unmint::Unmint;

fn main() -> Result<()> {
    let unmint = Unmint::new("https://api.mainnet-beta.solana.com");

    let tx_sig = unmint.send_and_close(
        "",                                             //Base58 private key of the sender
        "",                                             //Base58 private key of the recipient
        "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB", //usdc mint token address or other
        None,                                           // sender fee payer
    )?;

    println!("Transaction signature: {:?}", tx_sig);
    Ok(())
}
