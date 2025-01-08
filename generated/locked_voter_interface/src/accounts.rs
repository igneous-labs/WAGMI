use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use crate::*;
pub const LOCKER_ACCOUNT_DISCM: [u8; 8] = [74, 246, 6, 113, 249, 228, 75, 169];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Locker {
    pub base: Pubkey,
    pub bump: u8,
    pub token_mint: Pubkey,
    pub locked_supply: u64,
    pub total_escrow: u64,
    pub governor: Pubkey,
    pub params: LockerParams,
    pub buffers: [u128; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct LockerAccount(pub Locker);
impl LockerAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LOCKER_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        LOCKER_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(Locker::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LOCKER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const ESCROW_ACCOUNT_DISCM: [u8; 8] = [31, 213, 123, 187, 186, 22, 218, 155];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Escrow {
    pub locker: Pubkey,
    pub owner: Pubkey,
    pub bump: u8,
    pub tokens: Pubkey,
    pub amount: u64,
    pub escrow_started_at: i64,
    pub escrow_ends_at: i64,
    pub vote_delegate: Pubkey,
    pub is_max_lock: bool,
    pub partial_unstaking_amount: u64,
    pub padding: u64,
    pub buffers: [u128; 9],
}
#[derive(Clone, Debug, PartialEq)]
pub struct EscrowAccount(pub Escrow);
impl EscrowAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ESCROW_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        ESCROW_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(Escrow::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ESCROW_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PARTIAL_UNSTAKING_ACCOUNT_DISCM: [u8; 8] = [
    172,
    146,
    58,
    213,
    40,
    250,
    107,
    63,
];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PartialUnstaking {
    pub escrow: Pubkey,
    pub amount: u64,
    pub expiration: i64,
    pub buffers: [u128; 6],
    pub memo: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PartialUnstakingAccount(pub PartialUnstaking);
impl PartialUnstakingAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PARTIAL_UNSTAKING_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        PARTIAL_UNSTAKING_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(PartialUnstaking::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PARTIAL_UNSTAKING_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
