import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Lmsol2 } from "../target/types/lmsol2";

describe("lmsol2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Lmsol2 as Program<Lmsol2>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
