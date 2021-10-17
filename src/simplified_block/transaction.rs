use crate::solana_blocks::{
    EncodedTransaction, EncodedTransactionWithStatusMeta, TransactionError, UiMessage,
    UiRawMessage, UiTokenAmount, UiTransaction, UiTransactionStatusMeta, UiTransactionTokenBalance,
};
use crate::{Rewards, SolProbeError, SolProbeResult, StringAmount, StringDecimals};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub signatures: Vec<String>,
    pub message: Message,
    pub metadata: Option<Metadata>,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            signatures: Vec::default(),
            message: Message::default(),
            metadata: Option::default(),
        }
    }

    pub fn decode_tx(
        &mut self,
        encoded_transaction: &EncodedTransactionWithStatusMeta,
    ) -> SolProbeResult<&mut Self> {
        let transaction = self.destruct_encoding(&encoded_transaction.transaction)?;
        let message = self.destruct_ui_message(&transaction.message)?;

        self.signatures = transaction.signatures;
        self.message = Message {
            account_keys: message.account_keys,
            num_required_signatures: message.header.num_required_signatures,
            num_readonly_signed_accounts: message.header.num_readonly_signed_accounts,
            num_readonly_unsigned_accounts: message.header.num_readonly_unsigned_accounts,
        };
        self.metadata = {
            match &encoded_transaction.meta {
                Some(metadata) => Some(metadata.clone().into()),
                None => None,
            }
        };

        Ok(self)
    }

    fn destruct_encoding(&self, encoding: &EncodedTransaction) -> SolProbeResult<UiTransaction> {
        match encoding {
            EncodedTransaction::Json(encoded_data) => Ok(encoded_data.clone()),
            _ => Err(SolProbeError::UnsupportedEncoding),
        }
    }

    pub fn destruct_ui_message(
        &self,
        encoding: &UiMessage,
    ) -> core::result::Result<UiRawMessage, SolProbeError> {
        match encoding {
            UiMessage::Raw(raw_message) => Ok(raw_message.clone()),
            _ => Err(SolProbeError::UnsupportedMessageType),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub err: Option<TransactionError>,
    pub fee: u64,
    pub pre_balances: Vec<u64>,
    pub post_balances: Vec<u64>,
    pub log_messages: Vec<String>,
    pub pre_token_balances: Vec<TokenBalance>,
    pub post_token_balances: Vec<TokenBalance>,
    pub rewards: Rewards,
}

impl From<UiTransactionStatusMeta> for Metadata {
    fn from(value: UiTransactionStatusMeta) -> Self {
        Self {
            err: value.err,
            fee: value.fee,
            pre_balances: value.pre_balances,
            post_balances: value.post_balances,
            log_messages: {
                match value.log_messages {
                    Some(messages) => messages,
                    None => Vec::default(),
                }
            },
            pre_token_balances: {
                match value.pre_token_balances {
                    Some(balances) => TokenBalance::to_token_balance(balances),
                    None => Vec::default(),
                }
            },
            post_token_balances: {
                match value.post_token_balances {
                    Some(balances) => TokenBalance::to_token_balance(balances),
                    None => Vec::default(),
                }
            },
            rewards: match value.rewards {
                Some(rewards) => rewards,
                None => Vec::default(),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalance {
    pub account_index: u8,
    pub mint: String,
    pub token_amount: TokenAmount,
}

impl From<UiTransactionTokenBalance> for TokenBalance {
    fn from(value: UiTransactionTokenBalance) -> Self {
        Self {
            account_index: value.account_index,
            mint: value.mint,
            token_amount: value.ui_token_amount.into(),
        }
    }
}

impl TokenBalance {
    pub fn to_token_balance(value: Vec<UiTransactionTokenBalance>) -> Vec<Self> {
        let mut token_balances: Vec<TokenBalance> = Vec::default();
        value
            .into_iter()
            .for_each(|token_balance| token_balances.push(token_balance.into()));

        token_balances
    }
}

impl From<UiTokenAmount> for TokenAmount {
    fn from(value: UiTokenAmount) -> Self {
        Self {
            amount: value.ui_amount,
            decimals: value.decimals,
            string_amount: value.amount,
            string_decimals: value.ui_amount_string,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TokenAmount {
    pub amount: Option<f64>,
    pub decimals: u8,
    pub string_amount: StringAmount,
    pub string_decimals: StringDecimals,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    pub pubkey: String,
    pub lamports: i64,
    pub post_balance: u64, // Account balance in lamports after `lamports` was applied
    pub reward_type: Option<RewardType>,
    pub commission: Option<u8>, // Vote account commission when the reward was credited, only present for voting and staking rewards
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum RewardType {
    Fee,
    Rent,
    Staking,
    Voting,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub account_keys: Vec<String>,
    /// The number of signatures required for this message to be considered valid. The
    /// signatures must match the first `num_required_signatures` of `account_keys`.
    /// NOTE: Serialization-related changes must be paired with the direct read at sigverify.
    pub num_required_signatures: u8,
    /// The last num_readonly_signed_accounts of the signed keys are read-only accounts. Programs
    /// may process multiple transactions that load read-only accounts within a single PoH entry,
    /// but are not permitted to credit or debit lamports or modify account data. Transactions
    /// targeting the same read-write account are evaluated sequentially.
    pub num_readonly_signed_accounts: u8,
    /// The last num_readonly_unsigned_accounts of the unsigned keys are read-only accounts.
    pub num_readonly_unsigned_accounts: u8,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            account_keys: Vec::default(),
            num_required_signatures: u8::default(),
            num_readonly_signed_accounts: u8::default(),
            num_readonly_unsigned_accounts: u8::default(),
        }
    }
}
