use super::*;
use crate as pallet_ordinals;

use frame_support::{construct_runtime, derive_impl};
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Ordinals: pallet_ordinals,
    }
);

pub type AccountId = u64;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
}

impl pallet_ordinals::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type InscriptionId = u32;
    type WeightInfo = weights::SubstrateWeight<Test>;
    #[cfg(feature = "runtime-benchmarks")]
    type Helper = OrdinalsHelper;
}

pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
