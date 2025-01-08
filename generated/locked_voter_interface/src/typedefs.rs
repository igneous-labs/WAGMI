use borsh::{BorshDeserialize, BorshSerialize};
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LockerParams {
    pub max_stake_vote_multiplier: u8,
    pub min_stake_duration: u64,
    pub max_stake_duration: u64,
    pub proposal_activation_min_votes: u64,
}
