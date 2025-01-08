use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use crate::*;
pub const GOVERNOR_ACCOUNT_DISCM: [u8; 8] = [37, 136, 44, 80, 68, 85, 213, 178];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Governor {
    pub base: Pubkey,
    pub bump: u8,
    pub proposal_count: u64,
    pub locker: Pubkey,
    pub smart_wallet: Pubkey,
    pub params: GovernanceParameters,
    pub voting_reward: VotingReward,
    pub buffers: [u128; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct GovernorAccount(pub Governor);
impl GovernorAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GOVERNOR_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        GOVERNOR_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(Governor::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GOVERNOR_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROPOSAL_ACCOUNT_DISCM: [u8; 8] = [26, 94, 189, 187, 116, 136, 53, 33];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Proposal {
    pub governor: Pubkey,
    pub index: u64,
    pub bump: u8,
    pub proposer: Pubkey,
    pub quorum_votes: u64,
    pub max_option: u8,
    pub option_votes: Vec<u64>,
    pub canceled_at: i64,
    pub created_at: i64,
    pub activated_at: i64,
    pub voting_ends_at: i64,
    pub queued_at: i64,
    pub queued_transaction: Pubkey,
    pub voting_reward: VotingReward,
    pub total_claimed_reward: u64,
    pub proposal_type: u8,
    pub buffers: [u128; 10],
    pub instructions: Vec<ProposalInstruction>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProposalAccount(pub Proposal);
impl ProposalAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROPOSAL_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        PROPOSAL_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(Proposal::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROPOSAL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PROPOSAL_META_ACCOUNT_DISCM: [u8; 8] = [50, 100, 46, 24, 151, 174, 216, 78];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProposalMeta {
    pub proposal: Pubkey,
    pub title: String,
    pub description_link: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProposalMetaAccount(pub ProposalMeta);
impl ProposalMetaAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PROPOSAL_META_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        PROPOSAL_META_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ProposalMeta::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PROPOSAL_META_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OPTION_PROPOSAL_META_ACCOUNT_DISCM: [u8; 8] = [
    200,
    56,
    229,
    124,
    113,
    154,
    32,
    26,
];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OptionProposalMeta {
    pub proposal: Pubkey,
    pub option_descriptions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OptionProposalMetaAccount(pub OptionProposalMeta);
impl OptionProposalMetaAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPTION_PROPOSAL_META_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        OPTION_PROPOSAL_META_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(OptionProposalMeta::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPTION_PROPOSAL_META_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const VOTE_ACCOUNT_DISCM: [u8; 8] = [96, 91, 104, 57, 145, 35, 172, 155];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vote {
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub bump: u8,
    pub side: u8,
    pub voting_power: u64,
    pub claimed: bool,
    pub buffers: [u8; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct VoteAccount(pub Vote);
impl VoteAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != VOTE_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        VOTE_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(Vote::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&VOTE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
