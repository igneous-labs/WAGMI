use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use crate::*;
pub const PROPOSAL_ACTIVATE_EVENT_EVENT_DISCM: [u8; 8] = [
    247,
    53,
    166,
    250,
    118,
    62,
    53,
    80,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ProposalActivateEvent {
    governor: Pubkey,
    proposal: Pubkey,
    voting_ends_at: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProposalActivateEventEvent(pub ProposalActivateEvent);
impl BorshSerialize for ProposalActivateEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        PROPOSAL_ACTIVATE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ProposalActivateEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != PROPOSAL_ACTIVATE_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        PROPOSAL_ACTIVATE_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ProposalActivateEvent::deserialize(buf)?))
    }
}
pub const PROPOSAL_CANCEL_EVENT_EVENT_DISCM: [u8; 8] = [
    24,
    49,
    11,
    182,
    23,
    59,
    122,
    220,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ProposalCancelEvent {
    governor: Pubkey,
    proposal: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProposalCancelEventEvent(pub ProposalCancelEvent);
impl BorshSerialize for ProposalCancelEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        PROPOSAL_CANCEL_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ProposalCancelEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != PROPOSAL_CANCEL_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        PROPOSAL_CANCEL_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ProposalCancelEvent::deserialize(buf)?))
    }
}
pub const CLAIM_REWARD_EVENT_EVENT_DISCM: [u8; 8] = [207, 16, 14, 170, 176, 71, 40, 53];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ClaimRewardEvent {
    governor: Pubkey,
    voter: Pubkey,
    proposal: Pubkey,
    voting_reward: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimRewardEventEvent(pub ClaimRewardEvent);
impl BorshSerialize for ClaimRewardEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CLAIM_REWARD_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ClaimRewardEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CLAIM_REWARD_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLAIM_REWARD_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ClaimRewardEvent::deserialize(buf)?))
    }
}
pub const GOVERNOR_CREATE_EVENT_EVENT_DISCM: [u8; 8] = [117, 24, 15, 85, 39, 58, 62, 23];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct GovernorCreateEvent {
    governor: Pubkey,
    locker: Pubkey,
    smart_wallet: Pubkey,
    parameters: GovernanceParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GovernorCreateEventEvent(pub GovernorCreateEvent);
impl BorshSerialize for GovernorCreateEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        GOVERNOR_CREATE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl GovernorCreateEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != GOVERNOR_CREATE_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        GOVERNOR_CREATE_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(GovernorCreateEvent::deserialize(buf)?))
    }
}
pub const OPTION_PROPOSAL_META_CREATE_EVENT_EVENT_DISCM: [u8; 8] = [
    120,
    126,
    65,
    125,
    85,
    200,
    75,
    206,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct OptionProposalMetaCreateEvent {
    governor: Pubkey,
    proposal: Pubkey,
    option_descriptions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OptionProposalMetaCreateEventEvent(pub OptionProposalMetaCreateEvent);
impl BorshSerialize for OptionProposalMetaCreateEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        OPTION_PROPOSAL_META_CREATE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl OptionProposalMetaCreateEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != OPTION_PROPOSAL_META_CREATE_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        OPTION_PROPOSAL_META_CREATE_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(OptionProposalMetaCreateEvent::deserialize(buf)?))
    }
}
pub const PROPOSAL_META_CREATE_EVENT_EVENT_DISCM: [u8; 8] = [
    50,
    59,
    195,
    75,
    85,
    227,
    187,
    82,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ProposalMetaCreateEvent {
    governor: Pubkey,
    proposal: Pubkey,
    title: String,
    description_link: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProposalMetaCreateEventEvent(pub ProposalMetaCreateEvent);
impl BorshSerialize for ProposalMetaCreateEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        PROPOSAL_META_CREATE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ProposalMetaCreateEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != PROPOSAL_META_CREATE_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        PROPOSAL_META_CREATE_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ProposalMetaCreateEvent::deserialize(buf)?))
    }
}
pub const PROPOSAL_CREATE_EVENT_EVENT_DISCM: [u8; 8] = [
    121,
    18,
    213,
    155,
    223,
    158,
    95,
    70,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ProposalCreateEvent {
    governor: Pubkey,
    proposal: Pubkey,
    proposer: Pubkey,
    proposal_type: u8,
    max_option: u8,
    index: u64,
    instructions: Vec<ProposalInstruction>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProposalCreateEventEvent(pub ProposalCreateEvent);
impl BorshSerialize for ProposalCreateEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        PROPOSAL_CREATE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ProposalCreateEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != PROPOSAL_CREATE_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        PROPOSAL_CREATE_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ProposalCreateEvent::deserialize(buf)?))
    }
}
pub const PROPOSAL_QUEUE_EVENT_EVENT_DISCM: [u8; 8] = [
    48,
    219,
    123,
    209,
    140,
    210,
    248,
    14,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ProposalQueueEvent {
    governor: Pubkey,
    proposal: Pubkey,
    transaction: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProposalQueueEventEvent(pub ProposalQueueEvent);
impl BorshSerialize for ProposalQueueEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        PROPOSAL_QUEUE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ProposalQueueEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != PROPOSAL_QUEUE_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        PROPOSAL_QUEUE_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ProposalQueueEvent::deserialize(buf)?))
    }
}
pub const GOVERNOR_SET_PARAMS_EVENT_EVENT_DISCM: [u8; 8] = [
    169,
    129,
    187,
    152,
    130,
    17,
    81,
    157,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct GovernorSetParamsEvent {
    governor: Pubkey,
    prev_params: GovernanceParameters,
    params: GovernanceParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GovernorSetParamsEventEvent(pub GovernorSetParamsEvent);
impl BorshSerialize for GovernorSetParamsEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        GOVERNOR_SET_PARAMS_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl GovernorSetParamsEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != GOVERNOR_SET_PARAMS_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        GOVERNOR_SET_PARAMS_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(GovernorSetParamsEvent::deserialize(buf)?))
    }
}
pub const GOVERNOR_SET_VOTER_EVENT_EVENT_DISCM: [u8; 8] = [
    31,
    141,
    33,
    222,
    105,
    177,
    230,
    207,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct GovernorSetVoterEvent {
    governor: Pubkey,
    prev_locker: Pubkey,
    new_locker: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GovernorSetVoterEventEvent(pub GovernorSetVoterEvent);
impl BorshSerialize for GovernorSetVoterEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        GOVERNOR_SET_VOTER_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl GovernorSetVoterEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != GOVERNOR_SET_VOTER_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        GOVERNOR_SET_VOTER_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(GovernorSetVoterEvent::deserialize(buf)?))
    }
}
pub const VOTE_SET_EVENT_EVENT_DISCM: [u8; 8] = [175, 119, 30, 108, 176, 233, 151, 252];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct VoteSetEvent {
    governor: Pubkey,
    proposal: Pubkey,
    voter: Pubkey,
    vote: Pubkey,
    side: u8,
    voting_power: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct VoteSetEventEvent(pub VoteSetEvent);
impl BorshSerialize for VoteSetEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        VOTE_SET_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl VoteSetEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != VOTE_SET_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        VOTE_SET_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(VoteSetEvent::deserialize(buf)?))
    }
}
pub const GOVERNOR_SET_VOTING_REWARD_EVENT_DISCM: [u8; 8] = [
    74,
    82,
    223,
    19,
    41,
    16,
    148,
    200,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct GovernorSetVotingReward {
    governor: Pubkey,
    reward_mint: Pubkey,
    reward_per_proposal: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GovernorSetVotingRewardEvent(pub GovernorSetVotingReward);
impl BorshSerialize for GovernorSetVotingRewardEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        GOVERNOR_SET_VOTING_REWARD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl GovernorSetVotingRewardEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != GOVERNOR_SET_VOTING_REWARD_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        GOVERNOR_SET_VOTING_REWARD_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(GovernorSetVotingReward::deserialize(buf)?))
    }
}
