use crate::solana_blocks::EncodedConfirmedBlock;
use crate::{Rewards, SolProbeResult, Transaction, UnixTimestamp};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmedBlock {
    pub blockhash: String,
    pub transactions: Vec<Transaction>,
    pub rewards: Rewards,
    pub block_time: Option<UnixTimestamp>,
    pub block_height: Option<u64>,
}

impl ConfirmedBlock {
    pub fn new() -> Self {
        Self {
            blockhash: String::default(),
            transactions: Vec::default(),
            rewards: Rewards::default(),
            block_time: Option::default(),
            block_height: Option::default(),
        }
    }

    pub fn decode(&mut self, block: &EncodedConfirmedBlock) -> SolProbeResult<&mut Self> {
        self.blockhash = block.blockhash.clone();
        self.block_height = block.block_height;
        self.block_time = block.block_time.clone();
        self.rewards = block.rewards.clone();

        let mut transactions: Vec<Transaction> = Vec::default();

        for tx in block.transactions.iter() {
            let mut transaction = Transaction::new();
            transaction.decode_tx(&tx)?;

            transactions.push(transaction);
        }

        self.transactions = transactions;

        Ok(self)
    }
}
