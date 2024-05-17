use crate::{mock::*, types::InscriptionInfo, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_core::ConstU32;

#[test]
fn basic_inscribe() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let content: BoundedVec<u8, ConstU32<4048>> = vec![0, 1, 2].try_into().unwrap();
        let content_type: BoundedVec<u8, ConstU32<128>> = vec![0, 1, 2].try_into().unwrap();
        let metadata: BoundedVec<u8, ConstU32<128>> = vec![0, 1, 2].try_into().unwrap();

        let inscription = InscriptionInfo {
            owner: ALICE,
            inscriber: ALICE,
            content: content.clone(),
            content_type: content_type.clone(),
            metadata: metadata.clone(),
        };

        assert_ok!(Ordinals::inscribe(
            RuntimeOrigin::signed(ALICE),
            content_type,
            content,
            metadata
        ));
        assert_eq!(Ordinals::inscriptions(1), Some(inscription));
        System::assert_last_event(
            Event::NewInscribed {
                inscription_id: 1,
                inscriber: ALICE,
            }
            .into(),
        );
    });
}

#[test]
fn basic_transfer() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let content: BoundedVec<u8, ConstU32<4048>> = vec![0, 1, 2].try_into().unwrap();
        let content_type: BoundedVec<u8, ConstU32<128>> = vec![0, 1, 2].try_into().unwrap();
        let metadata: BoundedVec<u8, ConstU32<128>> = vec![0, 1, 2].try_into().unwrap();

        assert_ok!(Ordinals::inscribe(
            RuntimeOrigin::signed(ALICE),
            content_type.clone(),
            content.clone(),
            metadata.clone()
        ));
        assert_ok!(Ordinals::inscribe(
            RuntimeOrigin::signed(BOB),
            content_type,
            content,
            metadata
        ));

        assert_ok!(Ordinals::transfer(RuntimeOrigin::signed(ALICE), 1, BOB));
        // Transfer again, should return error
        assert_noop!(
            Ordinals::transfer(RuntimeOrigin::signed(ALICE), 1, BOB),
            Error::<Test>::SenderNotOwner
        );

        assert_ok!(Ordinals::transfer(RuntimeOrigin::signed(BOB), 1, ALICE));
    });
}
