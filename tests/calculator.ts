import * as anchor from "@project-serum/anchor";
const { SystemProgram } = anchor.web3;


describe("Entering data into the calculator... ", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Calculator;
  const add = anchor.web3.Keypair.generate();
  const div = anchor.web3.Keypair.generate();
  const sub = anchor.web3.Keypair.generate();
  const mul = anchor.web3.Keypair.generate();
  const squa = anchor.web3.Keypair.generate();
  const perc = anchor.web3.Keypair.generate();

  it("Add", async () => {

    const txSignature = await program.rpc.add(
      new anchor.BN(2), 
      new anchor.BN(45), 
    {
      accounts: {
        numbers: add.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [add],
    });
      
      console.log("The result of the sum is ", txSignature);
    });

  it("Div", async () => {

    const txSignature = await program.rpc.div(
      new anchor.BN(78), 
      new anchor.BN(9), 
    {
      accounts: {
        numbers: div.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [div],
    });
      
      console.log("The result of the division is ", txSignature);
    });

  it("Sub", async () => {

      const txSignature = await program.rpc.sub(
        new anchor.BN(75), 
        new anchor.BN(36), 
      {
        accounts: {
          numbers: sub.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [sub],
      });
        
        console.log("The result of the subtraction is ", txSignature);
      });

    it("Mul", async () => {

        const txSignature = await program.rpc.mul(
          new anchor.BN(78), 
          new anchor.BN(9), 
        {
          accounts: {
            numbers: mul.publicKey,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
          },
          signers: [mul],
        });
          
          console.log("The result of the multiplication is ", txSignature);
        });
    it("Squa", async () => {
      const txSignature = await program.rpc.squa(
        new anchor.BN(49), 
      {
         accounts: {
          numbers: squa.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [squa],
      });  
  console.log("The result of the square is ", txSignature);
});

it("Perc", async () => {
  const txSignature = await program.rpc.perc(
    new anchor.BN(9), 
    new anchor.BN(123), 
  {
    accounts: {
    numbers: perc.publicKey,
    user: provider.wallet.publicKey,
    systemProgram: SystemProgram.programId,
  },
    signers: [perc],
    });    
    console.log("The result of the percentage is ", txSignature);
  });
  console.log("​​mateolafalce3@gmail.com​")
});