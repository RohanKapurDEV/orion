import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LendingController } from "../target/types/lending_controller";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  mintToChecked,
  createAccount,
} from "@solana/spl-token";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { bnTo1, delay } from "./utils";
import BN from "bn.js";

describe("lending-controller", async () => {
  const provider = anchor.Provider.env();

  anchor.setProvider(anchor.Provider.env());
  anchor.Provider.local(undefined, {
    commitment: "finalized",
    preflightCommitment: "finalized",
  });

  it("Initialize merchant authority", async () => {});
});
