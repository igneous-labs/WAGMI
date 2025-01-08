use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use crate::*;
pub const SMART_WALLET_ACCOUNT_DISCM: [u8; 8] = [67, 59, 220, 179, 41, 10, 60, 177];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SmartWallet {
    pub base: Pubkey,
    pub bump: u8,
    pub threshold: u64,
    pub minimum_delay: i64,
    pub grace_period: i64,
    pub max_owners: u8,
    pub owner_set_seqno: u32,
    pub num_transactions: u64,
    pub owners: Vec<Pubkey>,
    pub reserved: [u64; 16],
}
#[derive(Clone, Debug, PartialEq)]
pub struct SmartWalletAccount(pub SmartWallet);
impl SmartWalletAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SMART_WALLET_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SMART_WALLET_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SmartWallet::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SMART_WALLET_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TRANSACTION_ACCOUNT_DISCM: [u8; 8] = [11, 24, 174, 129, 203, 117, 242, 23];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Transaction {
    pub smart_wallet: Pubkey,
    pub index: u64,
    pub bump: u8,
    pub proposer: Pubkey,
    pub instructions: Vec<TXInstruction>,
    pub signers: Vec<bool>,
    pub owner_set_seqno: u32,
    pub eta: i64,
    pub executor: Pubkey,
    pub executed_at: i64,
    pub created_at: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransactionAccount(pub Transaction);
impl TransactionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSACTION_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        TRANSACTION_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(Transaction::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSACTION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SUBACCOUNT_INFO_ACCOUNT_DISCM: [u8; 8] = [255, 94, 30, 46, 165, 11, 49, 76];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubaccountInfo {
    pub smart_wallet: Pubkey,
    pub subaccount_type: SubaccountType,
    pub index: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SubaccountInfoAccount(pub SubaccountInfo);
impl SubaccountInfoAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SUBACCOUNT_INFO_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SUBACCOUNT_INFO_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SubaccountInfo::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SUBACCOUNT_INFO_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STAGED_TX_INSTRUCTION_ACCOUNT_DISCM: [u8; 8] = [
    137,
    99,
    181,
    207,
    235,
    6,
    73,
    255,
];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StagedTXInstruction {
    pub smart_wallet: Pubkey,
    pub index: u64,
    pub owner_invoker_bump: u8,
    pub owner: Pubkey,
    pub owner_set_seqno: u32,
    pub ix: TXInstruction,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StagedTXInstructionAccount(pub StagedTXInstruction);
impl StagedTXInstructionAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STAGED_TX_INSTRUCTION_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        STAGED_TX_INSTRUCTION_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(StagedTXInstruction::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STAGED_TX_INSTRUCTION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
