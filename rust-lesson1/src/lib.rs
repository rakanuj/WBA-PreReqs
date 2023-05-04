
#[cfg(test)]
mod tests {
    use solana_client::rpc_client::RpcClient;
    use solana_program::{
        pubkey::Pubkey,
        system_instruction::transfer
    };
    use solana_sdk::{
        signature::{Keypair, Signer, read_keypair_file},
        transaction::Transaction,
        message::Message,
    };
    use std::str::FromStr;

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        // Create a new keypair
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
            },
            Err(e) => println!("Oops, something went wrong: {}", e.to_string())
        };
    }

    #[test]
    fn transfer_sol() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // Define our WBA public key
        let to_pubkey = Pubkey::from_str("4GEMhfsoYJ1tdNU3iB8BXJ3ckPJSRuvSgCSaMF9QAanZ").unwrap();

        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
                &keypair.pubkey(),
                &to_pubkey,
                1_000_000
            )], 
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
        );

        // Send the transacåtion
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
    fn dump_all_sol() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        
        // Define our WBA public key
        let to_pubkey = Pubkey::from_str("4GEMhfsoYJ1tdNU3iB8BXJ3ckPJSRuvSgCSaMF9QAanZ").unwrap();
        
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);
        
        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
    
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
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        // Deduct fee from lamports amount and create a TX with correct balance
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

        // Send the transacåtion
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
}