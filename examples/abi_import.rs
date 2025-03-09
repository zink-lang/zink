use zink::primitives::address::Address;
use zink::primitives::u256::U256;
use tiny_keccak::Hasher;

#[cfg(feature = "abi-import")]
use zink::import;

fn main() {
    #[cfg(feature = "abi-import")]
    run_example();

    #[cfg(not(feature = "abi-import"))]
    println!("This example requires the abi-import feature to be enabled.");
}

#[cfg(feature = "abi-import")]
fn run_example() {
    println!("Testing ERC20 ABI Import...");
    
    // import the ERC20 contract ABI
    // this should generate a struct called 'ERC20'
    import!("examples/ERC20.json");
    
    let contract_address = Address::empty(); 
    
    let token = ERC20::new(contract_address);
    
    match token.decimals() {
        Ok(decimals) => println!("Token has {} decimals", decimals),
        Err(e) => println!("Error fetching decimals: {}", e),
    }
    
    let owner_address = Address::empty(); 
    match token.balance_of(owner_address) {
        Ok(balance) => {
            println!("Token balance: [U256 value]");
        },
        Err(e) => println!("Error fetching balance: {}", e),
    }
    
    let recipient = Address::empty();
    
    let amount = U256::empty();
    
    match token.transfer(recipient, amount) {
        Ok(success) => println!("Transfer successful: {}", success),
        Err(e) => println!("Transfer failed: {}", e),
    }
    
    println!("ERC20 ABI Import test complete!");
}