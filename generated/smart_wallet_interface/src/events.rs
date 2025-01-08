use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use crate::*;
pub const TRANSACTION_APPROVE_EVENT_EVENT_DISCM: [u8; 8] = [
    242,
    40,
    121,
    28,
    8,
    37,
    100,
    34,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct TransactionApproveEvent {
    smart_wallet: Pubkey,
    transaction: Pubkey,
    owner: Pubkey,
    timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransactionApproveEventEvent(pub TransactionApproveEvent);
impl BorshSerialize for TransactionApproveEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        TRANSACTION_APPROVE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl TransactionApproveEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != TRANSACTION_APPROVE_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        TRANSACTION_APPROVE_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(TransactionApproveEvent::deserialize(buf)?))
    }
}
pub const TRANSACTION_UNAPPROVE_EVENT_EVENT_DISCM: [u8; 8] = [
    222,
    125,
    200,
    243,
    34,
    2,
    133,
    138,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct TransactionUnapproveEvent {
    smart_wallet: Pubkey,
    transaction: Pubkey,
    owner: Pubkey,
    timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransactionUnapproveEventEvent(pub TransactionUnapproveEvent);
impl BorshSerialize for TransactionUnapproveEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        TRANSACTION_UNAPPROVE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl TransactionUnapproveEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != TRANSACTION_UNAPPROVE_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        TRANSACTION_UNAPPROVE_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(TransactionUnapproveEvent::deserialize(buf)?))
    }
}
pub const WALLET_SET_OWNERS_EVENT_EVENT_DISCM: [u8; 8] = [
    249,
    124,
    34,
    124,
    180,
    190,
    2,
    92,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct WalletSetOwnersEvent {
    smart_wallet: Pubkey,
    owners: Vec<Pubkey>,
    timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WalletSetOwnersEventEvent(pub WalletSetOwnersEvent);
impl BorshSerialize for WalletSetOwnersEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        WALLET_SET_OWNERS_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl WalletSetOwnersEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != WALLET_SET_OWNERS_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WALLET_SET_OWNERS_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(WalletSetOwnersEvent::deserialize(buf)?))
    }
}
pub const WALLET_CHANGE_THRESHOLD_EVENT_EVENT_DISCM: [u8; 8] = [
    212,
    82,
    195,
    41,
    129,
    175,
    142,
    142,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct WalletChangeThresholdEvent {
    smart_wallet: Pubkey,
    threshold: u64,
    timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WalletChangeThresholdEventEvent(pub WalletChangeThresholdEvent);
impl BorshSerialize for WalletChangeThresholdEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        WALLET_CHANGE_THRESHOLD_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl WalletChangeThresholdEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != WALLET_CHANGE_THRESHOLD_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WALLET_CHANGE_THRESHOLD_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(WalletChangeThresholdEvent::deserialize(buf)?))
    }
}
pub const WALLET_CREATE_EVENT_EVENT_DISCM: [u8; 8] = [27, 9, 0, 75, 49, 213, 27, 111];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct WalletCreateEvent {
    smart_wallet: Pubkey,
    owners: Vec<Pubkey>,
    threshold: u64,
    minimum_delay: i64,
    timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WalletCreateEventEvent(pub WalletCreateEvent);
impl BorshSerialize for WalletCreateEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        WALLET_CREATE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl WalletCreateEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != WALLET_CREATE_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WALLET_CREATE_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(WalletCreateEvent::deserialize(buf)?))
    }
}
pub const TRANSACTION_CREATE_EVENT_EVENT_DISCM: [u8; 8] = [
    104,
    226,
    254,
    214,
    235,
    132,
    10,
    44,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct TransactionCreateEvent {
    smart_wallet: Pubkey,
    transaction: Pubkey,
    proposer: Pubkey,
    instructions: Vec<TXInstruction>,
    eta: i64,
    timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransactionCreateEventEvent(pub TransactionCreateEvent);
impl BorshSerialize for TransactionCreateEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        TRANSACTION_CREATE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl TransactionCreateEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != TRANSACTION_CREATE_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        TRANSACTION_CREATE_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(TransactionCreateEvent::deserialize(buf)?))
    }
}
pub const TRANSACTION_EXECUTE_EVENT_EVENT_DISCM: [u8; 8] = [
    114,
    75,
    70,
    78,
    74,
    159,
    82,
    70,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct TransactionExecuteEvent {
    smart_wallet: Pubkey,
    transaction: Pubkey,
    executor: Pubkey,
    timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransactionExecuteEventEvent(pub TransactionExecuteEvent);
impl BorshSerialize for TransactionExecuteEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        TRANSACTION_EXECUTE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl TransactionExecuteEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != TRANSACTION_EXECUTE_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        TRANSACTION_EXECUTE_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(TransactionExecuteEvent::deserialize(buf)?))
    }
}
pub const TRANSACTION_REMOVE_EVENT_EVENT_DISCM: [u8; 8] = [
    35,
    241,
    116,
    157,
    164,
    9,
    165,
    10,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct TransactionRemoveEvent {
    smart_wallet: Pubkey,
    transaction: Pubkey,
    proposer: Pubkey,
    timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransactionRemoveEventEvent(pub TransactionRemoveEvent);
impl BorshSerialize for TransactionRemoveEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        TRANSACTION_REMOVE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl TransactionRemoveEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != TRANSACTION_REMOVE_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        TRANSACTION_REMOVE_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(TransactionRemoveEvent::deserialize(buf)?))
    }
}
