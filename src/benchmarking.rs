#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use crate::Pallet as OrdinalBench;

use crate::Event;
use frame_benchmarking::v1::{account, benchmarks, vec, whitelisted_caller};
use frame_system::RawOrigin;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
    inscribe {
        let inscriber: T::AccountId = whitelisted_caller();
        let inscription_id: T::CollectionId = T::Helper::to_inscription(1);

        let content_type = vec![0, 1, 2].try_into().unwrap();
        let content = vec![0, 1, 2].try_into().unwrap();
        let metadata = vec![0, 1, 2].try_into().unwrap();

    }: _(RawOrigin::Signed(inscriber.clone()), content_type, content, metadata)
    verify {
        assert_last_event::<T>(Event::NewInscribed { inscription_id, inscriber }.into());
    }

    transfer {
        let sender: T::AccountId = whitelisted_caller();
        let inscription_id: T::CollectionId = T::Helper::to_inscription(1);

        let content_type = vec![0, 1, 2].try_into().unwrap();
        let content = vec![0, 1, 2].try_into().unwrap();
        let metadata = vec![0, 1, 2].try_into().unwrap();

        let receiver: T::AccountId = account("r", 1, 0);

        let _ = OrdinalBench::<T>::inscribe(RawOrigin::Signed(sender.clone()).into(), content_type, content, metadata);
    }: _(RawOrigin::Signed(sender.clone()), inscription_id, receiver)
    verify {
        assert_last_event::<T>(Event::NftTransfered { inscription_id, from: sender, to: receiver }.into());
    }

    impl_benchmark_test_suite!(OrdinalBench, crate::mock::new_test_ext(), crate::mock::Test);
}
