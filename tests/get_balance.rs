use anyhow::Result;
use sol_unmint::TokenProgram;

use crate::setup_unmint::setup_unmint;
mod setup_unmint;

#[test]
fn test_get_balance() -> Result<()> {
    let unmint = setup_unmint(TokenProgram::Legacy);

    let balance = unmint.balance(
        "57ksuWYrkEnrUDfisoPYw6Wb1hmsjFBYSwv9HULex1yj",
        "J6pQQ3FAcJQeWPPGppWRb4nM8jU3wLyYbRrLh7feMfvd",
    );

    println!("Balance: {:?}", balance);
    Ok(())
}
