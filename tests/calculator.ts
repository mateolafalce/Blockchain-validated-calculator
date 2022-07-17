import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";
import { Calculator } from "../target/types/calculator";
const { SystemProgram } = anchor.web3;
import { expect, assert } from 'chai';


describe("calculator", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Calculator;
  const datastore = anchor.web3.Keypair.generate();

  it("Creaste una calculadora", async () => {

    const txSignature = await program.rpc.initialize(
      provider.wallet.publicKey, //Owner Key
      add1 = new anchor.BN(2), //add1
      add2 = new anchor.BN(45), //add2
      new anchor.BN(78), //div1
      new anchor.BN(9), //div2
      new anchor.BN(75), //sub1
      new anchor.BN(36), //sub2
      new anchor.BN(49), //mul1
      new anchor.BN(28), //mul2
      new anchor.BN(2), //square number
      new anchor.BN(9), //percentage
      new anchor.BN(123), //number to percentage
    {
      accounts: {
        datastore: datastore.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [datastore],
    });
      
      console.log("Your transaction signature", txSignature);
      //assert.equal(txSignature.autoridad.toBase58(), program.provider.wallet.publicKey.toBase58());
      //assert.equal(txSignature.add1, anchor.BN(2));
      //assert.equal(txSignature.add2, anchor.BN(45));
      //assert.ok(txSignature.timestamp);
    });
  });

