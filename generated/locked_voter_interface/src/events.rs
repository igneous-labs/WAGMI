use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use crate::*;
pub const EXTEND_LOCK_DURATION_EVENT_EVENT_DISCM: [u8; 8] = [
    56,
    121,
    52,
    182,
    84,
    133,
    195,
    191,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ExtendLockDurationEvent {
    locker: Pubkey,
    escrow_owner: Pubkey,
    token_mint: Pubkey,
    locker_supply: u64,
    duration: i64,
    prev_escrow_ends_at: i64,
    next_escrow_ends_at: i64,
    next_escrow_started_at: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExtendLockDurationEventEvent(pub ExtendLockDurationEvent);
impl BorshSerialize for ExtendLockDurationEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EXTEND_LOCK_DURATION_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ExtendLockDurationEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EXTEND_LOCK_DURATION_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EXTEND_LOCK_DURATION_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ExtendLockDurationEvent::deserialize(buf)?))
    }
}
pub const INCREASE_LOCKED_AMOUNT_EVENT_EVENT_DISCM: [u8; 8] = [
    100,
    70,
    156,
    246,
    40,
    169,
    119,
    10,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct IncreaseLockedAmountEvent {
    locker: Pubkey,
    escrow_owner: Pubkey,
    token_mint: Pubkey,
    amount: u64,
    locker_supply: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseLockedAmountEventEvent(pub IncreaseLockedAmountEvent);
impl BorshSerialize for IncreaseLockedAmountEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        INCREASE_LOCKED_AMOUNT_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl IncreaseLockedAmountEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != INCREASE_LOCKED_AMOUNT_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INCREASE_LOCKED_AMOUNT_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(IncreaseLockedAmountEvent::deserialize(buf)?))
    }
}
pub const MERGE_PARTIAL_UNSTAKING_EVENT_EVENT_DISCM: [u8; 8] = [
    144,
    54,
    22,
    42,
    231,
    68,
    85,
    65,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct MergePartialUnstakingEvent {
    partial_unstake: Pubkey,
    escrow: Pubkey,
    amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MergePartialUnstakingEventEvent(pub MergePartialUnstakingEvent);
impl BorshSerialize for MergePartialUnstakingEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        MERGE_PARTIAL_UNSTAKING_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl MergePartialUnstakingEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != MERGE_PARTIAL_UNSTAKING_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        MERGE_PARTIAL_UNSTAKING_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(MergePartialUnstakingEvent::deserialize(buf)?))
    }
}
pub const NEW_ESCROW_EVENT_EVENT_DISCM: [u8; 8] = [96, 82, 181, 204, 84, 177, 72, 141];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct NewEscrowEvent {
    escrow: Pubkey,
    escrow_owner: Pubkey,
    locker: Pubkey,
    timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NewEscrowEventEvent(pub NewEscrowEvent);
impl BorshSerialize for NewEscrowEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        NEW_ESCROW_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl NewEscrowEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != NEW_ESCROW_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        NEW_ESCROW_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(NewEscrowEvent::deserialize(buf)?))
    }
}
pub const NEW_LOCKER_EVENT_EVENT_DISCM: [u8; 8] = [149, 31, 207, 106, 172, 155, 65, 110];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct NewLockerEvent {
    governor: Pubkey,
    locker: Pubkey,
    token_mint: Pubkey,
    params: LockerParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NewLockerEventEvent(pub NewLockerEvent);
impl BorshSerialize for NewLockerEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        NEW_LOCKER_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl NewLockerEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != NEW_LOCKER_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        NEW_LOCKER_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(NewLockerEvent::deserialize(buf)?))
    }
}
pub const OPEN_PARTIAL_STAKING_EVENT_EVENT_DISCM: [u8; 8] = [
    56,
    151,
    51,
    15,
    89,
    64,
    183,
    201,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct OpenPartialStakingEvent {
    partial_unstake: Pubkey,
    escrow: Pubkey,
    amount: u64,
    expiration: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenPartialStakingEventEvent(pub OpenPartialStakingEvent);
impl BorshSerialize for OpenPartialStakingEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        OPEN_PARTIAL_STAKING_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl OpenPartialStakingEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != OPEN_PARTIAL_STAKING_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        OPEN_PARTIAL_STAKING_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(OpenPartialStakingEvent::deserialize(buf)?))
    }
}
pub const LOCKER_SET_PARAMS_EVENT_EVENT_DISCM: [u8; 8] = [
    239,
    24,
    209,
    234,
    210,
    143,
    7,
    202,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct LockerSetParamsEvent {
    locker: Pubkey,
    prev_params: LockerParams,
    params: LockerParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LockerSetParamsEventEvent(pub LockerSetParamsEvent);
impl BorshSerialize for LockerSetParamsEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        LOCKER_SET_PARAMS_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl LockerSetParamsEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != LOCKER_SET_PARAMS_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        LOCKER_SET_PARAMS_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(LockerSetParamsEvent::deserialize(buf)?))
    }
}
pub const SET_VOTE_DELEGATE_EVENT_EVENT_DISCM: [u8; 8] = [
    165,
    160,
    157,
    241,
    121,
    34,
    54,
    8,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct SetVoteDelegateEvent {
    escrow_owner: Pubkey,
    old_delegate: Pubkey,
    new_delegate: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetVoteDelegateEventEvent(pub SetVoteDelegateEvent);
impl BorshSerialize for SetVoteDelegateEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        SET_VOTE_DELEGATE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl SetVoteDelegateEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SET_VOTE_DELEGATE_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_VOTE_DELEGATE_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetVoteDelegateEvent::deserialize(buf)?))
    }
}
pub const WITHDRAW_PARTIAL_UNSTAKING_EVENT_EVENT_DISCM: [u8; 8] = [
    19,
    40,
    16,
    28,
    107,
    45,
    42,
    83,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct WithdrawPartialUnstakingEvent {
    escrow_owner: Pubkey,
    locker: Pubkey,
    partial_unstaking: Pubkey,
    timestamp: i64,
    locker_supply: u64,
    released_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawPartialUnstakingEventEvent(pub WithdrawPartialUnstakingEvent);
impl BorshSerialize for WithdrawPartialUnstakingEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        WITHDRAW_PARTIAL_UNSTAKING_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl WithdrawPartialUnstakingEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != WITHDRAW_PARTIAL_UNSTAKING_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_PARTIAL_UNSTAKING_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(WithdrawPartialUnstakingEvent::deserialize(buf)?))
    }
}
pub const EXIT_ESCROW_EVENT_EVENT_DISCM: [u8; 8] = [
    218,
    91,
    68,
    189,
    102,
    152,
    212,
    166,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ExitEscrowEvent {
    escrow_owner: Pubkey,
    locker: Pubkey,
    timestamp: i64,
    locker_supply: u64,
    released_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExitEscrowEventEvent(pub ExitEscrowEvent);
impl BorshSerialize for ExitEscrowEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EXIT_ESCROW_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ExitEscrowEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EXIT_ESCROW_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EXIT_ESCROW_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ExitEscrowEvent::deserialize(buf)?))
    }
}
