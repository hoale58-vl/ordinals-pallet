use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::BoundedVec;
use scale_info::TypeInfo;

#[derive(Encode, Decode, Debug, TypeInfo, MaxEncodedLen, PartialEq)]
pub struct InscriptionInfo<AccountId> {
    pub owner: AccountId,
    pub inscriber: AccountId,
    pub content_type: BoundedVec<u8, frame_support::traits::ConstU32<128>>,
    pub content: BoundedVec<u8, frame_support::traits::ConstU32<4048>>,
    pub metadata: BoundedVec<u8, frame_support::traits::ConstU32<128>>,
}
