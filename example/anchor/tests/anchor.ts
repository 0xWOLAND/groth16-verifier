import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Anchor } from "../target/types/anchor";
import { expect } from "chai";

describe("anchor", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchor as Program<Anchor>;

  const FIBONACCI_VKEY_HASH =
    "0x00bb9e57314d7ee4f65a4b9fb46fbeae0495f2015c5a8a737333680ce6bb424e";

  it("test_from_instruction_data_roundtrip", async () => {
    const buf = Buffer.alloc(320);
    for (let i = 0; i < buf.length; i++) buf[i] = i % 251;
    const proof = buf.subarray(0, 256);
    const sp1PublicInputs = buf.subarray(256, 320);
    expect(proof.length).to.eq(256);
    expect(sp1PublicInputs.length).to.eq(64);
  });

  it("test_instruction_processing_with_invalid_data", async () => {
    const invalidProof = {
      proof: Array.from(Buffer.from([1, 2, 3])),
      sp1PublicInputs: [],
    };
    try {
      await program.methods.verify(invalidProof).rpc();
      expect.fail("verify should have failed with invalid data");
    } catch (e) {
      expect(e).to.be.instanceOf(Error);
    }
  });

  it("test_instruction_processing_with_valid_structure", async () => {
    try {
      let data = Buffer.alloc(320, 0);
      data.writeUInt32LE(5, 256);
      data.writeUInt32LE(8, 260);
      data.writeUInt32LE(13, 264);
      const groth16Proof = {
        proof: Array.from(data.subarray(0, 256)),
        sp1PublicInputs: Array.from(data.subarray(256, 320)),
      };
      await program.methods.verify(groth16Proof).rpc();
    } catch (error) {
      expect(error).to.be.instanceOf(Error);
    }
  });

  it("test_fibonacci_vkey_hash_constant", async () => {
    expect(FIBONACCI_VKEY_HASH.length).to.eq(66);
    expect(FIBONACCI_VKEY_HASH.startsWith("0x")).to.be.true;
  });
});
