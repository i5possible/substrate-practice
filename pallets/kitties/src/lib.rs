#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResult,
		traits::{Currency, Randomness},
		pallet_prelude::*
	};
	use frame_system::pallet_prelude::*;
	use codec::{Encode, Decode};
	use scale_info::TypeInfo;
	use sp_io::hashing::blake2_128;

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


	#[derive(Encode, Decode, Clone, PartialEq, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T:Config> {
		pub dna: [u8; 16],
		pub price: Option<BalanceOf<T>>,
	}

	type KittyIndex = u32;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::storage]
	#[pallet::getter(fn kitties_count)]
	pub type KittiesCount<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyIndex, Option<Kitty<T>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> = StorageMap<_, Blake2_128Concat, KittyIndex, Option<T::AccountId>, ValueQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn kitty_cnt)]
	pub(super) type KittyCnt<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreate(T::AccountId, KittyIndex),
		KittyTransfer(T::AccountId, T::AccountId, KittyIndex)
	}

	#[pallet::error]
	pub enum Error<T> {
		KittiesCountOverflow,
		NotOwner,
		InvalidKittyIndex,
		SameParentIndex,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _kitty_id = Self::mint(&who, None);

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(origin: OriginFor<T>, new_owner: T::AccountId, kitty_id: KittyIndex) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Some(who.clone()) == Owner::<T>::get(kitty_id), Error::<T>::NotOwner);

			Owner::<T>::insert(kitty_id, Some(new_owner.clone()));

			Self::deposit_event(Event::KittyTransfer(who, new_owner, kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn breed(origin: OriginFor<T>, kitty_id_1: KittyIndex, kitty_id_2: KittyIndex) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameParentIndex);

			let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyIndex)?;
			let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyIndex)?;

			let dna_1 = kitty1.dna;
			let dna_2 = kitty2.dna;

			let selector = Self::random_value(&who);
			let mut dna = [0u8; 16];

			for i in 0..dna_1.len() {
				dna[i] = (selector[i] & dna_1[i]) | (!selector[i] & dna_2[i]);
			}

			let _kitty_id = Self::mint(&who, Some(dna));
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn sell(_origin: OriginFor<T>) -> DispatchResult {
			// TODO: check the kitty exists
			// TODO: check the kitty belongs to the sender
			// TODO: set the price for the kitty
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn buy(_origin: OriginFor<T>, _kitty_id: KittyIndex) -> DispatchResult {
			// TODO: check the kitty exists
			// TODO: check the price of kitty is present
			// TODO: check the buyer has enough balance
			// TODO: update the kitty's owner, reset the price to None
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
		}

		fn new_kitty_id() -> Result<KittyIndex, Error<T>> {
			match Self::kitties_count() {
				Some(id) => {
					ensure!(id != KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					Ok(id)
				},
				None => {Ok(1)}
			}
		}

		fn mint(owner: &T::AccountId, dna: Option<[u8; 16]>) -> Result<u32, Error<T>>{
			let kitty_id = Self::new_kitty_id()?;

			let dna = dna.unwrap_or(Self::random_value(owner));

			// TODO: staking token when create kitty
			Kitties::<T>::insert(kitty_id,Some(Kitty::<T> {
				dna: dna,
				price: None,
			}));

			Owner::<T>::insert(kitty_id, Some(owner.clone()));

			KittiesCount::<T>::put(kitty_id + 1);

			Self::deposit_event(Event::KittyCreate(owner.clone(), kitty_id));

			Ok(kitty_id)
		}
	}
}

