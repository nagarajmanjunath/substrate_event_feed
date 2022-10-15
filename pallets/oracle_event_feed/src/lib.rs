#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::storage::bounded_vec::BoundedVec;
	use frame_support::{
		
		dispatch::{DispatchResult, PartialEq},
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use frame_support::traits::UnixTime;


	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type MaxValue: Get<u32>;
		type KeyLimit: Get<u32>;
		type TimeProvider: UnixTime;

	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);


	// The struct to organize anf group together each events
	#[derive(Encode, Decode, PartialEq, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct OracleEvent<T: Config> {
		event_name: BoundedVec<u8, T::KeyLimit>,
		event_description : BoundedVec<u8, T::KeyLimit>,
		added_block: T::BlockNumber,
		time_stamp : u64
	}


	// The storage area, where we store the created events
	#[pallet::storage]
	// #[pallet::getter(fn get_RootOracleEvent)]
	pub type RootOracleEvent<T: Config> =
		StorageValue<_, BoundedVec<OracleEvent<T>, T::KeyLimit>, ValueQuery>;


	
	// Events inform users on successful completion of certain action
	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		EventCreatedSuccessfully
	}


	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Too many events created
		TooManyEvents
	}


	//We are using on_initialize hook to check and remove the event older than 1 hour based on the timestamp
	// we are fething timestamp from timestamp_pallet by loose coupling

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_: T::BlockNumber) -> Weight {
			// here we fetch the current blocktime - 3600 seconds(1 hour)
			let time: u64 = T::TimeProvider::now().as_secs().saturating_sub(10000) ;
			// And we iterate over the stored event's time stamp
			//And remove those event which is less than or equal to 'time' variable value
			<RootOracleEvent<T>>::mutate(|event_list| {
				if let Some(index) = event_list.iter().position(|member| member.time_stamp <= time) {
						event_list.remove(index);
				
					
				}
				
			});
					
			// Weight::zero()
			T::DbWeight::get().writes(1)

		}

	
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		//The create_oracle_event is a sudo transaction
		//It can be only performed by a sudo user (Alice is the sudo user by default) in dev chain
		//This function will take two arguments
		// _event_name => name of the event &&  _event_description => a random description
		//we are using BoundVec as the type of input arguments
		//Bcoz substrate wont allow to using string => #![cfg_attr(not(feature = "std"), no_std)]


		#[pallet::weight(10000)]
		pub fn create_oracle_event(
			origin: OriginFor<T>,
			_event_name:BoundedVec<u8, T::KeyLimit>,
			_event_description:BoundedVec<u8, T::KeyLimit>

		) -> DispatchResult {

			//check weather the call is made by sudo user
			//In dev chain by default `Alice` is the sudo user
			ensure_root(origin)?;
			

	        //fetch the current block number
			let current_block = <frame_system::Pallet<T>>::block_number();

	        //returns the current timestamp (Unix format, ex: 1664302723)
			let time: u64 = T::TimeProvider::now().as_secs();

			// create a new_event based on OracleEvent struct and assign incoming values to corresponding fields
		    let new_event = OracleEvent::<T> { 
				event_name: _event_name,
			    event_description: _event_description,
				added_block: current_block,
				time_stamp: time
			};

			//Push the new_event struct into the Root RootOracleEvent storage item () into th chain state
			// Throw trnsaction error if the RootOracle event exceeds the BoundVec limit
			<RootOracleEvent<T>>::mutate(|event_list| event_list.try_push(new_event))
				.map_err(|_| <Error<T>>::TooManyEvents)?;

			//This event let the know know that, event has been succefully emitted
			Self::deposit_event(Event::EventCreatedSuccessfully);

	         // returns  Ok as successful execution
			Ok(())
		}

	
	}



}