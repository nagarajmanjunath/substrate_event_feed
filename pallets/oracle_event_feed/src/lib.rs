#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::storage::bounded_vec::BoundedVec;
    use frame_support::traits::UnixTime;
    use frame_support::{
        dispatch::{DispatchResult, PartialEq},
        pallet_prelude::*,
    };
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;

    /// Configure the pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type MaxValue: Get<u32>;
        type KeyLimit: Get<u32>;
        type TimeProvider: UnixTime;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);


    #[derive(Encode, Decode, PartialEq, MaxEncodedLen, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    #[codec(mel_bound())]
    pub struct OracleEvent<T: Config> {
        feed_name: BoundedVec<u8, T::KeyLimit>,
        feed_description: BoundedVec<u8, T::KeyLimit>,
        added_block: T::BlockNumber,
        time_stamp: u64,
    }

    // Define storage
    #[pallet::storage]
    pub type RootOracleEvent<T: Config> =
        StorageValue<_, BoundedVec<OracleEvent<T>, T::KeyLimit>, ValueQuery>;

    // Defining Event
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        EventCreatedSuccessfully,
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Too many events created
        TooManyEvents,
    }

 
    //Initialize the hooks to check the event timing and remove if its more then 10 sec
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_: T::BlockNumber) -> Weight {
            let time: u64 = T::TimeProvider::now().as_secs().saturating_sub(100);
            <RootOracleEvent<T>>::mutate(|event_list| {
                if let Some(index) = event_list
                    .iter()
                    .position(|member| member.time_stamp <= time)
                {
                    event_list.remove(index);
                }
            });

            T::DbWeight::get().writes(1)
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10000)]
        pub fn publish_oracle_event(
            origin: OriginFor<T>,
            _event_name: BoundedVec<u8, T::KeyLimit>,
            _event_description: BoundedVec<u8, T::KeyLimit>,
        ) -> DispatchResult {
            //check the sudo user/root user
            ensure_root(origin)?;
            //fetch the current block number
            let current_block = <frame_system::Pallet<T>>::block_number();

            //returns the current timestamp (Unix format, ex: 1664302723)
            let time: u64 = T::TimeProvider::now().as_secs();

            // create a new_event based on OracleEvent struct and assign incoming values to corresponding fields
            let new_event = OracleEvent::<T> {
                feed_name: _event_name,
                feed_description: _event_description,
                added_block: current_block,
                time_stamp: time,
            };

            //Push the event feed to RootOracleEvent
            <RootOracleEvent<T>>::mutate(|event_list| event_list.try_push(new_event))
                .map_err(|_| <Error<T>>::TooManyEvents)?;

            //Emit the event
            Self::deposit_event(Event::EventCreatedSuccessfully);

            Ok(())
        }
    }
}
