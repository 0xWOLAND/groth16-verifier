use anchor_lang::prelude::*;
use sp1_solana::verify_proof;

declare_id!("Dss5a3USoyQZaHRY8SYFxYW1KLzRDKrXpEQs1C4k5ZHY");

/// Derived as follows:
///
/// ```
/// let client = sp1_sdk::ProverClient::new();
/// let (pk, vk) = client.setup(YOUR_ELF_HERE);
/// let vkey_hash = vk.bytes32();
/// ```
const FIBONACCI_VKEY_HASH: &str =
    "0x00bb9e57314d7ee4f65a4b9fb46fbeae0495f2015c5a8a737333680ce6bb424e";

#[account]
pub struct SP1Groth16Proof {
    pub proof: [u8; 256],
    pub sp1_public_inputs: [u8; 64],
}

#[program]
pub mod anchor {
    use super::*;

    pub fn verify(_ctx: Context<Verify>, groth16_proof: SP1Groth16Proof) -> Result<()> {
        if groth16_proof.proof.len() != 256 {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        // Get the SP1 Groth16 verification key from the `sp1-solana` crate
        let vk = sp1_solana::GROTH16_VK_5_0_0_BYTES;

        // Verify proof
        verify_proof(
            &groth16_proof.proof,
            &groth16_proof.sp1_public_inputs,
            FIBONACCI_VKEY_HASH,
            vk,
        )
        .map_err(|_| ProgramError::InvalidInstructionData)?;

        // Print out the public values.
        let mut reader = &groth16_proof.sp1_public_inputs[..];
        let n = u32::deserialize(&mut reader).unwrap();
        let a = u32::deserialize(&mut reader).unwrap();
        let b = u32::deserialize(&mut reader).unwrap();
        msg!("Public values: (n: {}, a: {}, b: {})", n, a, b);

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(groth16_proof: SP1Groth16Proof)]
pub struct Verify {}
