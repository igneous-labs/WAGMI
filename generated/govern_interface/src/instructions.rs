use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey, program_error::ProgramError,
};
use std::io::Read;
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum GovernProgramIx {
    CreateGovernor(CreateGovernorIxArgs),
    CreateProposal(CreateProposalIxArgs),
    ActivateProposal,
    CancelProposal,
    QueueProposal,
    NewVote(NewVoteIxArgs),
    SetVote(SetVoteIxArgs),
    SetGovernanceParams(SetGovernanceParamsIxArgs),
    SetVotingReward(SetVotingRewardIxArgs),
    ClaimReward,
    SetLocker(SetLockerIxArgs),
    CreateProposalMeta(CreateProposalMetaIxArgs),
    CreateOptionProposalMeta(CreateOptionProposalMetaIxArgs),
}
impl GovernProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            CREATE_GOVERNOR_IX_DISCM => {
                Ok(Self::CreateGovernor(CreateGovernorIxArgs::deserialize(&mut reader)?))
            }
            CREATE_PROPOSAL_IX_DISCM => {
                Ok(Self::CreateProposal(CreateProposalIxArgs::deserialize(&mut reader)?))
            }
            ACTIVATE_PROPOSAL_IX_DISCM => Ok(Self::ActivateProposal),
            CANCEL_PROPOSAL_IX_DISCM => Ok(Self::CancelProposal),
            QUEUE_PROPOSAL_IX_DISCM => Ok(Self::QueueProposal),
            NEW_VOTE_IX_DISCM => {
                Ok(Self::NewVote(NewVoteIxArgs::deserialize(&mut reader)?))
            }
            SET_VOTE_IX_DISCM => {
                Ok(Self::SetVote(SetVoteIxArgs::deserialize(&mut reader)?))
            }
            SET_GOVERNANCE_PARAMS_IX_DISCM => {
                Ok(
                    Self::SetGovernanceParams(
                        SetGovernanceParamsIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            SET_VOTING_REWARD_IX_DISCM => {
                Ok(
                    Self::SetVotingReward(
                        SetVotingRewardIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            CLAIM_REWARD_IX_DISCM => Ok(Self::ClaimReward),
            SET_LOCKER_IX_DISCM => {
                Ok(Self::SetLocker(SetLockerIxArgs::deserialize(&mut reader)?))
            }
            CREATE_PROPOSAL_META_IX_DISCM => {
                Ok(
                    Self::CreateProposalMeta(
                        CreateProposalMetaIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            CREATE_OPTION_PROPOSAL_META_IX_DISCM => {
                Ok(
                    Self::CreateOptionProposalMeta(
                        CreateOptionProposalMetaIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            _ => {
                Err(
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("discm {:?} not found", maybe_discm),
                    ),
                )
            }
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CreateGovernor(args) => {
                writer.write_all(&CREATE_GOVERNOR_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreateProposal(args) => {
                writer.write_all(&CREATE_PROPOSAL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ActivateProposal => writer.write_all(&ACTIVATE_PROPOSAL_IX_DISCM),
            Self::CancelProposal => writer.write_all(&CANCEL_PROPOSAL_IX_DISCM),
            Self::QueueProposal => writer.write_all(&QUEUE_PROPOSAL_IX_DISCM),
            Self::NewVote(args) => {
                writer.write_all(&NEW_VOTE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetVote(args) => {
                writer.write_all(&SET_VOTE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetGovernanceParams(args) => {
                writer.write_all(&SET_GOVERNANCE_PARAMS_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetVotingReward(args) => {
                writer.write_all(&SET_VOTING_REWARD_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ClaimReward => writer.write_all(&CLAIM_REWARD_IX_DISCM),
            Self::SetLocker(args) => {
                writer.write_all(&SET_LOCKER_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreateProposalMeta(args) => {
                writer.write_all(&CREATE_PROPOSAL_META_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreateOptionProposalMeta(args) => {
                writer.write_all(&CREATE_OPTION_PROPOSAL_META_IX_DISCM)?;
                args.serialize(&mut writer)
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
}
pub const CREATE_GOVERNOR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateGovernorAccounts<'me, 'info> {
    pub base: &'me AccountInfo<'info>,
    pub governor: &'me AccountInfo<'info>,
    pub smart_wallet: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateGovernorKeys {
    pub base: Pubkey,
    pub governor: Pubkey,
    pub smart_wallet: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateGovernorAccounts<'_, '_>> for CreateGovernorKeys {
    fn from(accounts: CreateGovernorAccounts) -> Self {
        Self {
            base: *accounts.base.key,
            governor: *accounts.governor.key,
            smart_wallet: *accounts.smart_wallet.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateGovernorKeys> for [AccountMeta; CREATE_GOVERNOR_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateGovernorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.governor,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_GOVERNOR_IX_ACCOUNTS_LEN]> for CreateGovernorKeys {
    fn from(pubkeys: [Pubkey; CREATE_GOVERNOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            base: pubkeys[0],
            governor: pubkeys[1],
            smart_wallet: pubkeys[2],
            payer: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CreateGovernorAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_GOVERNOR_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateGovernorAccounts<'_, 'info>) -> Self {
        [
            accounts.base.clone(),
            accounts.governor.clone(),
            accounts.smart_wallet.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_GOVERNOR_IX_ACCOUNTS_LEN]>
for CreateGovernorAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_GOVERNOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            base: &arr[0],
            governor: &arr[1],
            smart_wallet: &arr[2],
            payer: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CREATE_GOVERNOR_IX_DISCM: [u8; 8] = [103, 30, 78, 252, 28, 128, 40, 3];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateGovernorIxArgs {
    pub locker: Pubkey,
    pub params: GovernanceParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateGovernorIxData(pub CreateGovernorIxArgs);
impl From<CreateGovernorIxArgs> for CreateGovernorIxData {
    fn from(args: CreateGovernorIxArgs) -> Self {
        Self(args)
    }
}
impl CreateGovernorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_GOVERNOR_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_GOVERNOR_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateGovernorIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_GOVERNOR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_governor_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateGovernorKeys,
    args: CreateGovernorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_GOVERNOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateGovernorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_governor_ix(
    keys: CreateGovernorKeys,
    args: CreateGovernorIxArgs,
) -> std::io::Result<Instruction> {
    create_governor_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_governor_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateGovernorAccounts<'_, '_>,
    args: CreateGovernorIxArgs,
) -> ProgramResult {
    let keys: CreateGovernorKeys = accounts.into();
    let ix = create_governor_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_governor_invoke(
    accounts: CreateGovernorAccounts<'_, '_>,
    args: CreateGovernorIxArgs,
) -> ProgramResult {
    create_governor_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_governor_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateGovernorAccounts<'_, '_>,
    args: CreateGovernorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateGovernorKeys = accounts.into();
    let ix = create_governor_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_governor_invoke_signed(
    accounts: CreateGovernorAccounts<'_, '_>,
    args: CreateGovernorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_governor_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_governor_verify_account_keys(
    accounts: CreateGovernorAccounts<'_, '_>,
    keys: CreateGovernorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.base.key, keys.base),
        (*accounts.governor.key, keys.governor),
        (*accounts.smart_wallet.key, keys.smart_wallet),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_governor_verify_writable_privileges<'me, 'info>(
    accounts: CreateGovernorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.governor, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_governor_verify_signer_privileges<'me, 'info>(
    accounts: CreateGovernorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.base, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_governor_verify_account_privileges<'me, 'info>(
    accounts: CreateGovernorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_governor_verify_writable_privileges(accounts)?;
    create_governor_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_PROPOSAL_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct CreateProposalAccounts<'me, 'info> {
    pub governor: &'me AccountInfo<'info>,
    pub proposal: &'me AccountInfo<'info>,
    pub smart_wallet: &'me AccountInfo<'info>,
    pub proposer: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateProposalKeys {
    pub governor: Pubkey,
    pub proposal: Pubkey,
    pub smart_wallet: Pubkey,
    pub proposer: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateProposalAccounts<'_, '_>> for CreateProposalKeys {
    fn from(accounts: CreateProposalAccounts) -> Self {
        Self {
            governor: *accounts.governor.key,
            proposal: *accounts.proposal.key,
            smart_wallet: *accounts.smart_wallet.key,
            proposer: *accounts.proposer.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateProposalKeys> for [AccountMeta; CREATE_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.governor,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.proposer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_PROPOSAL_IX_ACCOUNTS_LEN]> for CreateProposalKeys {
    fn from(pubkeys: [Pubkey; CREATE_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: pubkeys[0],
            proposal: pubkeys[1],
            smart_wallet: pubkeys[2],
            proposer: pubkeys[3],
            payer: pubkeys[4],
            system_program: pubkeys[5],
            event_authority: pubkeys[6],
            program: pubkeys[7],
        }
    }
}
impl<'info> From<CreateProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.governor.clone(),
            accounts.proposal.clone(),
            accounts.smart_wallet.clone(),
            accounts.proposer.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_PROPOSAL_IX_ACCOUNTS_LEN]>
for CreateProposalAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: &arr[0],
            proposal: &arr[1],
            smart_wallet: &arr[2],
            proposer: &arr[3],
            payer: &arr[4],
            system_program: &arr[5],
            event_authority: &arr[6],
            program: &arr[7],
        }
    }
}
pub const CREATE_PROPOSAL_IX_DISCM: [u8; 8] = [132, 116, 68, 174, 216, 160, 198, 22];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateProposalIxArgs {
    pub proposal_type: u8,
    pub max_option: u8,
    pub instructions: Vec<ProposalInstruction>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateProposalIxData(pub CreateProposalIxArgs);
impl From<CreateProposalIxArgs> for CreateProposalIxData {
    fn from(args: CreateProposalIxArgs) -> Self {
        Self(args)
    }
}
impl CreateProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PROPOSAL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_PROPOSAL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateProposalIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PROPOSAL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateProposalKeys,
    args: CreateProposalIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_PROPOSAL_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateProposalIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_proposal_ix(
    keys: CreateProposalKeys,
    args: CreateProposalIxArgs,
) -> std::io::Result<Instruction> {
    create_proposal_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateProposalAccounts<'_, '_>,
    args: CreateProposalIxArgs,
) -> ProgramResult {
    let keys: CreateProposalKeys = accounts.into();
    let ix = create_proposal_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_proposal_invoke(
    accounts: CreateProposalAccounts<'_, '_>,
    args: CreateProposalIxArgs,
) -> ProgramResult {
    create_proposal_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateProposalAccounts<'_, '_>,
    args: CreateProposalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateProposalKeys = accounts.into();
    let ix = create_proposal_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_proposal_invoke_signed(
    accounts: CreateProposalAccounts<'_, '_>,
    args: CreateProposalIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_proposal_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_proposal_verify_account_keys(
    accounts: CreateProposalAccounts<'_, '_>,
    keys: CreateProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.governor.key, keys.governor),
        (*accounts.proposal.key, keys.proposal),
        (*accounts.smart_wallet.key, keys.smart_wallet),
        (*accounts.proposer.key, keys.proposer),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_proposal_verify_writable_privileges<'me, 'info>(
    accounts: CreateProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.governor, accounts.proposal, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_proposal_verify_signer_privileges<'me, 'info>(
    accounts: CreateProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.proposer, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_proposal_verify_account_privileges<'me, 'info>(
    accounts: CreateProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_proposal_verify_writable_privileges(accounts)?;
    create_proposal_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ACTIVATE_PROPOSAL_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ActivateProposalAccounts<'me, 'info> {
    pub governor: &'me AccountInfo<'info>,
    pub proposal: &'me AccountInfo<'info>,
    pub locker: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ActivateProposalKeys {
    pub governor: Pubkey,
    pub proposal: Pubkey,
    pub locker: Pubkey,
}
impl From<ActivateProposalAccounts<'_, '_>> for ActivateProposalKeys {
    fn from(accounts: ActivateProposalAccounts) -> Self {
        Self {
            governor: *accounts.governor.key,
            proposal: *accounts.proposal.key,
            locker: *accounts.locker.key,
        }
    }
}
impl From<ActivateProposalKeys> for [AccountMeta; ACTIVATE_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: ActivateProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.governor,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.locker,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ACTIVATE_PROPOSAL_IX_ACCOUNTS_LEN]> for ActivateProposalKeys {
    fn from(pubkeys: [Pubkey; ACTIVATE_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: pubkeys[0],
            proposal: pubkeys[1],
            locker: pubkeys[2],
        }
    }
}
impl<'info> From<ActivateProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; ACTIVATE_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: ActivateProposalAccounts<'_, 'info>) -> Self {
        [accounts.governor.clone(), accounts.proposal.clone(), accounts.locker.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ACTIVATE_PROPOSAL_IX_ACCOUNTS_LEN]>
for ActivateProposalAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ACTIVATE_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: &arr[0],
            proposal: &arr[1],
            locker: &arr[2],
        }
    }
}
pub const ACTIVATE_PROPOSAL_IX_DISCM: [u8; 8] = [90, 186, 203, 234, 70, 185, 191, 21];
#[derive(Clone, Debug, PartialEq)]
pub struct ActivateProposalIxData;
impl ActivateProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ACTIVATE_PROPOSAL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        ACTIVATE_PROPOSAL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ACTIVATE_PROPOSAL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn activate_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: ActivateProposalKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ACTIVATE_PROPOSAL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ActivateProposalIxData.try_to_vec()?,
    })
}
pub fn activate_proposal_ix(keys: ActivateProposalKeys) -> std::io::Result<Instruction> {
    activate_proposal_ix_with_program_id(crate::ID, keys)
}
pub fn activate_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ActivateProposalAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ActivateProposalKeys = accounts.into();
    let ix = activate_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn activate_proposal_invoke(
    accounts: ActivateProposalAccounts<'_, '_>,
) -> ProgramResult {
    activate_proposal_invoke_with_program_id(crate::ID, accounts)
}
pub fn activate_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ActivateProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ActivateProposalKeys = accounts.into();
    let ix = activate_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn activate_proposal_invoke_signed(
    accounts: ActivateProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    activate_proposal_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn activate_proposal_verify_account_keys(
    accounts: ActivateProposalAccounts<'_, '_>,
    keys: ActivateProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.governor.key, keys.governor),
        (*accounts.proposal.key, keys.proposal),
        (*accounts.locker.key, keys.locker),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn activate_proposal_verify_writable_privileges<'me, 'info>(
    accounts: ActivateProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.proposal] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn activate_proposal_verify_signer_privileges<'me, 'info>(
    accounts: ActivateProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.locker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn activate_proposal_verify_account_privileges<'me, 'info>(
    accounts: ActivateProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    activate_proposal_verify_writable_privileges(accounts)?;
    activate_proposal_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_PROPOSAL_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CancelProposalAccounts<'me, 'info> {
    pub governor: &'me AccountInfo<'info>,
    pub proposal: &'me AccountInfo<'info>,
    pub proposer: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelProposalKeys {
    pub governor: Pubkey,
    pub proposal: Pubkey,
    pub proposer: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CancelProposalAccounts<'_, '_>> for CancelProposalKeys {
    fn from(accounts: CancelProposalAccounts) -> Self {
        Self {
            governor: *accounts.governor.key,
            proposal: *accounts.proposal.key,
            proposer: *accounts.proposer.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CancelProposalKeys> for [AccountMeta; CANCEL_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.governor,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.proposer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CANCEL_PROPOSAL_IX_ACCOUNTS_LEN]> for CancelProposalKeys {
    fn from(pubkeys: [Pubkey; CANCEL_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: pubkeys[0],
            proposal: pubkeys[1],
            proposer: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<CancelProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.governor.clone(),
            accounts.proposal.clone(),
            accounts.proposer.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_PROPOSAL_IX_ACCOUNTS_LEN]>
for CancelProposalAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CANCEL_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: &arr[0],
            proposal: &arr[1],
            proposer: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const CANCEL_PROPOSAL_IX_DISCM: [u8; 8] = [106, 74, 128, 146, 19, 65, 39, 23];
#[derive(Clone, Debug, PartialEq)]
pub struct CancelProposalIxData;
impl CancelProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_PROPOSAL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CANCEL_PROPOSAL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_PROPOSAL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelProposalKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_PROPOSAL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CancelProposalIxData.try_to_vec()?,
    })
}
pub fn cancel_proposal_ix(keys: CancelProposalKeys) -> std::io::Result<Instruction> {
    cancel_proposal_ix_with_program_id(crate::ID, keys)
}
pub fn cancel_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelProposalAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CancelProposalKeys = accounts.into();
    let ix = cancel_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_proposal_invoke(
    accounts: CancelProposalAccounts<'_, '_>,
) -> ProgramResult {
    cancel_proposal_invoke_with_program_id(crate::ID, accounts)
}
pub fn cancel_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelProposalKeys = accounts.into();
    let ix = cancel_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_proposal_invoke_signed(
    accounts: CancelProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_proposal_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn cancel_proposal_verify_account_keys(
    accounts: CancelProposalAccounts<'_, '_>,
    keys: CancelProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.governor.key, keys.governor),
        (*accounts.proposal.key, keys.proposal),
        (*accounts.proposer.key, keys.proposer),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_proposal_verify_writable_privileges<'me, 'info>(
    accounts: CancelProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.proposal] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_proposal_verify_signer_privileges<'me, 'info>(
    accounts: CancelProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.proposer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_proposal_verify_account_privileges<'me, 'info>(
    accounts: CancelProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_proposal_verify_writable_privileges(accounts)?;
    cancel_proposal_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const QUEUE_PROPOSAL_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct QueueProposalAccounts<'me, 'info> {
    pub governor: &'me AccountInfo<'info>,
    pub proposal: &'me AccountInfo<'info>,
    pub transaction: &'me AccountInfo<'info>,
    pub smart_wallet: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub smart_wallet_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QueueProposalKeys {
    pub governor: Pubkey,
    pub proposal: Pubkey,
    pub transaction: Pubkey,
    pub smart_wallet: Pubkey,
    pub payer: Pubkey,
    pub smart_wallet_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<QueueProposalAccounts<'_, '_>> for QueueProposalKeys {
    fn from(accounts: QueueProposalAccounts) -> Self {
        Self {
            governor: *accounts.governor.key,
            proposal: *accounts.proposal.key,
            transaction: *accounts.transaction.key,
            smart_wallet: *accounts.smart_wallet.key,
            payer: *accounts.payer.key,
            smart_wallet_program: *accounts.smart_wallet_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<QueueProposalKeys> for [AccountMeta; QUEUE_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: QueueProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.governor,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transaction,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.smart_wallet_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; QUEUE_PROPOSAL_IX_ACCOUNTS_LEN]> for QueueProposalKeys {
    fn from(pubkeys: [Pubkey; QUEUE_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: pubkeys[0],
            proposal: pubkeys[1],
            transaction: pubkeys[2],
            smart_wallet: pubkeys[3],
            payer: pubkeys[4],
            smart_wallet_program: pubkeys[5],
            system_program: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<QueueProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; QUEUE_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: QueueProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.governor.clone(),
            accounts.proposal.clone(),
            accounts.transaction.clone(),
            accounts.smart_wallet.clone(),
            accounts.payer.clone(),
            accounts.smart_wallet_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; QUEUE_PROPOSAL_IX_ACCOUNTS_LEN]>
for QueueProposalAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; QUEUE_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: &arr[0],
            proposal: &arr[1],
            transaction: &arr[2],
            smart_wallet: &arr[3],
            payer: &arr[4],
            smart_wallet_program: &arr[5],
            system_program: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const QUEUE_PROPOSAL_IX_DISCM: [u8; 8] = [168, 219, 139, 211, 205, 152, 125, 110];
#[derive(Clone, Debug, PartialEq)]
pub struct QueueProposalIxData;
impl QueueProposalIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != QUEUE_PROPOSAL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        QUEUE_PROPOSAL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&QUEUE_PROPOSAL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn queue_proposal_ix_with_program_id(
    program_id: Pubkey,
    keys: QueueProposalKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; QUEUE_PROPOSAL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: QueueProposalIxData.try_to_vec()?,
    })
}
pub fn queue_proposal_ix(keys: QueueProposalKeys) -> std::io::Result<Instruction> {
    queue_proposal_ix_with_program_id(crate::ID, keys)
}
pub fn queue_proposal_invoke_with_program_id(
    program_id: Pubkey,
    accounts: QueueProposalAccounts<'_, '_>,
) -> ProgramResult {
    let keys: QueueProposalKeys = accounts.into();
    let ix = queue_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn queue_proposal_invoke(accounts: QueueProposalAccounts<'_, '_>) -> ProgramResult {
    queue_proposal_invoke_with_program_id(crate::ID, accounts)
}
pub fn queue_proposal_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: QueueProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: QueueProposalKeys = accounts.into();
    let ix = queue_proposal_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn queue_proposal_invoke_signed(
    accounts: QueueProposalAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    queue_proposal_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn queue_proposal_verify_account_keys(
    accounts: QueueProposalAccounts<'_, '_>,
    keys: QueueProposalKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.governor.key, keys.governor),
        (*accounts.proposal.key, keys.proposal),
        (*accounts.transaction.key, keys.transaction),
        (*accounts.smart_wallet.key, keys.smart_wallet),
        (*accounts.payer.key, keys.payer),
        (*accounts.smart_wallet_program.key, keys.smart_wallet_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn queue_proposal_verify_writable_privileges<'me, 'info>(
    accounts: QueueProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.proposal,
        accounts.transaction,
        accounts.smart_wallet,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn queue_proposal_verify_signer_privileges<'me, 'info>(
    accounts: QueueProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn queue_proposal_verify_account_privileges<'me, 'info>(
    accounts: QueueProposalAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    queue_proposal_verify_writable_privileges(accounts)?;
    queue_proposal_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const NEW_VOTE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct NewVoteAccounts<'me, 'info> {
    pub proposal: &'me AccountInfo<'info>,
    pub vote: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NewVoteKeys {
    pub proposal: Pubkey,
    pub vote: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<NewVoteAccounts<'_, '_>> for NewVoteKeys {
    fn from(accounts: NewVoteAccounts) -> Self {
        Self {
            proposal: *accounts.proposal.key,
            vote: *accounts.vote.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<NewVoteKeys> for [AccountMeta; NEW_VOTE_IX_ACCOUNTS_LEN] {
    fn from(keys: NewVoteKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; NEW_VOTE_IX_ACCOUNTS_LEN]> for NewVoteKeys {
    fn from(pubkeys: [Pubkey; NEW_VOTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: pubkeys[0],
            vote: pubkeys[1],
            payer: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<NewVoteAccounts<'_, 'info>>
for [AccountInfo<'info>; NEW_VOTE_IX_ACCOUNTS_LEN] {
    fn from(accounts: NewVoteAccounts<'_, 'info>) -> Self {
        [
            accounts.proposal.clone(),
            accounts.vote.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NEW_VOTE_IX_ACCOUNTS_LEN]>
for NewVoteAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; NEW_VOTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: &arr[0],
            vote: &arr[1],
            payer: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const NEW_VOTE_IX_DISCM: [u8; 8] = [163, 108, 157, 189, 140, 80, 13, 143];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewVoteIxArgs {
    pub voter: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NewVoteIxData(pub NewVoteIxArgs);
impl From<NewVoteIxArgs> for NewVoteIxData {
    fn from(args: NewVoteIxArgs) -> Self {
        Self(args)
    }
}
impl NewVoteIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != NEW_VOTE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        NEW_VOTE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(NewVoteIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&NEW_VOTE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn new_vote_ix_with_program_id(
    program_id: Pubkey,
    keys: NewVoteKeys,
    args: NewVoteIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NEW_VOTE_IX_ACCOUNTS_LEN] = keys.into();
    let data: NewVoteIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn new_vote_ix(
    keys: NewVoteKeys,
    args: NewVoteIxArgs,
) -> std::io::Result<Instruction> {
    new_vote_ix_with_program_id(crate::ID, keys, args)
}
pub fn new_vote_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NewVoteAccounts<'_, '_>,
    args: NewVoteIxArgs,
) -> ProgramResult {
    let keys: NewVoteKeys = accounts.into();
    let ix = new_vote_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn new_vote_invoke(
    accounts: NewVoteAccounts<'_, '_>,
    args: NewVoteIxArgs,
) -> ProgramResult {
    new_vote_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn new_vote_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NewVoteAccounts<'_, '_>,
    args: NewVoteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NewVoteKeys = accounts.into();
    let ix = new_vote_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn new_vote_invoke_signed(
    accounts: NewVoteAccounts<'_, '_>,
    args: NewVoteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    new_vote_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn new_vote_verify_account_keys(
    accounts: NewVoteAccounts<'_, '_>,
    keys: NewVoteKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.proposal.key, keys.proposal),
        (*accounts.vote.key, keys.vote),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn new_vote_verify_writable_privileges<'me, 'info>(
    accounts: NewVoteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vote, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn new_vote_verify_signer_privileges<'me, 'info>(
    accounts: NewVoteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn new_vote_verify_account_privileges<'me, 'info>(
    accounts: NewVoteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    new_vote_verify_writable_privileges(accounts)?;
    new_vote_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_VOTE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetVoteAccounts<'me, 'info> {
    pub governor: &'me AccountInfo<'info>,
    pub proposal: &'me AccountInfo<'info>,
    pub vote: &'me AccountInfo<'info>,
    pub locker: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetVoteKeys {
    pub governor: Pubkey,
    pub proposal: Pubkey,
    pub vote: Pubkey,
    pub locker: Pubkey,
}
impl From<SetVoteAccounts<'_, '_>> for SetVoteKeys {
    fn from(accounts: SetVoteAccounts) -> Self {
        Self {
            governor: *accounts.governor.key,
            proposal: *accounts.proposal.key,
            vote: *accounts.vote.key,
            locker: *accounts.locker.key,
        }
    }
}
impl From<SetVoteKeys> for [AccountMeta; SET_VOTE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetVoteKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.governor,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.locker,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_VOTE_IX_ACCOUNTS_LEN]> for SetVoteKeys {
    fn from(pubkeys: [Pubkey; SET_VOTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: pubkeys[0],
            proposal: pubkeys[1],
            vote: pubkeys[2],
            locker: pubkeys[3],
        }
    }
}
impl<'info> From<SetVoteAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_VOTE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetVoteAccounts<'_, 'info>) -> Self {
        [
            accounts.governor.clone(),
            accounts.proposal.clone(),
            accounts.vote.clone(),
            accounts.locker.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_VOTE_IX_ACCOUNTS_LEN]>
for SetVoteAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_VOTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: &arr[0],
            proposal: &arr[1],
            vote: &arr[2],
            locker: &arr[3],
        }
    }
}
pub const SET_VOTE_IX_DISCM: [u8; 8] = [171, 33, 83, 172, 148, 215, 239, 97];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetVoteIxArgs {
    pub side: u8,
    pub weight: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetVoteIxData(pub SetVoteIxArgs);
impl From<SetVoteIxArgs> for SetVoteIxData {
    fn from(args: SetVoteIxArgs) -> Self {
        Self(args)
    }
}
impl SetVoteIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_VOTE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_VOTE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetVoteIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_VOTE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_vote_ix_with_program_id(
    program_id: Pubkey,
    keys: SetVoteKeys,
    args: SetVoteIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_VOTE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetVoteIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_vote_ix(
    keys: SetVoteKeys,
    args: SetVoteIxArgs,
) -> std::io::Result<Instruction> {
    set_vote_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_vote_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetVoteAccounts<'_, '_>,
    args: SetVoteIxArgs,
) -> ProgramResult {
    let keys: SetVoteKeys = accounts.into();
    let ix = set_vote_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_vote_invoke(
    accounts: SetVoteAccounts<'_, '_>,
    args: SetVoteIxArgs,
) -> ProgramResult {
    set_vote_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_vote_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetVoteAccounts<'_, '_>,
    args: SetVoteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetVoteKeys = accounts.into();
    let ix = set_vote_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_vote_invoke_signed(
    accounts: SetVoteAccounts<'_, '_>,
    args: SetVoteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_vote_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_vote_verify_account_keys(
    accounts: SetVoteAccounts<'_, '_>,
    keys: SetVoteKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.governor.key, keys.governor),
        (*accounts.proposal.key, keys.proposal),
        (*accounts.vote.key, keys.vote),
        (*accounts.locker.key, keys.locker),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_vote_verify_writable_privileges<'me, 'info>(
    accounts: SetVoteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.proposal, accounts.vote] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_vote_verify_signer_privileges<'me, 'info>(
    accounts: SetVoteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.locker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_vote_verify_account_privileges<'me, 'info>(
    accounts: SetVoteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_vote_verify_writable_privileges(accounts)?;
    set_vote_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_GOVERNANCE_PARAMS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetGovernanceParamsAccounts<'me, 'info> {
    pub governor: &'me AccountInfo<'info>,
    pub smart_wallet: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetGovernanceParamsKeys {
    pub governor: Pubkey,
    pub smart_wallet: Pubkey,
}
impl From<SetGovernanceParamsAccounts<'_, '_>> for SetGovernanceParamsKeys {
    fn from(accounts: SetGovernanceParamsAccounts) -> Self {
        Self {
            governor: *accounts.governor.key,
            smart_wallet: *accounts.smart_wallet.key,
        }
    }
}
impl From<SetGovernanceParamsKeys>
for [AccountMeta; SET_GOVERNANCE_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetGovernanceParamsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.governor,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_GOVERNANCE_PARAMS_IX_ACCOUNTS_LEN]> for SetGovernanceParamsKeys {
    fn from(pubkeys: [Pubkey; SET_GOVERNANCE_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: pubkeys[0],
            smart_wallet: pubkeys[1],
        }
    }
}
impl<'info> From<SetGovernanceParamsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_GOVERNANCE_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetGovernanceParamsAccounts<'_, 'info>) -> Self {
        [accounts.governor.clone(), accounts.smart_wallet.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_GOVERNANCE_PARAMS_IX_ACCOUNTS_LEN]>
for SetGovernanceParamsAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; SET_GOVERNANCE_PARAMS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            governor: &arr[0],
            smart_wallet: &arr[1],
        }
    }
}
pub const SET_GOVERNANCE_PARAMS_IX_DISCM: [u8; 8] = [175, 187, 3, 73, 8, 251, 67, 178];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetGovernanceParamsIxArgs {
    pub params: GovernanceParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetGovernanceParamsIxData(pub SetGovernanceParamsIxArgs);
impl From<SetGovernanceParamsIxArgs> for SetGovernanceParamsIxData {
    fn from(args: SetGovernanceParamsIxArgs) -> Self {
        Self(args)
    }
}
impl SetGovernanceParamsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_GOVERNANCE_PARAMS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_GOVERNANCE_PARAMS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetGovernanceParamsIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_GOVERNANCE_PARAMS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_governance_params_ix_with_program_id(
    program_id: Pubkey,
    keys: SetGovernanceParamsKeys,
    args: SetGovernanceParamsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_GOVERNANCE_PARAMS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetGovernanceParamsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_governance_params_ix(
    keys: SetGovernanceParamsKeys,
    args: SetGovernanceParamsIxArgs,
) -> std::io::Result<Instruction> {
    set_governance_params_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_governance_params_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetGovernanceParamsAccounts<'_, '_>,
    args: SetGovernanceParamsIxArgs,
) -> ProgramResult {
    let keys: SetGovernanceParamsKeys = accounts.into();
    let ix = set_governance_params_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_governance_params_invoke(
    accounts: SetGovernanceParamsAccounts<'_, '_>,
    args: SetGovernanceParamsIxArgs,
) -> ProgramResult {
    set_governance_params_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_governance_params_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetGovernanceParamsAccounts<'_, '_>,
    args: SetGovernanceParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetGovernanceParamsKeys = accounts.into();
    let ix = set_governance_params_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_governance_params_invoke_signed(
    accounts: SetGovernanceParamsAccounts<'_, '_>,
    args: SetGovernanceParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_governance_params_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_governance_params_verify_account_keys(
    accounts: SetGovernanceParamsAccounts<'_, '_>,
    keys: SetGovernanceParamsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.governor.key, keys.governor),
        (*accounts.smart_wallet.key, keys.smart_wallet),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_governance_params_verify_writable_privileges<'me, 'info>(
    accounts: SetGovernanceParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.governor] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_governance_params_verify_signer_privileges<'me, 'info>(
    accounts: SetGovernanceParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.smart_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_governance_params_verify_account_privileges<'me, 'info>(
    accounts: SetGovernanceParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_governance_params_verify_writable_privileges(accounts)?;
    set_governance_params_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_VOTING_REWARD_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetVotingRewardAccounts<'me, 'info> {
    pub governor: &'me AccountInfo<'info>,
    pub reward_mint: &'me AccountInfo<'info>,
    pub smart_wallet: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetVotingRewardKeys {
    pub governor: Pubkey,
    pub reward_mint: Pubkey,
    pub smart_wallet: Pubkey,
}
impl From<SetVotingRewardAccounts<'_, '_>> for SetVotingRewardKeys {
    fn from(accounts: SetVotingRewardAccounts) -> Self {
        Self {
            governor: *accounts.governor.key,
            reward_mint: *accounts.reward_mint.key,
            smart_wallet: *accounts.smart_wallet.key,
        }
    }
}
impl From<SetVotingRewardKeys> for [AccountMeta; SET_VOTING_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: SetVotingRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.governor,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_VOTING_REWARD_IX_ACCOUNTS_LEN]> for SetVotingRewardKeys {
    fn from(pubkeys: [Pubkey; SET_VOTING_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: pubkeys[0],
            reward_mint: pubkeys[1],
            smart_wallet: pubkeys[2],
        }
    }
}
impl<'info> From<SetVotingRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_VOTING_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetVotingRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.governor.clone(),
            accounts.reward_mint.clone(),
            accounts.smart_wallet.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_VOTING_REWARD_IX_ACCOUNTS_LEN]>
for SetVotingRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_VOTING_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: &arr[0],
            reward_mint: &arr[1],
            smart_wallet: &arr[2],
        }
    }
}
pub const SET_VOTING_REWARD_IX_DISCM: [u8; 8] = [227, 241, 48, 137, 30, 26, 104, 70];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetVotingRewardIxArgs {
    pub reward_per_proposal: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetVotingRewardIxData(pub SetVotingRewardIxArgs);
impl From<SetVotingRewardIxArgs> for SetVotingRewardIxData {
    fn from(args: SetVotingRewardIxArgs) -> Self {
        Self(args)
    }
}
impl SetVotingRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_VOTING_REWARD_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_VOTING_REWARD_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetVotingRewardIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_VOTING_REWARD_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_voting_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: SetVotingRewardKeys,
    args: SetVotingRewardIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_VOTING_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetVotingRewardIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_voting_reward_ix(
    keys: SetVotingRewardKeys,
    args: SetVotingRewardIxArgs,
) -> std::io::Result<Instruction> {
    set_voting_reward_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_voting_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetVotingRewardAccounts<'_, '_>,
    args: SetVotingRewardIxArgs,
) -> ProgramResult {
    let keys: SetVotingRewardKeys = accounts.into();
    let ix = set_voting_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_voting_reward_invoke(
    accounts: SetVotingRewardAccounts<'_, '_>,
    args: SetVotingRewardIxArgs,
) -> ProgramResult {
    set_voting_reward_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_voting_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetVotingRewardAccounts<'_, '_>,
    args: SetVotingRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetVotingRewardKeys = accounts.into();
    let ix = set_voting_reward_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_voting_reward_invoke_signed(
    accounts: SetVotingRewardAccounts<'_, '_>,
    args: SetVotingRewardIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_voting_reward_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_voting_reward_verify_account_keys(
    accounts: SetVotingRewardAccounts<'_, '_>,
    keys: SetVotingRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.governor.key, keys.governor),
        (*accounts.reward_mint.key, keys.reward_mint),
        (*accounts.smart_wallet.key, keys.smart_wallet),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_voting_reward_verify_writable_privileges<'me, 'info>(
    accounts: SetVotingRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.governor] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_voting_reward_verify_signer_privileges<'me, 'info>(
    accounts: SetVotingRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.smart_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_voting_reward_verify_account_privileges<'me, 'info>(
    accounts: SetVotingRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_voting_reward_verify_writable_privileges(accounts)?;
    set_voting_reward_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_REWARD_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct ClaimRewardAccounts<'me, 'info> {
    pub governor: &'me AccountInfo<'info>,
    pub reward_vault: &'me AccountInfo<'info>,
    pub proposal: &'me AccountInfo<'info>,
    pub vote: &'me AccountInfo<'info>,
    pub voter: &'me AccountInfo<'info>,
    pub voter_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimRewardKeys {
    pub governor: Pubkey,
    pub reward_vault: Pubkey,
    pub proposal: Pubkey,
    pub vote: Pubkey,
    pub voter: Pubkey,
    pub voter_token_account: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimRewardAccounts<'_, '_>> for ClaimRewardKeys {
    fn from(accounts: ClaimRewardAccounts) -> Self {
        Self {
            governor: *accounts.governor.key,
            reward_vault: *accounts.reward_vault.key,
            proposal: *accounts.proposal.key,
            vote: *accounts.vote.key,
            voter: *accounts.voter.key,
            voter_token_account: *accounts.voter_token_account.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimRewardKeys> for [AccountMeta; CLAIM_REWARD_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimRewardKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.governor,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reward_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vote,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.voter,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.voter_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_REWARD_IX_ACCOUNTS_LEN]> for ClaimRewardKeys {
    fn from(pubkeys: [Pubkey; CLAIM_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: pubkeys[0],
            reward_vault: pubkeys[1],
            proposal: pubkeys[2],
            vote: pubkeys[3],
            voter: pubkeys[4],
            voter_token_account: pubkeys[5],
            token_program: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<ClaimRewardAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_REWARD_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimRewardAccounts<'_, 'info>) -> Self {
        [
            accounts.governor.clone(),
            accounts.reward_vault.clone(),
            accounts.proposal.clone(),
            accounts.vote.clone(),
            accounts.voter.clone(),
            accounts.voter_token_account.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_REWARD_IX_ACCOUNTS_LEN]>
for ClaimRewardAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_REWARD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: &arr[0],
            reward_vault: &arr[1],
            proposal: &arr[2],
            vote: &arr[3],
            voter: &arr[4],
            voter_token_account: &arr[5],
            token_program: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const CLAIM_REWARD_IX_DISCM: [u8; 8] = [149, 95, 181, 242, 94, 90, 158, 162];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimRewardIxData;
impl ClaimRewardIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_REWARD_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLAIM_REWARD_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_REWARD_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_reward_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimRewardKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_REWARD_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimRewardIxData.try_to_vec()?,
    })
}
pub fn claim_reward_ix(keys: ClaimRewardKeys) -> std::io::Result<Instruction> {
    claim_reward_ix_with_program_id(crate::ID, keys)
}
pub fn claim_reward_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimRewardAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimRewardKeys = accounts.into();
    let ix = claim_reward_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_reward_invoke(accounts: ClaimRewardAccounts<'_, '_>) -> ProgramResult {
    claim_reward_invoke_with_program_id(crate::ID, accounts)
}
pub fn claim_reward_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimRewardAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimRewardKeys = accounts.into();
    let ix = claim_reward_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_reward_invoke_signed(
    accounts: ClaimRewardAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_reward_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn claim_reward_verify_account_keys(
    accounts: ClaimRewardAccounts<'_, '_>,
    keys: ClaimRewardKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.governor.key, keys.governor),
        (*accounts.reward_vault.key, keys.reward_vault),
        (*accounts.proposal.key, keys.proposal),
        (*accounts.vote.key, keys.vote),
        (*accounts.voter.key, keys.voter),
        (*accounts.voter_token_account.key, keys.voter_token_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_reward_verify_writable_privileges<'me, 'info>(
    accounts: ClaimRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.governor,
        accounts.reward_vault,
        accounts.proposal,
        accounts.vote,
        accounts.voter_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_reward_verify_signer_privileges<'me, 'info>(
    accounts: ClaimRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.voter] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_reward_verify_account_privileges<'me, 'info>(
    accounts: ClaimRewardAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_reward_verify_writable_privileges(accounts)?;
    claim_reward_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_LOCKER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetLockerAccounts<'me, 'info> {
    pub governor: &'me AccountInfo<'info>,
    pub smart_wallet: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetLockerKeys {
    pub governor: Pubkey,
    pub smart_wallet: Pubkey,
}
impl From<SetLockerAccounts<'_, '_>> for SetLockerKeys {
    fn from(accounts: SetLockerAccounts) -> Self {
        Self {
            governor: *accounts.governor.key,
            smart_wallet: *accounts.smart_wallet.key,
        }
    }
}
impl From<SetLockerKeys> for [AccountMeta; SET_LOCKER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetLockerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.governor,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_LOCKER_IX_ACCOUNTS_LEN]> for SetLockerKeys {
    fn from(pubkeys: [Pubkey; SET_LOCKER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: pubkeys[0],
            smart_wallet: pubkeys[1],
        }
    }
}
impl<'info> From<SetLockerAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_LOCKER_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetLockerAccounts<'_, 'info>) -> Self {
        [accounts.governor.clone(), accounts.smart_wallet.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_LOCKER_IX_ACCOUNTS_LEN]>
for SetLockerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_LOCKER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            governor: &arr[0],
            smart_wallet: &arr[1],
        }
    }
}
pub const SET_LOCKER_IX_DISCM: [u8; 8] = [17, 6, 101, 72, 250, 23, 152, 96];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetLockerIxArgs {
    pub new_locker: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetLockerIxData(pub SetLockerIxArgs);
impl From<SetLockerIxArgs> for SetLockerIxData {
    fn from(args: SetLockerIxArgs) -> Self {
        Self(args)
    }
}
impl SetLockerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_LOCKER_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_LOCKER_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetLockerIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_LOCKER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_locker_ix_with_program_id(
    program_id: Pubkey,
    keys: SetLockerKeys,
    args: SetLockerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_LOCKER_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetLockerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_locker_ix(
    keys: SetLockerKeys,
    args: SetLockerIxArgs,
) -> std::io::Result<Instruction> {
    set_locker_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_locker_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetLockerAccounts<'_, '_>,
    args: SetLockerIxArgs,
) -> ProgramResult {
    let keys: SetLockerKeys = accounts.into();
    let ix = set_locker_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_locker_invoke(
    accounts: SetLockerAccounts<'_, '_>,
    args: SetLockerIxArgs,
) -> ProgramResult {
    set_locker_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_locker_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetLockerAccounts<'_, '_>,
    args: SetLockerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetLockerKeys = accounts.into();
    let ix = set_locker_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_locker_invoke_signed(
    accounts: SetLockerAccounts<'_, '_>,
    args: SetLockerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_locker_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_locker_verify_account_keys(
    accounts: SetLockerAccounts<'_, '_>,
    keys: SetLockerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.governor.key, keys.governor),
        (*accounts.smart_wallet.key, keys.smart_wallet),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_locker_verify_writable_privileges<'me, 'info>(
    accounts: SetLockerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.governor] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_locker_verify_signer_privileges<'me, 'info>(
    accounts: SetLockerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.smart_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_locker_verify_account_privileges<'me, 'info>(
    accounts: SetLockerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_locker_verify_writable_privileges(accounts)?;
    set_locker_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_PROPOSAL_META_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreateProposalMetaAccounts<'me, 'info> {
    pub proposal: &'me AccountInfo<'info>,
    pub proposer: &'me AccountInfo<'info>,
    pub proposal_meta: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateProposalMetaKeys {
    pub proposal: Pubkey,
    pub proposer: Pubkey,
    pub proposal_meta: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateProposalMetaAccounts<'_, '_>> for CreateProposalMetaKeys {
    fn from(accounts: CreateProposalMetaAccounts) -> Self {
        Self {
            proposal: *accounts.proposal.key,
            proposer: *accounts.proposer.key,
            proposal_meta: *accounts.proposal_meta.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateProposalMetaKeys>
for [AccountMeta; CREATE_PROPOSAL_META_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateProposalMetaKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.proposer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.proposal_meta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_PROPOSAL_META_IX_ACCOUNTS_LEN]> for CreateProposalMetaKeys {
    fn from(pubkeys: [Pubkey; CREATE_PROPOSAL_META_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: pubkeys[0],
            proposer: pubkeys[1],
            proposal_meta: pubkeys[2],
            payer: pubkeys[3],
            system_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<CreateProposalMetaAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_PROPOSAL_META_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateProposalMetaAccounts<'_, 'info>) -> Self {
        [
            accounts.proposal.clone(),
            accounts.proposer.clone(),
            accounts.proposal_meta.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_PROPOSAL_META_IX_ACCOUNTS_LEN]>
for CreateProposalMetaAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_PROPOSAL_META_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            proposal: &arr[0],
            proposer: &arr[1],
            proposal_meta: &arr[2],
            payer: &arr[3],
            system_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const CREATE_PROPOSAL_META_IX_DISCM: [u8; 8] = [238, 138, 212, 160, 46, 53, 51, 88];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateProposalMetaIxArgs {
    pub bump: u8,
    pub title: String,
    pub description_link: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateProposalMetaIxData(pub CreateProposalMetaIxArgs);
impl From<CreateProposalMetaIxArgs> for CreateProposalMetaIxData {
    fn from(args: CreateProposalMetaIxArgs) -> Self {
        Self(args)
    }
}
impl CreateProposalMetaIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PROPOSAL_META_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_PROPOSAL_META_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateProposalMetaIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PROPOSAL_META_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_proposal_meta_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateProposalMetaKeys,
    args: CreateProposalMetaIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_PROPOSAL_META_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateProposalMetaIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_proposal_meta_ix(
    keys: CreateProposalMetaKeys,
    args: CreateProposalMetaIxArgs,
) -> std::io::Result<Instruction> {
    create_proposal_meta_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_proposal_meta_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateProposalMetaAccounts<'_, '_>,
    args: CreateProposalMetaIxArgs,
) -> ProgramResult {
    let keys: CreateProposalMetaKeys = accounts.into();
    let ix = create_proposal_meta_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_proposal_meta_invoke(
    accounts: CreateProposalMetaAccounts<'_, '_>,
    args: CreateProposalMetaIxArgs,
) -> ProgramResult {
    create_proposal_meta_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_proposal_meta_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateProposalMetaAccounts<'_, '_>,
    args: CreateProposalMetaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateProposalMetaKeys = accounts.into();
    let ix = create_proposal_meta_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_proposal_meta_invoke_signed(
    accounts: CreateProposalMetaAccounts<'_, '_>,
    args: CreateProposalMetaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_proposal_meta_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_proposal_meta_verify_account_keys(
    accounts: CreateProposalMetaAccounts<'_, '_>,
    keys: CreateProposalMetaKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.proposal.key, keys.proposal),
        (*accounts.proposer.key, keys.proposer),
        (*accounts.proposal_meta.key, keys.proposal_meta),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_proposal_meta_verify_writable_privileges<'me, 'info>(
    accounts: CreateProposalMetaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.proposal_meta, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_proposal_meta_verify_signer_privileges<'me, 'info>(
    accounts: CreateProposalMetaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.proposer, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_proposal_meta_verify_account_privileges<'me, 'info>(
    accounts: CreateProposalMetaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_proposal_meta_verify_writable_privileges(accounts)?;
    create_proposal_meta_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_OPTION_PROPOSAL_META_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreateOptionProposalMetaAccounts<'me, 'info> {
    pub proposal: &'me AccountInfo<'info>,
    pub proposer: &'me AccountInfo<'info>,
    pub option_proposal_meta: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateOptionProposalMetaKeys {
    pub proposal: Pubkey,
    pub proposer: Pubkey,
    pub option_proposal_meta: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CreateOptionProposalMetaAccounts<'_, '_>> for CreateOptionProposalMetaKeys {
    fn from(accounts: CreateOptionProposalMetaAccounts) -> Self {
        Self {
            proposal: *accounts.proposal.key,
            proposer: *accounts.proposer.key,
            option_proposal_meta: *accounts.option_proposal_meta.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CreateOptionProposalMetaKeys>
for [AccountMeta; CREATE_OPTION_PROPOSAL_META_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateOptionProposalMetaKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.proposal,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.proposer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.option_proposal_meta,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_OPTION_PROPOSAL_META_IX_ACCOUNTS_LEN]>
for CreateOptionProposalMetaKeys {
    fn from(pubkeys: [Pubkey; CREATE_OPTION_PROPOSAL_META_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            proposal: pubkeys[0],
            proposer: pubkeys[1],
            option_proposal_meta: pubkeys[2],
            payer: pubkeys[3],
            system_program: pubkeys[4],
            event_authority: pubkeys[5],
            program: pubkeys[6],
        }
    }
}
impl<'info> From<CreateOptionProposalMetaAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_OPTION_PROPOSAL_META_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateOptionProposalMetaAccounts<'_, 'info>) -> Self {
        [
            accounts.proposal.clone(),
            accounts.proposer.clone(),
            accounts.option_proposal_meta.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_OPTION_PROPOSAL_META_IX_ACCOUNTS_LEN]>
for CreateOptionProposalMetaAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_OPTION_PROPOSAL_META_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            proposal: &arr[0],
            proposer: &arr[1],
            option_proposal_meta: &arr[2],
            payer: &arr[3],
            system_program: &arr[4],
            event_authority: &arr[5],
            program: &arr[6],
        }
    }
}
pub const CREATE_OPTION_PROPOSAL_META_IX_DISCM: [u8; 8] = [
    152,
    144,
    104,
    228,
    245,
    234,
    164,
    224,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateOptionProposalMetaIxArgs {
    pub bump: u8,
    pub option_descriptions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateOptionProposalMetaIxData(pub CreateOptionProposalMetaIxArgs);
impl From<CreateOptionProposalMetaIxArgs> for CreateOptionProposalMetaIxData {
    fn from(args: CreateOptionProposalMetaIxArgs) -> Self {
        Self(args)
    }
}
impl CreateOptionProposalMetaIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_OPTION_PROPOSAL_META_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_OPTION_PROPOSAL_META_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateOptionProposalMetaIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_OPTION_PROPOSAL_META_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_option_proposal_meta_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateOptionProposalMetaKeys,
    args: CreateOptionProposalMetaIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_OPTION_PROPOSAL_META_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateOptionProposalMetaIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_option_proposal_meta_ix(
    keys: CreateOptionProposalMetaKeys,
    args: CreateOptionProposalMetaIxArgs,
) -> std::io::Result<Instruction> {
    create_option_proposal_meta_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_option_proposal_meta_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateOptionProposalMetaAccounts<'_, '_>,
    args: CreateOptionProposalMetaIxArgs,
) -> ProgramResult {
    let keys: CreateOptionProposalMetaKeys = accounts.into();
    let ix = create_option_proposal_meta_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_option_proposal_meta_invoke(
    accounts: CreateOptionProposalMetaAccounts<'_, '_>,
    args: CreateOptionProposalMetaIxArgs,
) -> ProgramResult {
    create_option_proposal_meta_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_option_proposal_meta_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateOptionProposalMetaAccounts<'_, '_>,
    args: CreateOptionProposalMetaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateOptionProposalMetaKeys = accounts.into();
    let ix = create_option_proposal_meta_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_option_proposal_meta_invoke_signed(
    accounts: CreateOptionProposalMetaAccounts<'_, '_>,
    args: CreateOptionProposalMetaIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_option_proposal_meta_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_option_proposal_meta_verify_account_keys(
    accounts: CreateOptionProposalMetaAccounts<'_, '_>,
    keys: CreateOptionProposalMetaKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.proposal.key, keys.proposal),
        (*accounts.proposer.key, keys.proposer),
        (*accounts.option_proposal_meta.key, keys.option_proposal_meta),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_option_proposal_meta_verify_writable_privileges<'me, 'info>(
    accounts: CreateOptionProposalMetaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.option_proposal_meta, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_option_proposal_meta_verify_signer_privileges<'me, 'info>(
    accounts: CreateOptionProposalMetaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.proposer, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_option_proposal_meta_verify_account_privileges<'me, 'info>(
    accounts: CreateOptionProposalMetaAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_option_proposal_meta_verify_writable_privileges(accounts)?;
    create_option_proposal_meta_verify_signer_privileges(accounts)?;
    Ok(())
}
