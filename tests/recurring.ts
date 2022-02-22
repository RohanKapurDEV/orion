import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Recurring } from "../target/types/recurring";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { SystemProgram } from "@solana/web3.js";
import { bnTo8 } from "./utils";
import BN from "bn.js";

describe("recurring", async () => {
  // Configure the client to use the local cluster.
  anchor.Provider.env();

  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Recurring as Program<Recurring>;

  const index = 0;
  const payer = anchor.web3.Keypair.generate();
  const authority = anchor.web3.Keypair.generate();
  const [merchantAuthority, merchantAuthorityBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("merchant_authority"),
        bnTo8(new BN(index)),
        authority.publicKey.toBytes(),
      ],
      program.programId
    );

  it("Create MerchantAuthority account!", async () => {
    let tx = await program.methods
      .initializeMerchantAuthority(index)
      .accounts({
        payer: payer.publicKey,
        merchantAuthority: merchantAuthority,
        authority: authority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([payer])
      .rpc();

    console.log(tx);
  });
});
