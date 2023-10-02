//! zink number implementation

/// 256 bit unsigned integer
pub struct U256([u64; 4]);

impl From<&[u8]> for U256 {
    fn from(bytes: &[u8]) -> Self {
        if bytes.len() > 32 {
            return U256([0; 4]);
        }

        let mut src = [0; 32];
        src.copy_from_slice(bytes);

        let mut dst = [0u64; 4];
        for i in 0..4 {
            dst[i] = u64::from_le_bytes(src[i * 8..(i + 1) * 8].try_into().unwrap());
        }

        U256(dst)
    }
}
