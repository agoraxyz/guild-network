use crate::{Decode, Encode, TypeInfo};
use ed25519_zebra::{Signature as EdSig, VerificationKey as EdKey};
use schnorrkel::{PublicKey as SrKey, Signature as SrSig};
use secp256k1::{
    ecdsa::{RecoverableSignature, RecoveryId},
    Message, Secp256k1,
};
use sp_core::keccak_256;

pub const ETHEREUM_HASH_PREFIX: &str = "\x19Ethereum Signed Message:\n";
pub const SR_SIGNING_CTX: &[u8] = b"substrate";

#[derive(Encode, Decode, TypeInfo, Eq, PartialEq, Clone, Debug)]
pub enum Identity {
    Address20([u8; 20]),
    Address32([u8; 32]),
    Raw(Vec<u8>),
}

#[derive(Encode, Decode, TypeInfo, Eq, PartialEq, Clone, Debug)]
pub enum IdentityWithAuth {
    Ecdsa(Identity, sp_core::ecdsa::Signature),
    Ed25519(Identity, sp_core::ed25519::Signature),
    Sr25519(Identity, sp_core::sr25519::Signature),
    Raw(Identity, Vec<u8>),
}

impl IdentityWithAuth {
    pub fn verify<M: AsRef<[u8]>>(&self, msg: M) -> bool {
        match self {
            // Ethereum specific ecdsa
            Self::Ecdsa(Identity::Address20(address), sig) => {
                let prehashed_msg = eth_hash_message(msg);
                let Some(recovered_pk) = eth_recover_prehashed(prehashed_msg, sig) else {
                    return false
                };

                let serialized_pk = recovered_pk.serialize_uncompressed();
                debug_assert_eq!(serialized_pk[0], 0x04);
                &keccak_256(&serialized_pk[1..])[12..] == address
            }
            // generic ecdsa
            Self::Ecdsa(Identity::Address32(pubkey), sig) => {
                let Some(recovered_pk) = sig.recover(&msg) else {
                    return false
                };
                &recovered_pk.0[1..] == pubkey
            }
            Self::Ed25519(Identity::Address32(pubkey), sig) => {
                let Ok(ed_key) = EdKey::try_from(pubkey.as_ref()) else {
                    return false
                };

                let Ok(ed_sig) = EdSig::try_from(&sig.0[..]) else {
                    return false
                };

                ed_key.verify(&ed_sig, msg.as_ref()).is_ok()
            }
            Self::Sr25519(Identity::Address32(pubkey), sig) => {
                let Ok(sr_key) = SrKey::from_bytes(pubkey.as_ref()) else {
                    return false
                };

                let Ok(sr_sig) = SrSig::from_bytes(&sig.0) else {
                    return false
                };

                sr_key
                    .verify_simple(SR_SIGNING_CTX, msg.as_ref(), &sr_sig)
                    .is_ok()
            }
            Self::Raw(_, _) => true, // not authenticating for now
            _ => false,
        }
    }
}

fn eth_recover_prehashed(
    message: [u8; 32],
    signature: &sp_core::ecdsa::Signature,
) -> Option<secp256k1::PublicKey> {
    let rid = RecoveryId::from_i32(signature.0[64] as i32).ok()?;
    let sig = RecoverableSignature::from_compact(&signature.0[..64], rid).ok()?;
    // NOTE this never fails because the prehashed message is 32 bytes
    let message = Message::from_slice(&message).expect("Message is 32 bytes; qed");
    Secp256k1::verification_only()
        .recover_ecdsa(&message, &sig)
        .ok()
}

