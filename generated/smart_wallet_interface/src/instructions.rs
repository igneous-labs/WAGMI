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
pub enum SmartWalletProgramIx {
    CreateSmartWallet(CreateSmartWalletIxArgs),
    SetOwners(SetOwnersIxArgs),
    ChangeThreshold(ChangeThresholdIxArgs),
    CreateTransaction(CreateTransactionIxArgs),
    RemoveTransaction,
    CreateTransactionWithTimelock(CreateTransactionWithTimelockIxArgs),
    Approve,
    Unapprove,
    ExecuteTransaction,
    ExecuteTransactionDerived(ExecuteTransactionDerivedIxArgs),
    OwnerInvokeInstruction(OwnerInvokeInstructionIxArgs),
    OwnerInvokeInstructionV2(OwnerInvokeInstructionV2IxArgs),
    CreateSubaccountInfo(CreateSubaccountInfoIxArgs),
}
impl SmartWalletProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            CREATE_SMART_WALLET_IX_DISCM => {
                Ok(
                    Self::CreateSmartWallet(
                        CreateSmartWalletIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            SET_OWNERS_IX_DISCM => {
                Ok(Self::SetOwners(SetOwnersIxArgs::deserialize(&mut reader)?))
            }
            CHANGE_THRESHOLD_IX_DISCM => {
                Ok(
                    Self::ChangeThreshold(
                        ChangeThresholdIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            CREATE_TRANSACTION_IX_DISCM => {
                Ok(
                    Self::CreateTransaction(
                        CreateTransactionIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            REMOVE_TRANSACTION_IX_DISCM => Ok(Self::RemoveTransaction),
            CREATE_TRANSACTION_WITH_TIMELOCK_IX_DISCM => {
                Ok(
                    Self::CreateTransactionWithTimelock(
                        CreateTransactionWithTimelockIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            APPROVE_IX_DISCM => Ok(Self::Approve),
            UNAPPROVE_IX_DISCM => Ok(Self::Unapprove),
            EXECUTE_TRANSACTION_IX_DISCM => Ok(Self::ExecuteTransaction),
            EXECUTE_TRANSACTION_DERIVED_IX_DISCM => {
                Ok(
                    Self::ExecuteTransactionDerived(
                        ExecuteTransactionDerivedIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            OWNER_INVOKE_INSTRUCTION_IX_DISCM => {
                Ok(
                    Self::OwnerInvokeInstruction(
                        OwnerInvokeInstructionIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            OWNER_INVOKE_INSTRUCTION_V2_IX_DISCM => {
                Ok(
                    Self::OwnerInvokeInstructionV2(
                        OwnerInvokeInstructionV2IxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            CREATE_SUBACCOUNT_INFO_IX_DISCM => {
                Ok(
                    Self::CreateSubaccountInfo(
                        CreateSubaccountInfoIxArgs::deserialize(&mut reader)?,
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
            Self::CreateSmartWallet(args) => {
                writer.write_all(&CREATE_SMART_WALLET_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetOwners(args) => {
                writer.write_all(&SET_OWNERS_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ChangeThreshold(args) => {
                writer.write_all(&CHANGE_THRESHOLD_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreateTransaction(args) => {
                writer.write_all(&CREATE_TRANSACTION_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RemoveTransaction => writer.write_all(&REMOVE_TRANSACTION_IX_DISCM),
            Self::CreateTransactionWithTimelock(args) => {
                writer.write_all(&CREATE_TRANSACTION_WITH_TIMELOCK_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Approve => writer.write_all(&APPROVE_IX_DISCM),
            Self::Unapprove => writer.write_all(&UNAPPROVE_IX_DISCM),
            Self::ExecuteTransaction => writer.write_all(&EXECUTE_TRANSACTION_IX_DISCM),
            Self::ExecuteTransactionDerived(args) => {
                writer.write_all(&EXECUTE_TRANSACTION_DERIVED_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::OwnerInvokeInstruction(args) => {
                writer.write_all(&OWNER_INVOKE_INSTRUCTION_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::OwnerInvokeInstructionV2(args) => {
                writer.write_all(&OWNER_INVOKE_INSTRUCTION_V2_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreateSubaccountInfo(args) => {
                writer.write_all(&CREATE_SUBACCOUNT_INFO_IX_DISCM)?;
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
pub const CREATE_SMART_WALLET_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CreateSmartWalletAccounts<'me, 'info> {
    pub base: &'me AccountInfo<'info>,
    pub smart_wallet: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateSmartWalletKeys {
    pub base: Pubkey,
    pub smart_wallet: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateSmartWalletAccounts<'_, '_>> for CreateSmartWalletKeys {
    fn from(accounts: CreateSmartWalletAccounts) -> Self {
        Self {
            base: *accounts.base.key,
            smart_wallet: *accounts.smart_wallet.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateSmartWalletKeys> for [AccountMeta; CREATE_SMART_WALLET_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateSmartWalletKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
                is_writable: false,
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
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_SMART_WALLET_IX_ACCOUNTS_LEN]> for CreateSmartWalletKeys {
    fn from(pubkeys: [Pubkey; CREATE_SMART_WALLET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            base: pubkeys[0],
            smart_wallet: pubkeys[1],
            payer: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<CreateSmartWalletAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_SMART_WALLET_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateSmartWalletAccounts<'_, 'info>) -> Self {
        [
            accounts.base.clone(),
            accounts.smart_wallet.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_SMART_WALLET_IX_ACCOUNTS_LEN]>
for CreateSmartWalletAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_SMART_WALLET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            base: &arr[0],
            smart_wallet: &arr[1],
            payer: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const CREATE_SMART_WALLET_IX_DISCM: [u8; 8] = [129, 39, 235, 18, 132, 68, 203, 19];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateSmartWalletIxArgs {
    pub max_owners: u8,
    pub owners: Vec<Pubkey>,
    pub threshold: u64,
    pub minimum_delay: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateSmartWalletIxData(pub CreateSmartWalletIxArgs);
impl From<CreateSmartWalletIxArgs> for CreateSmartWalletIxData {
    fn from(args: CreateSmartWalletIxArgs) -> Self {
        Self(args)
    }
}
impl CreateSmartWalletIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_SMART_WALLET_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_SMART_WALLET_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateSmartWalletIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_SMART_WALLET_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_smart_wallet_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateSmartWalletKeys,
    args: CreateSmartWalletIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_SMART_WALLET_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateSmartWalletIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_smart_wallet_ix(
    keys: CreateSmartWalletKeys,
    args: CreateSmartWalletIxArgs,
) -> std::io::Result<Instruction> {
    create_smart_wallet_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_smart_wallet_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateSmartWalletAccounts<'_, '_>,
    args: CreateSmartWalletIxArgs,
) -> ProgramResult {
    let keys: CreateSmartWalletKeys = accounts.into();
    let ix = create_smart_wallet_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_smart_wallet_invoke(
    accounts: CreateSmartWalletAccounts<'_, '_>,
    args: CreateSmartWalletIxArgs,
) -> ProgramResult {
    create_smart_wallet_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_smart_wallet_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateSmartWalletAccounts<'_, '_>,
    args: CreateSmartWalletIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateSmartWalletKeys = accounts.into();
    let ix = create_smart_wallet_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_smart_wallet_invoke_signed(
    accounts: CreateSmartWalletAccounts<'_, '_>,
    args: CreateSmartWalletIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_smart_wallet_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_smart_wallet_verify_account_keys(
    accounts: CreateSmartWalletAccounts<'_, '_>,
    keys: CreateSmartWalletKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.base.key, keys.base),
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
pub fn create_smart_wallet_verify_writable_privileges<'me, 'info>(
    accounts: CreateSmartWalletAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.smart_wallet, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_smart_wallet_verify_signer_privileges<'me, 'info>(
    accounts: CreateSmartWalletAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.base, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_smart_wallet_verify_account_privileges<'me, 'info>(
    accounts: CreateSmartWalletAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_smart_wallet_verify_writable_privileges(accounts)?;
    create_smart_wallet_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_OWNERS_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct SetOwnersAccounts<'me, 'info> {
    pub smart_wallet: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetOwnersKeys {
    pub smart_wallet: Pubkey,
}
impl From<SetOwnersAccounts<'_, '_>> for SetOwnersKeys {
    fn from(accounts: SetOwnersAccounts) -> Self {
        Self {
            smart_wallet: *accounts.smart_wallet.key,
        }
    }
}
impl From<SetOwnersKeys> for [AccountMeta; SET_OWNERS_IX_ACCOUNTS_LEN] {
    fn from(keys: SetOwnersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SET_OWNERS_IX_ACCOUNTS_LEN]> for SetOwnersKeys {
    fn from(pubkeys: [Pubkey; SET_OWNERS_IX_ACCOUNTS_LEN]) -> Self {
        Self { smart_wallet: pubkeys[0] }
    }
}
impl<'info> From<SetOwnersAccounts<'_, 'info>>
for [AccountInfo<'info>; SET_OWNERS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetOwnersAccounts<'_, 'info>) -> Self {
        [accounts.smart_wallet.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_OWNERS_IX_ACCOUNTS_LEN]>
for SetOwnersAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SET_OWNERS_IX_ACCOUNTS_LEN]) -> Self {
        Self { smart_wallet: &arr[0] }
    }
}
pub const SET_OWNERS_IX_DISCM: [u8; 8] = [134, 145, 42, 122, 94, 64, 76, 218];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetOwnersIxArgs {
    pub owners: Vec<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetOwnersIxData(pub SetOwnersIxArgs);
impl From<SetOwnersIxArgs> for SetOwnersIxData {
    fn from(args: SetOwnersIxArgs) -> Self {
        Self(args)
    }
}
impl SetOwnersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_OWNERS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SET_OWNERS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SetOwnersIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_OWNERS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_owners_ix_with_program_id(
    program_id: Pubkey,
    keys: SetOwnersKeys,
    args: SetOwnersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_OWNERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetOwnersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_owners_ix(
    keys: SetOwnersKeys,
    args: SetOwnersIxArgs,
) -> std::io::Result<Instruction> {
    set_owners_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_owners_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetOwnersAccounts<'_, '_>,
    args: SetOwnersIxArgs,
) -> ProgramResult {
    let keys: SetOwnersKeys = accounts.into();
    let ix = set_owners_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_owners_invoke(
    accounts: SetOwnersAccounts<'_, '_>,
    args: SetOwnersIxArgs,
) -> ProgramResult {
    set_owners_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_owners_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetOwnersAccounts<'_, '_>,
    args: SetOwnersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetOwnersKeys = accounts.into();
    let ix = set_owners_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_owners_invoke_signed(
    accounts: SetOwnersAccounts<'_, '_>,
    args: SetOwnersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_owners_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_owners_verify_account_keys(
    accounts: SetOwnersAccounts<'_, '_>,
    keys: SetOwnersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(*accounts.smart_wallet.key, keys.smart_wallet)] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_owners_verify_writable_privileges<'me, 'info>(
    accounts: SetOwnersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.smart_wallet] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_owners_verify_signer_privileges<'me, 'info>(
    accounts: SetOwnersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.smart_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_owners_verify_account_privileges<'me, 'info>(
    accounts: SetOwnersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_owners_verify_writable_privileges(accounts)?;
    set_owners_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CHANGE_THRESHOLD_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct ChangeThresholdAccounts<'me, 'info> {
    pub smart_wallet: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChangeThresholdKeys {
    pub smart_wallet: Pubkey,
}
impl From<ChangeThresholdAccounts<'_, '_>> for ChangeThresholdKeys {
    fn from(accounts: ChangeThresholdAccounts) -> Self {
        Self {
            smart_wallet: *accounts.smart_wallet.key,
        }
    }
}
impl From<ChangeThresholdKeys> for [AccountMeta; CHANGE_THRESHOLD_IX_ACCOUNTS_LEN] {
    fn from(keys: ChangeThresholdKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CHANGE_THRESHOLD_IX_ACCOUNTS_LEN]> for ChangeThresholdKeys {
    fn from(pubkeys: [Pubkey; CHANGE_THRESHOLD_IX_ACCOUNTS_LEN]) -> Self {
        Self { smart_wallet: pubkeys[0] }
    }
}
impl<'info> From<ChangeThresholdAccounts<'_, 'info>>
for [AccountInfo<'info>; CHANGE_THRESHOLD_IX_ACCOUNTS_LEN] {
    fn from(accounts: ChangeThresholdAccounts<'_, 'info>) -> Self {
        [accounts.smart_wallet.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHANGE_THRESHOLD_IX_ACCOUNTS_LEN]>
for ChangeThresholdAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CHANGE_THRESHOLD_IX_ACCOUNTS_LEN]) -> Self {
        Self { smart_wallet: &arr[0] }
    }
}
pub const CHANGE_THRESHOLD_IX_DISCM: [u8; 8] = [146, 151, 213, 63, 121, 79, 9, 29];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeThresholdIxArgs {
    pub threshold: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeThresholdIxData(pub ChangeThresholdIxArgs);
impl From<ChangeThresholdIxArgs> for ChangeThresholdIxData {
    fn from(args: ChangeThresholdIxArgs) -> Self {
        Self(args)
    }
}
impl ChangeThresholdIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CHANGE_THRESHOLD_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CHANGE_THRESHOLD_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ChangeThresholdIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CHANGE_THRESHOLD_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn change_threshold_ix_with_program_id(
    program_id: Pubkey,
    keys: ChangeThresholdKeys,
    args: ChangeThresholdIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHANGE_THRESHOLD_IX_ACCOUNTS_LEN] = keys.into();
    let data: ChangeThresholdIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn change_threshold_ix(
    keys: ChangeThresholdKeys,
    args: ChangeThresholdIxArgs,
) -> std::io::Result<Instruction> {
    change_threshold_ix_with_program_id(crate::ID, keys, args)
}
pub fn change_threshold_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChangeThresholdAccounts<'_, '_>,
    args: ChangeThresholdIxArgs,
) -> ProgramResult {
    let keys: ChangeThresholdKeys = accounts.into();
    let ix = change_threshold_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn change_threshold_invoke(
    accounts: ChangeThresholdAccounts<'_, '_>,
    args: ChangeThresholdIxArgs,
) -> ProgramResult {
    change_threshold_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn change_threshold_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChangeThresholdAccounts<'_, '_>,
    args: ChangeThresholdIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChangeThresholdKeys = accounts.into();
    let ix = change_threshold_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn change_threshold_invoke_signed(
    accounts: ChangeThresholdAccounts<'_, '_>,
    args: ChangeThresholdIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    change_threshold_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn change_threshold_verify_account_keys(
    accounts: ChangeThresholdAccounts<'_, '_>,
    keys: ChangeThresholdKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(*accounts.smart_wallet.key, keys.smart_wallet)] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn change_threshold_verify_writable_privileges<'me, 'info>(
    accounts: ChangeThresholdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.smart_wallet] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn change_threshold_verify_signer_privileges<'me, 'info>(
    accounts: ChangeThresholdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.smart_wallet] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn change_threshold_verify_account_privileges<'me, 'info>(
    accounts: ChangeThresholdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    change_threshold_verify_writable_privileges(accounts)?;
    change_threshold_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TRANSACTION_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateTransactionAccounts<'me, 'info> {
    pub smart_wallet: &'me AccountInfo<'info>,
    pub transaction: &'me AccountInfo<'info>,
    pub proposer: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTransactionKeys {
    pub smart_wallet: Pubkey,
    pub transaction: Pubkey,
    pub proposer: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateTransactionAccounts<'_, '_>> for CreateTransactionKeys {
    fn from(accounts: CreateTransactionAccounts) -> Self {
        Self {
            smart_wallet: *accounts.smart_wallet.key,
            transaction: *accounts.transaction.key,
            proposer: *accounts.proposer.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateTransactionKeys> for [AccountMeta; CREATE_TRANSACTION_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTransactionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transaction,
                is_signer: false,
                is_writable: true,
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
        ]
    }
}
impl From<[Pubkey; CREATE_TRANSACTION_IX_ACCOUNTS_LEN]> for CreateTransactionKeys {
    fn from(pubkeys: [Pubkey; CREATE_TRANSACTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            smart_wallet: pubkeys[0],
            transaction: pubkeys[1],
            proposer: pubkeys[2],
            payer: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CreateTransactionAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_TRANSACTION_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTransactionAccounts<'_, 'info>) -> Self {
        [
            accounts.smart_wallet.clone(),
            accounts.transaction.clone(),
            accounts.proposer.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TRANSACTION_IX_ACCOUNTS_LEN]>
for CreateTransactionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_TRANSACTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            smart_wallet: &arr[0],
            transaction: &arr[1],
            proposer: &arr[2],
            payer: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CREATE_TRANSACTION_IX_DISCM: [u8; 8] = [227, 193, 53, 239, 55, 126, 112, 105];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTransactionIxArgs {
    pub bump: u8,
    pub instructions: Vec<TXInstruction>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTransactionIxData(pub CreateTransactionIxArgs);
impl From<CreateTransactionIxArgs> for CreateTransactionIxData {
    fn from(args: CreateTransactionIxArgs) -> Self {
        Self(args)
    }
}
impl CreateTransactionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TRANSACTION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_TRANSACTION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateTransactionIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TRANSACTION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_transaction_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTransactionKeys,
    args: CreateTransactionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TRANSACTION_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateTransactionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_transaction_ix(
    keys: CreateTransactionKeys,
    args: CreateTransactionIxArgs,
) -> std::io::Result<Instruction> {
    create_transaction_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_transaction_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTransactionAccounts<'_, '_>,
    args: CreateTransactionIxArgs,
) -> ProgramResult {
    let keys: CreateTransactionKeys = accounts.into();
    let ix = create_transaction_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_transaction_invoke(
    accounts: CreateTransactionAccounts<'_, '_>,
    args: CreateTransactionIxArgs,
) -> ProgramResult {
    create_transaction_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_transaction_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTransactionAccounts<'_, '_>,
    args: CreateTransactionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTransactionKeys = accounts.into();
    let ix = create_transaction_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_transaction_invoke_signed(
    accounts: CreateTransactionAccounts<'_, '_>,
    args: CreateTransactionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_transaction_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_transaction_verify_account_keys(
    accounts: CreateTransactionAccounts<'_, '_>,
    keys: CreateTransactionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.smart_wallet.key, keys.smart_wallet),
        (*accounts.transaction.key, keys.transaction),
        (*accounts.proposer.key, keys.proposer),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_transaction_verify_writable_privileges<'me, 'info>(
    accounts: CreateTransactionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.smart_wallet,
        accounts.transaction,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_transaction_verify_signer_privileges<'me, 'info>(
    accounts: CreateTransactionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.proposer, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_transaction_verify_account_privileges<'me, 'info>(
    accounts: CreateTransactionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_transaction_verify_writable_privileges(accounts)?;
    create_transaction_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_TRANSACTION_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct RemoveTransactionAccounts<'me, 'info> {
    pub smart_wallet: &'me AccountInfo<'info>,
    pub transaction: &'me AccountInfo<'info>,
    pub proposer: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveTransactionKeys {
    pub smart_wallet: Pubkey,
    pub transaction: Pubkey,
    pub proposer: Pubkey,
}
impl From<RemoveTransactionAccounts<'_, '_>> for RemoveTransactionKeys {
    fn from(accounts: RemoveTransactionAccounts) -> Self {
        Self {
            smart_wallet: *accounts.smart_wallet.key,
            transaction: *accounts.transaction.key,
            proposer: *accounts.proposer.key,
        }
    }
}
impl From<RemoveTransactionKeys> for [AccountMeta; REMOVE_TRANSACTION_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveTransactionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transaction,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.proposer,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_TRANSACTION_IX_ACCOUNTS_LEN]> for RemoveTransactionKeys {
    fn from(pubkeys: [Pubkey; REMOVE_TRANSACTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            smart_wallet: pubkeys[0],
            transaction: pubkeys[1],
            proposer: pubkeys[2],
        }
    }
}
impl<'info> From<RemoveTransactionAccounts<'_, 'info>>
for [AccountInfo<'info>; REMOVE_TRANSACTION_IX_ACCOUNTS_LEN] {
    fn from(accounts: RemoveTransactionAccounts<'_, 'info>) -> Self {
        [
            accounts.smart_wallet.clone(),
            accounts.transaction.clone(),
            accounts.proposer.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_TRANSACTION_IX_ACCOUNTS_LEN]>
for RemoveTransactionAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_TRANSACTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            smart_wallet: &arr[0],
            transaction: &arr[1],
            proposer: &arr[2],
        }
    }
}
pub const REMOVE_TRANSACTION_IX_DISCM: [u8; 8] = [11, 129, 185, 56, 38, 17, 111, 190];
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveTransactionIxData;
impl RemoveTransactionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_TRANSACTION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REMOVE_TRANSACTION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_TRANSACTION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_transaction_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveTransactionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_TRANSACTION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveTransactionIxData.try_to_vec()?,
    })
}
pub fn remove_transaction_ix(
    keys: RemoveTransactionKeys,
) -> std::io::Result<Instruction> {
    remove_transaction_ix_with_program_id(crate::ID, keys)
}
pub fn remove_transaction_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveTransactionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveTransactionKeys = accounts.into();
    let ix = remove_transaction_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_transaction_invoke(
    accounts: RemoveTransactionAccounts<'_, '_>,
) -> ProgramResult {
    remove_transaction_invoke_with_program_id(crate::ID, accounts)
}
pub fn remove_transaction_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveTransactionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveTransactionKeys = accounts.into();
    let ix = remove_transaction_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_transaction_invoke_signed(
    accounts: RemoveTransactionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_transaction_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn remove_transaction_verify_account_keys(
    accounts: RemoveTransactionAccounts<'_, '_>,
    keys: RemoveTransactionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.smart_wallet.key, keys.smart_wallet),
        (*accounts.transaction.key, keys.transaction),
        (*accounts.proposer.key, keys.proposer),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn remove_transaction_verify_writable_privileges<'me, 'info>(
    accounts: RemoveTransactionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.smart_wallet,
        accounts.transaction,
        accounts.proposer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_transaction_verify_signer_privileges<'me, 'info>(
    accounts: RemoveTransactionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.proposer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_transaction_verify_account_privileges<'me, 'info>(
    accounts: RemoveTransactionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_transaction_verify_writable_privileges(accounts)?;
    remove_transaction_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TRANSACTION_WITH_TIMELOCK_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateTransactionWithTimelockAccounts<'me, 'info> {
    pub smart_wallet: &'me AccountInfo<'info>,
    pub transaction: &'me AccountInfo<'info>,
    pub proposer: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTransactionWithTimelockKeys {
    pub smart_wallet: Pubkey,
    pub transaction: Pubkey,
    pub proposer: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateTransactionWithTimelockAccounts<'_, '_>>
for CreateTransactionWithTimelockKeys {
    fn from(accounts: CreateTransactionWithTimelockAccounts) -> Self {
        Self {
            smart_wallet: *accounts.smart_wallet.key,
            transaction: *accounts.transaction.key,
            proposer: *accounts.proposer.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateTransactionWithTimelockKeys>
for [AccountMeta; CREATE_TRANSACTION_WITH_TIMELOCK_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTransactionWithTimelockKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transaction,
                is_signer: false,
                is_writable: true,
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
        ]
    }
}
impl From<[Pubkey; CREATE_TRANSACTION_WITH_TIMELOCK_IX_ACCOUNTS_LEN]>
for CreateTransactionWithTimelockKeys {
    fn from(
        pubkeys: [Pubkey; CREATE_TRANSACTION_WITH_TIMELOCK_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            smart_wallet: pubkeys[0],
            transaction: pubkeys[1],
            proposer: pubkeys[2],
            payer: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CreateTransactionWithTimelockAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_TRANSACTION_WITH_TIMELOCK_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTransactionWithTimelockAccounts<'_, 'info>) -> Self {
        [
            accounts.smart_wallet.clone(),
            accounts.transaction.clone(),
            accounts.proposer.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; CREATE_TRANSACTION_WITH_TIMELOCK_IX_ACCOUNTS_LEN]>
for CreateTransactionWithTimelockAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_TRANSACTION_WITH_TIMELOCK_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            smart_wallet: &arr[0],
            transaction: &arr[1],
            proposer: &arr[2],
            payer: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CREATE_TRANSACTION_WITH_TIMELOCK_IX_DISCM: [u8; 8] = [
    93,
    252,
    41,
    108,
    86,
    76,
    89,
    237,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTransactionWithTimelockIxArgs {
    pub bump: u8,
    pub instructions: Vec<TXInstruction>,
    pub eta: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTransactionWithTimelockIxData(pub CreateTransactionWithTimelockIxArgs);
impl From<CreateTransactionWithTimelockIxArgs> for CreateTransactionWithTimelockIxData {
    fn from(args: CreateTransactionWithTimelockIxArgs) -> Self {
        Self(args)
    }
}
impl CreateTransactionWithTimelockIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TRANSACTION_WITH_TIMELOCK_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_TRANSACTION_WITH_TIMELOCK_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateTransactionWithTimelockIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TRANSACTION_WITH_TIMELOCK_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_transaction_with_timelock_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTransactionWithTimelockKeys,
    args: CreateTransactionWithTimelockIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TRANSACTION_WITH_TIMELOCK_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: CreateTransactionWithTimelockIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_transaction_with_timelock_ix(
    keys: CreateTransactionWithTimelockKeys,
    args: CreateTransactionWithTimelockIxArgs,
) -> std::io::Result<Instruction> {
    create_transaction_with_timelock_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_transaction_with_timelock_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTransactionWithTimelockAccounts<'_, '_>,
    args: CreateTransactionWithTimelockIxArgs,
) -> ProgramResult {
    let keys: CreateTransactionWithTimelockKeys = accounts.into();
    let ix = create_transaction_with_timelock_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn create_transaction_with_timelock_invoke(
    accounts: CreateTransactionWithTimelockAccounts<'_, '_>,
    args: CreateTransactionWithTimelockIxArgs,
) -> ProgramResult {
    create_transaction_with_timelock_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_transaction_with_timelock_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTransactionWithTimelockAccounts<'_, '_>,
    args: CreateTransactionWithTimelockIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTransactionWithTimelockKeys = accounts.into();
    let ix = create_transaction_with_timelock_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_transaction_with_timelock_invoke_signed(
    accounts: CreateTransactionWithTimelockAccounts<'_, '_>,
    args: CreateTransactionWithTimelockIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_transaction_with_timelock_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_transaction_with_timelock_verify_account_keys(
    accounts: CreateTransactionWithTimelockAccounts<'_, '_>,
    keys: CreateTransactionWithTimelockKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.smart_wallet.key, keys.smart_wallet),
        (*accounts.transaction.key, keys.transaction),
        (*accounts.proposer.key, keys.proposer),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_transaction_with_timelock_verify_writable_privileges<'me, 'info>(
    accounts: CreateTransactionWithTimelockAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.smart_wallet,
        accounts.transaction,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_transaction_with_timelock_verify_signer_privileges<'me, 'info>(
    accounts: CreateTransactionWithTimelockAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.proposer, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_transaction_with_timelock_verify_account_privileges<'me, 'info>(
    accounts: CreateTransactionWithTimelockAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_transaction_with_timelock_verify_writable_privileges(accounts)?;
    create_transaction_with_timelock_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const APPROVE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ApproveAccounts<'me, 'info> {
    pub smart_wallet: &'me AccountInfo<'info>,
    pub transaction: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ApproveKeys {
    pub smart_wallet: Pubkey,
    pub transaction: Pubkey,
    pub owner: Pubkey,
}
impl From<ApproveAccounts<'_, '_>> for ApproveKeys {
    fn from(accounts: ApproveAccounts) -> Self {
        Self {
            smart_wallet: *accounts.smart_wallet.key,
            transaction: *accounts.transaction.key,
            owner: *accounts.owner.key,
        }
    }
}
impl From<ApproveKeys> for [AccountMeta; APPROVE_IX_ACCOUNTS_LEN] {
    fn from(keys: ApproveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.transaction,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; APPROVE_IX_ACCOUNTS_LEN]> for ApproveKeys {
    fn from(pubkeys: [Pubkey; APPROVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            smart_wallet: pubkeys[0],
            transaction: pubkeys[1],
            owner: pubkeys[2],
        }
    }
}
impl<'info> From<ApproveAccounts<'_, 'info>>
for [AccountInfo<'info>; APPROVE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ApproveAccounts<'_, 'info>) -> Self {
        [
            accounts.smart_wallet.clone(),
            accounts.transaction.clone(),
            accounts.owner.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; APPROVE_IX_ACCOUNTS_LEN]>
for ApproveAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; APPROVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            smart_wallet: &arr[0],
            transaction: &arr[1],
            owner: &arr[2],
        }
    }
}
pub const APPROVE_IX_DISCM: [u8; 8] = [69, 74, 217, 36, 115, 117, 97, 76];
#[derive(Clone, Debug, PartialEq)]
pub struct ApproveIxData;
impl ApproveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != APPROVE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        APPROVE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&APPROVE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn approve_ix_with_program_id(
    program_id: Pubkey,
    keys: ApproveKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; APPROVE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ApproveIxData.try_to_vec()?,
    })
}
pub fn approve_ix(keys: ApproveKeys) -> std::io::Result<Instruction> {
    approve_ix_with_program_id(crate::ID, keys)
}
pub fn approve_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ApproveAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ApproveKeys = accounts.into();
    let ix = approve_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn approve_invoke(accounts: ApproveAccounts<'_, '_>) -> ProgramResult {
    approve_invoke_with_program_id(crate::ID, accounts)
}
pub fn approve_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ApproveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ApproveKeys = accounts.into();
    let ix = approve_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn approve_invoke_signed(
    accounts: ApproveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    approve_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn approve_verify_account_keys(
    accounts: ApproveAccounts<'_, '_>,
    keys: ApproveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.smart_wallet.key, keys.smart_wallet),
        (*accounts.transaction.key, keys.transaction),
        (*accounts.owner.key, keys.owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn approve_verify_writable_privileges<'me, 'info>(
    accounts: ApproveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.transaction] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn approve_verify_signer_privileges<'me, 'info>(
    accounts: ApproveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn approve_verify_account_privileges<'me, 'info>(
    accounts: ApproveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    approve_verify_writable_privileges(accounts)?;
    approve_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UNAPPROVE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UnapproveAccounts<'me, 'info> {
    pub smart_wallet: &'me AccountInfo<'info>,
    pub transaction: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnapproveKeys {
    pub smart_wallet: Pubkey,
    pub transaction: Pubkey,
    pub owner: Pubkey,
}
impl From<UnapproveAccounts<'_, '_>> for UnapproveKeys {
    fn from(accounts: UnapproveAccounts) -> Self {
        Self {
            smart_wallet: *accounts.smart_wallet.key,
            transaction: *accounts.transaction.key,
            owner: *accounts.owner.key,
        }
    }
}
impl From<UnapproveKeys> for [AccountMeta; UNAPPROVE_IX_ACCOUNTS_LEN] {
    fn from(keys: UnapproveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.transaction,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UNAPPROVE_IX_ACCOUNTS_LEN]> for UnapproveKeys {
    fn from(pubkeys: [Pubkey; UNAPPROVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            smart_wallet: pubkeys[0],
            transaction: pubkeys[1],
            owner: pubkeys[2],
        }
    }
}
impl<'info> From<UnapproveAccounts<'_, 'info>>
for [AccountInfo<'info>; UNAPPROVE_IX_ACCOUNTS_LEN] {
    fn from(accounts: UnapproveAccounts<'_, 'info>) -> Self {
        [
            accounts.smart_wallet.clone(),
            accounts.transaction.clone(),
            accounts.owner.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNAPPROVE_IX_ACCOUNTS_LEN]>
for UnapproveAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UNAPPROVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            smart_wallet: &arr[0],
            transaction: &arr[1],
            owner: &arr[2],
        }
    }
}
pub const UNAPPROVE_IX_DISCM: [u8; 8] = [5, 92, 229, 161, 250, 166, 122, 171];
#[derive(Clone, Debug, PartialEq)]
pub struct UnapproveIxData;
impl UnapproveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNAPPROVE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UNAPPROVE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNAPPROVE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn unapprove_ix_with_program_id(
    program_id: Pubkey,
    keys: UnapproveKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UNAPPROVE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UnapproveIxData.try_to_vec()?,
    })
}
pub fn unapprove_ix(keys: UnapproveKeys) -> std::io::Result<Instruction> {
    unapprove_ix_with_program_id(crate::ID, keys)
}
pub fn unapprove_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UnapproveAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UnapproveKeys = accounts.into();
    let ix = unapprove_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn unapprove_invoke(accounts: UnapproveAccounts<'_, '_>) -> ProgramResult {
    unapprove_invoke_with_program_id(crate::ID, accounts)
}
pub fn unapprove_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UnapproveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UnapproveKeys = accounts.into();
    let ix = unapprove_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn unapprove_invoke_signed(
    accounts: UnapproveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    unapprove_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn unapprove_verify_account_keys(
    accounts: UnapproveAccounts<'_, '_>,
    keys: UnapproveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.smart_wallet.key, keys.smart_wallet),
        (*accounts.transaction.key, keys.transaction),
        (*accounts.owner.key, keys.owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn unapprove_verify_writable_privileges<'me, 'info>(
    accounts: UnapproveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.transaction] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn unapprove_verify_signer_privileges<'me, 'info>(
    accounts: UnapproveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn unapprove_verify_account_privileges<'me, 'info>(
    accounts: UnapproveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    unapprove_verify_writable_privileges(accounts)?;
    unapprove_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTE_TRANSACTION_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ExecuteTransactionAccounts<'me, 'info> {
    pub smart_wallet: &'me AccountInfo<'info>,
    pub transaction: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecuteTransactionKeys {
    pub smart_wallet: Pubkey,
    pub transaction: Pubkey,
    pub owner: Pubkey,
}
impl From<ExecuteTransactionAccounts<'_, '_>> for ExecuteTransactionKeys {
    fn from(accounts: ExecuteTransactionAccounts) -> Self {
        Self {
            smart_wallet: *accounts.smart_wallet.key,
            transaction: *accounts.transaction.key,
            owner: *accounts.owner.key,
        }
    }
}
impl From<ExecuteTransactionKeys>
for [AccountMeta; EXECUTE_TRANSACTION_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecuteTransactionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.transaction,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; EXECUTE_TRANSACTION_IX_ACCOUNTS_LEN]> for ExecuteTransactionKeys {
    fn from(pubkeys: [Pubkey; EXECUTE_TRANSACTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            smart_wallet: pubkeys[0],
            transaction: pubkeys[1],
            owner: pubkeys[2],
        }
    }
}
impl<'info> From<ExecuteTransactionAccounts<'_, 'info>>
for [AccountInfo<'info>; EXECUTE_TRANSACTION_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExecuteTransactionAccounts<'_, 'info>) -> Self {
        [
            accounts.smart_wallet.clone(),
            accounts.transaction.clone(),
            accounts.owner.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EXECUTE_TRANSACTION_IX_ACCOUNTS_LEN]>
for ExecuteTransactionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; EXECUTE_TRANSACTION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            smart_wallet: &arr[0],
            transaction: &arr[1],
            owner: &arr[2],
        }
    }
}
pub const EXECUTE_TRANSACTION_IX_DISCM: [u8; 8] = [231, 173, 49, 91, 235, 24, 68, 19];
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteTransactionIxData;
impl ExecuteTransactionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_TRANSACTION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EXECUTE_TRANSACTION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_TRANSACTION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn execute_transaction_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecuteTransactionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTE_TRANSACTION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ExecuteTransactionIxData.try_to_vec()?,
    })
}
pub fn execute_transaction_ix(
    keys: ExecuteTransactionKeys,
) -> std::io::Result<Instruction> {
    execute_transaction_ix_with_program_id(crate::ID, keys)
}
pub fn execute_transaction_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteTransactionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ExecuteTransactionKeys = accounts.into();
    let ix = execute_transaction_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn execute_transaction_invoke(
    accounts: ExecuteTransactionAccounts<'_, '_>,
) -> ProgramResult {
    execute_transaction_invoke_with_program_id(crate::ID, accounts)
}
pub fn execute_transaction_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteTransactionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecuteTransactionKeys = accounts.into();
    let ix = execute_transaction_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn execute_transaction_invoke_signed(
    accounts: ExecuteTransactionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    execute_transaction_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn execute_transaction_verify_account_keys(
    accounts: ExecuteTransactionAccounts<'_, '_>,
    keys: ExecuteTransactionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.smart_wallet.key, keys.smart_wallet),
        (*accounts.transaction.key, keys.transaction),
        (*accounts.owner.key, keys.owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn execute_transaction_verify_writable_privileges<'me, 'info>(
    accounts: ExecuteTransactionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.transaction] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn execute_transaction_verify_signer_privileges<'me, 'info>(
    accounts: ExecuteTransactionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn execute_transaction_verify_account_privileges<'me, 'info>(
    accounts: ExecuteTransactionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    execute_transaction_verify_writable_privileges(accounts)?;
    execute_transaction_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTE_TRANSACTION_DERIVED_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ExecuteTransactionDerivedAccounts<'me, 'info> {
    pub smart_wallet: &'me AccountInfo<'info>,
    pub transaction: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecuteTransactionDerivedKeys {
    pub smart_wallet: Pubkey,
    pub transaction: Pubkey,
    pub owner: Pubkey,
}
impl From<ExecuteTransactionDerivedAccounts<'_, '_>> for ExecuteTransactionDerivedKeys {
    fn from(accounts: ExecuteTransactionDerivedAccounts) -> Self {
        Self {
            smart_wallet: *accounts.smart_wallet.key,
            transaction: *accounts.transaction.key,
            owner: *accounts.owner.key,
        }
    }
}
impl From<ExecuteTransactionDerivedKeys>
for [AccountMeta; EXECUTE_TRANSACTION_DERIVED_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecuteTransactionDerivedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.transaction,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; EXECUTE_TRANSACTION_DERIVED_IX_ACCOUNTS_LEN]>
for ExecuteTransactionDerivedKeys {
    fn from(pubkeys: [Pubkey; EXECUTE_TRANSACTION_DERIVED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            smart_wallet: pubkeys[0],
            transaction: pubkeys[1],
            owner: pubkeys[2],
        }
    }
}
impl<'info> From<ExecuteTransactionDerivedAccounts<'_, 'info>>
for [AccountInfo<'info>; EXECUTE_TRANSACTION_DERIVED_IX_ACCOUNTS_LEN] {
    fn from(accounts: ExecuteTransactionDerivedAccounts<'_, 'info>) -> Self {
        [
            accounts.smart_wallet.clone(),
            accounts.transaction.clone(),
            accounts.owner.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; EXECUTE_TRANSACTION_DERIVED_IX_ACCOUNTS_LEN]>
for ExecuteTransactionDerivedAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; EXECUTE_TRANSACTION_DERIVED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            smart_wallet: &arr[0],
            transaction: &arr[1],
            owner: &arr[2],
        }
    }
}
pub const EXECUTE_TRANSACTION_DERIVED_IX_DISCM: [u8; 8] = [
    121,
    1,
    232,
    181,
    156,
    185,
    93,
    88,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExecuteTransactionDerivedIxArgs {
    pub index: u64,
    pub bump: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteTransactionDerivedIxData(pub ExecuteTransactionDerivedIxArgs);
impl From<ExecuteTransactionDerivedIxArgs> for ExecuteTransactionDerivedIxData {
    fn from(args: ExecuteTransactionDerivedIxArgs) -> Self {
        Self(args)
    }
}
impl ExecuteTransactionDerivedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_TRANSACTION_DERIVED_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        EXECUTE_TRANSACTION_DERIVED_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ExecuteTransactionDerivedIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_TRANSACTION_DERIVED_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn execute_transaction_derived_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecuteTransactionDerivedKeys,
    args: ExecuteTransactionDerivedIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTE_TRANSACTION_DERIVED_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExecuteTransactionDerivedIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn execute_transaction_derived_ix(
    keys: ExecuteTransactionDerivedKeys,
    args: ExecuteTransactionDerivedIxArgs,
) -> std::io::Result<Instruction> {
    execute_transaction_derived_ix_with_program_id(crate::ID, keys, args)
}
pub fn execute_transaction_derived_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteTransactionDerivedAccounts<'_, '_>,
    args: ExecuteTransactionDerivedIxArgs,
) -> ProgramResult {
    let keys: ExecuteTransactionDerivedKeys = accounts.into();
    let ix = execute_transaction_derived_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn execute_transaction_derived_invoke(
    accounts: ExecuteTransactionDerivedAccounts<'_, '_>,
    args: ExecuteTransactionDerivedIxArgs,
) -> ProgramResult {
    execute_transaction_derived_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn execute_transaction_derived_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteTransactionDerivedAccounts<'_, '_>,
    args: ExecuteTransactionDerivedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecuteTransactionDerivedKeys = accounts.into();
    let ix = execute_transaction_derived_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn execute_transaction_derived_invoke_signed(
    accounts: ExecuteTransactionDerivedAccounts<'_, '_>,
    args: ExecuteTransactionDerivedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    execute_transaction_derived_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn execute_transaction_derived_verify_account_keys(
    accounts: ExecuteTransactionDerivedAccounts<'_, '_>,
    keys: ExecuteTransactionDerivedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.smart_wallet.key, keys.smart_wallet),
        (*accounts.transaction.key, keys.transaction),
        (*accounts.owner.key, keys.owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn execute_transaction_derived_verify_writable_privileges<'me, 'info>(
    accounts: ExecuteTransactionDerivedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.transaction] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn execute_transaction_derived_verify_signer_privileges<'me, 'info>(
    accounts: ExecuteTransactionDerivedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn execute_transaction_derived_verify_account_privileges<'me, 'info>(
    accounts: ExecuteTransactionDerivedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    execute_transaction_derived_verify_writable_privileges(accounts)?;
    execute_transaction_derived_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OWNER_INVOKE_INSTRUCTION_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct OwnerInvokeInstructionAccounts<'me, 'info> {
    pub smart_wallet: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OwnerInvokeInstructionKeys {
    pub smart_wallet: Pubkey,
    pub owner: Pubkey,
}
impl From<OwnerInvokeInstructionAccounts<'_, '_>> for OwnerInvokeInstructionKeys {
    fn from(accounts: OwnerInvokeInstructionAccounts) -> Self {
        Self {
            smart_wallet: *accounts.smart_wallet.key,
            owner: *accounts.owner.key,
        }
    }
}
impl From<OwnerInvokeInstructionKeys>
for [AccountMeta; OWNER_INVOKE_INSTRUCTION_IX_ACCOUNTS_LEN] {
    fn from(keys: OwnerInvokeInstructionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; OWNER_INVOKE_INSTRUCTION_IX_ACCOUNTS_LEN]>
for OwnerInvokeInstructionKeys {
    fn from(pubkeys: [Pubkey; OWNER_INVOKE_INSTRUCTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            smart_wallet: pubkeys[0],
            owner: pubkeys[1],
        }
    }
}
impl<'info> From<OwnerInvokeInstructionAccounts<'_, 'info>>
for [AccountInfo<'info>; OWNER_INVOKE_INSTRUCTION_IX_ACCOUNTS_LEN] {
    fn from(accounts: OwnerInvokeInstructionAccounts<'_, 'info>) -> Self {
        [accounts.smart_wallet.clone(), accounts.owner.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; OWNER_INVOKE_INSTRUCTION_IX_ACCOUNTS_LEN]>
for OwnerInvokeInstructionAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OWNER_INVOKE_INSTRUCTION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            smart_wallet: &arr[0],
            owner: &arr[1],
        }
    }
}
pub const OWNER_INVOKE_INSTRUCTION_IX_DISCM: [u8; 8] = [
    204,
    35,
    69,
    185,
    159,
    100,
    140,
    165,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OwnerInvokeInstructionIxArgs {
    pub index: u64,
    pub bump: u8,
    pub ix: TXInstruction,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OwnerInvokeInstructionIxData(pub OwnerInvokeInstructionIxArgs);
impl From<OwnerInvokeInstructionIxArgs> for OwnerInvokeInstructionIxData {
    fn from(args: OwnerInvokeInstructionIxArgs) -> Self {
        Self(args)
    }
}
impl OwnerInvokeInstructionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OWNER_INVOKE_INSTRUCTION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        OWNER_INVOKE_INSTRUCTION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(OwnerInvokeInstructionIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OWNER_INVOKE_INSTRUCTION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn owner_invoke_instruction_ix_with_program_id(
    program_id: Pubkey,
    keys: OwnerInvokeInstructionKeys,
    args: OwnerInvokeInstructionIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OWNER_INVOKE_INSTRUCTION_IX_ACCOUNTS_LEN] = keys.into();
    let data: OwnerInvokeInstructionIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn owner_invoke_instruction_ix(
    keys: OwnerInvokeInstructionKeys,
    args: OwnerInvokeInstructionIxArgs,
) -> std::io::Result<Instruction> {
    owner_invoke_instruction_ix_with_program_id(crate::ID, keys, args)
}
pub fn owner_invoke_instruction_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OwnerInvokeInstructionAccounts<'_, '_>,
    args: OwnerInvokeInstructionIxArgs,
) -> ProgramResult {
    let keys: OwnerInvokeInstructionKeys = accounts.into();
    let ix = owner_invoke_instruction_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn owner_invoke_instruction_invoke(
    accounts: OwnerInvokeInstructionAccounts<'_, '_>,
    args: OwnerInvokeInstructionIxArgs,
) -> ProgramResult {
    owner_invoke_instruction_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn owner_invoke_instruction_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OwnerInvokeInstructionAccounts<'_, '_>,
    args: OwnerInvokeInstructionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OwnerInvokeInstructionKeys = accounts.into();
    let ix = owner_invoke_instruction_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn owner_invoke_instruction_invoke_signed(
    accounts: OwnerInvokeInstructionAccounts<'_, '_>,
    args: OwnerInvokeInstructionIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    owner_invoke_instruction_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn owner_invoke_instruction_verify_account_keys(
    accounts: OwnerInvokeInstructionAccounts<'_, '_>,
    keys: OwnerInvokeInstructionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.smart_wallet.key, keys.smart_wallet),
        (*accounts.owner.key, keys.owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn owner_invoke_instruction_verify_signer_privileges<'me, 'info>(
    accounts: OwnerInvokeInstructionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn owner_invoke_instruction_verify_account_privileges<'me, 'info>(
    accounts: OwnerInvokeInstructionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    owner_invoke_instruction_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const OWNER_INVOKE_INSTRUCTION_V2_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct OwnerInvokeInstructionV2Accounts<'me, 'info> {
    pub smart_wallet: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OwnerInvokeInstructionV2Keys {
    pub smart_wallet: Pubkey,
    pub owner: Pubkey,
}
impl From<OwnerInvokeInstructionV2Accounts<'_, '_>> for OwnerInvokeInstructionV2Keys {
    fn from(accounts: OwnerInvokeInstructionV2Accounts) -> Self {
        Self {
            smart_wallet: *accounts.smart_wallet.key,
            owner: *accounts.owner.key,
        }
    }
}
impl From<OwnerInvokeInstructionV2Keys>
for [AccountMeta; OWNER_INVOKE_INSTRUCTION_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: OwnerInvokeInstructionV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.smart_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; OWNER_INVOKE_INSTRUCTION_V2_IX_ACCOUNTS_LEN]>
for OwnerInvokeInstructionV2Keys {
    fn from(pubkeys: [Pubkey; OWNER_INVOKE_INSTRUCTION_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            smart_wallet: pubkeys[0],
            owner: pubkeys[1],
        }
    }
}
impl<'info> From<OwnerInvokeInstructionV2Accounts<'_, 'info>>
for [AccountInfo<'info>; OWNER_INVOKE_INSTRUCTION_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: OwnerInvokeInstructionV2Accounts<'_, 'info>) -> Self {
        [accounts.smart_wallet.clone(), accounts.owner.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; OWNER_INVOKE_INSTRUCTION_V2_IX_ACCOUNTS_LEN]>
for OwnerInvokeInstructionV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; OWNER_INVOKE_INSTRUCTION_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            smart_wallet: &arr[0],
            owner: &arr[1],
        }
    }
}
pub const OWNER_INVOKE_INSTRUCTION_V2_IX_DISCM: [u8; 8] = [
    169,
    161,
    80,
    52,
    188,
    19,
    232,
    97,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OwnerInvokeInstructionV2IxArgs {
    pub index: u64,
    pub bump: u8,
    pub invoker: Pubkey,
    pub data: bytes,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OwnerInvokeInstructionV2IxData(pub OwnerInvokeInstructionV2IxArgs);
impl From<OwnerInvokeInstructionV2IxArgs> for OwnerInvokeInstructionV2IxData {
    fn from(args: OwnerInvokeInstructionV2IxArgs) -> Self {
        Self(args)
    }
}
impl OwnerInvokeInstructionV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OWNER_INVOKE_INSTRUCTION_V2_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        OWNER_INVOKE_INSTRUCTION_V2_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(OwnerInvokeInstructionV2IxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OWNER_INVOKE_INSTRUCTION_V2_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn owner_invoke_instruction_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: OwnerInvokeInstructionV2Keys,
    args: OwnerInvokeInstructionV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OWNER_INVOKE_INSTRUCTION_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: OwnerInvokeInstructionV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn owner_invoke_instruction_v2_ix(
    keys: OwnerInvokeInstructionV2Keys,
    args: OwnerInvokeInstructionV2IxArgs,
) -> std::io::Result<Instruction> {
    owner_invoke_instruction_v2_ix_with_program_id(crate::ID, keys, args)
}
pub fn owner_invoke_instruction_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OwnerInvokeInstructionV2Accounts<'_, '_>,
    args: OwnerInvokeInstructionV2IxArgs,
) -> ProgramResult {
    let keys: OwnerInvokeInstructionV2Keys = accounts.into();
    let ix = owner_invoke_instruction_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn owner_invoke_instruction_v2_invoke(
    accounts: OwnerInvokeInstructionV2Accounts<'_, '_>,
    args: OwnerInvokeInstructionV2IxArgs,
) -> ProgramResult {
    owner_invoke_instruction_v2_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn owner_invoke_instruction_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OwnerInvokeInstructionV2Accounts<'_, '_>,
    args: OwnerInvokeInstructionV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OwnerInvokeInstructionV2Keys = accounts.into();
    let ix = owner_invoke_instruction_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn owner_invoke_instruction_v2_invoke_signed(
    accounts: OwnerInvokeInstructionV2Accounts<'_, '_>,
    args: OwnerInvokeInstructionV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    owner_invoke_instruction_v2_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn owner_invoke_instruction_v2_verify_account_keys(
    accounts: OwnerInvokeInstructionV2Accounts<'_, '_>,
    keys: OwnerInvokeInstructionV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.smart_wallet.key, keys.smart_wallet),
        (*accounts.owner.key, keys.owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn owner_invoke_instruction_v2_verify_signer_privileges<'me, 'info>(
    accounts: OwnerInvokeInstructionV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn owner_invoke_instruction_v2_verify_account_privileges<'me, 'info>(
    accounts: OwnerInvokeInstructionV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    owner_invoke_instruction_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_SUBACCOUNT_INFO_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CreateSubaccountInfoAccounts<'me, 'info> {
    pub subaccount_info: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateSubaccountInfoKeys {
    pub subaccount_info: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateSubaccountInfoAccounts<'_, '_>> for CreateSubaccountInfoKeys {
    fn from(accounts: CreateSubaccountInfoAccounts) -> Self {
        Self {
            subaccount_info: *accounts.subaccount_info.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateSubaccountInfoKeys>
for [AccountMeta; CREATE_SUBACCOUNT_INFO_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateSubaccountInfoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.subaccount_info,
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
impl From<[Pubkey; CREATE_SUBACCOUNT_INFO_IX_ACCOUNTS_LEN]>
for CreateSubaccountInfoKeys {
    fn from(pubkeys: [Pubkey; CREATE_SUBACCOUNT_INFO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            subaccount_info: pubkeys[0],
            payer: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CreateSubaccountInfoAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_SUBACCOUNT_INFO_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateSubaccountInfoAccounts<'_, 'info>) -> Self {
        [
            accounts.subaccount_info.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_SUBACCOUNT_INFO_IX_ACCOUNTS_LEN]>
for CreateSubaccountInfoAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; CREATE_SUBACCOUNT_INFO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            subaccount_info: &arr[0],
            payer: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CREATE_SUBACCOUNT_INFO_IX_DISCM: [u8; 8] = [196, 132, 49, 16, 91, 57, 67, 139];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateSubaccountInfoIxArgs {
    pub subaccount: Pubkey,
    pub smart_wallet: Pubkey,
    pub index: u64,
    pub subaccount_type: SubaccountType,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateSubaccountInfoIxData(pub CreateSubaccountInfoIxArgs);
impl From<CreateSubaccountInfoIxArgs> for CreateSubaccountInfoIxData {
    fn from(args: CreateSubaccountInfoIxArgs) -> Self {
        Self(args)
    }
}
impl CreateSubaccountInfoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_SUBACCOUNT_INFO_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_SUBACCOUNT_INFO_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateSubaccountInfoIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_SUBACCOUNT_INFO_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_subaccount_info_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateSubaccountInfoKeys,
    args: CreateSubaccountInfoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_SUBACCOUNT_INFO_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateSubaccountInfoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_subaccount_info_ix(
    keys: CreateSubaccountInfoKeys,
    args: CreateSubaccountInfoIxArgs,
) -> std::io::Result<Instruction> {
    create_subaccount_info_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_subaccount_info_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateSubaccountInfoAccounts<'_, '_>,
    args: CreateSubaccountInfoIxArgs,
) -> ProgramResult {
    let keys: CreateSubaccountInfoKeys = accounts.into();
    let ix = create_subaccount_info_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_subaccount_info_invoke(
    accounts: CreateSubaccountInfoAccounts<'_, '_>,
    args: CreateSubaccountInfoIxArgs,
) -> ProgramResult {
    create_subaccount_info_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_subaccount_info_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateSubaccountInfoAccounts<'_, '_>,
    args: CreateSubaccountInfoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateSubaccountInfoKeys = accounts.into();
    let ix = create_subaccount_info_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_subaccount_info_invoke_signed(
    accounts: CreateSubaccountInfoAccounts<'_, '_>,
    args: CreateSubaccountInfoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_subaccount_info_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn create_subaccount_info_verify_account_keys(
    accounts: CreateSubaccountInfoAccounts<'_, '_>,
    keys: CreateSubaccountInfoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.subaccount_info.key, keys.subaccount_info),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_subaccount_info_verify_writable_privileges<'me, 'info>(
    accounts: CreateSubaccountInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.subaccount_info, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_subaccount_info_verify_signer_privileges<'me, 'info>(
    accounts: CreateSubaccountInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_subaccount_info_verify_account_privileges<'me, 'info>(
    accounts: CreateSubaccountInfoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_subaccount_info_verify_writable_privileges(accounts)?;
    create_subaccount_info_verify_signer_privileges(accounts)?;
    Ok(())
}
