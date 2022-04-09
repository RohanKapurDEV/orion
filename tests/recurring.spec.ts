import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Recurring } from "../target/types/recurring";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  mintToChecked,
  createAccount,
} from "@solana/spl-token";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { bnTo1, delay } from "./utils";
import BN from "bn.js";

describe("recurring", async () => {
  const provider = anchor.Provider.env();

  anchor.Provider.local(undefined, {
    commitment: "finalized",
    preflightCommitment: "finalized",
  });

  const program = anchor.workspace.Recurring as Program<Recurring>;

  const mintDecimals = 6;
  const merchantAuthorityIndex = 0;
  const payer = anchor.web3.Keypair.generate();
  const authority = anchor.web3.Keypair.generate();

  // Update these together
  const delayInSeconds = 10;
  const delayInMilliseconds = 10000;

  let [merchantAuthority, _merchantAuthorityBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("merchant_authority"),
        bnTo1(new BN(merchantAuthorityIndex)),
        authority.publicKey.toBytes(),
      ],
      program.programId
    );

  const paymentConfigIndex = 0;
  const [paymentConfig, _paymentConfigBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("payment_config"),
        bnTo1(new BN(paymentConfigIndex)),
        merchantAuthority.toBytes(),
      ],
      program.programId
    );
  const paymentTokenAccount = anchor.web3.Keypair.generate(); // PaymentConfig payment_token_account

  let paymentMint = PublicKey.default;

  const consumer = anchor.web3.Keypair.generate(); // Payer in the initialize_payment_metadata instruction
  let ownerPaymentAccount: PublicKey;

  const [paymentMetadata, _paymentMetadataBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("payment_metadata"),
        consumer.publicKey.toBytes(),
        paymentConfig.toBytes(),
      ],
      program.programId
    );

  const [programAsSigner, programAsSignerBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("program"), Buffer.from("signer")],
      program.programId
    );

  const newAuthority = anchor.web3.Keypair.generate();

  // Airdrop some sweet, sweet lamports first
  before(async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer.publicKey, 10_000_000_000), // 10 SOL
      "confirmed"
    );

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        authority.publicKey,
        10_000_000_000
      ), // 10 SOL
      "confirmed"
    );

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        consumer.publicKey,
        10_000_000_000
      ), // 10 SOL
      "confirmed"
    );

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        newAuthority.publicKey,
        10_000_000_000
      ), // 10 SOL
      "confirmed"
    );
  });

  console.log("Payer public key: " + payer.publicKey.toString());
  console.log("Authority public key: " + authority.publicKey.toString());
  console.log("MerchantAuthority public key: " + merchantAuthority.toString());
  console.log("PaymentConfig public key: " + paymentConfig.toString());
  console.log("PaymentMint public key: " + paymentMint.toString());

  it("Create MerchantAuthority account!", async () => {
    await program.methods
      .initializeMerchantAuthority(merchantAuthorityIndex)
      .accounts({
        payer: payer.publicKey,
        merchantAuthority: merchantAuthority,
        authority: authority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([payer])
      .rpc();
  });

  it("Create PaymentConfig account!", async () => {
    let x = await createMint(
      provider.connection,
      payer,
      payer.publicKey,
      payer.publicKey,
      mintDecimals
    );
    paymentMint = x;

    let paymentConfigParams = {
      index: 0,
      spacingPeriod: delayInSeconds,
      collectOnInit: true,
      amountToCollectOnInit: 10 * Math.pow(10, mintDecimals),
      amountToCollectPerPeriod: 10 * Math.pow(10, mintDecimals),
    };

    let tx = await program.methods
      .initializePaymentConfig(
        paymentConfigParams.index,
        new BN(paymentConfigParams.spacingPeriod),
        paymentConfigParams.collectOnInit,
        new BN(paymentConfigParams.amountToCollectOnInit),
        new BN(paymentConfigParams.amountToCollectPerPeriod.toString())
      )
      .accounts({
        payer: authority.publicKey,
        paymentConfig: paymentConfig,
        merchantAuth: merchantAuthority,
        paymentMint: paymentMint,
        paymentTokenAccount: paymentTokenAccount.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([authority, paymentTokenAccount])
      .rpc();
  });

  it("Create PaymentMetadata account!", async () => {
    // Create ownerPaymentAccount and mint some tokens to it
    let x = await createAccount(
      provider.connection,
      payer,
      paymentMint,
      consumer.publicKey
    );
    ownerPaymentAccount = x;

    await mintToChecked(
      provider.connection,
      payer,
      paymentMint,
      ownerPaymentAccount,
      payer,
      10000000000,
      mintDecimals
    );

    let paymentMetadataParams = {
      amountDelegated: 50 * Math.pow(10, mintDecimals), // Must match paymentConfigParams.amountToCollectPerPeriod
    };

    await program.methods
      .initializePaymentMetadata(new BN(paymentMetadataParams.amountDelegated))
      .accounts({
        payer: consumer.publicKey,
        paymentMetadata: paymentMetadata,
        paymentConfig: paymentConfig,
        ownerPaymentAccount: ownerPaymentAccount,
        paymentTokenAccount: paymentTokenAccount.publicKey,
        programAsSigner: programAsSigner,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([consumer])
      .rpc();
  });

  it("Transfer MerchantAuthority account!", async () => {
    await program.methods
      .transferMerchantAuthority(0)
      .accounts({
        payer: authority.publicKey,
        merchantAuthority: merchantAuthority,
        initAuthority: authority.publicKey,
        proposedAuthority: newAuthority.publicKey,
      })
      .signers([authority])
      .rpc();
  });

  it("Collect payment from PaymentMetadata account!", async () => {
    // Delay by paymentConfig.spacerPeriod
    await delay(delayInMilliseconds).then(async () => {
      await program.methods
        .collectPayment()
        .accounts({
          payer: authority.publicKey,
          merchantAuthority: merchantAuthority,
          paymentConfig: paymentConfig,
          paymentMetadata: paymentMetadata,
          ownerPaymentAccount: ownerPaymentAccount,
          paymentTokenAccount: paymentTokenAccount.publicKey,
          programAsSigner: programAsSigner,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([authority])
        .rpc();
    });

    await delay(delayInMilliseconds).then(async () => {
      let tx = await program.methods
        .collectPayment()
        .accounts({
          payer: authority.publicKey,
          merchantAuthority: merchantAuthority,
          paymentConfig: paymentConfig,
          paymentMetadata: paymentMetadata,
          ownerPaymentAccount: ownerPaymentAccount,
          paymentTokenAccount: paymentTokenAccount.publicKey,
          programAsSigner: programAsSigner,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([authority])
        .rpc();
    });

    await delay(delayInMilliseconds).then(async () => {
      let tx = await program.methods
        .collectPayment()
        .accounts({
          payer: authority.publicKey,
          merchantAuthority: merchantAuthority,
          paymentConfig: paymentConfig,
          paymentMetadata: paymentMetadata,
          ownerPaymentAccount: ownerPaymentAccount,
          paymentTokenAccount: paymentTokenAccount.publicKey,
          programAsSigner: programAsSigner,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([authority])
        .rpc();
    });
  });

  it("Merchant can withdraw payments collected from consumers", async () => {
    let x = await createAccount(
      provider.connection,
      payer,
      paymentMint,
      authority.publicKey
    );

    let withdrawFromMerchantTokenAccountParams = {
      paymentConfigIndex: paymentConfigIndex,
      merchantAuthorityIndex: merchantAuthorityIndex,
      amountToWithdraw: 20 * Math.pow(10, mintDecimals),
    };

    await program.methods
      .withdrawFromMerchantTokenAccount(
        withdrawFromMerchantTokenAccountParams.paymentConfigIndex,
        withdrawFromMerchantTokenAccountParams.merchantAuthorityIndex,
        new BN(withdrawFromMerchantTokenAccountParams.amountToWithdraw)
      )
      .accounts({
        payer: authority.publicKey,
        metadataOwner: consumer.publicKey,
        paymentMetadata: paymentMetadata,
        merchantAuthority: merchantAuthority,
        paymentConfig: paymentConfig,
        paymentTokenAccount: paymentTokenAccount.publicKey,
        receiverTokenAccount: x,
        initAuthority: authority.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([authority])
      .rpc();
  });

  it("Accept MerchantAuthority account!", async () => {
    await program.methods
      .acceptMerchantAuthority(0)
      .accounts({
        payer: newAuthority.publicKey,
        merchantAuthority: merchantAuthority,
        initAuthority: authority.publicKey,
      })
      .signers([newAuthority])
      .rpc();
  });

  it("Close PaymentMetadata account!", async () => {
    await program.methods
      .closePaymentMetadata(0, 0)
      .accounts({
        payer: consumer.publicKey,
        paymentMetadata: paymentMetadata,
        ownerPaymentAccount: ownerPaymentAccount,
        merchantAuthority: merchantAuthority,
        paymentConfig: paymentConfig,
        initAuthority: authority.publicKey,
        programAsSigner: programAsSigner,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([consumer])
      .rpc();
  });

  it("Close PaymentConfig account!", async () => {
    await program.methods
      .closePaymentConfig(paymentConfigIndex, merchantAuthorityIndex)
      .accounts({
        payer: newAuthority.publicKey,
        merchantAuthority: merchantAuthority,
        paymentConfig: paymentConfig,
        initAuthority: authority.publicKey,
      })
      .signers([newAuthority])
      .rpc();
  });

  it("Close MerchantAuthority account!", async () => {
    await program.methods
      .closeMerchantAuthority(0)
      .accounts({
        payer: newAuthority.publicKey,
        merchantAuthority: merchantAuthority,
        initAuthority: authority.publicKey,
      })
      .signers([newAuthority])
      .rpc();
  });
});
