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
  let mintPlayer2: Keypair;
  let config: PublicKey;
  let institutionAccount: PublicKey;
  let institutionQuoteAta: PublicKey;
  let campaignAccount: PublicKey;
  let campaignAccount2: PublicKey;
  //let mintPlayer: PublicKey;
  //let mintQuote: PublicKey;
  let userQuoteAccount;
  let userPlayerTokenAccount;
  let receipt: PublicKey;
  let receipt2: PublicKey;
  let connection: Connection;

  let initialBalance;

  before('', async () => {
    admin = Keypair.generate();
    user = Keypair.generate();
    institutionAdmin = Keypair.generate();
    user = Keypair.generate();
    mintPlayer = Keypair.generate();
    mintQuote = Keypair.generate();
    mintPlayer2 = Keypair.generate();
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

    // user = the Keypair who wants to buy tokens
    institutionQuoteAta = (await getOrCreateAssociatedTokenAccount(
      connection,
      institutionAdmin, // payer for account creation
      mintQuote.publicKey, // the mint for the quote token
      institutionAdmin.publicKey, // the owner of the ATA
      false, // allowOwnerOffCurve: false since user is a regular keypair
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    )).address;

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

    assert(cfg.authority.equals(admin.publicKey));
    assert(cfg.institutionAuthority.equals(admin.publicKey));
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


    // Fetch the updated config account
    const updatedConfig = await program.account.config.fetch(config);

    // Verify that the authority was changed correctly
    assert(updatedConfig.authority.equals(newAuthority.publicKey), "Authority was not updated correctly");
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

    campaignAccount = tx.pubkeys.campaign;
  });

  it("buying tokens!", async () => {

    const tx = await program.methods
      .buyToken(
        new BN(1000)
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

    receipt = tx.pubkeys.receipt;

    await connection.confirmTransaction(tx.signature);

    const accountTest = await program.account.campaign.fetch(campaignAccount);
    assert(accountTest.currentTokens.toString() === "1000");
    assert(accountTest.currentTokens.toString() === accountTest.targetAmount.toString());
  });

  it("finish campaign!", async () => {

    const tx = await program.methods
      .finishCampaign(
    )
      .accountsPartial({
        mintQuote: mintQuote.publicKey,
        campaign: campaignAccount,
        userTokenAccountQuote: institutionQuoteAta,
        authority: institutionAdmin.publicKey,
        institution: institutionAccount,
      })
      .signers([institutionAdmin])
      .rpcAndKeys();

    await connection.confirmTransaction(tx.signature);

    const accountTest = await program.account.campaign.fetch(campaignAccount);
    assert(accountTest.status.successful);
  });

  it("redeem tokens!", async () => {

    userPlayerTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      user,                    // payer (also ATA owner here)
      mintPlayer.publicKey,    // the new token mint
      user.publicKey           // ATA owner
    );

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
  });


  //// START REDEEM WORKFLOW TESTS ////


  it("creates a campaign!", async () => {

    const tx = await program.methods
      .createCampaign("ze",
        "URL_contract",
        "URL_Image",
        "Invest in ze",
        10,
        new BN(1000),
        new BN(1747250749),
        new BN(1767250740)
      )
      .accountsPartial({
        mintPlayer: mintPlayer2.publicKey,
        mintQuote: mintQuote.publicKey,
        authority: institutionAdmin.publicKey,
        config: config,
        institution: institutionAccount,
      })
      .signers([institutionAdmin, mintPlayer2])
      .rpcAndKeys();

    campaignAccount2 = tx.pubkeys.campaign;
  });

  it("buying tokens!", async () => {
    initialBalance = (await getAccount(connection, userQuoteAccount.address)).amount;

    const tx = await program.methods
      .buyToken(
        new BN(100)
      )
      .accountsPartial({
        user: user.publicKey,
        mintQuote: mintQuote.publicKey,
        config: config,
        campaign: campaignAccount2,
        userQuoteAta: userQuoteAccount.address,

      })
      .signers([user])
      .rpcAndKeys();

    receipt2 = tx.pubkeys.receipt;

    await connection.confirmTransaction(tx.signature);
  });

  it("finish campaign!", async () => {

    const tx = await program.methods
      .finishCampaign(
    )
      .accountsPartial({
        mintQuote: mintQuote.publicKey,
        campaign: campaignAccount2,
        userTokenAccountQuote: institutionQuoteAta,
        authority: institutionAdmin.publicKey,
        institution: institutionAccount,
      })
      .signers([institutionAdmin])
      .rpcAndKeys();

    await connection.confirmTransaction(tx.signature);

    const accountTest = await program.account.campaign.fetch(campaignAccount2);
    assert(accountTest.status.failed);
  });

  it("refund receipt!", async () => {

    const tx = await program.methods
      .refundReceipt()
      .accountsPartial({
        receipt: receipt2,
        user: user.publicKey,
        mintQuote: mintQuote.publicKey,
        campaign: campaignAccount2,
        userTokenAccountQuote: userQuoteAccount.address,
      })
      .signers([user])
      .rpcAndKeys();

    await connection.confirmTransaction(tx.signature);

    let finalBalance = (await getAccount(connection, userQuoteAccount.address)).amount;
    assert(finalBalance.toString() === initialBalance.toString());
  });
});
