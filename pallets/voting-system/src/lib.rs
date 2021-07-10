//! A pallet to demonstrate usage of a simple storage map
//!
//! Storage maps map a key type to a value type. The hasher used to hash the key can be customized.
//! This pallet uses the `blake2_128_concat` hasher. This is a good default hasher.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
};
use frame_system::ensure_signed;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as SimpleMap {
		Administrator get(fn adm_map): map hasher(blake2_128_concat)  u32 => T::AccountId;
		Party get(fn party_map): map hasher(blake2_128_concat)  u32 => u32;
		Voter get(fn user__map): map hasher(blake2_128_concat)  T::AccountId => u32;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		
		EntryAdm(u32, AccountId),

		EntryGot(u32, AccountId),

		EntryParty(u32,u32),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// The requested user has not stored a value yet
		NoValueStored,

		/// Alredy stored a value
		AlredyAdmStored,

		//Party alredy stored 
		PartyAlredyStored,

		/// User has alredy voted once
		UserAlreadyVoted,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

		// Initialize errors
		type Error = Error<T>;

		// Initialize events
		fn deposit_event() = default;

		
		#[weight = 10_000]
		fn set_adm(origin, account: T::AccountId) -> DispatchResult {
			// A user can only set their own entry
			let user = ensure_signed(origin)?;

			ensure!(!<Administrator<T>>::contains_key(1), Error::<T>::AlredyAdmStored);
			<Administrator<T>>::insert(1, account);
			Self::deposit_event(RawEvent::EntryAdm(1,user));
			Ok(())
		}

		#[weight = 10_000]
		fn get_adm(origin) -> DispatchResult {
		
			ensure!(<Administrator<T>>::contains_key(1), Error::<T>::NoValueStored);
			let entry = <Administrator<T>>::get(1);
			Self::deposit_event(RawEvent::EntryGot(1, entry));
			Ok(())
		}

		#[weight = 10_000]
		fn add_party(origin, party_id: u32) -> DispatchResult {
			// A user can only set their own entry
			let _user = ensure_signed(origin)?;

			ensure!(!Party::contains_key(party_id), Error::<T>::PartyAlredyStored);
			Party::insert(party_id, 0);
			Self::deposit_event(RawEvent::EntryParty(party_id, 0));
			Ok(())
		}

		#[weight = 10_000]
		fn vote(origin, account: T::AccountId,  party_id: u32) -> DispatchResult {
			// A user can only set their own entry
			let user = ensure_signed(origin)?;

			// check if user has alredy voted
			ensure!(!<Voter<T>>::contains_key(&account), Error::<T>::UserAlreadyVoted);
			<Voter<T>>::insert(&account,1);

			//vote, updat the vote 
			ensure!(Party::contains_key(party_id), Error::<T>::NoValueStored);
			let original_value = Party::get(party_id);
			Party::insert(party_id, original_value + 1);
	
			Self::deposit_event(RawEvent::EntryParty(party_id, original_value + 1));
			Ok(())
		}
	}
}
