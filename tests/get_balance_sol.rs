use anyhow::Result;
use sol_unmint::TokenProgram;

use crate::setup_unmint::setup_unmint;
mod setup_unmint;

#[test]
fn test_get_balance() -> Result<()> {
    let unmint = setup_unmint(TokenProgram::Legacy);

    let balance = unmint.balance_sol("57ksuWYrkEnrUDfisoPYw6Wb1hmsjFBYSwv9HULex1yj");

    println!("Balance: {:?}", balance);
    Ok(())
}
