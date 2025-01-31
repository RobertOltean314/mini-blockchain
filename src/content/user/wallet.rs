use secp256k1::{Secp256k1, SecretKey, PublicKey, Message, ecdsa::Signature};
use secp256k1::rand::rngs::OsRng;
use sha2::{Sha256, Digest};

use crate::content::blockchain::Blockchain;

use super::Transaction;

#[derive(Debug,  Clone)]
pub struct Wallet {
    secret_key: SecretKey, 
    pub public_key: PublicKey,
    pub is_miner: bool
}

impl Wallet {
    pub fn new(is_miner: bool) -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        Wallet { secret_key, public_key, is_miner }
    }

    pub fn address(&self) -> String {
        hex::encode(self.public_key.serialize())
    }

    pub fn sign(&self, data: &[u8]) -> Signature {
        let secp = Secp256k1::new();
        let hash = Sha256::digest(data);
        let message = Message::from_digest(hash.into());
        secp.sign_ecdsa(&message, &self.secret_key)
    }

    pub fn send_money(&self, receiver: &Wallet, amount: f64, blockchain: &mut Blockchain) -> Result<(), String> {
        let fee = amount * 0.01;
    
        let sender_balance = blockchain.get_balance(&self.address());
        if sender_balance < amount + fee {
            return Err(format!("Address: {} does not have enough funds", self.address()).to_string());
        }
    
        let mut tx = Transaction::new(
            &self.address(),
            &receiver.address(),
            amount,
            fee
        );
    
        let tx_hash = tx.hash();
        let signature = self.sign(&tx_hash);
        tx.signature = hex::encode(signature.serialize_der().as_ref());
        
        blockchain.mempool.push(tx);
        
        // Commenting out immediate mining for all wallets
        // blockchain.mine_pending_transactions(&self.address());
    
        // If this wallet is a miner, it might simulate trying to mine after adding a transaction
        if self.is_miner {
            // Here, instead of mining, we could simulate the miner adding this transaction to their pool for later mining
            println!("Miner {} added transaction to mining pool", self.address());
        }
    
        Ok(())
    }
}