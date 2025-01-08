import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet, web3 } from "@coral-xyz/anchor";
import { Govern, IDL as GovernIDL } from "../../target/types/govern";
import {
	MerkleDistributor,
	IDL as MerkleDistributorIDL,
} from "../../target/types/merkle_distributor";
import { SmartWallet, IDL as SmartWalletIDL } from "../../target/types/smart_wallet";
import { MetVoter, IDL as MetVoterIDL } from "../../target/types/met_voter";
import { LockedVoter, IDL as LockedVoterIDL } from "../../target/types/locked_voter";

export function createMerkleDistributorProgram(wallet: Wallet, programId: web3.PublicKey) {
	const provider = new anchor.AnchorProvider(
		anchor.AnchorProvider.env().connection,
		wallet,
		anchor.AnchorProvider.defaultOptions()
	);
	const program = new Program<MerkleDistributor>(MerkleDistributorIDL, programId, provider);

	return program;
}

export function createSmartWalletProgram(wallet: Wallet, programId: web3.PublicKey) {
	const provider = new anchor.AnchorProvider(
		anchor.AnchorProvider.env().connection,
		wallet,
		anchor.AnchorProvider.defaultOptions()
	);
	const program = new Program<SmartWallet>(SmartWalletIDL, programId, provider);

	return program;
}

export function createGovernProgram(wallet: Wallet, programId: web3.PublicKey) {
	const provider = new anchor.AnchorProvider(
		anchor.AnchorProvider.env().connection,
		wallet,
		anchor.AnchorProvider.defaultOptions()
	);
	const program = new Program<Govern>(GovernIDL, programId, provider);

	return program;
}

export function createMetVoterProgram(wallet: Wallet, programId: web3.PublicKey) {
	const provider = new anchor.AnchorProvider(
		anchor.AnchorProvider.env().connection,
		wallet,
		anchor.AnchorProvider.defaultOptions()
	);
	const program = new Program<MetVoter>(MetVoterIDL, programId, provider);

	return program;
}

export function createLockedVoterProgram(wallet: Wallet, programId: web3.PublicKey) {
	const provider = new anchor.AnchorProvider(
		anchor.AnchorProvider.env().connection,
		wallet,
		anchor.AnchorProvider.defaultOptions()
	);
	const program = new Program<LockedVoter>(LockedVoterIDL, programId, provider);

	return program;
}

export const MERKLE_DISTRIBUTOR_PROGRAM_ID = new web3.PublicKey(
	"MRKgRBL5XCCT5rwUGnim4yioq9wR4c6rj2EZkw8KdyZ"
);

export const GOVERN_PROGRAM_ID = new web3.PublicKey("HWKB16ucqqFRZ5W768FA58fVuqJLGw6v9YdGtmfBUAgj");

export const MET_VOTER_PROGRAM_ID = new web3.PublicKey(
	"voteXZxajNhmCGpqzBhVArCANMKra5nwqtaaLA6v9CX"
);

export const LOCKED_VOTER_PROGRAM_ID = new web3.PublicKey(
	"GJ3DSRYMLRoqjVgtj1JwdFXQU4FNzvgjGUfYyE4u1JMa"
);

export const SMART_WALLET_PROGRAM_ID = new web3.PublicKey(
	"3dQhmbCccZTtU1KYqW13XCD5CDKgw4s6NoiGLqPwv4QF"
);
