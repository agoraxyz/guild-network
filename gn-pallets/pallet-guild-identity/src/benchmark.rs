use super::*;
use crate::Pallet as GuildIdentity;
use pallet_oracle::Pallet as Oracle;

use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::assert_ok;
use frame_support::traits::Get;
use frame_system::RawOrigin;
use gn_sig::webcrypto::{hash_pubkey, wallet::Wallet};
use parity_scale_codec::Encode;

const ACCOUNT: &str = "account";
const SEED: u32 = 999;

benchmarks! {
    register {
        let user: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(user.clone()))
    verify {
        assert!(GuildIdentity::<T>::addresses(user).is_some())
    }

    deregister {
        let user: T::AccountId = whitelisted_caller();
        let operator: T::AccountId = account(ACCOUNT, 10, SEED);

        let wallet_0 = Wallet::from_seed([10u8; 32]).unwrap();
        let wallet_1 = Wallet::from_seed([11u8; 32]).unwrap();
        let authority_0 = hash_pubkey(&wallet_0.pubkey());
        let authority_1 = hash_pubkey(&wallet_1.pubkey());

        oracle_init_and_register::<T>(&user, &operator);
        assert_ok!(GuildIdentity::<T>::authorize(RawOrigin::Signed(user.clone()).into(), authority_0, false));
        assert_ok!(GuildIdentity::<T>::authorize(RawOrigin::Signed(user.clone()).into(), authority_1, true));

        for i in 0..<T as Config>::MaxLinkedAddressTypes::get() {
            let prefix = [i as u8; 8];
            for j in 0..<T as Config>::MaxLinkedAddresses::get() {
                let account: T::AccountId = account(ACCOUNT, i + j, SEED);
                let signature = wallet_0.sign(account.encode()).unwrap();
                assert_ok!(GuildIdentity::<T>::link_address(
                    RawOrigin::Signed(account).into(),
                    user.clone(),
                    prefix,
                    signature
                ));
            }
        }

        for i in 0..<T as Config>::MaxLinkedIdentities::get() {
            let prefix = [i as u8; 8];
            let identity = [i as u8; 32];
            assert_ok!(GuildIdentity::<T>::link_identity(
                RawOrigin::Signed(user.clone()).into(),
                prefix,
                identity,
            ));
            assert_ok!(GuildIdentity::<T>::callback(RawOrigin::Signed(operator.clone()).into(), i.into(), true));
        }

        let address_map = GuildIdentity::<T>::addresses(&user).unwrap();
        assert_eq!(address_map.len(), <T as Config>::MaxLinkedAddressTypes::get() as usize);
        for address_vec in address_map.values() {
            assert_eq!(address_vec.len(), <T as Config>::MaxLinkedAddresses::get() as usize);
        }
        assert_eq!(GuildIdentity::<T>::identities(&user).unwrap().len(), <T as Config>::MaxLinkedIdentities::get() as usize);
    }: _(RawOrigin::Signed(user.clone()))
    verify {
        assert!(GuildIdentity::<T>::addresses(user).is_none())
    }

    authorize {
        let user: T::AccountId = whitelisted_caller();
        let authority = [2u8; 32];
        assert_ok!(GuildIdentity::<T>::register(RawOrigin::Signed(user.clone()).into()));
    }: _(RawOrigin::Signed(user.clone()), authority, false)
    verify {
        assert_eq!(GuildIdentity::<T>::authorities(user).unwrap(), [authority, [0u8; 32]]);
    }

    link_address {
        let user: T::AccountId = account(ACCOUNT, 10, SEED);
        let linked: T::AccountId = whitelisted_caller();
        let wallet = Wallet::from_seed([10u8; 32]).unwrap();
        let authority = hash_pubkey(&wallet.pubkey());
        let signature = wallet.sign(linked.encode()).unwrap();
        let prefix = [0u8; 8];
        assert_ok!(GuildIdentity::<T>::register(RawOrigin::Signed(user.clone()).into()));
        assert_ok!(GuildIdentity::<T>::authorize(RawOrigin::Signed(user.clone()).into(), authority, false));
    }: _(RawOrigin::Signed(linked.clone()), user.clone(), prefix, signature)
    verify {
        let address_map = GuildIdentity::<T>::addresses(user).unwrap();
        let address_vec = address_map.get(&prefix).unwrap().clone().into_inner();
        assert_eq!(address_vec, &[linked]);
    }

    unlink_address {
        let user: T::AccountId =  whitelisted_caller();
        let linked: T::AccountId = account(ACCOUNT, 10, SEED);
        let wallet = Wallet::from_seed([10u8; 32]).unwrap();
        let authority = hash_pubkey(&wallet.pubkey());
        let signature = wallet.sign(linked.encode()).unwrap();
        let prefix = [0u8; 8];
        assert_ok!(GuildIdentity::<T>::register(RawOrigin::Signed(user.clone()).into()));
        assert_ok!(GuildIdentity::<T>::authorize(RawOrigin::Signed(user.clone()).into(), authority, false));
        assert_ok!(GuildIdentity::<T>::link_address(RawOrigin::Signed(linked.clone()).into(), user.clone(), prefix, signature));
    }: _(RawOrigin::Signed(user.clone()), prefix, linked)
    verify {
        let address_map = GuildIdentity::<T>::addresses(user).unwrap();
        let address_vec = address_map.get(&prefix).unwrap();
        assert!(address_vec.is_empty());
    }

    remove_addresses {
        let user: T::AccountId =  whitelisted_caller();
        let wallet = Wallet::from_seed([10u8; 32]).unwrap();
        let authority = hash_pubkey(&wallet.pubkey());
        assert_ok!(GuildIdentity::<T>::register(RawOrigin::Signed(user.clone()).into()));
        assert_ok!(GuildIdentity::<T>::authorize(RawOrigin::Signed(user.clone()).into(), authority, false));

        let prefix = [0u8; 8];
        for i in 0..<T as Config>::MaxLinkedAddresses::get() {
            let linked: T::AccountId = account(ACCOUNT, i, SEED);
            let signature = wallet.sign(linked.encode()).unwrap();
            assert_ok!(GuildIdentity::<T>::link_address(
                RawOrigin::Signed(linked.clone()).into(),
                user.clone(),
                prefix,
                signature
            ));
        }

        let address_map = GuildIdentity::<T>::addresses(&user).unwrap();
        let address_vec = address_map.get(&prefix).unwrap();
        assert_eq!(address_vec.len(), <T as Config>::MaxLinkedAddresses::get() as usize);
    }: _(RawOrigin::Signed(user.clone()), prefix)
    verify {
        let address_map = GuildIdentity::<T>::addresses(&user).unwrap();
        assert!(address_map.get(&prefix).is_none());
    }

    link_identity {
        let user: T::AccountId =  whitelisted_caller();
        let operator: T::AccountId = account(ACCOUNT, 10, SEED);
        let prefix = [0u8; 8];
        let identity = [123u8; 32];
        oracle_init_and_register::<T>(&user, &operator);
    }: _(RawOrigin::Signed(user.clone()), prefix, identity)
    verify {
        assert_ok!(GuildIdentity::<T>::callback(RawOrigin::Signed(operator).into(), 0, true));
        let identity_map = GuildIdentity::<T>::identities(&user).unwrap();
        assert_eq!(identity_map.get(&prefix), Some(&identity));
    }

    unlink_identity {
        let user: T::AccountId =  whitelisted_caller();
        let operator: T::AccountId = account(ACCOUNT, 10, SEED);
        let prefix = [0u8; 8];
        let identity = [123u8; 32];
        oracle_init_and_register::<T>(&user, &operator);
        assert_ok!(GuildIdentity::<T>::link_identity(
            RawOrigin::Signed(user.clone()).into(),
            prefix,
            identity,
        ));
        assert_ok!(GuildIdentity::<T>::callback(RawOrigin::Signed(operator).into(), 0, true));
        let identity_map = GuildIdentity::<T>::identities(&user).unwrap();
        assert_eq!(identity_map.get(&prefix), Some(&identity));
    }: _(RawOrigin::Signed(user.clone()), prefix)
    verify {
        let identity_map = GuildIdentity::<T>::identities(&user).unwrap();
        assert!(identity_map.get(&prefix).is_none());
    }

    impl_benchmark_test_suite!(GuildIdentity, crate::mock::new_test_ext(), crate::mock::TestRuntime, extra = false);
}

fn oracle_init_and_register<T: Config>(user: &T::AccountId, operator: &T::AccountId) {
    assert_ok!(Oracle::<T>::register_operator(
        RawOrigin::Root.into(),
        operator.clone(),
    ));
    assert_ok!(Oracle::<T>::activate_operator(
        RawOrigin::Signed(operator.clone()).into(),
    ));

    assert_ok!(GuildIdentity::<T>::register(
        RawOrigin::Signed(user.clone()).into(),
    ));
}
