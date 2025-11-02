use anyhow::Result;
use sol_unmint::TokenProgram;

use crate::setup_unmint::setup_unmint;
mod setup_unmint;

#[test]
fn test_send_sol() -> Result<()> {
    let unmint = setup_unmint(TokenProgram::Legacy);

    let tx_sig = unmint.transfer_sol("", "57ksuWYrkEnrUDfisoPYw6Wb1hmsjFBYSwv9HULex1yj", 0.001);

    println!("Transaction signature: {:?}", tx_sig);
    Ok(())
}