fn eth_hash_message<M: AsRef<[u8]>>(message: M) -> [u8; 32] {
    let mut eth_message = format!("{ETHEREUM_HASH_PREFIX}{}", message.as_ref().len()).into_bytes();
    eth_message.extend_from_slice(message.as_ref());
    keccak_256(&eth_message)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::verification_msg;
    use ethers::core::k256::ecdsa::SigningKey;
    use ethers::core::k256::elliptic_curve::sec1::ToEncodedPoint;
    use ethers::core::k256::elliptic_curve::PublicKey;
    use ethers::signers::{LocalWallet, Signer as EthSigner};
    use sp_core::Pair as PairT;

    const TEST_ACCOUNT: &str = "test-account-0xabcde";

    #[test]
    fn test_eth_msg_hashing() {
        let msg = verification_msg(TEST_ACCOUNT);
        let eth_hashed_msg = eth_hash_message(&msg);
        assert_eq!(ethers::utils::hash_message(&msg).as_bytes(), eth_hashed_msg);
    }

    #[tokio::test]
    async fn ethereum_ecdsa() {
        // check ethereum specific message hashing
        let msg = verification_msg(TEST_ACCOUNT);
        let eth_hashed_msg = eth_hash_message(&msg);

        // generate signers
        let seed = [2u8; 32];
        let sp_signer = sp_core::ecdsa::Pair::from_seed_slice(&seed).unwrap();
        let signing_key = SigningKey::from_bytes(&seed).unwrap();
        let eth_signer = LocalWallet::from(signing_key);

        // generate signatures
        let mut eth_signature = eth_signer.sign_message(&msg).await.unwrap().to_vec();
        eth_signature[64] -= 27; // 'v' is normalized via eip-155
        let sp_signature = sp_signer.sign_prehashed(&eth_hashed_msg);
        assert_eq!(eth_signature, sp_signature.0);

        // recover encoded key
        let recovered_key = sp_signature.recover_prehashed(&eth_hashed_msg).unwrap();
        let eth_pk = PublicKey::from(eth_signer.signer().verifying_key()).to_encoded_point(true); // encode = true
        assert_eq!(&recovered_key.0, eth_pk.as_bytes());

        // check a signature generated via ethers
        let sp_signature = sp_core::ecdsa::Signature::from_raw(eth_signature.try_into().unwrap());
        let sp_address = Identity::Address20(eth_signer.address().to_fixed_bytes());
        let id_with_auth = IdentityWithAuth::Ecdsa(sp_address, sp_signature);

        assert!(id_with_auth.verify(&msg));
        assert!(!id_with_auth.verify(b"wrong msg"))
    }

    #[test]
    fn generic_ecdsa() {
        let msg = verification_msg(TEST_ACCOUNT);
        let seed = [2u8; 32];
        let signer = sp_core::ecdsa::Pair::from_seed_slice(&seed).unwrap();

        let signature = signer.sign(msg.as_ref());
        let recovered_key = signature.recover(&msg).unwrap();
        assert_eq!(recovered_key, signer.public());
        let address = Identity::Address32(signer.public().0[1..].try_into().unwrap());
        let id_with_auth = IdentityWithAuth::Ecdsa(address, signature);

        assert!(id_with_auth.verify(&msg));
        assert!(!id_with_auth.verify(b"wrong msg"));
    }

    #[test]
    fn generic_edwards() {
        let msg = verification_msg(TEST_ACCOUNT);
        let seed = [2u8; 32];
        let signer = sp_core::ed25519::Pair::from_seed_slice(&seed).unwrap();

        let signature = signer.sign(msg.as_ref());
        let address = Identity::Address32(signer.public().0);
        let id_with_auth = IdentityWithAuth::Ed25519(address, signature);

        assert!(id_with_auth.verify(&msg));
        assert!(!id_with_auth.verify(b"wrong msg"));
    }

    #[test]
    fn generic_ristretto() {
        let msg = verification_msg(TEST_ACCOUNT);
        let seed = [2u8; 32];
        let signer = sp_core::sr25519::Pair::from_seed_slice(&seed).unwrap();

        let signature = signer.sign(msg.as_ref());
        let address = Identity::Address32(signer.public().0);
        let id_with_auth = IdentityWithAuth::Sr25519(address, signature);

        assert!(id_with_auth.verify(&msg));
        assert!(!id_with_auth.verify(b"wrong msg"));
    }

    #[test]
    fn raw_unchecked() {
        let id_with_auth = IdentityWithAuth::Raw(Identity::Raw(vec![1, 2, 3]), vec![]);
        assert!(id_with_auth.verify(b""));
        assert!(id_with_auth.verify(b"hello"));
    }
}
