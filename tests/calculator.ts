import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";
import { Calculator } from "../target/types/calculator";
//const { SystemProgram } = anchor.web3;
import { expect, assert } from 'chai';


describe("calculator", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.Calculator as Program<Calculator>;
  const datastore = anchor.web3.Keypair.generate()
  const soyyo = anchor.Wallet.local();
  anchor.setProvider(anchor.AnchorProvider.env());
  const autoridad = provider.wallet.publicKey;


  it("Creaste una calculadora", async () => {
    const txSignature = await program.rpc.initialize(provider.wallet.publicKey, {
      accounts: {
        datastore: datastore.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [soyyo.payer],
    });
      
      console.log("Your transaction signature", txSignature);
    });
  });

