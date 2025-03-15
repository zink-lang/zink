#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

#[cfg(feature = "abi-import")]
use zink::import;

// Custom assertion macro for no_std
#[allow(unused_macros)]
macro_rules! assert_eq_no_std {
    ($left:expr, $right:expr, $msg:literal) => {
        if $left != $right {
            panic!();
        }
    };
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use zink::primitives::address::Address;
    #[allow(unused_imports)]
    use zink::primitives::u256::U256;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_my_erc20_full_flow() -> anyhow::Result<()> {
        #[cfg(feature = "abi-import")]
        {
            import!("examples/ERC20.json", "my_erc20");

            let contract_address = Address::from(revm::CONTRACT);
            let token = ERC20::new(contract_address);

            let owner = Address::from(revm::ALICE);
            let spender = Address::from([42u8; 20]);
            let recipient = Address::from([43u8; 20]);
            let approve_amount = U256::from(500);
            let transfer_amount = U256::from(300);

            // Test initial balance
            let initial_balance = token.balance_of(owner).unwrap();
            assert_eq_no_std!(
                initial_balance,
                U256::from(1000),
                "Expected initial balance to be 1000"
            );

            // Approve spender
            match token.approve(spender, approve_amount) {
                Ok(success) => assert_eq_no_std!(success, true, "Expected approve to succeed"),
                Err(_) => panic!(),
            }

            // Check allowance
            let allowance = token.allowance(owner, spender).unwrap();
            assert_eq_no_std!(allowance, approve_amount, "Expected allowance to be 500");

            // Transfer tokens
            match token.transfer(recipient, transfer_amount) {
                Ok(success) => assert_eq_no_std!(success, true, "Expected transfer to succeed"),
                Err(_) => panic!(),
            }

            // Verify balances after transfer
            let owner_balance = token.balance_of(owner).unwrap();
            let recipient_balance = token.balance_of(recipient).unwrap();
            assert_eq_no_std!(
                owner_balance,
                U256::from(700),
                "Expected owner balance to be 700"
            );
            assert_eq_no_std!(
                recipient_balance,
                transfer_amount,
                "Expected recipient balance to be 300"
            );

            // Spend allowance
            match token.spend_allowance(spender, U256::from(200)) {
                Ok(success) => {
                    assert_eq_no_std!(success, true, "Expected spend_allowance to succeed")
                }
                Err(_) => panic!(),
            }

            // Check updated allowance
            let updated_allowance = token.allowance(owner, spender).unwrap();
            assert_eq_no_std!(
                updated_allowance,
                U256::from(300),
                "Expected allowance to be 300"
            );
        }
        Ok(())
    }

    #[test]
    fn test_erc20_edge_case_empty_address() -> anyhow::Result<()> {
        #[cfg(feature = "abi-import")]
        {
            import!("examples/ERC20.json", "my_erc20");

            let contract_address = Address::from(revm::CONTRACT);
            let token = ERC20::new(contract_address);

            let recipient = Address::empty();
            let amount = U256::from(0);
            match token.transfer(recipient, amount) {
                Ok(_) => panic!(),
                Err(e) => assert_eq_no_std!(
                    e,
                    "ERC20: Transfer to zero address",
                    "Expected transfer to revert"
                ),
            }
        }
        Ok(())
    }

    #[test]
    fn test_erc20_name() -> anyhow::Result<()> {
        #[cfg(feature = "abi-import")]
        {
            import!("examples/ERC20.json", "my_erc20");

            let contract_address = Address::from(revm::CONTRACT);
            let token = ERC20::new(contract_address);

            match token.name() {
                Ok(name) => {
                    assert_eq_no_std!(
                        name,
                        String::from("ZinkToken"),
                        "Expected name to be 'ZinkToken'"
                    );
                }
                Err(_) => panic!(),
            }
        }
        Ok(())
    }

    #[test]
    fn test_erc20_symbol() -> anyhow::Result<()> {
        #[cfg(feature = "abi-import")]
        {
            import!("examples/ERC20.json", "my_erc20");

            let contract_address = Address::from(revm::CONTRACT);
            let token = ERC20::new(contract_address);

            match token.symbol() {
                Ok(symbol) => {
                    assert_eq_no_std!(symbol, String::from("ZTK"), "Expected symbol to be 'ZTK'");
                }
                Err(_) => panic!(),
            }
        }
        Ok(())
    }

    #[test]
    fn test_erc20_large_amount() -> anyhow::Result<()> {
        #[cfg(feature = "abi-import")]
        {
            import!("examples/ERC20.json", "my_erc20");

            let contract_address = Address::from(revm::CONTRACT);
            let token = ERC20::new(contract_address);

            let recipient = Address::from([42u8; 20]);
            let amount = U256::from(1000); // Within ALICE's balance
            match token.transfer(recipient, amount) {
                Ok(success) => {
                    assert_eq_no_std!(success, true, "Expected transfer to return true");
                    let mut evm = token.evm.clone();
                    let sender_key = zink::storage::Mapping::<Address, U256>::storage_key(
                        Address::from(revm::ALICE),
                    );
                    let recipient_key =
                        zink::storage::Mapping::<Address, U256>::storage_key(recipient);
                    let sender_balance = U256::from_be_bytes(
                        evm.storage(*contract_address.as_bytes(), sender_key)
                            .unwrap_or([0u8; 32]),
                    );
                    let recipient_balance = U256::from_be_bytes(
                        evm.storage(*contract_address.as_bytes(), recipient_key)
                            .unwrap_or([0u8; 32]),
                    );
                    assert_eq_no_std!(
                        sender_balance,
                        U256::from(0),
                        "Expected sender balance to be 0"
                    );
                    assert_eq_no_std!(
                        recipient_balance,
                        U256::from(1000),
                        "Expected recipient balance to be 1000"
                    );
                }
                Err(_) => panic!(),
            }

            let spender = Address::from([42u8; 20]);
            match token.approve(spender, amount) {
                Ok(success) => {
                    assert_eq_no_std!(success, true, "Expected approve to return true");
                }
                Err(_) => panic!(),
            }
        }
        Ok(())
    }
}
