import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor"
import { IDL, WbaPrereq } from "./programs/wba_prereq";
import wallet from "./wba-wallet.json"
import base58 from "bs58";
import * as fs from 'fs';

import { BorshInstructionCoder } from "@coral-xyz/anchor";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Github account
const github = Buffer.from("matiman", "utf8");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), { commitment: "confirmed" });

// Create our program
const program: Program<WbaPrereq> = new Program(IDL, provider);

// Create the PDA for our enrollment account
const enrollment_seeds = [Buffer.from("prereq"), keypair.publicKey.toBuffer()];

const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(enrollment_seeds, program.programId);

(async () => {

   try {
         const txhash = await program.methods
         .complete(github) // Call the complete method with the github account
         .accounts({ // Pass in the accounts required by the program
             signer: keypair.publicKey, // The signer is the keypair's public key
         })
         .signers([
             keypair
         ]).rpc();
         console.log(`Success! Check out your TX here:
         https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
     } catch(e) {
         console.error(`Oops, something went wrong: ${e}`)
   } 

})();


//TO verify a tx content 

// const verifyTx = async() => {
//   const connection = new Connection("https://api.devnet.solana.com");
//   const tx = await connection.getTransaction("vST4BLCwNmMkdQMPBcH9G7tRWx4552xE5ib7Mr53bAPPZM7yrcBSE7UmYfgy7waCuC875ck4iiuitqWvdFBk2wP");
                                                                          
//   if (tx) {
//     // Verify the program ID called is the WBA program id
//     // WBA program ID: WBAQSygkwMox2VuWKU133NxFrpDZUBdvSBeaBEue2Jq
//     console.log("Program IDs:");
//     tx.transaction.message.programIds().forEach(id => console.log(id.toString()));

//     const ixs = tx.transaction.message.instructions;
//     // Decodes the program instructions (in our case, the `complete` instruction)
//     // https://coral-xyz.github.io/anchor/ts/classes/BorshInstructionCoder.html#format
//     const coder = new BorshInstructionCoder(IDL);

//     ixs.forEach(ix => {
//       const msg = coder.decode(ix.data, "base58");
//       console.log("instruction name: ", msg?.name);

//       const ixData = msg?.data;
//       // @ts-ignore
//       // Typescript hack since it doesn't know that the `github` args exists in the
//       // params to the `complete` instruction
//       const githubBuffer = ixData?.github as Buffer;
//       console.log("github username: ", githubBuffer.toString("utf8"));
//     });
//   }
// };

//verifyTx();
// const PRIVATE_KEY = "5xb4XeuuCSETmUTeZE3JcPdVqxd6jfZsSNYVNt2BUJYzqzQ2YXiXT5uodnxC5e44Q7xKEXqqGkBSpnaH6TYEvuew"; // Private key from phantom
// const PUBLIC_KEY = "52ZigEcKGg8tUH6mUGMvq7bUASw7YPa52qyPeRuvMQfR"; // Fill with your address to verify
// const secret = base58.decode(PRIVATE_KEY);

// // Check if the pk is correct 
// const pair = Keypair.fromSecretKey(secret);


// if (pair.publicKey.toString() == PUBLIC_KEY) {
//   fs.writeFileSync(
//     'wba-wallet.json',
//     JSON.stringify(Array.from(secret))
//   );
// }