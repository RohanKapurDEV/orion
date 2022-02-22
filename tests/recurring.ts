import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Recurring } from "../target/types/recurring";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { SystemProgram } from "@solana/web3.js";

describe("recurring", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();

  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Recurring as Program<Recurring>;

  const payer = anchor.web3.Keypair.generate();
  const authority = anchor.web3.Keypair.generate();
  const merchantAuthority = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    const tx = await program.rpc.initializeMerchantAuthority({
      accounts: {
        authority: authority.publicKey,
        merchantAuthority: merchantAuthority.publicKey,
        payer: payer.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [payer],
    });

    console.log("Your transaction signature", tx);

    const merchantAuthorityAccount =
      await program.account.merchantAuthority.fetch(
        merchantAuthority.publicKey
      );

    console.log(merchantAuthorityAccount);
  });
});
