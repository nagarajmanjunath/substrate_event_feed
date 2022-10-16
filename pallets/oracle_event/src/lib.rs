#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;



#[frame_support::pallet]
pub mod pallet {
	use frame_support::{inherent::Vec,pallet_prelude::*,traits::UnixTime};
	use frame_system::pallet_prelude::*;
	use frame_support::codec::{Encode, Decode};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[derive(Encode, Decode, Clone, TypeInfo, Debug, PartialEq, Eq)]
	#[codec(mel_bound())]
    pub struct OracleFeed {
    	event_name: Vec<u8>,
    	event_details: Vec<u8>,
		created_at: u64
    }

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type UnixTime: UnixTime;
	}


	#[pallet::storage]
	#[pallet::getter(fn SetEventFeed)]
	pub type SetEventFeed<T> = StorageValue<_, Vec<OracleFeed>, OptionQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		EventStored(OracleFeed, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
     	///Event was discarded
		EventDiscarded
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_event(origin: OriginFor<T>,_event_name:Vec<u8>,_event_details:Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			let new_event = OracleFeed{
				event_name:_event_name,
				event_details:_event_details,
				created_at: T::UnixTime::now().as_secs()
			};
			// Update storage.
			<SetEventFeed<T>>::append(new_event.clone());
			// Emit an event.
			Self::deposit_event(Event::EventStored(new_event.clone(), who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

	

	}
	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_initialize(_block_number: T::BlockNumber) -> Weight {
			// Remove all events that are older than one hour
			// 1 hour = 60 minutes = 60 * 60 seconds = 3600 seconds
			Self::validate_event(50);
			Weight::zero()
		}
	}
	impl<T: Config> Pallet<T> {
	pub fn validate_event(time_threshold: u64) {
		let previous_hour = T::UnixTime::now().as_secs().saturating_sub(time_threshold);
		let valid_feed: Option<Vec<OracleFeed>> = match SetEventFeed::<T>::take() {
			Some(event_feed) => {
				let v: Vec<OracleFeed> =
					event_feed.into_iter().filter(|event| event.created_at > previous_hour).collect();
				match v.is_empty() {
					true => None,
					false => Some(v),
				}
			},
			None => None,
		};
		SetEventFeed::<T>::set(valid_feed);
	}
}
}
