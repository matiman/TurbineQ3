
mod programs;

#[cfg(test)]
mod tests {

use solana_sdk::{message::Message,signature::{Keypair, Signer,read_keypair_file},transaction::Transaction,system_program};
use bs58;
use std::io::{self, BufRead};
use solana_client::rpc_client::RpcClient;
use solana_program::{
    pubkey::Pubkey,
    system_instruction::transfer,
    };
use std::str::FromStr;
use crate::programs::wba_prereq::{WbaPrereqProgram, CompleteArgs, UpdateArgs};

const RPC_URL: &str = "https://api.devnet.solana.com";

//WBA'S program HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1
    #[test]
    fn keygen() {
        //Create a new keypair
        //wallet created 47dvmNjstTNwUyQYquTaYSef3Gw45WdL7RvJkF3SZgu2
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        
        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);

        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
            println!("Success! Check out your TX here:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            }, Err(e) => println!("Oops, something went wrong: {}", e.to_string())
            };
    }

    #[test]
    fn transfer_sol() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // Define our WBA public key
        let to_pubkey = Pubkey::from_str("4J7kYZUjgWz4r1TA82tYup8xxrvqRBXuvWyhEPdN1pfa").unwrap();
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Get recent blockhash
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        // Get balance of dev wallet
        let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");

        // Create a test transaction to calculate fees
        let message = Message::new_with_blockhash(
            &[transfer(
            &keypair.pubkey(),
            &to_pubkey,
            balance,
            )],
            Some(&keypair.pubkey()),
            &recent_blockhash
            );
        
        // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
        let fee = rpc_client.get_fee_for_message(&message).expect("Failed to get fee calculator");   

        //Tx to empty SOL
        //Deduct fee from lamports amount and create a TX with correct balance
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
            &keypair.pubkey(),
            &to_pubkey,
            balance - fee,
            )],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
        ); 

        //Transaction to send 0.1 SOL
        // let transaction = Transaction::new_signed_with_payer(
        //     &[transfer(
        //     &keypair.pubkey(),
        //     &to_pubkey,
        //     1_000_000_00
        //     )],
        //     Some(&keypair.pubkey()),
        //     &vec![&keypair],
        //     recent_blockhash
        //     );

        // Send the transaction
        let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

        // Print our transaction out
        println!(
        "Success! Check out your TX here:
        https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
        );
    }

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);

        // Let's define our accounts
        let signer = read_keypair_file("wba-wallet.json").expect("Couldn't find wallet file");

        let prereq = WbaPrereqProgram::derive_program_address(&[b"prereq",
        signer.pubkey().to_bytes().as_ref()]);

        // Define our instruction data
        let args = CompleteArgs {
            github: b"matiman".to_vec()
        };

        // Get recent blockhash
        let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
       
        // Now we can invoke the "complete" function
        let transaction = WbaPrereqProgram::complete(
        &[&signer.pubkey(), &prereq, &system_program::id()],
        &args,
        Some(&signer.pubkey()),
        &[&signer],
        blockhash
        );

        let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

        // Print our transaction out
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature);
        
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }
        
    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet =
        stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').split(',')
        .map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }
}