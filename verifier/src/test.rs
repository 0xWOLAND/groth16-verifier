use num_bigint::BigUint;
use num_traits::Num;
use sp1_sdk::SP1ProofWithPublicValues;

#[test]
fn test_verify_from_sp1() {
    use crate::{verify_proof, GROTH16_VK_6_1_0_BYTES};

    // Read the serialized SP1ProofWithPublicValues from the file.
    let sp1_proof_with_public_values_file = "../proofs/fibonacci_proof.bin";
    let sp1_proof_with_public_values =
        SP1ProofWithPublicValues::load(sp1_proof_with_public_values_file).unwrap();

    let proof_bytes = sp1_proof_with_public_values.bytes();
    let sp1_public_inputs = sp1_proof_with_public_values.public_values.to_vec();

    let proof = sp1_proof_with_public_values
        .proof
        .try_as_groth_16()
        .expect("Failed to convert proof to Groth16 proof");

    // Convert vkey hash to bytes.
    let vkey_hash = BigUint::from_str_radix(&proof.public_inputs[0], 10)
        .unwrap()
        .to_bytes_be();

    let vkey_hash = left_pad_32(&vkey_hash);

    let sp1_vkey_hash = format!("0x{}", hex::encode(vkey_hash));

    assert!(verify_proof(
        &proof_bytes,
        &sp1_public_inputs,
        &sp1_vkey_hash,
        GROTH16_VK_6_1_0_BYTES
    )
    .is_ok());
}

#[test]
fn test_hash_public_inputs_() {
    use crate::utils::{hash_public_inputs, hash_public_inputs_blake3};

    // Read the serialized SP1ProofWithPublicValues from the file.
    let sp1_proof_with_public_values_file = "../proofs/fibonacci_proof.bin";
    let sp1_proof_with_public_values =
        SP1ProofWithPublicValues::load(sp1_proof_with_public_values_file).unwrap();

    let proof = sp1_proof_with_public_values
        .proof
        .try_as_groth_16()
        .expect("Failed to convert proof to Groth16 proof");

    let public_values_digest = BigUint::from_str_radix(&proof.public_inputs[1], 10)
        .unwrap()
        .to_bytes_be();

    let public_values_digest = left_pad_32(&public_values_digest);

    let public_values = sp1_proof_with_public_values.public_values.to_vec();
    assert!(
        public_values_digest == hash_public_inputs(&public_values)
            || public_values_digest == hash_public_inputs_blake3(&public_values)
    );
}

#[test]
fn test_decode_sp1_vkey_hash() {
    use crate::utils::decode_sp1_vkey_hash;

    let sp1_vkey_hash = "0x0054c0e58911dd8b993c6d8f249aa50a2e523114ec4b7ef9dd355c5f6bfbf3ce";
    let decoded_sp1_vkey_hash = decode_sp1_vkey_hash(sp1_vkey_hash).unwrap();
    assert_eq!(
        decoded_sp1_vkey_hash,
        hex_literal::hex!("0054c0e58911dd8b993c6d8f249aa50a2e523114ec4b7ef9dd355c5f6bfbf3ce")
    );
}

fn left_pad_32(bytes: &[u8]) -> Vec<u8> {
    assert!(bytes.len() <= 32);
    let mut padded = vec![0u8; 32 - bytes.len()];
    padded.extend_from_slice(bytes);
    padded
}
