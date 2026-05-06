import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { expect } from "chai";
import { Calc } from "../target/types/calc";

describe("calc", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.calc as Program<Calc>;
  const user = provider.wallet;

  // Derive the Calculator PDA
  const [calculatorPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("calculator"), user.publicKey.toBuffer()],
    program.programId
  );

  it("Initializes the calculator", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({ user: user.publicKey })
      .rpc();
    console.log("Initialize tx:", tx);

    const account = await program.account.calculator.fetch(calculatorPda);
    expect(account.authority.toBase58()).to.equal(user.publicKey.toBase58());
    expect(account.result.toNumber()).to.equal(0);
  });

  it("Adds two numbers", async () => {
    const tx = await program.methods
      .add(new anchor.BN(10), new anchor.BN(5))
      .accounts({ user: user.publicKey })
      .rpc();
    console.log("Add tx:", tx);

    const account = await program.account.calculator.fetch(calculatorPda);
    expect(account.result.toNumber()).to.equal(15);
  });

  it("Subtracts two numbers", async () => {
    const tx = await program.methods
      .subtract(new anchor.BN(20), new anchor.BN(7))
      .accounts({ user: user.publicKey })
      .rpc();
    console.log("Subtract tx:", tx);

    const account = await program.account.calculator.fetch(calculatorPda);
    expect(account.result.toNumber()).to.equal(13);
  });

  it("Multiplies two numbers", async () => {
    const tx = await program.methods
      .multiply(new anchor.BN(6), new anchor.BN(7))
      .accounts({ user: user.publicKey })
      .rpc();
    console.log("Multiply tx:", tx);

    const account = await program.account.calculator.fetch(calculatorPda);
    expect(account.result.toNumber()).to.equal(42);
  });

  it("Divides two numbers", async () => {
    const tx = await program.methods
      .divide(new anchor.BN(100), new anchor.BN(4))
      .accounts({ user: user.publicKey })
      .rpc();
    console.log("Divide tx:", tx);

    const account = await program.account.calculator.fetch(calculatorPda);
    expect(account.result.toNumber()).to.equal(25);
  });

  it("Fails on divide by zero", async () => {
    try {
      await program.methods
        .divide(new anchor.BN(10), new anchor.BN(0))
        .accounts({ user: user.publicKey })
        .rpc();
      expect.fail("Should have thrown an error");
    } catch (err: any) {
      expect(err.toString()).to.include("DivisionByZero");
    }
  });

  it("Handles negative numbers", async () => {
    const tx = await program.methods
      .add(new anchor.BN(-10), new anchor.BN(3))
      .accounts({ user: user.publicKey })
      .rpc();
    console.log("Negative add tx:", tx);

    const account = await program.account.calculator.fetch(calculatorPda);
    expect(account.result.toNumber()).to.equal(-7);
  });
});
