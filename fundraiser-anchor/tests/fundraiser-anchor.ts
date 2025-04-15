import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FundraiserAnchor } from "../target/types/fundraiser_anchor";

describe("fundraiser-anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.FundraiserAnchor as Program<FundraiserAnchor>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
