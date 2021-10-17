pub type UnixTimestamp = i64;

pub type StringAmount = String;

pub type StringDecimals = String;

pub type Rewards = Vec<crate::solana_blocks::Reward>;

pub type Slot = u64;

pub type BlockResult<T> = core::result::Result<T, crate::solana_blocks::TransactionError>;
