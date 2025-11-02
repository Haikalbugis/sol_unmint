use sol_unmint::{TokenProgram, Unmint};

pub fn setup_unmint(token_program: TokenProgram) -> Unmint {
    Unmint::new("https://api.mainnet-beta.solana.com", token_program)
}
