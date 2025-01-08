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
pub enum LockedVoterProgramIx {
    NewLocker(NewLockerIxArgs),
    NewEscrow,
    IncreaseLockedAmount(IncreaseLockedAmountIxArgs),
    ExtendLockDuration(ExtendLockDurationIxArgs),
    Withdraw,
    ActivateProposal,
    CastVote(CastVoteIxArgs),
    SetVoteDelegate(SetVoteDelegateIxArgs),
    SetLockerParams(SetLockerParamsIxArgs),
    OpenPartialUnstaking(OpenPartialUnstakingIxArgs),
    MergePartialUnstaking,
    WithdrawPartialUnstaking,
}
impl LockedVoterProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            NEW_LOCKER_IX_DISCM => {
                Ok(Self::NewLocker(NewLockerIxArgs::deserialize(&mut reader)?))
            }
            NEW_ESCROW_IX_DISCM => Ok(Self::NewEscrow),
            INCREASE_LOCKED_AMOUNT_IX_DISCM => {
                Ok(
                    Self::IncreaseLockedAmount(
                        IncreaseLockedAmountIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            EXTEND_LOCK_DURATION_IX_DISCM => {
                Ok(
                    Self::ExtendLockDuration(
                        ExtendLockDurationIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            WITHDRAW_IX_DISCM => Ok(Self::Withdraw),
            ACTIVATE_PROPOSAL_IX_DISCM => Ok(Self::ActivateProposal),
            CAST_VOTE_IX_DISCM => {
                Ok(Self::CastVote(CastVoteIxArgs::deserialize(&mut reader)?))
            }
            SET_VOTE_DELEGATE_IX_DISCM => {
                Ok(
                    Self::SetVoteDelegate(
                        SetVoteDelegateIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            SET_LOCKER_PARAMS_IX_DISCM => {
                Ok(
                    Self::SetLockerParams(
                        SetLockerParamsIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            OPEN_PARTIAL_UNSTAKING_IX_DISCM => {
                Ok(
                    Self::OpenPartialUnstaking(
                        OpenPartialUnstakingIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            MERGE_PARTIAL_UNSTAKING_IX_DISCM => Ok(Self::MergePartialUnstaking),
            WITHDRAW_PARTIAL_UNSTAKING_IX_DISCM => Ok(Self::WithdrawPartialUnstaking),
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
            Self::NewLocker(args) => {
                writer.write_all(&NEW_LOCKER_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::NewEscrow => writer.write_all(&NEW_ESCROW_IX_DISCM),
            Self::IncreaseLockedAmount(args) => {
                writer.write_all(&INCREASE_LOCKED_AMOUNT_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ExtendLockDuration(args) => {
                writer.write_all(&EXTEND_LOCK_DURATION_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Withdraw => writer.write_all(&WITHDRAW_IX_DISCM),
            Self::ActivateProposal => writer.write_all(&ACTIVATE_PROPOSAL_IX_DISCM),
            Self::CastVote(args) => {
                writer.write_all(&CAST_VOTE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetVoteDelegate(args) => {
                writer.write_all(&SET_VOTE_DELEGATE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetLockerParams(args) => {
                writer.write_all(&SET_LOCKER_PARAMS_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::OpenPartialUnstaking(args) => {
                writer.write_all(&OPEN_PARTIAL_UNSTAKING_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::MergePartialUnstaking => {
                writer.write_all(&MERGE_PARTIAL_UNSTAKING_IX_DISCM)
            }
            Self::WithdrawPartialUnstaking => {
                writer.write_all(&WITHDRAW_PARTIAL_UNSTAKING_IX_DISCM)
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
pub const NEW_LOCKER_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct NewLockerAccounts<'me, 'info> {
    pub base: &'me AccountInfo<'info>,
    pub locker: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub governor: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NewLockerKeys {
    pub base: Pubkey,
    pub locker: Pubkey,
    pub token_mint: Pubkey,
    pub governor: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<NewLockerAccounts<'_, '_>> for NewLockerKeys {
    fn from(accounts: NewLockerAccounts) -> Self {
        Self {
            base: *accounts.base.key,
            locker: *accounts.locker.key,
            token_mint: *accounts.token_mint.key,
            governor: *accounts.governor.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<NewLockerKeys> for [AccountMeta; NEW_LOCKER_IX_ACCOUNTS_LEN] {
    fn from(keys: NewLockerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.locker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.governor,
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
impl From<[Pubkey; NEW_LOCKER_IX_ACCOUNTS_LEN]> for NewLockerKeys {
    fn from(pubkeys: [Pubkey; NEW_LOCKER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            base: pubkeys[0],
            locker: pubkeys[1],
            token_mint: pubkeys[2],
            governor: pubkeys[3],
            payer: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<NewLockerAccounts<'_, 'info>>
for [AccountInfo<'info>; NEW_LOCKER_IX_ACCOUNTS_LEN] {
    fn from(accounts: NewLockerAccounts<'_, 'info>) -> Self {
        [
            accounts.base.clone(),
            accounts.locker.clone(),
            accounts.token_mint.clone(),
            accounts.governor.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NEW_LOCKER_IX_ACCOUNTS_LEN]>
for NewLockerAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; NEW_LOCKER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            base: &arr[0],
            locker: &arr[1],
            token_mint: &arr[2],
            governor: &arr[3],
            payer: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const NEW_LOCKER_IX_DISCM: [u8; 8] = [177, 133, 32, 90, 229, 216, 131, 47];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewLockerIxArgs {
    pub params: LockerParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NewLockerIxData(pub NewLockerIxArgs);
impl From<NewLockerIxArgs> for NewLockerIxData {
    fn from(args: NewLockerIxArgs) -> Self {
        Self(args)
    }
}
impl NewLockerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != NEW_LOCKER_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        NEW_LOCKER_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(NewLockerIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&NEW_LOCKER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn new_locker_ix_with_program_id(
    program_id: Pubkey,
    keys: NewLockerKeys,
    args: NewLockerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NEW_LOCKER_IX_ACCOUNTS_LEN] = keys.into();
    let data: NewLockerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn new_locker_ix(
    keys: NewLockerKeys,
    args: NewLockerIxArgs,
) -> std::io::Result<Instruction> {
    new_locker_ix_with_program_id(crate::ID, keys, args)
}
pub fn new_locker_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NewLockerAccounts<'_, '_>,
    args: NewLockerIxArgs,
) -> ProgramResult {
    let keys: NewLockerKeys = accounts.into();
    let ix = new_locker_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn new_locker_invoke(
    accounts: NewLockerAccounts<'_, '_>,
    args: NewLockerIxArgs,
) -> ProgramResult {
    new_locker_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn new_locker_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NewLockerAccounts<'_, '_>,
    args: NewLockerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NewLockerKeys = accounts.into();
    let ix = new_locker_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn new_locker_invoke_signed(
    accounts: NewLockerAccounts<'_, '_>,
    args: NewLockerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    new_locker_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn new_locker_verify_account_keys(
    accounts: NewLockerAccounts<'_, '_>,
    keys: NewLockerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.base.key, keys.base),
        (*accounts.locker.key, keys.locker),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.governor.key, keys.governor),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn new_locker_verify_writable_privileges<'me, 'info>(
    accounts: NewLockerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.locker, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn new_locker_verify_signer_privileges<'me, 'info>(
    accounts: NewLockerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.base, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn new_locker_verify_account_privileges<'me, 'info>(
    accounts: NewLockerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    new_locker_verify_writable_privileges(accounts)?;
    new_locker_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const NEW_ESCROW_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct NewEscrowAccounts<'me, 'info> {
    pub locker: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub escrow_owner: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NewEscrowKeys {
    pub locker: Pubkey,
    pub escrow: Pubkey,
    pub escrow_owner: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<NewEscrowAccounts<'_, '_>> for NewEscrowKeys {
    fn from(accounts: NewEscrowAccounts) -> Self {
        Self {
            locker: *accounts.locker.key,
            escrow: *accounts.escrow.key,
            escrow_owner: *accounts.escrow_owner.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<NewEscrowKeys> for [AccountMeta; NEW_ESCROW_IX_ACCOUNTS_LEN] {
    fn from(keys: NewEscrowKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.locker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_owner,
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
impl From<[Pubkey; NEW_ESCROW_IX_ACCOUNTS_LEN]> for NewEscrowKeys {
    fn from(pubkeys: [Pubkey; NEW_ESCROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            locker: pubkeys[0],
            escrow: pubkeys[1],
            escrow_owner: pubkeys[2],
            payer: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<NewEscrowAccounts<'_, 'info>>
for [AccountInfo<'info>; NEW_ESCROW_IX_ACCOUNTS_LEN] {
    fn from(accounts: NewEscrowAccounts<'_, 'info>) -> Self {
        [
            accounts.locker.clone(),
            accounts.escrow.clone(),
            accounts.escrow_owner.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NEW_ESCROW_IX_ACCOUNTS_LEN]>
for NewEscrowAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; NEW_ESCROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            locker: &arr[0],
            escrow: &arr[1],
            escrow_owner: &arr[2],
            payer: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const NEW_ESCROW_IX_DISCM: [u8; 8] = [216, 182, 143, 11, 220, 38, 86, 185];
#[derive(Clone, Debug, PartialEq)]
pub struct NewEscrowIxData;
impl NewEscrowIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != NEW_ESCROW_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        NEW_ESCROW_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&NEW_ESCROW_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn new_escrow_ix_with_program_id(
    program_id: Pubkey,
    keys: NewEscrowKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NEW_ESCROW_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: NewEscrowIxData.try_to_vec()?,
    })
}
pub fn new_escrow_ix(keys: NewEscrowKeys) -> std::io::Result<Instruction> {
    new_escrow_ix_with_program_id(crate::ID, keys)
}
pub fn new_escrow_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NewEscrowAccounts<'_, '_>,
) -> ProgramResult {
    let keys: NewEscrowKeys = accounts.into();
    let ix = new_escrow_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn new_escrow_invoke(accounts: NewEscrowAccounts<'_, '_>) -> ProgramResult {
    new_escrow_invoke_with_program_id(crate::ID, accounts)
}
pub fn new_escrow_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NewEscrowAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NewEscrowKeys = accounts.into();
    let ix = new_escrow_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn new_escrow_invoke_signed(
    accounts: NewEscrowAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    new_escrow_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn new_escrow_verify_account_keys(
    accounts: NewEscrowAccounts<'_, '_>,
    keys: NewEscrowKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.locker.key, keys.locker),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.escrow_owner.key, keys.escrow_owner),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn new_escrow_verify_writable_privileges<'me, 'info>(
    accounts: NewEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.locker, accounts.escrow, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn new_escrow_verify_signer_privileges<'me, 'info>(
    accounts: NewEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn new_escrow_verify_account_privileges<'me, 'info>(
    accounts: NewEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    new_escrow_verify_writable_privileges(accounts)?;
    new_escrow_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_LOCKED_AMOUNT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct IncreaseLockedAmountAccounts<'me, 'info> {
    pub locker: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub escrow_tokens: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub source_tokens: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IncreaseLockedAmountKeys {
    pub locker: Pubkey,
    pub escrow: Pubkey,
    pub escrow_tokens: Pubkey,
    pub payer: Pubkey,
    pub source_tokens: Pubkey,
    pub token_program: Pubkey,
}
impl From<IncreaseLockedAmountAccounts<'_, '_>> for IncreaseLockedAmountKeys {
    fn from(accounts: IncreaseLockedAmountAccounts) -> Self {
        Self {
            locker: *accounts.locker.key,
            escrow: *accounts.escrow.key,
            escrow_tokens: *accounts.escrow_tokens.key,
            payer: *accounts.payer.key,
            source_tokens: *accounts.source_tokens.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<IncreaseLockedAmountKeys>
for [AccountMeta; INCREASE_LOCKED_AMOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: IncreaseLockedAmountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.locker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_tokens,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_tokens,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INCREASE_LOCKED_AMOUNT_IX_ACCOUNTS_LEN]>
for IncreaseLockedAmountKeys {
    fn from(pubkeys: [Pubkey; INCREASE_LOCKED_AMOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            locker: pubkeys[0],
            escrow: pubkeys[1],
            escrow_tokens: pubkeys[2],
            payer: pubkeys[3],
            source_tokens: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<IncreaseLockedAmountAccounts<'_, 'info>>
for [AccountInfo<'info>; INCREASE_LOCKED_AMOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: IncreaseLockedAmountAccounts<'_, 'info>) -> Self {
        [
            accounts.locker.clone(),
            accounts.escrow.clone(),
            accounts.escrow_tokens.clone(),
            accounts.payer.clone(),
            accounts.source_tokens.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INCREASE_LOCKED_AMOUNT_IX_ACCOUNTS_LEN]>
for IncreaseLockedAmountAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INCREASE_LOCKED_AMOUNT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            locker: &arr[0],
            escrow: &arr[1],
            escrow_tokens: &arr[2],
            payer: &arr[3],
            source_tokens: &arr[4],
            token_program: &arr[5],
        }
    }
}
pub const INCREASE_LOCKED_AMOUNT_IX_DISCM: [u8; 8] = [5, 168, 118, 53, 72, 46, 203, 146];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IncreaseLockedAmountIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseLockedAmountIxData(pub IncreaseLockedAmountIxArgs);
impl From<IncreaseLockedAmountIxArgs> for IncreaseLockedAmountIxData {
    fn from(args: IncreaseLockedAmountIxArgs) -> Self {
        Self(args)
    }
}
impl IncreaseLockedAmountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INCREASE_LOCKED_AMOUNT_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INCREASE_LOCKED_AMOUNT_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(IncreaseLockedAmountIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INCREASE_LOCKED_AMOUNT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_locked_amount_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreaseLockedAmountKeys,
    args: IncreaseLockedAmountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_LOCKED_AMOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncreaseLockedAmountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_locked_amount_ix(
    keys: IncreaseLockedAmountKeys,
    args: IncreaseLockedAmountIxArgs,
) -> std::io::Result<Instruction> {
    increase_locked_amount_ix_with_program_id(crate::ID, keys, args)
}
pub fn increase_locked_amount_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLockedAmountAccounts<'_, '_>,
    args: IncreaseLockedAmountIxArgs,
) -> ProgramResult {
    let keys: IncreaseLockedAmountKeys = accounts.into();
    let ix = increase_locked_amount_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_locked_amount_invoke(
    accounts: IncreaseLockedAmountAccounts<'_, '_>,
    args: IncreaseLockedAmountIxArgs,
) -> ProgramResult {
    increase_locked_amount_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn increase_locked_amount_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseLockedAmountAccounts<'_, '_>,
    args: IncreaseLockedAmountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreaseLockedAmountKeys = accounts.into();
    let ix = increase_locked_amount_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_locked_amount_invoke_signed(
    accounts: IncreaseLockedAmountAccounts<'_, '_>,
    args: IncreaseLockedAmountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_locked_amount_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn increase_locked_amount_verify_account_keys(
    accounts: IncreaseLockedAmountAccounts<'_, '_>,
    keys: IncreaseLockedAmountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.locker.key, keys.locker),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.escrow_tokens.key, keys.escrow_tokens),
        (*accounts.payer.key, keys.payer),
        (*accounts.source_tokens.key, keys.source_tokens),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn increase_locked_amount_verify_writable_privileges<'me, 'info>(
    accounts: IncreaseLockedAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.locker,
        accounts.escrow,
        accounts.escrow_tokens,
        accounts.source_tokens,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_locked_amount_verify_signer_privileges<'me, 'info>(
    accounts: IncreaseLockedAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_locked_amount_verify_account_privileges<'me, 'info>(
    accounts: IncreaseLockedAmountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_locked_amount_verify_writable_privileges(accounts)?;
    increase_locked_amount_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXTEND_LOCK_DURATION_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ExtendLockDurationAccounts<'me, 'info> {
    pub locker: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub escrow_owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExtendLockDurationKeys {
    pub locker: Pubkey,
    pub escrow: Pubkey,
    pub escrow_owner: Pubkey,
}
impl From<ExtendLockDurationAccounts<'_, '_>> for ExtendLockDurationKeys {
    fn from(accounts: ExtendLockDurationAccounts) -> Self {
        Self {
            locker: *accounts.locker.key,
            escrow: *accounts.escrow.key,
            escrow_owner: *accounts.escrow_owner.key,
        }
    }
}
impl From<ExtendLockDurationKeys>
for [AccountMeta; EXTEND_LOCK_DURATION_IX_ACCOUNTS_LEN] {
    fn from(keys: ExtendLockDurationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.locker,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; EXTEND_LOCK_DURATION_IX_ACCOUNTS_LEN]> for ExtendLockDurationKeys {
    fn from(pubkeys: [Pubkey; EXTEND_LOCK_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            locker: pubkeys[0],
            escrow: pubkeys[1],
            escrow_owner: pubkeys[2],
        }
    }
}
impl<'info> From<ExtendLockDurationAccounts<'_, 'info>>
for [AccountInfo<'info>; EXTEND_LOCK_DURATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExtendLockDurationAccounts<'_, 'info>) -> Self {
        [accounts.locker.clone(), accounts.escrow.clone(), accounts.escrow_owner.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EXTEND_LOCK_DURATION_IX_ACCOUNTS_LEN]>
for ExtendLockDurationAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; EXTEND_LOCK_DURATION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            locker: &arr[0],
            escrow: &arr[1],
            escrow_owner: &arr[2],
        }
    }
}
pub const EXTEND_LOCK_DURATION_IX_DISCM: [u8; 8] = [
    177,
    105,
    196,
    129,
    153,
    137,
    136,
    230,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendLockDurationIxArgs {
    pub duration: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExtendLockDurationIxData(pub ExtendLockDurationIxArgs);
impl From<ExtendLockDurationIxArgs> for ExtendLockDurationIxData {
    fn from(args: ExtendLockDurationIxArgs) -> Self {
        Self(args)
    }
}
impl ExtendLockDurationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXTEND_LOCK_DURATION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EXTEND_LOCK_DURATION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ExtendLockDurationIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXTEND_LOCK_DURATION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn extend_lock_duration_ix_with_program_id(
    program_id: Pubkey,
    keys: ExtendLockDurationKeys,
    args: ExtendLockDurationIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXTEND_LOCK_DURATION_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExtendLockDurationIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn extend_lock_duration_ix(
    keys: ExtendLockDurationKeys,
    args: ExtendLockDurationIxArgs,
) -> std::io::Result<Instruction> {
    extend_lock_duration_ix_with_program_id(crate::ID, keys, args)
}
pub fn extend_lock_duration_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExtendLockDurationAccounts<'_, '_>,
    args: ExtendLockDurationIxArgs,
) -> ProgramResult {
    let keys: ExtendLockDurationKeys = accounts.into();
    let ix = extend_lock_duration_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn extend_lock_duration_invoke(
    accounts: ExtendLockDurationAccounts<'_, '_>,
    args: ExtendLockDurationIxArgs,
) -> ProgramResult {
    extend_lock_duration_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn extend_lock_duration_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExtendLockDurationAccounts<'_, '_>,
    args: ExtendLockDurationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExtendLockDurationKeys = accounts.into();
    let ix = extend_lock_duration_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn extend_lock_duration_invoke_signed(
    accounts: ExtendLockDurationAccounts<'_, '_>,
    args: ExtendLockDurationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    extend_lock_duration_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn extend_lock_duration_verify_account_keys(
    accounts: ExtendLockDurationAccounts<'_, '_>,
    keys: ExtendLockDurationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.locker.key, keys.locker),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.escrow_owner.key, keys.escrow_owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn extend_lock_duration_verify_writable_privileges<'me, 'info>(
    accounts: ExtendLockDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.escrow] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn extend_lock_duration_verify_signer_privileges<'me, 'info>(
    accounts: ExtendLockDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.escrow_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn extend_lock_duration_verify_account_privileges<'me, 'info>(
    accounts: ExtendLockDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    extend_lock_duration_verify_writable_privileges(accounts)?;
    extend_lock_duration_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub locker: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub escrow_owner: &'me AccountInfo<'info>,
    pub escrow_tokens: &'me AccountInfo<'info>,
    pub destination_tokens: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub locker: Pubkey,
    pub escrow: Pubkey,
    pub escrow_owner: Pubkey,
    pub escrow_tokens: Pubkey,
    pub destination_tokens: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            locker: *accounts.locker.key,
            escrow: *accounts.escrow.key,
            escrow_owner: *accounts.escrow_owner.key,
            escrow_tokens: *accounts.escrow_tokens.key,
            destination_tokens: *accounts.destination_tokens.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.locker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.escrow_tokens,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_tokens,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            locker: pubkeys[0],
            escrow: pubkeys[1],
            escrow_owner: pubkeys[2],
            escrow_tokens: pubkeys[3],
            destination_tokens: pubkeys[4],
            payer: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.locker.clone(),
            accounts.escrow.clone(),
            accounts.escrow_owner.clone(),
            accounts.escrow_tokens.clone(),
            accounts.destination_tokens.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            locker: &arr[0],
            escrow: &arr[1],
            escrow_owner: &arr[2],
            escrow_tokens: &arr[3],
            destination_tokens: &arr[4],
            payer: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const WITHDRAW_IX_DISCM: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawIxData;
impl WithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawIxData.try_to_vec()?,
    })
}
pub fn withdraw_ix(keys: WithdrawKeys) -> std::io::Result<Instruction> {
    withdraw_ix_with_program_id(crate::ID, keys)
}
pub fn withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawKeys = accounts.into();
    let ix = withdraw_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_invoke(accounts: WithdrawAccounts<'_, '_>) -> ProgramResult {
    withdraw_invoke_with_program_id(crate::ID, accounts)
}
pub fn withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawKeys = accounts.into();
    let ix = withdraw_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_invoke_signed(
    accounts: WithdrawAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.locker.key, keys.locker),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.escrow_owner.key, keys.escrow_owner),
        (*accounts.escrow_tokens.key, keys.escrow_tokens),
        (*accounts.destination_tokens.key, keys.destination_tokens),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.locker,
        accounts.escrow,
        accounts.escrow_tokens,
        accounts.destination_tokens,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.escrow_owner, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_verify_account_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_verify_writable_privileges(accounts)?;
    withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ACTIVATE_PROPOSAL_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ActivateProposalAccounts<'me, 'info> {
    pub locker: &'me AccountInfo<'info>,
    pub governor: &'me AccountInfo<'info>,
    pub proposal: &'me AccountInfo<'info>,
    pub govern_program: &'me AccountInfo<'info>,
    pub smart_wallet: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ActivateProposalKeys {
    pub locker: Pubkey,
    pub governor: Pubkey,
    pub proposal: Pubkey,
    pub govern_program: Pubkey,
    pub smart_wallet: Pubkey,
}
impl From<ActivateProposalAccounts<'_, '_>> for ActivateProposalKeys {
    fn from(accounts: ActivateProposalAccounts) -> Self {
        Self {
            locker: *accounts.locker.key,
            governor: *accounts.governor.key,
            proposal: *accounts.proposal.key,
            govern_program: *accounts.govern_program.key,
            smart_wallet: *accounts.smart_wallet.key,
        }
    }
}
impl From<ActivateProposalKeys> for [AccountMeta; ACTIVATE_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(keys: ActivateProposalKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.locker,
                is_signer: false,
                is_writable: false,
            },
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
                pubkey: keys.govern_program,
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
impl From<[Pubkey; ACTIVATE_PROPOSAL_IX_ACCOUNTS_LEN]> for ActivateProposalKeys {
    fn from(pubkeys: [Pubkey; ACTIVATE_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            locker: pubkeys[0],
            governor: pubkeys[1],
            proposal: pubkeys[2],
            govern_program: pubkeys[3],
            smart_wallet: pubkeys[4],
        }
    }
}
impl<'info> From<ActivateProposalAccounts<'_, 'info>>
for [AccountInfo<'info>; ACTIVATE_PROPOSAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: ActivateProposalAccounts<'_, 'info>) -> Self {
        [
            accounts.locker.clone(),
            accounts.governor.clone(),
            accounts.proposal.clone(),
            accounts.govern_program.clone(),
            accounts.smart_wallet.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ACTIVATE_PROPOSAL_IX_ACCOUNTS_LEN]>
for ActivateProposalAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; ACTIVATE_PROPOSAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            locker: &arr[0],
            governor: &arr[1],
            proposal: &arr[2],
            govern_program: &arr[3],
            smart_wallet: &arr[4],
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
        (*accounts.locker.key, keys.locker),
        (*accounts.governor.key, keys.governor),
        (*accounts.proposal.key, keys.proposal),
        (*accounts.govern_program.key, keys.govern_program),
        (*accounts.smart_wallet.key, keys.smart_wallet),
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
    for should_be_signer in [accounts.smart_wallet] {
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
pub const CAST_VOTE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CastVoteAccounts<'me, 'info> {
    pub locker: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub vote_delegate: &'me AccountInfo<'info>,
    pub proposal: &'me AccountInfo<'info>,
    pub vote: &'me AccountInfo<'info>,
    pub governor: &'me AccountInfo<'info>,
    pub govern_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CastVoteKeys {
    pub locker: Pubkey,
    pub escrow: Pubkey,
    pub vote_delegate: Pubkey,
    pub proposal: Pubkey,
    pub vote: Pubkey,
    pub governor: Pubkey,
    pub govern_program: Pubkey,
}
impl From<CastVoteAccounts<'_, '_>> for CastVoteKeys {
    fn from(accounts: CastVoteAccounts) -> Self {
        Self {
            locker: *accounts.locker.key,
            escrow: *accounts.escrow.key,
            vote_delegate: *accounts.vote_delegate.key,
            proposal: *accounts.proposal.key,
            vote: *accounts.vote.key,
            governor: *accounts.governor.key,
            govern_program: *accounts.govern_program.key,
        }
    }
}
impl From<CastVoteKeys> for [AccountMeta; CAST_VOTE_IX_ACCOUNTS_LEN] {
    fn from(keys: CastVoteKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.locker,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vote_delegate,
                is_signer: true,
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
                pubkey: keys.governor,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.govern_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CAST_VOTE_IX_ACCOUNTS_LEN]> for CastVoteKeys {
    fn from(pubkeys: [Pubkey; CAST_VOTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            locker: pubkeys[0],
            escrow: pubkeys[1],
            vote_delegate: pubkeys[2],
            proposal: pubkeys[3],
            vote: pubkeys[4],
            governor: pubkeys[5],
            govern_program: pubkeys[6],
        }
    }
}
impl<'info> From<CastVoteAccounts<'_, 'info>>
for [AccountInfo<'info>; CAST_VOTE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CastVoteAccounts<'_, 'info>) -> Self {
        [
            accounts.locker.clone(),
            accounts.escrow.clone(),
            accounts.vote_delegate.clone(),
            accounts.proposal.clone(),
            accounts.vote.clone(),
            accounts.governor.clone(),
            accounts.govern_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CAST_VOTE_IX_ACCOUNTS_LEN]>
for CastVoteAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CAST_VOTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            locker: &arr[0],
            escrow: &arr[1],
            vote_delegate: &arr[2],
            proposal: &arr[3],
            vote: &arr[4],
            governor: &arr[5],
            govern_program: &arr[6],
        }
    }
}
pub const CAST_VOTE_IX_DISCM: [u8; 8] = [20, 212, 15, 189, 69, 180, 69, 151];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CastVoteIxArgs {
    pub side: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CastVoteIxData(pub CastVoteIxArgs);
impl From<CastVoteIxArgs> for CastVoteIxData {
    fn from(args: CastVoteIxArgs) -> Self {
        Self(args)
    }
}
impl CastVoteIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CAST_VOTE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CAST_VOTE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CastVoteIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CAST_VOTE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cast_vote_ix_with_program_id(
    program_id: Pubkey,
    keys: CastVoteKeys,
    args: CastVoteIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CAST_VOTE_IX_ACCOUNTS_LEN] = keys.into();
    let data: CastVoteIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cast_vote_ix(
    keys: CastVoteKeys,
    args: CastVoteIxArgs,
) -> std::io::Result<Instruction> {
    cast_vote_ix_with_program_id(crate::ID, keys, args)
}
pub fn cast_vote_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CastVoteAccounts<'_, '_>,
    args: CastVoteIxArgs,
) -> ProgramResult {
    let keys: CastVoteKeys = accounts.into();
    let ix = cast_vote_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cast_vote_invoke(
    accounts: CastVoteAccounts<'_, '_>,
    args: CastVoteIxArgs,
) -> ProgramResult {
    cast_vote_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn cast_vote_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CastVoteAccounts<'_, '_>,
    args: CastVoteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CastVoteKeys = accounts.into();
    let ix = cast_vote_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cast_vote_invoke_signed(
    accounts: CastVoteAccounts<'_, '_>,
    args: CastVoteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cast_vote_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn cast_vote_verify_account_keys(
    accounts: CastVoteAccounts<'_, '_>,
    keys: CastVoteKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.locker.key, keys.locker),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.vote_delegate.key, keys.vote_delegate),
        (*accounts.proposal.key, keys.proposal),
        (*accounts.vote.key, keys.vote),
        (*accounts.governor.key, keys.governor),
        (*accounts.govern_program.key, keys.govern_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cast_vote_verify_writable_privileges<'me, 'info>(
    accounts: CastVoteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.proposal, accounts.vote] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cast_vote_verify_signer_privileges<'me, 'info>(
    accounts: CastVoteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.vote_delegate] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cast_vote_verify_account_privileges<'me, 'info>(
    accounts: CastVoteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cast_vote_verify_writable_privileges(accounts)?;
    cast_vote_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_VOTE_DELEGATE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetVoteDelegateAccounts<'me, 'info> {
    pub escrow: &'me AccountInfo<'info>,
    pub escrow_owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetVoteDelegateKeys {
    pub escrow: Pubkey,
    pub escrow_owner: Pubkey,
}
impl From<SetVoteDelegateAccounts<'_, '_>> for SetVoteDelegateKeys {
    fn from(accounts: SetVoteDelegateAccounts) -> Self {
        Self {
            escrow: *accounts.escrow.key,
            escrow_owner: *accounts.escrow_owner.key,
        }
    }
}
impl From<SetVoteDelegateKeys> for [AccountMeta; SET_VOTE_DELEGATE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetVoteDelegateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_VOTE_DELEGATE_IX_ACCOUNTS_LEN]> for SetVoteDelegateKeys {
    fn from(pubkeys: [Pubkey; SET_VOTE_DELEGATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            escrow: pubkeys[0],
            escrow_owner: pubkeys[1],
        }
    }
}
impl<'info> From<SetVoteDelegateAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_VOTE_DELEGATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetVoteDelegateAccounts<'_, 'info>) -> Self {
        [accounts.escrow.clone(), accounts.escrow_owner.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_VOTE_DELEGATE_IX_ACCOUNTS_LEN]>
for SetVoteDelegateAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_VOTE_DELEGATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            escrow: &arr[0],
            escrow_owner: &arr[1],
        }
    }
}
pub const SET_VOTE_DELEGATE_IX_DISCM: [u8; 8] = [46, 236, 241, 243, 251, 108, 156, 12];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetVoteDelegateIxArgs {
    pub new_delegate: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetVoteDelegateIxData(pub SetVoteDelegateIxArgs);
impl From<SetVoteDelegateIxArgs> for SetVoteDelegateIxData {
    fn from(args: SetVoteDelegateIxArgs) -> Self {
        Self(args)
    }
}
impl SetVoteDelegateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_VOTE_DELEGATE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_VOTE_DELEGATE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetVoteDelegateIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_VOTE_DELEGATE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_vote_delegate_ix_with_program_id(
    program_id: Pubkey,
    keys: SetVoteDelegateKeys,
    args: SetVoteDelegateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_VOTE_DELEGATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetVoteDelegateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_vote_delegate_ix(
    keys: SetVoteDelegateKeys,
    args: SetVoteDelegateIxArgs,
) -> std::io::Result<Instruction> {
    set_vote_delegate_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_vote_delegate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetVoteDelegateAccounts<'_, '_>,
    args: SetVoteDelegateIxArgs,
) -> ProgramResult {
    let keys: SetVoteDelegateKeys = accounts.into();
    let ix = set_vote_delegate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_vote_delegate_invoke(
    accounts: SetVoteDelegateAccounts<'_, '_>,
    args: SetVoteDelegateIxArgs,
) -> ProgramResult {
    set_vote_delegate_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_vote_delegate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetVoteDelegateAccounts<'_, '_>,
    args: SetVoteDelegateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetVoteDelegateKeys = accounts.into();
    let ix = set_vote_delegate_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_vote_delegate_invoke_signed(
    accounts: SetVoteDelegateAccounts<'_, '_>,
    args: SetVoteDelegateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_vote_delegate_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_vote_delegate_verify_account_keys(
    accounts: SetVoteDelegateAccounts<'_, '_>,
    keys: SetVoteDelegateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.escrow.key, keys.escrow),
        (*accounts.escrow_owner.key, keys.escrow_owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_vote_delegate_verify_writable_privileges<'me, 'info>(
    accounts: SetVoteDelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.escrow] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_vote_delegate_verify_signer_privileges<'me, 'info>(
    accounts: SetVoteDelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.escrow_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_vote_delegate_verify_account_privileges<'me, 'info>(
    accounts: SetVoteDelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_vote_delegate_verify_writable_privileges(accounts)?;
    set_vote_delegate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_LOCKER_PARAMS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetLockerParamsAccounts<'me, 'info> {
    pub locker: &'me AccountInfo<'info>,
    pub governor: &'me AccountInfo<'info>,
    pub smart_wallet: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetLockerParamsKeys {
    pub locker: Pubkey,
    pub governor: Pubkey,
    pub smart_wallet: Pubkey,
}
impl From<SetLockerParamsAccounts<'_, '_>> for SetLockerParamsKeys {
    fn from(accounts: SetLockerParamsAccounts) -> Self {
        Self {
            locker: *accounts.locker.key,
            governor: *accounts.governor.key,
            smart_wallet: *accounts.smart_wallet.key,
        }
    }
}
impl From<SetLockerParamsKeys> for [AccountMeta; SET_LOCKER_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetLockerParamsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.locker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.governor,
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
impl From<[Pubkey; SET_LOCKER_PARAMS_IX_ACCOUNTS_LEN]> for SetLockerParamsKeys {
    fn from(pubkeys: [Pubkey; SET_LOCKER_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            locker: pubkeys[0],
            governor: pubkeys[1],
            smart_wallet: pubkeys[2],
        }
    }
}
impl<'info> From<SetLockerParamsAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_LOCKER_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetLockerParamsAccounts<'_, 'info>) -> Self {
        [
            accounts.locker.clone(),
            accounts.governor.clone(),
            accounts.smart_wallet.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_LOCKER_PARAMS_IX_ACCOUNTS_LEN]>
for SetLockerParamsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_LOCKER_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            locker: &arr[0],
            governor: &arr[1],
            smart_wallet: &arr[2],
        }
    }
}
pub const SET_LOCKER_PARAMS_IX_DISCM: [u8; 8] = [106, 39, 132, 84, 254, 77, 161, 169];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetLockerParamsIxArgs {
    pub params: LockerParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetLockerParamsIxData(pub SetLockerParamsIxArgs);
impl From<SetLockerParamsIxArgs> for SetLockerParamsIxData {
    fn from(args: SetLockerParamsIxArgs) -> Self {
        Self(args)
    }
}
impl SetLockerParamsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_LOCKER_PARAMS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_LOCKER_PARAMS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetLockerParamsIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_LOCKER_PARAMS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_locker_params_ix_with_program_id(
    program_id: Pubkey,
    keys: SetLockerParamsKeys,
    args: SetLockerParamsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_LOCKER_PARAMS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetLockerParamsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_locker_params_ix(
    keys: SetLockerParamsKeys,
    args: SetLockerParamsIxArgs,
) -> std::io::Result<Instruction> {
    set_locker_params_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_locker_params_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetLockerParamsAccounts<'_, '_>,
    args: SetLockerParamsIxArgs,
) -> ProgramResult {
    let keys: SetLockerParamsKeys = accounts.into();
    let ix = set_locker_params_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_locker_params_invoke(
    accounts: SetLockerParamsAccounts<'_, '_>,
    args: SetLockerParamsIxArgs,
) -> ProgramResult {
    set_locker_params_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_locker_params_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetLockerParamsAccounts<'_, '_>,
    args: SetLockerParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetLockerParamsKeys = accounts.into();
    let ix = set_locker_params_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_locker_params_invoke_signed(
    accounts: SetLockerParamsAccounts<'_, '_>,
    args: SetLockerParamsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_locker_params_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_locker_params_verify_account_keys(
    accounts: SetLockerParamsAccounts<'_, '_>,
    keys: SetLockerParamsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.locker.key, keys.locker),
        (*accounts.governor.key, keys.governor),
        (*accounts.smart_wallet.key, keys.smart_wallet),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_locker_params_verify_writable_privileges<'me, 'info>(
    accounts: SetLockerParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.locker] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_locker_params_verify_signer_privileges<'me, 'info>(
    accounts: SetLockerParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.smart_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_locker_params_verify_account_privileges<'me, 'info>(
    accounts: SetLockerParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_locker_params_verify_writable_privileges(accounts)?;
    set_locker_params_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OPEN_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct OpenPartialUnstakingAccounts<'me, 'info> {
    pub locker: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub partial_unstake: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenPartialUnstakingKeys {
    pub locker: Pubkey,
    pub escrow: Pubkey,
    pub partial_unstake: Pubkey,
    pub owner: Pubkey,
    pub system_program: Pubkey,
}
impl From<OpenPartialUnstakingAccounts<'_, '_>> for OpenPartialUnstakingKeys {
    fn from(accounts: OpenPartialUnstakingAccounts) -> Self {
        Self {
            locker: *accounts.locker.key,
            escrow: *accounts.escrow.key,
            partial_unstake: *accounts.partial_unstake.key,
            owner: *accounts.owner.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<OpenPartialUnstakingKeys>
for [AccountMeta; OPEN_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenPartialUnstakingKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.locker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.partial_unstake,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
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
impl From<[Pubkey; OPEN_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN]>
for OpenPartialUnstakingKeys {
    fn from(pubkeys: [Pubkey; OPEN_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            locker: pubkeys[0],
            escrow: pubkeys[1],
            partial_unstake: pubkeys[2],
            owner: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<OpenPartialUnstakingAccounts<'_, 'info>>
for [AccountInfo<'info>; OPEN_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN] {
    fn from(accounts: OpenPartialUnstakingAccounts<'_, 'info>) -> Self {
        [
            accounts.locker.clone(),
            accounts.escrow.clone(),
            accounts.partial_unstake.clone(),
            accounts.owner.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPEN_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN]>
for OpenPartialUnstakingAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OPEN_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            locker: &arr[0],
            escrow: &arr[1],
            partial_unstake: &arr[2],
            owner: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const OPEN_PARTIAL_UNSTAKING_IX_DISCM: [u8; 8] = [
    201,
    137,
    207,
    175,
    79,
    95,
    220,
    27,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenPartialUnstakingIxArgs {
    pub amount: u64,
    pub memo: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpenPartialUnstakingIxData(pub OpenPartialUnstakingIxArgs);
impl From<OpenPartialUnstakingIxArgs> for OpenPartialUnstakingIxData {
    fn from(args: OpenPartialUnstakingIxArgs) -> Self {
        Self(args)
    }
}
impl OpenPartialUnstakingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_PARTIAL_UNSTAKING_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        OPEN_PARTIAL_UNSTAKING_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(OpenPartialUnstakingIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_PARTIAL_UNSTAKING_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn open_partial_unstaking_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenPartialUnstakingKeys,
    args: OpenPartialUnstakingIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN] = keys.into();
    let data: OpenPartialUnstakingIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn open_partial_unstaking_ix(
    keys: OpenPartialUnstakingKeys,
    args: OpenPartialUnstakingIxArgs,
) -> std::io::Result<Instruction> {
    open_partial_unstaking_ix_with_program_id(crate::ID, keys, args)
}
pub fn open_partial_unstaking_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenPartialUnstakingAccounts<'_, '_>,
    args: OpenPartialUnstakingIxArgs,
) -> ProgramResult {
    let keys: OpenPartialUnstakingKeys = accounts.into();
    let ix = open_partial_unstaking_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn open_partial_unstaking_invoke(
    accounts: OpenPartialUnstakingAccounts<'_, '_>,
    args: OpenPartialUnstakingIxArgs,
) -> ProgramResult {
    open_partial_unstaking_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn open_partial_unstaking_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenPartialUnstakingAccounts<'_, '_>,
    args: OpenPartialUnstakingIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenPartialUnstakingKeys = accounts.into();
    let ix = open_partial_unstaking_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn open_partial_unstaking_invoke_signed(
    accounts: OpenPartialUnstakingAccounts<'_, '_>,
    args: OpenPartialUnstakingIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_partial_unstaking_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn open_partial_unstaking_verify_account_keys(
    accounts: OpenPartialUnstakingAccounts<'_, '_>,
    keys: OpenPartialUnstakingKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.locker.key, keys.locker),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.partial_unstake.key, keys.partial_unstake),
        (*accounts.owner.key, keys.owner),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn open_partial_unstaking_verify_writable_privileges<'me, 'info>(
    accounts: OpenPartialUnstakingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.locker,
        accounts.escrow,
        accounts.partial_unstake,
        accounts.owner,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn open_partial_unstaking_verify_signer_privileges<'me, 'info>(
    accounts: OpenPartialUnstakingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.partial_unstake, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn open_partial_unstaking_verify_account_privileges<'me, 'info>(
    accounts: OpenPartialUnstakingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_partial_unstaking_verify_writable_privileges(accounts)?;
    open_partial_unstaking_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MERGE_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct MergePartialUnstakingAccounts<'me, 'info> {
    pub locker: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub partial_unstake: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MergePartialUnstakingKeys {
    pub locker: Pubkey,
    pub escrow: Pubkey,
    pub partial_unstake: Pubkey,
    pub owner: Pubkey,
}
impl From<MergePartialUnstakingAccounts<'_, '_>> for MergePartialUnstakingKeys {
    fn from(accounts: MergePartialUnstakingAccounts) -> Self {
        Self {
            locker: *accounts.locker.key,
            escrow: *accounts.escrow.key,
            partial_unstake: *accounts.partial_unstake.key,
            owner: *accounts.owner.key,
        }
    }
}
impl From<MergePartialUnstakingKeys>
for [AccountMeta; MERGE_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN] {
    fn from(keys: MergePartialUnstakingKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.locker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.partial_unstake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; MERGE_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN]>
for MergePartialUnstakingKeys {
    fn from(pubkeys: [Pubkey; MERGE_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            locker: pubkeys[0],
            escrow: pubkeys[1],
            partial_unstake: pubkeys[2],
            owner: pubkeys[3],
        }
    }
}
impl<'info> From<MergePartialUnstakingAccounts<'_, 'info>>
for [AccountInfo<'info>; MERGE_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN] {
    fn from(accounts: MergePartialUnstakingAccounts<'_, 'info>) -> Self {
        [
            accounts.locker.clone(),
            accounts.escrow.clone(),
            accounts.partial_unstake.clone(),
            accounts.owner.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MERGE_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN]>
for MergePartialUnstakingAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MERGE_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            locker: &arr[0],
            escrow: &arr[1],
            partial_unstake: &arr[2],
            owner: &arr[3],
        }
    }
}
pub const MERGE_PARTIAL_UNSTAKING_IX_DISCM: [u8; 8] = [
    190,
    154,
    163,
    153,
    168,
    115,
    40,
    173,
];
#[derive(Clone, Debug, PartialEq)]
pub struct MergePartialUnstakingIxData;
impl MergePartialUnstakingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MERGE_PARTIAL_UNSTAKING_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        MERGE_PARTIAL_UNSTAKING_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MERGE_PARTIAL_UNSTAKING_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn merge_partial_unstaking_ix_with_program_id(
    program_id: Pubkey,
    keys: MergePartialUnstakingKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MERGE_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MergePartialUnstakingIxData.try_to_vec()?,
    })
}
pub fn merge_partial_unstaking_ix(
    keys: MergePartialUnstakingKeys,
) -> std::io::Result<Instruction> {
    merge_partial_unstaking_ix_with_program_id(crate::ID, keys)
}
pub fn merge_partial_unstaking_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MergePartialUnstakingAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MergePartialUnstakingKeys = accounts.into();
    let ix = merge_partial_unstaking_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn merge_partial_unstaking_invoke(
    accounts: MergePartialUnstakingAccounts<'_, '_>,
) -> ProgramResult {
    merge_partial_unstaking_invoke_with_program_id(crate::ID, accounts)
}
pub fn merge_partial_unstaking_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MergePartialUnstakingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MergePartialUnstakingKeys = accounts.into();
    let ix = merge_partial_unstaking_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn merge_partial_unstaking_invoke_signed(
    accounts: MergePartialUnstakingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    merge_partial_unstaking_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn merge_partial_unstaking_verify_account_keys(
    accounts: MergePartialUnstakingAccounts<'_, '_>,
    keys: MergePartialUnstakingKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.locker.key, keys.locker),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.partial_unstake.key, keys.partial_unstake),
        (*accounts.owner.key, keys.owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn merge_partial_unstaking_verify_writable_privileges<'me, 'info>(
    accounts: MergePartialUnstakingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.locker,
        accounts.escrow,
        accounts.partial_unstake,
        accounts.owner,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn merge_partial_unstaking_verify_signer_privileges<'me, 'info>(
    accounts: MergePartialUnstakingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn merge_partial_unstaking_verify_account_privileges<'me, 'info>(
    accounts: MergePartialUnstakingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    merge_partial_unstaking_verify_writable_privileges(accounts)?;
    merge_partial_unstaking_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawPartialUnstakingAccounts<'me, 'info> {
    pub locker: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub partial_unstake: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub escrow_tokens: &'me AccountInfo<'info>,
    pub destination_tokens: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawPartialUnstakingKeys {
    pub locker: Pubkey,
    pub escrow: Pubkey,
    pub partial_unstake: Pubkey,
    pub owner: Pubkey,
    pub escrow_tokens: Pubkey,
    pub destination_tokens: Pubkey,
    pub payer: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawPartialUnstakingAccounts<'_, '_>> for WithdrawPartialUnstakingKeys {
    fn from(accounts: WithdrawPartialUnstakingAccounts) -> Self {
        Self {
            locker: *accounts.locker.key,
            escrow: *accounts.escrow.key,
            partial_unstake: *accounts.partial_unstake.key,
            owner: *accounts.owner.key,
            escrow_tokens: *accounts.escrow_tokens.key,
            destination_tokens: *accounts.destination_tokens.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawPartialUnstakingKeys>
for [AccountMeta; WITHDRAW_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawPartialUnstakingKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.locker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.partial_unstake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.escrow_tokens,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_tokens,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN]>
for WithdrawPartialUnstakingKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            locker: pubkeys[0],
            escrow: pubkeys[1],
            partial_unstake: pubkeys[2],
            owner: pubkeys[3],
            escrow_tokens: pubkeys[4],
            destination_tokens: pubkeys[5],
            payer: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<WithdrawPartialUnstakingAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawPartialUnstakingAccounts<'_, 'info>) -> Self {
        [
            accounts.locker.clone(),
            accounts.escrow.clone(),
            accounts.partial_unstake.clone(),
            accounts.owner.clone(),
            accounts.escrow_tokens.clone(),
            accounts.destination_tokens.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN]>
for WithdrawPartialUnstakingAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            locker: &arr[0],
            escrow: &arr[1],
            partial_unstake: &arr[2],
            owner: &arr[3],
            escrow_tokens: &arr[4],
            destination_tokens: &arr[5],
            payer: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const WITHDRAW_PARTIAL_UNSTAKING_IX_DISCM: [u8; 8] = [
    201,
    202,
    137,
    124,
    2,
    3,
    245,
    87,
];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawPartialUnstakingIxData;
impl WithdrawPartialUnstakingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_PARTIAL_UNSTAKING_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_PARTIAL_UNSTAKING_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_PARTIAL_UNSTAKING_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_partial_unstaking_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawPartialUnstakingKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_PARTIAL_UNSTAKING_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawPartialUnstakingIxData.try_to_vec()?,
    })
}
pub fn withdraw_partial_unstaking_ix(
    keys: WithdrawPartialUnstakingKeys,
) -> std::io::Result<Instruction> {
    withdraw_partial_unstaking_ix_with_program_id(crate::ID, keys)
}
pub fn withdraw_partial_unstaking_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawPartialUnstakingAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawPartialUnstakingKeys = accounts.into();
    let ix = withdraw_partial_unstaking_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_partial_unstaking_invoke(
    accounts: WithdrawPartialUnstakingAccounts<'_, '_>,
) -> ProgramResult {
    withdraw_partial_unstaking_invoke_with_program_id(crate::ID, accounts)
}
pub fn withdraw_partial_unstaking_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawPartialUnstakingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawPartialUnstakingKeys = accounts.into();
    let ix = withdraw_partial_unstaking_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_partial_unstaking_invoke_signed(
    accounts: WithdrawPartialUnstakingAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_partial_unstaking_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn withdraw_partial_unstaking_verify_account_keys(
    accounts: WithdrawPartialUnstakingAccounts<'_, '_>,
    keys: WithdrawPartialUnstakingKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.locker.key, keys.locker),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.partial_unstake.key, keys.partial_unstake),
        (*accounts.owner.key, keys.owner),
        (*accounts.escrow_tokens.key, keys.escrow_tokens),
        (*accounts.destination_tokens.key, keys.destination_tokens),
        (*accounts.payer.key, keys.payer),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_partial_unstaking_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawPartialUnstakingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.locker,
        accounts.escrow,
        accounts.partial_unstake,
        accounts.escrow_tokens,
        accounts.destination_tokens,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_partial_unstaking_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawPartialUnstakingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_partial_unstaking_verify_account_privileges<'me, 'info>(
    accounts: WithdrawPartialUnstakingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_partial_unstaking_verify_writable_privileges(accounts)?;
    withdraw_partial_unstaking_verify_signer_privileges(accounts)?;
    Ok(())
}
