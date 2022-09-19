use super::*;
use codec::{Decode, Encode};
use frame_support::traits::OnFinalize;

pub fn last_event() -> crate::Event {
    System::events()
        .into_iter()
        .map(|r| r.event)
        .filter_map(|e| {
            if let crate::Event::Chainlink(inner) = e {
                Some(crate::Event::Chainlink(inner))
            } else {
                None
            }
        })
        .last()
        .unwrap()
}

fn get_minimum_fee() -> u64 {
    <TestRuntime as pallet_chainlink::Config>::MinimumFee::get() as u64
}

#[test]
fn operator_registration_valid() {
    new_test_runtime().execute_with(|| {
        // This is required for some reason otherwise the last_event() method fails
        System::set_block_number(1);

        assert!(!<Chainlink>::operator(1));
        assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
        assert_eq!(
            last_event(),
            crate::Event::Chainlink(pallet_chainlink::Event::OperatorRegistered(1))
        );
        assert!(<Chainlink>::operator(1));
    });
}

#[test]
fn operator_registration_invalid_operator_already_registered() {
    new_test_runtime().execute_with(|| {
        assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
        assert!(<Chainlink>::operator(1));

        // Operator already registered error
        assert!(<Chainlink>::register_operator(Origin::signed(1)).is_err());
        assert!(<Chainlink>::operator(1));
    });
}

#[test]
fn operator_unregistration_valid() {
    new_test_runtime().execute_with(|| {
        // This is required for some reason otherwise the last_event() method fails
        System::set_block_number(1);

        assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
        assert!(<Chainlink>::unregister_operator(Origin::signed(1)).is_ok());
        assert!(!<Chainlink>::operator(1));

        assert_eq!(
            last_event(),
            crate::Event::Chainlink(pallet_chainlink::Event::OperatorUnregistered(1))
        );
    });
}

#[test]
fn operator_unregistration_invalid_unknown_operator() {
    new_test_runtime().execute_with(|| {
        // Unknown operator error
        assert!(<Chainlink>::unregister_operator(Origin::signed(1)).is_err());
        assert!(!<Chainlink>::operator(1));
    });
}

#[test]
fn initiate_requests_valid() {
    new_test_runtime().execute_with(|| {
        System::set_block_number(1);
        assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
        assert_eq!(
            last_event(),
            crate::Event::Chainlink(pallet_chainlink::Event::OperatorRegistered(1))
        );

        let parameters = ("a", "b");
        let data = parameters.encode();
        assert!(<Chainlink>::initiate_request(
            Origin::signed(2),
            1,
            vec![],
            1,
            data.clone(),
            get_minimum_fee(),
            pallet_test_operator::Call::<TestRuntime>::callback { result: vec![] }
        )
        .is_ok());
        assert_eq!(
            last_event(),
            crate::Event::Chainlink(pallet_chainlink::Event::OracleRequest(
                1,
                vec![],
                0,
                2,
                1,
                data.clone(),
                "Chainlink.callback".into(),
                get_minimum_fee()
            ))
        );

        let r = <(Vec<u8>, Vec<u8>)>::decode(&mut &data[..]).unwrap().0;
        assert_eq!("a", std::str::from_utf8(&r).unwrap());

        let result: u64 = 10;
        assert!(<Chainlink>::callback(Origin::signed(1), 0, result.encode()).is_ok());

        let expected_answer = Chainlink::prepend_request_id(&mut result.encode(), 0);

        assert_eq!(
            last_event(),
            crate::Event::Chainlink(pallet_chainlink::Event::OracleAnswer(
                1,
                0,
                expected_answer,
                get_minimum_fee()
            ))
        );

        assert_eq!(<pallet_test_operator::Result<TestRuntime>>::get(), 10);
    });
}

#[test]
fn initiate_requests_invalid_unknown_operator() {
    new_test_runtime().execute_with(|| {
        // Unknown operator error
        assert!(<Chainlink>::initiate_request(
            Origin::signed(2),
            1,
            vec![],
            1,
            vec![],
            get_minimum_fee(),
            pallet_test_operator::Call::<TestRuntime>::callback { result: vec![] }
        )
        .is_err());
    });
}

#[test]
fn initiate_requests_invalid_insufficient_fee() {
    new_test_runtime().execute_with(|| {
        assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
        // Insufficient fee error
        assert!(<Chainlink>::initiate_request(
            Origin::signed(2),
            1,
            vec![],
            1,
            vec![],
            get_minimum_fee() - 1,
            pallet_test_operator::Call::<TestRuntime>::callback { result: vec![] }
        )
        .is_err());
    });
}

#[test]
fn initiate_requests_invalid_insufficient_balance_for_fee() {
    new_test_runtime().execute_with(|| {
        assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());

        // Insufficient balance error (System error)
        assert!(<Chainlink>::initiate_request(
            Origin::signed(2),
            1,
            vec![],
            1,
            vec![],
            GENESIS_BALANCE + 1,
            pallet_test_operator::Call::<TestRuntime>::callback { result: vec![] }
        )
        .is_err());
    });
}

#[test]
fn initiate_requests_invalid_wrong_operator() {
    new_test_runtime().execute_with(|| {
        assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
        assert!(<Chainlink>::initiate_request(
            Origin::signed(2),
            1,
            vec![],
            1,
            vec![],
            get_minimum_fee(),
            pallet_test_operator::Call::<TestRuntime>::callback { result: vec![] }
        )
        .is_ok());
        // Wrong operator error
        assert!(<Chainlink>::callback(Origin::signed(3), 0, 10.encode()).is_err());
    });
}

#[test]
fn callback_invalid_unknown_request() {
    new_test_runtime().execute_with(|| {
        // Unknown request error
        assert!(<Chainlink>::callback(Origin::signed(1), 0, 10.encode()).is_err());
    });
}

#[test]
fn kill_request() {
    new_test_runtime().execute_with(|| {
        assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
        assert!(<Chainlink>::initiate_request(
            Origin::signed(2),
            1,
            vec![],
            1,
            vec![],
            get_minimum_fee(),
            pallet_test_operator::Call::<TestRuntime>::callback { result: vec![] }
        )
        .is_ok());

        <Chainlink as OnFinalize<u64>>::on_finalize(
            <TestRuntime as pallet_chainlink::Config>::ValidityPeriod::get() + 1,
        );
        // Request has been killed, too old
        // Unknown request error
        assert!(<Chainlink>::callback(Origin::signed(1), 0, 10.encode()).is_err());
    });
}
