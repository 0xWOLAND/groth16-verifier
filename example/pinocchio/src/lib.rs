#[cfg(not(feature = "no-entrypoint"))]
use pinocchio::{
    account_info::AccountInfo, entrypoint, msg, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};
use sp1_solana::verify_proof;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

#[cfg(not(doctest))]
/// Derived as follows:
///
/// ```
/// let client = sp1_sdk::ProverClient::new();
/// let (pk, vk) = client.setup(YOUR_ELF_HERE);
/// let vkey_hash = vk.bytes32();
/// ```
const FIBONACCI_VKEY_HASH: &str =
    "0x00bb9e57314d7ee4f65a4b9fb46fbeae0495f2015c5a8a737333680ce6bb424e";

/// The instruction data for the program.
pub struct SP1Groth16Proof(*mut u8);

impl SP1Groth16Proof {
    pub const LEN: usize = 256 + 64;

    #[inline(always)]
    pub fn from_instruction_data(instruction_data: &[u8]) -> Self {
        Self(instruction_data.as_ptr() as *mut u8)
    }

    #[inline(always)]
    pub fn proof(&self) -> [u8; 256] {
        unsafe { *(self.0 as *const [u8; 256]) }
    }

    #[inline(always)]
    pub fn sp1_public_inputs(&self) -> [u8; 64] {
        unsafe { *(self.0.add(256) as *const [u8; 64]) }
    }
}

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() != SP1Groth16Proof::LEN {
        return Err(ProgramError::InvalidInstructionData);
    }
    // Deserialize the SP1Groth16Proof from the instruction data.
    let groth16_proof = SP1Groth16Proof::from_instruction_data(instruction_data);

    // Get the SP1 Groth16 verification key from the `sp1-solana` crate.
    let vk = sp1_solana::GROTH16_VK_5_0_0_BYTES;

    // Verify the proof.
    verify_proof(
        &groth16_proof.proof(),
        &groth16_proof.sp1_public_inputs(),
        FIBONACCI_VKEY_HASH,
        vk,
    )
    .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Print out the public values.
    let pi = groth16_proof.sp1_public_inputs();
    let n = unsafe { *(pi.as_ptr() as *const u32) };
    let a = unsafe { *(pi.as_ptr().add(4) as *const u32) };
    let b = unsafe { *(pi.as_ptr().add(8) as *const u32) };
    let values = String::from(format!("n: {}, a: {}, b: {}", n, a, b));
    msg!(&values);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use five8_const::decode_32_const;

    #[test]
    fn test_from_instruction_data_roundtrip() {
        let mut buf = [0u8; SP1Groth16Proof::LEN];
        for (i, b) in buf.iter_mut().enumerate() {
            *b = (i % 251) as u8;
        }
        let proof = SP1Groth16Proof::from_instruction_data(&buf);
        assert_eq!(&proof.proof()[..], &buf[0..256]);
        assert_eq!(&proof.sp1_public_inputs()[..], &buf[256..320]);
    }

    #[test]
    fn test_instruction_processing_with_invalid_data() {
        let program_id = Pubkey::from(decode_32_const(
            "99999999999999999999999999999999999999999999",
        ));
        let accounts = vec![];
        let invalid_data = vec![1, 2, 3];
        let result = process_instruction(&program_id, &accounts, &invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_instruction_processing_with_valid_structure() {
        let program_id = Pubkey::from(decode_32_const(
            "99999999999999999999999999999999999999999999",
        ));
        let accounts = vec![];
        let mut data = vec![0u8; SP1Groth16Proof::LEN];
        data[256..260].copy_from_slice(&5u32.to_le_bytes());
        data[260..264].copy_from_slice(&8u32.to_le_bytes());
        data[264..268].copy_from_slice(&13u32.to_le_bytes());
        let result = process_instruction(&program_id, &accounts, &data);
        assert!(result.is_err());
    }

    #[test]
    fn test_fibonacci_vkey_hash_constant() {
        assert_eq!(FIBONACCI_VKEY_HASH.len(), 66);
        assert!(FIBONACCI_VKEY_HASH.starts_with("0x"));
    }
}
