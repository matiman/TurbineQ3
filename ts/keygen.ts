import { Keypair } from "@solana/web3.js";
import bs58 from 'bs58';
import * as prompt from 'prompt-sync'

//Generate a new keypair

let kp = Keypair.generate()

console.log(`You have generated a new Solana wallet:${kp.publicKey.toBase58()}`)
//devnet wallet:  Bbb58XD8U2SH8wkrAVgbEBfpnVxVLdTv37JBahsZ1gAK
//MY WBA Wallet: 52ZigEcKGg8tUH6mUGMvq7bUASw7YPa52qyPeRuvMQfR 

console.log(`You have generated a new Solana wallet:${kp.secretKey}`)
