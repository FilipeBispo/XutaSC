import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { XutaSc } from "../target/types/xuta_sc";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { getAccount, getAssociatedTokenAddressSync } from "@solana/spl-token";
import {assert, expect} from 'chai';

describe("xuta-sc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.xutaSc as Program<XutaSc>;

  let admin: Keypair;
  let institutionAdmin: Keypair;
  let user: Keypair;
  let config: PublicKey;
  let institutionAccount: PublicKey;
  let userAccount: PublicKey;
  let mintPlayer: PublicKey;
  let mintQuote: PublicKey;
  let institutionQuoteAccount: PublicKey;
  let userQuoteAccount: PublicKey;
  let connection: Connection;

  before('', async () => {
    admin = Keypair.generate();
    user = Keypair.generate();
    institutionAdmin = Keypair.generate();

    console.log("Admin pk= ", admin.publicKey);
    connection = anchor.getProvider().connection;

    const sigs = await Promise.all([

      connection.requestAirdrop(admin.publicKey, LAMPORTS_PER_SOL * 10),
      connection.requestAirdrop(user.publicKey, LAMPORTS_PER_SOL * 10),
      connection.requestAirdrop(institutionAdmin.publicKey, LAMPORTS_PER_SOL * 10)]);

    await Promise.all([
      connection.confirmTransaction(sigs[0]),
      connection.confirmTransaction(sigs[1]),
      connection.confirmTransaction(sigs[2])
    ]);
  });


  it("Initialize config!", async () => {
    // Add your test here.
    const tx = await program.methods
    .init()
    .accounts({authority: admin.publicKey})
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
    .initInstitution("Soccer Stars","URL_PDF")
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

});
