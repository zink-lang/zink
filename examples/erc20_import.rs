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
    fn test_erc20_import() -> anyhow::Result<()> {
        #[cfg(feature = "abi-import")]
        {
            import!("examples/ERC20.json", "my_erc20");

            let contract_address = Address::from(revm::CONTRACT);
            let token = ERC20::new(contract_address);

            // Test decimals()
            match token.decimals() {
                Ok(decimals) => {
                    assert_eq_no_std!(decimals, 18, "Expected decimals to be 18");
                }
                Err(_) => panic!(),
            }

            // Test balance_of()
            let owner_address = Address::from(revm::ALICE);
            match token.balance_of(owner_address) {
                Ok(balance) => {
                    assert_eq_no_std!(balance, U256::from(1000), "Expected balance to be 1000");
                }
                Err(_) => panic!(),
            }
        }
        Ok(())
    }

    #[test]
    fn test_erc20_approve() -> anyhow::Result<()> {
        #[cfg(feature = "abi-import")]
        {
            import!("examples/ERC20.json", "my_erc20");

            let contract_address = Address::from(revm::CONTRACT);
            let token = ERC20::new(contract_address);

            let owner = Address::from(revm::ALICE);
            let spender = Address::from([42u8; 20]);
            let value = U256::from(100);

            match token.approve(spender, value) {
                Ok(success) => {
                    assert_eq_no_std!(success, true, "Expected approve to return true");
                    let mut evm = token.evm.clone();
                    let storage_key =
                        zink::storage::DoubleKeyMapping::<Address, Address, U256>::storage_key(
                            owner, spender,
                        );
                    let allowance_bytes = evm
                        .storage(*contract_address.as_bytes(), storage_key)
                        .unwrap_or([0u8; 32]);
                    let allowance = U256::from_be_bytes(allowance_bytes);
                    assert_eq_no_std!(allowance, value, "Expected allowance to be 100");
                }
                Err(_) => panic!(),
            }
        }
        Ok(())
    }

    #[test]
    fn test_erc20_allowance() -> anyhow::Result<()> {
        #[cfg(feature = "abi-import")]
        {
            import!("examples/ERC20.json", "my_erc20");

            let contract_address = Address::from(revm::CONTRACT);
            let token = ERC20::new(contract_address);

            let owner = Address::from(revm::ALICE);
            let spender = Address::from([42u8; 20]);
            let _ = token.approve(spender, U256::from(200)).unwrap();

            match token.allowance(owner, spender) {
                Ok(allowance) => {
                    assert_eq_no_std!(allowance, U256::from(200), "Expected allowance to be 200");
                }
                Err(_) => panic!(),
            }
        }
        Ok(())
    }

    #[test]
    fn test_erc20_spend_allowance() -> anyhow::Result<()> {
        #[cfg(feature = "abi-import")]
        {
            import!("examples/ERC20.json", "my_erc20");

            let contract_address = Address::from(revm::CONTRACT);
            let token = ERC20::new(contract_address);

            let owner = Address::from(revm::ALICE);
            let spender = Address::from([42u8; 20]);
            let initial_value = U256::from(100);
            let spend_value = U256::from(50);

            let _ = token.approve(spender, initial_value).unwrap();

            match token.spend_allowance(spender, spend_value) {
                Ok(success) => {
                    assert_eq_no_std!(success, true, "Expected spend_allowance to return true");
                    let mut evm = token.evm.clone();
                    let storage_key =
                        zink::storage::DoubleKeyMapping::<Address, Address, U256>::storage_key(
                            owner, spender,
                        );
                    let allowance_bytes = evm
                        .storage(*contract_address.as_bytes(), storage_key)
                        .unwrap_or([0u8; 32]);
                    let allowance = U256::from_be_bytes(allowance_bytes);
                    assert_eq_no_std!(
                        allowance,
                        initial_value - spend_value,
                        "Expected allowance to be 50"
                    );
                }
                Err(_) => panic!(),
            }
        }
        Ok(())
    }

    #[test]
    fn test_erc20_transfer() -> anyhow::Result<()> {
        #[cfg(feature = "abi-import")]
        {
            import!("examples/ERC20.json", "my_erc20");

            let contract_address = Address::from(revm::CONTRACT);
            let token = ERC20::new(contract_address);

            let owner = Address::from(revm::ALICE);
            let recipient = Address::from([42u8; 20]);
            let amount = U256::from(10);

            match token.transfer(recipient, amount) {
                Ok(success) => {
                    assert_eq_no_std!(success, true, "Expected transfer to return true");
                    let mut evm = token.evm.clone();
                    let sender_key = zink::storage::Mapping::<Address, U256>::storage_key(owner);
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
                        U256::from(990),
                        "Expected sender balance to be 990"
                    );
                    assert_eq_no_std!(
                        recipient_balance,
                        U256::from(10),
                        "Expected recipient balance to be 10"
                    );
                }
                Err(_) => panic!(),
            }
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
