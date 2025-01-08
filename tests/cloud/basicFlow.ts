import * as anchor from "@coral-xyz/anchor";
import { BN, Wallet, web3 } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID, createMint, mintTo } from "@solana/spl-token";
import {
	GOVERN_PROGRAM_ID,
	IProposalInstruction,
	SMART_WALLET_PROGRAM_ID,
	LOCKED_VOTER_PROGRAM_ID,
	VoteSide,
	createAndFundWallet,
	createGovernProgram,
	createGovernor,
	createProposal,
	createProposalMeta,
	createSmartWallet,
	createSmartWalletProgram,
	createLockedVoterProgram,
	deriveEscrow,
	deriveGovern,
	deriveLocker,
	deriveSmartWallet,
	deriveVote,
	getOnChainTime,
	getOrCreateATA,
	getOrCreateVote,
	invokeAndAssertError,
	sleep,
	deriveTransaction,
} from "../utils";
import { assert, expect } from "chai";

const provider = anchor.AnchorProvider.env();

const userCount = 3;
const lockAmount = new BN(100);

describe("Uses (mainly) locked voter to simulate a real world use case of $CLOUD staking", () => {
	let locker: web3.PublicKey;
	let govern: web3.PublicKey;
	let smartWallet: web3.PublicKey;

	let base_wallet: Wallet;
	let base_keypair: web3.Keypair;

	let cloudMint: web3.PublicKey;
	let stakerKeypairs: web3.Keypair[] = [];

	// Smart wallet config
	let smartWalletOwners: web3.PublicKey[] = [];
	let smartWalletThreshold: BN = new BN(1);

	// Govern config
	const votingPeriod: BN = new BN(5); // 10 seconds duration of voting on proposal
	const quorumVotes: BN = new BN(2); // 2 vote to pass

	// Voter config
	const maxStakeDuration: BN = new BN(10); // 10 seconds
	const minStakeDuration: BN = new BN(10); // 10 seconds
	const maxStakeVoteMultiplier: number = 1;
	const proposalActivationMinVotes: BN = new BN(2); // min 2 vote to activate proposal

	async function initializeLocker() {
		const voterProgram = createLockedVoterProgram(base_wallet, LOCKED_VOTER_PROGRAM_ID);
		await voterProgram.methods
			.newLocker({
				maxStakeDuration,
				maxStakeVoteMultiplier,
				minStakeDuration,
				proposalActivationMinVotes,
			})
			.accounts({
				base: base_keypair.publicKey,
				locker,
				tokenMint: cloudMint,
				governor: govern,
				payer: voterProgram.provider.publicKey,
				systemProgram: web3.SystemProgram.programId,
			})
			.rpc();
	}

	// This is mostly what we will do after program deploy once
	before(async () => {
		const result = await createAndFundWallet(provider.connection);
		base_keypair = result.keypair;
		base_wallet = result.wallet;

		const [lockerPda, _lBump] = deriveLocker(base_keypair.publicKey, LOCKED_VOTER_PROGRAM_ID);
		locker = lockerPda;

		const [governPda, _gBump] = deriveGovern(base_keypair.publicKey);
		govern = governPda;

		const [smartWalletPda, _sBump] = deriveSmartWallet(base_keypair.publicKey);
		smartWallet = smartWalletPda;

		smartWalletOwners.push(governPda);
		smartWalletOwners.push(base_wallet.publicKey);

		await createSmartWallet(
			smartWalletOwners,
			smartWalletOwners.length,
			new BN(0),
			smartWalletThreshold,
			base_keypair,
			createSmartWalletProgram(base_wallet, SMART_WALLET_PROGRAM_ID)
		);

		await createGovernor(
			new BN(0),
			votingPeriod,
			quorumVotes,
			new BN(0),
			base_keypair,
			smartWallet,
			createGovernProgram(base_wallet, GOVERN_PROGRAM_ID),
			LOCKED_VOTER_PROGRAM_ID
		);

		cloudMint = await createMint(
			provider.connection,
			base_keypair,
			base_keypair.publicKey,
			null,
			9
		);

		// Give each user $CLOUD
		for (let i = 0; i < userCount; i++) {
			const result = await createAndFundWallet(provider.connection);
			stakerKeypairs.push(result.keypair);

			const userATA = await getOrCreateATA(
				cloudMint,
				result.keypair.publicKey,
				result.keypair,
				provider.connection
			);

			await mintTo(
				provider.connection,
				base_keypair,
				cloudMint,
				userATA,
				base_keypair.publicKey,
				lockAmount.toNumber() * 2
			);
		}

		await initializeLocker();
	});

	// Multiple user stake-unstake-cancel-unstake-withdraw scenarios
	it("goes through the happy path", async () => {
		// Initialize escrows for our users and let them stake the first time
		for (const stakerKeypair of stakerKeypairs) {
			const stakerWallet = new Wallet(stakerKeypair);
			const voterProgram = createLockedVoterProgram(stakerWallet, LOCKED_VOTER_PROGRAM_ID);
			const [escrow, _bump] = deriveEscrow(locker, stakerWallet.publicKey, LOCKED_VOTER_PROGRAM_ID);

			// Escrow is what each user starts to have on first stake
			await voterProgram.methods
				.newEscrow()
				.accounts({
					escrow,
					escrowOwner: stakerWallet.publicKey,
					locker,
					payer: stakerWallet.publicKey,
					systemProgram: web3.SystemProgram.programId,
				})
				.rpc();

			// On the frontend probably just add create ata idempotent ix to the tx if user is new?
			const escrowATA = await getOrCreateATA(cloudMint, escrow, stakerKeypair, provider.connection);

			// Cloud ATA
			const userATA = await getOrCreateATA(
				cloudMint,
				stakerWallet.publicKey,
				stakerKeypair,
				provider.connection
			);

			let escrowAccountBefore = await voterProgram.account.escrow.fetch(escrow);

			// Escrow amount in the beginning must be 0
			assert(escrowAccountBefore.amount.toNumber() === 0, "Escrow initialized with amount > 0");
			assert(escrowAccountBefore.isMaxLock === true, "Escrow isn't max locked by default");

			await voterProgram.methods
				.increaseLockedAmount(lockAmount)
				.accounts({
					escrow,
					escrowTokens: escrowATA,
					locker,
					payer: voterProgram.provider.publicKey,
					sourceTokens: userATA,
					tokenProgram: TOKEN_PROGRAM_ID,
				})
				.rpc();

			let escrowAccountAfter = await voterProgram.account.escrow.fetch(escrow);
			const userATABalanceAfter = await provider.connection
				.getTokenAccountBalance(userATA)
				.then((b) => b.value.amount);
			const escrowATABalanceAfter = await provider.connection
				.getTokenAccountBalance(userATA)
				.then((b) => b.value.amount);

			// Set escrow data to staked amount
			assert(
				escrowAccountAfter.amount.toNumber() === lockAmount.toNumber(),
				"Escrow amount incorrect after staking"
			);
			// Check if users have less tokens now
			assert(userATABalanceAfter === String(lockAmount.toNumber()));
			// Check if escrow ata has more tokens now
			assert(escrowATABalanceAfter === String(lockAmount.toNumber()));
		}

		// Let users stake once more each (same process testing multiple staking attempts)
		for (const stakerKeypair of stakerKeypairs) {
			const stakerWallet = new Wallet(stakerKeypair);
			const voterProgram = createLockedVoterProgram(stakerWallet, LOCKED_VOTER_PROGRAM_ID);
			const [escrow, _bump] = deriveEscrow(locker, stakerWallet.publicKey, LOCKED_VOTER_PROGRAM_ID);

			const escrowATA = await getOrCreateATA(cloudMint, escrow, stakerKeypair, provider.connection);

			const userATA = await getOrCreateATA(
				cloudMint,
				stakerWallet.publicKey,
				stakerKeypair,
				provider.connection
			);

			let escrowAccountBefore = await voterProgram.account.escrow.fetch(escrow);

			await voterProgram.methods
				.increaseLockedAmount(lockAmount)
				.accounts({
					escrow,
					escrowTokens: escrowATA,
					locker,
					payer: voterProgram.provider.publicKey,
					sourceTokens: userATA,
					tokenProgram: TOKEN_PROGRAM_ID,
				})
				.rpc();

			let escrowAccountAfter = await voterProgram.account.escrow.fetch(escrow);
			assert(
				escrowAccountBefore.amount.toNumber() + lockAmount.toNumber() ===
					escrowAccountAfter.amount.toNumber(),
				"Escrow amount incorrect after staking"
			);
		}

		// Each user will unstake, cancel unstake, unstake again, claim unstake after period
		for (const stakerKeypair of stakerKeypairs) {
			const stakerWallet = new Wallet(stakerKeypair);
			const voterProgram = createLockedVoterProgram(stakerWallet, LOCKED_VOTER_PROGRAM_ID);
			const [escrow, _bump] = deriveEscrow(locker, stakerWallet.publicKey, LOCKED_VOTER_PROGRAM_ID);
			// We actually care about this keypair so we can later cancel/claim
			// i guess we can memcmp and use escrow and memo fields from acc data
			const partialUnstakeKP = web3.Keypair.generate();

			// This will set up partial unstake amount for the user (or full) with a delay of 30 days (read from our locker params)
			await voterProgram.methods
				//TODO: Maybe add unix timestamp to memo of unstake to have later chronological unstake requests while they're acive (open for suggestions)
				.openPartialUnstaking(lockAmount, "unix-timestamp maybe")
				.accounts({
					locker,
					escrow,
					partialUnstake: partialUnstakeKP.publicKey,
					owner: stakerWallet.publicKey,
					systemProgram: web3.SystemProgram.programId,
				})
				.signers([partialUnstakeKP, stakerKeypair])
				.rpc();

			let partialUnstakingState = await voterProgram.account.partialUnstaking.fetch(
				partialUnstakeKP.publicKey
			);

			let escrowStateAfterUnstake = await voterProgram.account.escrow.fetch(escrow);

			assert(partialUnstakingState.amount.toNumber() === lockAmount.toNumber());
			assert(partialUnstakingState.escrow.toString() === escrow.toString());
			// Since it's a partial unstake, the escrow amount should be the half (200-100)=100
			assert(escrowStateAfterUnstake.amount.toNumber() === lockAmount.toNumber());
			assert(escrowStateAfterUnstake.partialUnstakingAmount.toNumber() === lockAmount.toNumber());

			await voterProgram.methods
				.mergePartialUnstaking()
				.accounts({
					locker,
					escrow,
					partialUnstake: partialUnstakeKP.publicKey,
					owner: stakerWallet.publicKey,
				})
				.rpc();

			let escrowStateAfterCancel = await voterProgram.account.escrow.fetch(escrow);

			assert(
				escrowStateAfterCancel.amount.toNumber() ===
					escrowStateAfterUnstake.amount.toNumber() + lockAmount.toNumber()
			);

			await voterProgram.methods
				.openPartialUnstaking(lockAmount, "unix-timestamp maybe (2)")
				.accounts({
					locker,
					escrow,
					partialUnstake: partialUnstakeKP.publicKey,
					owner: stakerWallet.publicKey,
					systemProgram: web3.SystemProgram.programId,
				})
				.signers([partialUnstakeKP, stakerKeypair])
				.rpc();

			let escrowStateAfterRecreate = await voterProgram.account.escrow.fetch(escrow);

			let partialUnstakingStateAfterRecreate = await voterProgram.account.partialUnstaking.fetch(
				partialUnstakeKP.publicKey
			);

			assert(escrowStateAfterRecreate.amount.toNumber() === lockAmount.toNumber());
			assert(partialUnstakingStateAfterRecreate.amount.toNumber() === lockAmount.toNumber());

			const escrowATA = await getOrCreateATA(cloudMint, escrow, stakerKeypair, provider.connection);
			const userATA = await getOrCreateATA(
				cloudMint,
				stakerWallet.publicKey,
				stakerKeypair,
				provider.connection
			);

			const escrowATABalanceBefore = await provider.connection
				.getTokenAccountBalance(escrowATA)
				.then((b) => b.value.amount);

			const userATABalanceBefore = await provider.connection
				.getTokenAccountBalance(userATA)
				.then((b) => b.value.amount);

			while (true) {
				const [partialUnstakeStateBeforeWithdraw, onchainTimestamp] = await Promise.all([
					voterProgram.account.partialUnstaking.fetch(partialUnstakeKP.publicKey),
					getOnChainTime(provider.connection),
				]);

				if (partialUnstakeStateBeforeWithdraw.expiration.toNumber() > onchainTimestamp) {
					console.log(
						`${
							partialUnstakeStateBeforeWithdraw.expiration.toNumber() - onchainTimestamp
						} seconds until escrow expire`
					);
					await sleep(1000);
				} else {
					break;
				}
			}

			await voterProgram.methods
				.withdrawPartialUnstaking()
				.accounts({
					locker,
					payer: stakerWallet.publicKey,
					escrow,
					escrowTokens: escrowATA,
					owner: stakerWallet.publicKey,
					partialUnstake: partialUnstakeKP.publicKey,
					destinationTokens: userATA,
					tokenProgram: TOKEN_PROGRAM_ID,
				})
				.rpc();

			let escrowStateAfterClaim = await voterProgram.account.escrow.fetch(escrow);

			const escrowATABalanceAfter = await provider.connection
				.getTokenAccountBalance(userATA)
				.then((b) => b.value.amount);

			const userATABalanceAfter = await provider.connection
				.getTokenAccountBalance(userATA)
				.then((b) => b.value.amount);

			assert(escrowStateAfterClaim.amount.toNumber() === lockAmount.toNumber());
			assert(escrowATABalanceBefore === String(200));
			assert(escrowATABalanceAfter === String(100));
			assert(userATABalanceBefore === String(0));
			assert(userATABalanceAfter === String(100));

			// We don't have to wait for multiple 10 sec intervals i guess
			break;
		}
	});
});
