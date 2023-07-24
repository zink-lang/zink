use zint::EVM;

const ADDITION_BYTECODE: &str = "6000356020350160005260206000f3";
const INPUT: &str = "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002";
const RESULT: &str = "0000000000000000000000000000000000000000000000000000000000000003";

#[test]
fn addition() {
    let info = EVM::run(
        &hex::decode(ADDITION_BYTECODE).expect("Invalid bytecode"),
        &hex::decode(INPUT).expect("Invalid input"),
    );

    assert_eq!(info.ret, hex::decode(RESULT).expect("Invalid result"));
}
