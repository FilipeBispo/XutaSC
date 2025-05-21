import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { XutaSc } from "../target/types/xuta_sc";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { createMint, getOrCreateAssociatedTokenAccount, getAccount, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID, mintTo } from "@solana/spl-token";
import { assert, expect } from 'chai';
import { Campaigns } from "../Xuta-fe/src/containers";

describe("xuta-sc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.xutaSc as Program<XutaSc>;

  let admin: Keypair;
  let institutionAdmin: Keypair;
  let user: Keypair;
  let mintQuote: Keypair;
  let mintPlayer: Keypair;
  let config: PublicKey;
  let institutionAccount: PublicKey;
  let campaignAccount: PublicKey;
  //let mintPlayer: PublicKey;
  //let mintQuote: PublicKey;
  let userQuoteAccount;
  let userPlayerTokenAccount;
  let receipt: PublicKey;
  let connection: Connection;

  before('', async () => {
    admin = Keypair.generate();
    user = Keypair.generate();
    institutionAdmin = Keypair.generate();
    user = Keypair.generate();
    institutionAdmin = Keypair.generate();
    mintPlayer = Keypair.generate();
    mintQuote = Keypair.generate();

    console.log("Admin pk= ", admin.publicKey);
    connection = anchor.getProvider().connection;

    let airdropSignature = await connection.requestAirdrop(admin.publicKey, LAMPORTS_PER_SOL * 10);
    await connection.confirmTransaction(airdropSignature);


    const sigs = await Promise.all([

      connection.requestAirdrop(admin.publicKey, LAMPORTS_PER_SOL * 10),
      connection.requestAirdrop(user.publicKey, LAMPORTS_PER_SOL * 10),
      connection.requestAirdrop(institutionAdmin.publicKey, LAMPORTS_PER_SOL * 10)]);

    await Promise.all([
      connection.confirmTransaction(sigs[0]),
      connection.confirmTransaction(sigs[1]),
      connection.confirmTransaction(sigs[2])
    ]);

    await createMint(connection, admin, admin.publicKey, null, 6, mintQuote);

    // user = the Keypair who wants to buy tokens
    userQuoteAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      user, // payer for account creation
      mintQuote.publicKey, // the mint for the quote token
      user.publicKey, // the owner of the ATA
      false, // allowOwnerOffCurve: false since user is a regular keypair
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );

    await mintTo(connection, admin, mintQuote.publicKey, userQuoteAccount.address, admin, 100000);

  });


  it("Initialize config!", async () => {
    // Add your test here.
    const tx = await program.methods
      .init()
      .accounts({ authority: admin.publicKey })
      .signers([admin])
      .rpcAndKeys();

    let cfgPk = tx.pubkeys.config;
    const cfg = await program.account.config.fetch(cfgPk);
    console.log("cfg: ", cfg);

    assert(cfg.authority.equals(admin.publicKey));
    assert(cfg.institutionAuthority.equals(admin.publicKey));
    console.log("Your transaction signature", tx.signature);
    console.log(tx);
    config = tx.pubkeys.config;
  });

  it("inititialize institution!", async () => {

    const tx = await program.methods
      .initInstitution("Soccer Stars", "URL_PDF", "URL_IMAGE", "Soccer star is the best")
      .accountsPartial({
        institutionAuthority: admin.publicKey,
        newInstitutionAuthority: institutionAdmin.publicKey,

      })
      .signers([admin])
      .rpcAndKeys();

    console.log(tx);
    console.log("Your transaction signature", tx.signature);
    institutionAccount = tx.pubkeys.institution;
  });

  it("sets new authority!", async () => {
    // Create a new keypair for the new authority
    const newAuthority = Keypair.generate();

    // Request airdrop for the new authority
    const sig = await connection.requestAirdrop(newAuthority.publicKey, LAMPORTS_PER_SOL * 10);
    await connection.confirmTransaction(sig);

    // Call set_authority instruction
    const tx = await program.methods
      .setAuthority()
      .accountsPartial({
        authority: admin.publicKey,
        newAuthority: newAuthority.publicKey,
      }).signers([admin])
      .rpcAndKeys();

    console.log("Set authority transaction signature:", tx);

    // Fetch the updated config account
    const updatedConfig = await program.account.config.fetch(config);

    // Verify that the authority was changed correctly
    assert(updatedConfig.authority.equals(newAuthority.publicKey), "Authority was not updated correctly");
    console.log("New authority set successfully:", newAuthority.publicKey.toString());
    admin = newAuthority;
  });

  it("creates a campaign!", async () => {

    const tx = await program.methods
      .createCampaign("Ronaldo",
        "URL_contract",
        "URL_Image",
        "Invest in Ronaldo",
        10,
        new BN(1000),
        new BN(1747250749),
        new BN(1767250740)
      )
      .accountsPartial({
        mintPlayer: mintPlayer.publicKey,
        mintQuote: mintQuote.publicKey,
        authority: institutionAdmin.publicKey,
        config: config,
        institution: institutionAccount,
      })
      .signers([institutionAdmin, mintPlayer])
      .rpcAndKeys();

    console.log(tx);
    console.log("Your transaction signature", tx.signature);
    campaignAccount = tx.pubkeys.campaign;
  });

  it("buying tokens!", async () => {

    userPlayerTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      user,                    // payer (also ATA owner here)
      mintPlayer.publicKey,    // the new token mint
      user.publicKey           // ATA owner
    );
    
    console.log(userQuoteAccount);
    const tx = await program.methods
      .buyToken(
        new BN(100),
        1
      )
      .accountsPartial({
        user: user.publicKey,
        mintQuote: mintQuote.publicKey,
        config: config,
        campaign: campaignAccount,
        userQuoteAta: userQuoteAccount.address,

      })
      .signers([user])
      .rpcAndKeys();

    console.log(tx);
    console.log("Your transaction signature", tx.signature);
    receipt = tx.pubkeys.receipt;
  });

  it("redeem tokens!", async () => {

    console.log("User's ATA for mintPlayer:", userPlayerTokenAccount.address);
    const tx = await program.methods
      .redeemToken(
      )
      .accountsPartial({
        user: user.publicKey,
        mintPlayer: mintPlayer.publicKey,
        campaign: campaignAccount,
        userTokenAccount: userPlayerTokenAccount.address,
        receipt: receipt,
      })
      .signers([user])
      .rpcAndKeys();

    const tokenAccountInfo = await getAccount(connection, userPlayerTokenAccount.address);
    console.log("Balance:", tokenAccountInfo.amount.toString());

    console.log(tx);
    console.log("Your transaction signature", tx.signature);
  });
});
