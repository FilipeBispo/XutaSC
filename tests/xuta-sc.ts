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

});
