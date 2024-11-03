import * as borsh from "borsh";
import * as web3 from "@solana/web3.js";
// Manually initialize variables that are automatically defined in Playground
const PROGRAM_ID = new web3.PublicKey("2AwVjkiQG5vJe6bKfzK5xuNoEVu9U3cNzxY6VehFTwwP");
const connection = new web3.Connection("https://api.devnet.solana.com", "confirmed");
const wallet = { keypair: web3.Keypair.generate() };


/**
 * The state of a greeting account managed by the hello world program
 */
class GreetingAccount {
  counter = 0;
  constructor(fields: { counter: number } | undefined = undefined) {
    if (fields) {
      this.counter = fields.counter;
    }
  }
}

/**
 * Borsh schema definition for greeting accounts
 */
const GreetingSchema = new Map([
  [GreetingAccount, { kind: "struct", fields: [["counter", "u32"]] }],
]);

/**
 * The expected size of each greeting account.
 */
const GREETING_SIZE = borsh.serialize(
  GreetingSchema,
  new GreetingAccount()
).length;

describe("Test", () => {
  it("greet", async () => {
    const movieTitle = "Solana tutorial";
    const titleLength = movieTitle.length;
    const descriptionLength = 16;
    const instructionData = Buffer.alloc(
      1 + 4 + titleLength + 1 + 4 + descriptionLength
    );
    instructionData[0] = 1;
    instructionData.writeUInt32LE(titleLength, 1);
    instructionData.write(movieTitle, 1 + 4);
    let offset = 1 + 4 + titleLength;
    instructionData[offset] = 5;
    offset++;
    instructionData.writeUInt32LE(descriptionLength, offset);
    offset += 4;
    instructionData.write("This is amazing!", offset);

    const [pda] = await web3.PublicKey.findProgramAddress(
      [wallet.keypair.publicKey.toBuffer(), Buffer.from(movieTitle)],
      new web3.PublicKey(PROGRAM_ID)
    );

    const greetIx = new web3.TransactionInstruction({
      keys: [
        {
          pubkey: wallet.keypair.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: pda,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: web3.SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId: PROGRAM_ID,
      data: instructionData,
    });

    // Create transaction and add the instructions
    const tx = new web3.Transaction();
    tx.add(greetIx);

    // Send and confirm the transaction
    const txHash = await web3.sendAndConfirmTransaction(connection, tx, [
      wallet.keypair,
    ]);

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
  });
});
