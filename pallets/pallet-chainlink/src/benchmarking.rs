use super::*;
use crate::Pallet as Chainlink;

use codec::Encode;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use sp_std::{vec, vec::Vec as SpVec};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DummyCallback;
impl CallbackWithParameter for DummyCallback {
    fn with_result(&self, _result: SpVec<u8>) -> Option<Self> {
        None
    }
}

benchmarks! {
    register_operator {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller))
    verify {
    }
    deregister_operator {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller))
    verify {
    }
    // TODO it's hard to benchmark these as they are taking dynamic inputs
    //initiate_request {
    //    let caller: T::AccountId = whitelisted_caller();
    //    let operator: T::AccountId = account("operator", 1, 123);

    //    Chainlink::<T>::register_operator(RawOrigin::Signed(operator.clone()).into())?;

    //    let spec_index = vec![0; 5];
    //    let data_version = 987_u64;
    //    let data = ["this", "and", "that"].encode();
    //    let fee = T::Currency::minimum_balance();
    //}: _(RawOrigin::Signed(caller), operator, spec_index, data_version, data, fee,
    //verify {
    //}
    //callback {
    //    let caller: T::AccountId = whitelisted_caller();
    //    let request_id = 128_u64;
    //    let operator: T::AccountId = account("operator", 1, 123);

    //    Chainlink::<T>::register_operator(RawOrigin::Signed(operator.clone()).into())?;

    //}: _(RawOrigin::Signed(caller), request_id, vec![0; 128])
    //verify {
    //}
}
