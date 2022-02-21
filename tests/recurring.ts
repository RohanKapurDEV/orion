import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Recurring } from "../target/types/recurring";

describe("recurring", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Recurring as Program<Recurring>;

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.rpc.initialize({});
    // console.log("Your transaction signature", tx);
  });
});
