use crate::mock::*;
use frame_benchmarking::frame_support::BoundedVec;
use frame_support::{assert_noop, assert_ok, error::BadOrigin};

#[test]
fn create_oracle_event_should_work() {
    new_test_ext().execute_with(|| {
        let event_name: BoundedVec<_, KeyLimit> =
            BoundedVec::try_from(vec!["Oracle Event".as_ptr() as u8]).unwrap();
        let event_description: BoundedVec<_, KeyLimit> =
            BoundedVec::try_from(vec!["Its a test event for now".as_ptr() as u8]).unwrap();
        assert_ok!(OraclePalletTesting::publish_oracle_event(
            Origin::root(),
            event_name.clone(),
            event_description.clone()
        ));
        assert_eq!(
            OraclePalletTesting::publish_oracle_event(Origin::root(), event_name, event_description),
            Ok(())
        );
    })
}

#[test]
fn create_oracle_event_should_not_work() {
    new_test_ext().execute_with(|| {
        let event_name: BoundedVec<_, KeyLimit> =
            BoundedVec::try_from(vec!["Oracle Event_2".as_ptr() as u8]).unwrap();
        let event_description: BoundedVec<_, KeyLimit> =
            BoundedVec::try_from(vec!["Its a normal event".as_ptr() as u8]).unwrap();

        assert_noop!(
            OraclePalletTesting::publish_oracle_event(
                Origin::signed(2),
                event_name.clone(),
                event_description.clone()
            ),
            BadOrigin
        );

        assert_ne!(
            OraclePalletTesting::publish_oracle_event(
                Origin::signed(1),
                event_name,
                event_description
            ),
            Ok(())
        );
    })
}
