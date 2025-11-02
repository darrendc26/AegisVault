import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Aegisvault } from "../target/types/aegisvault";
import { Keypair, PublicKey } from "@solana/web3.js";
import {
  createMint, mintTo, getOrCreateAssociatedTokenAccount,
  getAssociatedTokenAddress, createAssociatedTokenAccountInstruction,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { assert } from "chai";
describe("aegisvault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const connection = anchor.getProvider().connection;
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const program = anchor.workspace.aegisvault as Program<Aegisvault>;

  let admin: Keypair;
  let collateralMint: PublicKey;
  let assetMint: PublicKey;
  let user = anchor.web3.Keypair.generate();

  admin = anchor.web3.Keypair.generate();
  collateralMint = anchor.web3.Keypair.generate().publicKey;
  assetMint = anchor.web3.Keypair.generate().publicKey;
  let userAssetAta: PublicKey;
  let vaultAssetAta: PublicKey;
  before(async () => {
    const airdropSignature = await connection.requestAirdrop(
      admin.publicKey,
      anchor.web3.LAMPORTS_PER_SOL // 1 SOL
    );
    await connection.confirmTransaction({
      signature: airdropSignature,
      blockhash: (await connection.getLatestBlockhash()).blockhash,
      lastValidBlockHeight: (await connection.getLatestBlockhash()).lastValidBlockHeight,
    });

    const airdropSignature2 = await connection.requestAirdrop(
      user.publicKey,
      anchor.web3.LAMPORTS_PER_SOL // 1 SOL
    );
    await connection.confirmTransaction({
      signature: airdropSignature2,
      blockhash: (await connection.getLatestBlockhash()).blockhash,
      lastValidBlockHeight: (await connection.getLatestBlockhash()).lastValidBlockHeight,
    });

    collateralMint = await createMint(
      connection,
      admin,
      admin.publicKey, // mint authority
      null,            // freeze authority
      6                // decimals
    );

    assetMint = await createMint(
      connection,
      admin,
      admin.publicKey,
      null,
      6
    );

    let [vault, vault_bump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("vault"),
      assetMint.toBuffer(),
      collateralMint.toBuffer()],
      program.programId
    );

    userAssetAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,            // 👈 user pays for it
      assetMint,
      user.publicKey
    );

    vaultAssetAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      admin,           // admin can pay here
      assetMint,
      vault,
      true
    );

    // console.log("Admin: ", admin.publicKey.toString());
    // console.log("User: ", user.publicKey.toString());
    // console.log("Collateral Mint: ", collateralMint.toString());
    // console.log("Asset Mint: ", assetMint.toString());
  });

  it("Initializes the vault", async () => {
    try {
      const tx = await program.methods.initializeVault()
        .accounts({
          admin: admin.publicKey,
          collateralMint: collateralMint,
          assetMint: assetMint,
          // vault: anchor.web3.Keypair.generate().publicKey,
          // systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([admin])
        .rpc();

      let [vault, vault_bump] = await PublicKey.findProgramAddressSync(
        [Buffer.from("vault"),
        assetMint.toBuffer(),
        collateralMint.toBuffer()],
        program.programId
      );

      const vault_account = await program.account.vault.fetch(vault);
      // console.log("Vault account: ", vault_account);

    } catch (e) {
      console.log(e);
    }
  });

  it("Initialzes the user", async () => {
    try {
      let [vault, vault_bump] = await PublicKey.findProgramAddressSync(
        [Buffer.from("vault"),
        assetMint.toBuffer(),
        collateralMint.toBuffer()],
        program.programId
      );

      const tx = await program.methods.initializeUser().accounts({
        user: user.publicKey,
        // vault: anchor.web3.Keypair.generate().publicKey,
        // systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([user]).rpc();

      let [user_pda, user_bump] = await PublicKey.findProgramAddressSync(
        [Buffer.from("user"),
        user.publicKey.toBuffer()],
        program.programId
      );

      let user_account = await program.account.user.fetch(user_pda);
      // console.log("User account: ", user_account);

    } catch (e) {
      console.log(e);
    }
  });

  it("Deposits wsol as collateral", async () => {
    try {
      // Derive PDAs
      const [vault] = PublicKey.findProgramAddressSync(
        [Buffer.from("vault"), assetMint.toBuffer(), collateralMint.toBuffer()],
        program.programId
      );

      const [userPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user"), user.publicKey.toBuffer()],
        program.programId
      );

      // ✅ Mint tokens to user (using admin as mint authority)
      await mintTo(
        provider.connection,
        admin,
        assetMint,
        userAssetAta.address,
        admin,            // admin is mint authority
        10000000000
      );

      // ✅ Deposit instruction
      const tx = await program.methods
        .depositWsol(new anchor.BN(10000000000))
        .accounts({
          user: user.publicKey,
          userAccount: userPda,
          vault,
          assetMint,
          collateralMint,
          userWsolAccount: userAssetAta.address,
          vaultWsolAccount: vaultAssetAta.address,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([user])   // 👈 must sign as Keypair, not PublicKey
        .rpc();

      // console.log("Deposit transaction: ", tx);

      // ✅ Verify state
      const userAccount = await program.account.user.fetch(userPda);
      const vaultAccount = await program.account.vault.fetch(vault);

      // console.log("User total WSOL deposits:", userAccount.totalWsolDeposits.toString());
      // console.log("Vault total WSOL deposits:", vaultAccount.totalWsolDeposits.toString());
      assert.equal(userAccount.totalWsolDeposits.toString(), "10000000000");
      assert.equal(vaultAccount.totalWsolDeposits.toString(), "10000000000");

    } catch (e) {
      console.error("Error in deposit test:", e);
    }
  });

  it("Withdraws wsol as collateral", async () => {
    try {
      // Derive PDAs
      const [vault] = PublicKey.findProgramAddressSync(
        [Buffer.from("vault"), assetMint.toBuffer(), collateralMint.toBuffer()],
        program.programId
      );

      const [userPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user"), user.publicKey.toBuffer()],
        program.programId
      );

      const txn = await program.methods
        .withdrawWsol(new anchor.BN(10000000000))
        .accounts({
          user: user.publicKey,
          userAccount: userPda,
          vault,
          assetMint,
          collateralMint,
          userWsolAccount: userAssetAta.address,
          vaultWsolAccount: vaultAssetAta.address,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([user])   // 👈 must sign as Keypair, not PublicKey
        .rpc();

      const userAccount = await program.account.user.fetch(userPda);
      const vaultAccount = await program.account.vault.fetch(vault);
      assert.equal(userAccount.totalWsolDeposits.toString(), "0");
      assert.equal(vaultAccount.totalWsolDeposits.toString(), "0");
    } catch (e) {
      console.error("Error in withdraw test:", e);
    }
  });
});