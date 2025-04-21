import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FundraiserAnchor } from "../target/types/fundraiser_anchor";
import {  createMint,  getAssociatedTokenAddressSync,  getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("fundraiser", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  console.log("provider", provider);

  const program = anchor.workspace.FundraiserAnchor as Program<FundraiserAnchor>;
  console.log("program", program);

  const admin = anchor.web3.Keypair.generate();
  console.log("admin", admin);

  let which_mint: anchor.web3.PublicKey;
  console.log("which_mint", which_mint);
  
  let contributorATA: anchor.web3.PublicKey;
  console.log("contributorATA", contributorATA);

  let adminATA: anchor.web3.PublicKey;
  console.log("adminATA", adminATA);

  const wallet = provider.wallet as NodeWallet;
  console.log("wallet", wallet);

  const fundraiser = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("fundraiser"), admin.publicKey.toBuffer()], program.programId)[0];
  console.log("fundraiser", fundraiser.toBase58());

  const contributor = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("contributor"), fundraiser.toBuffer(), provider.publicKey.toBuffer()], program.programId)[0];
  console.log("contributor", contributor.toBase58());

  const confirm = async (signature: string): Promise<string> => {
    const block = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  it("airdrop and mint", async () => {
    const airdrop = await provider.connection.requestAirdrop(admin.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL).then(confirm);
    console.log("airdropped 1 SOL to maker", airdrop);

    which_mint = await createMint(provider.connection, wallet.payer, provider.publicKey, provider.publicKey, 6);
    console.log("Mint created", which_mint.toBase58());

    contributorATA = (await getOrCreateAssociatedTokenAccount(provider.connection, wallet.payer, which_mint, wallet.publicKey)).address;

    adminATA = (await getOrCreateAssociatedTokenAccount(provider.connection, wallet.payer, which_mint, admin.publicKey)).address;

    const mintTx = await mintTo(provider.connection, wallet.payer, which_mint, contributorATA, wallet.publicKey, 1_000_000_0);
    console.log("Minted 10 tokens to contributor", mintTx);
  
  })

  it("init fundraiser", async () => {
    const vault = getAssociatedTokenAddressSync(which_mint, fundraiser, true);

    const tx = await program.methods.initialize(new anchor.BN(10000000), 0)
    .accountsPartial({
        admin: admin.publicKey, 
        whichMint: which_mint,
        fundraiser,
        vault,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID
      })
    .signers([admin])
    .rpc()
    .then(confirm)
    console.log("tx", tx);
  })

  it("contribute", async () => {
    const vault = getAssociatedTokenAddressSync(which_mint, fundraiser, true);

    const tx = await program.methods.contribute(new anchor.BN(1000000))
    .accountsPartial({
        contributor: provider.publicKey,
        whichMint: which_mint,
        fundraiser,
        contributorAcc: contributor,
        contributorAccAta: contributorATA,
        vault,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID
      })
      .rpc()
      .then(confirm)

    console.log(tx);
    console.log("Your transaction signature", tx);
    console.log("Vault balance", (await provider.connection.getTokenAccountBalance(vault)).value.amount);

    let contributorAccount = await program.account.contributor.fetch(contributor);
    console.log("Contributor balance", contributorAccount.amount.toString());

  })

  // it("done", async() => {
  //   const vault = getAssociatedTokenAddressSync(which_mint, fundraiser, true);
  //
  //   const tx = await program.methods.done()
  //   .accountsPartial({
  //     admin: admin.publicKey,
  //     whichMint: which_mint,
  //     fundraiser,
  //     vault,
  //     adminAta: adminATA,
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //     associatedTokenProgram: ASSOCIATED_PROGRAM_ID
  //   })
  //   .signers([admin])
  //   .rpc()
  //   .then(confirm)
  //
  //   console.log(tx);
  //
  //
  // })

  it("refund", async() => {
    const vault = getAssociatedTokenAddressSync(which_mint, fundraiser, true);

    let contributorAccount = await program.account.contributor.fetch(contributor);
    console.log("Contributor balance", contributorAccount.amount.toString());
    
    const tx = await program.methods.refund()
    .accountsPartial({
        contributor: provider.publicKey,
        admin: admin.publicKey,
        whichMint: which_mint,
        fundraiser,
        contributorAcc: contributor,
        contributorAccAta: contributorATA,
        vault,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID
      })
    .rpc()
    .then(confirm)
    console.log(tx);
    console.log("Vault balance", (await provider.connection.getTokenAccountBalance(vault)).value.amount);
  })
});
