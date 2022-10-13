import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { assert } from "chai";
import { OnchainVotingAnchorProgram } from "../target/types/onchain_voting_anchor_program";

describe("onchain_voting_anchor_program", () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace
		.OnchainVotingAnchorProgram as Program<OnchainVotingAnchorProgram>;

	let voteBank = anchor.web3.Keypair.generate();

	it("Creating proposal!", async () => {
		const tx = await program.methods
			.initialize("Proposal description")
			.accounts({
				voteAccount: voteBank.publicKey,
			})
			.signers([voteBank])
			.rpc();
		let voteBankData = await program.account.proposal.fetch(voteBank.publicKey);

		assert.equal(voteBankData.isOpenToVote, true);
	});

	it("Voting for yes.", async () => {
		const tx = await program.methods
			.giveVote({ yes: {} })
			.accounts({
				voteAccount: voteBank.publicKey,
			})
			.rpc();

		let voteBankData = await program.account.proposal.fetch(voteBank.publicKey);

		assert.equal(voteBankData.yes.toNumber(), 1);
	});

	it("Voting for no.", async () => {
		const tx = await program.methods
			.giveVote({ no: {} })
			.accounts({
				voteAccount: voteBank.publicKey,
			})
			.rpc();

		let voteBankData = await program.account.proposal.fetch(voteBank.publicKey);

		assert.equal(voteBankData.no.toNumber(), 1);
	});
});
