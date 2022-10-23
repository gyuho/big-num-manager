pub mod serde_format;

use std::io::{self, Error, ErrorKind};

use num_bigint::BigInt;

/// Parses the big.Int encoded in hex.
/// "0x52B7D2DCC80CD2E4000000" is "100000000000000000000000000" (100,000,000 AVAX).
/// "0x5f5e100" or "0x5F5E100" is "100000000".
/// "0x1312D00" is "20000000".
/// ref. https://www.rapidtables.com/convert/number/hex-to-decimal.html
pub fn from_hex_to_big_int(s: &str) -> io::Result<BigInt> {
    let sb = s.trim_start_matches("0x").as_bytes();

    // ref. https://docs.rs/num-bigint/latest/num_bigint/struct.BigInt.html
    let b = match BigInt::parse_bytes(sb, 16) {
        Some(v) => v,
        None => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("failed to parse hex big int {} (parse returned None)", s),
            ));
        }
    };
    Ok(b)
}

/// ref. https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html
pub fn big_int_to_upper_hex(v: &BigInt) -> String {
    format!("{:#X}", v)
}

/// ref. https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html
pub fn big_int_to_lower_hex(v: &BigInt) -> String {
    format!("{:#x}", v)
}

/// RUST_LOG=debug cargo test --all-features --package big-num-manager --lib -- test_hex --exact --show-output
#[test]
fn test_hex() {
    use num_bigint::ToBigInt;

    let big_num = BigInt::default();
    assert_eq!(from_hex_to_big_int("0x0").unwrap(), big_num);

    let big_num = ToBigInt::to_bigint(&100000000000000000000000000_i128).unwrap();
    assert_eq!(
        from_hex_to_big_int("0x52B7D2DCC80CD2E4000000").unwrap(),
        big_num
    );
    assert_eq!(big_int_to_upper_hex(&big_num), "0x52B7D2DCC80CD2E4000000",);

    let big_num = ToBigInt::to_bigint(&100000000_i128).unwrap();
    assert_eq!(from_hex_to_big_int("0x5F5E100").unwrap(), big_num);
    assert_eq!(big_int_to_lower_hex(&big_num), "0x5f5e100",);
    assert_eq!(big_int_to_upper_hex(&big_num), "0x5F5E100",);

    let big_num = ToBigInt::to_bigint(&20000000_i128).unwrap();
    assert_eq!(from_hex_to_big_int("0x1312D00").unwrap(), big_num);
    assert_eq!(big_int_to_lower_hex(&big_num), "0x1312d00",);
    assert_eq!(big_int_to_upper_hex(&big_num), "0x1312D00",);
}
