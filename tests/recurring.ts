import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Recurring } from "../target/types/recurring";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { SystemProgram } from "@solana/web3.js";
import { bnTo8, bnTo1 } from "./utils";
import BN from "bn.js";

describe("recurring", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();

  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Recurring as Program<Recurring>;

  const index = 0;
  const payer = anchor.web3.Keypair.generate();
  const authority = anchor.web3.Keypair.generate();

  let [merchantAuthority, _merchantAuthorityBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("merchant_authority"),
        bnTo1(new BN(index)),
        authority.publicKey.toBytes(),
      ],
      program.programId
    );

  console.log("Payer public key: " + payer.publicKey.toString());
  console.log("Authority public key: " + authority.publicKey.toString());
  console.log("MerchantAuthority public key: " + merchantAuthority.toString());

  // Airdrop some sweet, sweet lamports first
  before(async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer.publicKey, 10_000_000_000),
      "confirmed"
    );
  });

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

    console.log("initialize_merchant_account tx sig: " + tx);

    let merchant_authority_account =
      await program.account.merchantAuthority.fetch(merchantAuthority);

    console.log(merchant_authority_account);
  });

  it("Create PaymentConfig account!", async () => {});
  it("Create PaymentMetadata account!", async () => {});
  it("Collect payment from PaymentMetadata account!", async () => {});
  it("Transfer MerchantAuthority account!", async () => {});
  it("Accept MerchantAuthority account!", async () => {});
  it("Close PaymentMetadata account!", async () => {});
  it("Close PaymentConfig account!", async () => {});
  it("Close MerchantAuthority account!", async () => {});
});
