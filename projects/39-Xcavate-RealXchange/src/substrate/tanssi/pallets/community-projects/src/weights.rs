
//! Autogenerated weights for `pallet_community_projects`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `LAPTOP-DFFNONK6`, CPU: `11th Gen Intel(R) Core(TM) i7-1165G7 @ 2.80GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_community_projects
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallets/community-projects/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

pub trait WeightInfo {
	fn list_project() -> Weight;
	fn buy_nft() -> Weight;
	fn vote_on_milestone() -> Weight;
	fn bond_token() -> Weight;
}

/// Weight functions for `pallet_nft_marketplace`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Whitelist::WhitelistedAccounts` (r:1 w:0)
	/// Proof: `Whitelist::WhitelistedAccounts` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::NextCollectionId` (r:1 w:1)
	/// Proof: `Nfts::NextCollectionId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Collection` (r:1 w:1)
	/// Proof: `Nfts::Collection` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionMetadataOf` (r:1 w:1)
	/// Proof: `Nfts::CollectionMetadataOf` (`max_values`: None, `max_size`: Some(5038), added: 7513, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Item` (r:10 w:10)
	/// Proof: `Nfts::Item` (`max_values`: None, `max_size`: Some(861), added: 3336, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemConfigOf` (r:10 w:10)
	/// Proof: `Nfts::ItemConfigOf` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemMetadataOf` (r:10 w:10)
	/// Proof: `Nfts::ItemMetadataOf` (`max_values`: None, `max_size`: Some(5091), added: 7566, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::ListedNfts` (r:1 w:1)
	/// Proof: `CommunityProject::ListedNfts` (`max_values`: Some(1), `max_size`: Some(800004), added: 800499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::ListedNftsOfCollection` (r:1 w:1)
	/// Proof: `CommunityProject::ListedNftsOfCollection` (`max_values`: None, `max_size`: Some(20022), added: 22497, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::ListedNftTypes` (r:0 w:4)
	/// Proof: `CommunityProject::ListedNftTypes` (`max_values`: None, `max_size`: Some(20039), added: 22514, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::OngoingProjects` (r:0 w:1)
	/// Proof: `CommunityProject::OngoingProjects` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::OngoingNftDetails` (r:0 w:10)
	/// Proof: `CommunityProject::OngoingNftDetails` (`max_values`: None, `max_size`: Some(96), added: 2571, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionRoleOf` (r:0 w:1)
	/// Proof: `Nfts::CollectionRoleOf` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionConfigOf` (r:0 w:1)
	/// Proof: `Nfts::CollectionConfigOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Account` (r:0 w:10)
	/// Proof: `Nfts::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionAccount` (r:0 w:1)
	/// Proof: `Nfts::CollectionAccount` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn list_project() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316`
		//  Estimated: `3201489`
		// Minimum execution time: 636_035_000 picoseconds.
		Weight::from_parts(702_642_000, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			.saturating_add(T::DbWeight::get().reads(37))
			.saturating_add(T::DbWeight::get().writes(64))
	}
	/// Storage: `Whitelist::WhitelistedAccounts` (r:1 w:0)
	/// Proof: `Whitelist::WhitelistedAccounts` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::OngoingProjects` (r:1 w:1)
	/// Proof: `CommunityProject::OngoingProjects` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::ListedNftTypes` (r:4 w:4)
	/// Proof: `CommunityProject::ListedNftTypes` (`max_values`: None, `max_size`: Some(20039), added: 22514, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::OngoingNftDetails` (r:1 w:1)
	/// Proof: `CommunityProject::OngoingNftDetails` (`max_values`: None, `max_size`: Some(96), added: 2571, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Collection` (r:1 w:1)
	/// Proof: `Nfts::Collection` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Attribute` (r:400 w:0)
	/// Proof: `Nfts::Attribute` (`max_values`: None, `max_size`: Some(446), added: 2921, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionConfigOf` (r:1 w:0)
	/// Proof: `Nfts::CollectionConfigOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemConfigOf` (r:400 w:399)
	/// Proof: `Nfts::ItemConfigOf` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Item` (r:400 w:400)
	/// Proof: `Nfts::Item` (`max_values`: None, `max_size`: Some(861), added: 3336, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::ListedNftsOfCollection` (r:1 w:1)
	/// Proof: `CommunityProject::ListedNftsOfCollection` (`max_values`: None, `max_size`: Some(20022), added: 22497, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::ListedNfts` (r:1 w:1)
	/// Proof: `CommunityProject::ListedNfts` (`max_values`: Some(1), `max_size`: Some(800004), added: 800499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::NftHolder` (r:1 w:1)
	/// Proof: `CommunityProject::NftHolder` (`max_values`: None, `max_size`: Some(160022), added: 162497, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::VotingPower` (r:1 w:1)
	/// Proof: `CommunityProject::VotingPower` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemMetadataOf` (r:399 w:399)
	/// Proof: `Nfts::ItemMetadataOf` (`max_values`: None, `max_size`: Some(5091), added: 7566, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::MilestonePeriodExpiring` (r:1 w:1)
	/// Proof: `CommunityProject::MilestonePeriodExpiring` (`max_values`: None, `max_size`: Some(4022), added: 6497, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Account` (r:0 w:401)
	/// Proof: `Nfts::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemPriceOf` (r:0 w:400)
	/// Proof: `Nfts::ItemPriceOf` (`max_values`: None, `max_size`: Some(89), added: 2564, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemAttributesApprovalsOf` (r:0 w:399)
	/// Proof: `Nfts::ItemAttributesApprovalsOf` (`max_values`: None, `max_size`: Some(681), added: 3156, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::PendingSwapOf` (r:0 w:400)
	/// Proof: `Nfts::PendingSwapOf` (`max_values`: None, `max_size`: Some(71), added: 2546, mode: `MaxEncodedLen`)
	fn buy_nft() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2078850`
		//  Estimated: `3201489`
		// Minimum execution time: 18_939_769_000 picoseconds.
		Weight::from_parts(20_128_719_000, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			.saturating_add(T::DbWeight::get().reads(1617))
			.saturating_add(T::DbWeight::get().writes(2814))
	}
	/// Storage: `Whitelist::WhitelistedAccounts` (r:1 w:0)
	/// Proof: `Whitelist::WhitelistedAccounts` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::OngoingProjects` (r:1 w:1)
	/// Proof: `CommunityProject::OngoingProjects` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::TotalBonded` (r:1 w:1)
	/// Proof: `CommunityProject::TotalBonded` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::TokenBonder` (r:1 w:1)
	/// Proof: `CommunityProject::TokenBonder` (`max_values`: None, `max_size`: Some(160022), added: 162497, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::ProjectBonding` (r:1 w:1)
	/// Proof: `CommunityProject::ProjectBonding` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(849), added: 3324, mode: `MaxEncodedLen`)
	fn bond_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `791`
		//  Estimated: `3201489`
		// Minimum execution time: 69_656_000 picoseconds.
		Weight::from_parts(81_103_000, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Whitelist::WhitelistedAccounts` (r:1 w:0)
	/// Proof: `Whitelist::WhitelistedAccounts` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::OngoingVotes` (r:1 w:1)
	/// Proof: `CommunityProject::OngoingVotes` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::NftHolder` (r:1 w:0)
	/// Proof: `CommunityProject::NftHolder` (`max_values`: None, `max_size`: Some(160022), added: 162497, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::VotedUser` (r:1 w:1)
	/// Proof: `CommunityProject::VotedUser` (`max_values`: None, `max_size`: Some(160022), added: 162497, mode: `MaxEncodedLen`)
	/// Storage: `CommunityProject::VotingPower` (r:1 w:0)
	/// Proof: `CommunityProject::VotingPower` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	fn vote_on_milestone() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `533`
		//  Estimated: `3201489`
		// Minimum execution time: 38_590_000 picoseconds.
		Weight::from_parts(70_043_000, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}