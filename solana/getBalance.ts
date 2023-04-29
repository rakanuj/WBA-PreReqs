import { Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';


import wallet from "./dev-wallet.json"

// Import our dev wallet keypair from the wallet file
const from = Keypair.fromSecretKey(new Uint8Array(wallet));

const connection = new Connection("https://api.devnet.solana.com");

(async () => {
    try {
        // dev wallet balance
        let balance = await connection.getBalance(from.publicKey);
        balance /= LAMPORTS_PER_SOL;

        console.log(`Your balance is ${balance} sol
        In account: ${from.publicKey}`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();
