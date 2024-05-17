#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod types;
pub mod weights;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(feature = "runtime-benchmarks")]
pub trait BenchmarkHelper<IncriptionId> {
    fn to_inscription(i: u32) -> IncriptionId;
}
#[cfg(feature = "runtime-benchmarks")]
pub struct OrdinalsHelper;

#[cfg(feature = "runtime-benchmarks")]
impl<IncriptionId: From<u32>> BenchmarkHelper<IncriptionId> for OrdinalsHelper {
    fn to_inscription(i: u32) -> IncriptionId {
        i.into()
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{
        pallet_prelude::{DispatchResult, *},
        Blake2_128Concat,
    };
    use frame_system::pallet_prelude::*;

    use crate::types::InscriptionInfo;

    use sp_runtime::{
        traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize},
        FixedPointOperand, Saturating,
    };

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The type used to identify a unique inscription.
        type InscriptionId: Member
            + Parameter
            + MaxEncodedLen
            + Copy
            + Default
            + AtLeast32BitUnsigned
            + MaybeSerializeDeserialize
            + TypeInfo
            + FixedPointOperand;

        type WeightInfo: WeightInfo;

        #[cfg(feature = "runtime-benchmarks")]
        type Helper: crate::BenchmarkHelper<Self::CollectionId, Self::ItemId>;
    }

    /// Storages
    ///

    /// Total inscriptions supply.
    ///
    #[pallet::storage]
    #[pallet::getter(fn current_supply)]
    pub type CurrentSupply<T: Config> = StorageValue<_, T::InscriptionId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn inscriptions)]
    pub type Inscriptions<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::InscriptionId,
        crate::types::InscriptionInfo<T::AccountId>,
    >;

    #[pallet::storage]
    pub(super) type Account<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::InscriptionId,
        (),
        OptionQuery,
    >;

    /// Events
    ///
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NewInscribed {
            inscription_id: T::InscriptionId,
            inscriber: T::AccountId,
        },
        InscriptionTransfered {
            inscription_id: T::InscriptionId,
            from: T::AccountId,
            to: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        InscriptionExists,
        InscriptionNotFound,
        SenderNotOwner,
        NoOwner,
        SendToOriginNotAllowed,
    }

    /// Private methods
    impl<T: Config> Pallet<T> {
        pub fn inscription_exists(inscription_id: T::InscriptionId) -> bool {
            Inscriptions::<T>::get(inscription_id).is_some()
        }

        pub fn get_inscription(
            inscription_id: T::InscriptionId,
        ) -> Result<InscriptionInfo<T::AccountId>, Error<T>> {
            match Inscriptions::<T>::get(inscription_id) {
                Some(inscription) => Ok(inscription),
                None => Err(Error::<T>::InscriptionNotFound),
            }
        }

        pub fn owner(inscription_id: T::InscriptionId) -> Option<T::AccountId> {
            Inscriptions::<T>::get(inscription_id).map(|i| i.owner)
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn inscribe(
            origin: OriginFor<T>,
            content_type: BoundedVec<u8, ConstU32<128>>,
            content: BoundedVec<u8, ConstU32<4048>>,
            metadata: BoundedVec<u8, ConstU32<128>>,
        ) -> DispatchResult {
            let mut current_supply = <CurrentSupply<T>>::get();

            let sender = ensure_signed(origin)?;
            let inscription = InscriptionInfo {
                owner: sender.clone(),
                inscriber: sender.clone(),
                content_type,
                content,
                metadata,
            };

            // Increment supply.
            current_supply.saturating_inc();
            <CurrentSupply<T>>::put(current_supply);

            Inscriptions::<T>::insert(current_supply, inscription);

            // Transfer Inscribed to sender
            Account::<T>::insert(&sender, &current_supply, ());

            Self::deposit_event(Event::NewInscribed {
                inscription_id: current_supply,
                inscriber: sender,
            });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn transfer(
            origin: OriginFor<T>,
            inscription_id: T::InscriptionId,
            to: T::AccountId,
        ) -> DispatchResult {
            ensure!(
                Self::inscription_exists(inscription_id),
                Error::<T>::InscriptionExists
            );

            let sender = ensure_signed(origin)?;

            ensure!(sender != to, Error::<T>::SendToOriginNotAllowed);

            let owner_opt = Self::owner(inscription_id);
            if let Some(owner) = owner_opt {
                if owner != sender {
                    return Err(Error::<T>::SenderNotOwner.into());
                }
            } else {
                return Err(Error::<T>::NoOwner.into());
            }

            Account::<T>::remove(&sender, &inscription_id);
            Account::<T>::insert(&to, &inscription_id, ());

            Inscriptions::<T>::mutate(inscription_id, |inscription| {
                if let Some(inscription) = inscription {
                    inscription.owner = to.clone();
                }
            });

            Self::deposit_event(Event::InscriptionTransfered {
                inscription_id,
                from: sender,
                to,
            });

            Ok(())
        }
    }
}
