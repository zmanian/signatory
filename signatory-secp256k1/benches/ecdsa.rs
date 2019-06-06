//! secp256k1 provider benchmarks

#![allow(unused_imports)]
#![deny(warnings)]

#[macro_use]
extern crate criterion;
use signatory;

use criterion::Criterion;
use signatory::{
    ecdsa::{
        curve::secp256k1::{FixedSignature, SecretKey, SHA256_FIXED_SIZE_TEST_VECTORS},
        PublicKey,
    },
    generic_array::GenericArray,
    test_vector::TestVector,
    DigestSigner, DigestVerifier, Signature,
};
use signatory_secp256k1::{EcdsaSigner, EcdsaVerifier};

/// Test vector to use for benchmarking
const TEST_VECTOR: &TestVector = &SHA256_FIXED_SIZE_TEST_VECTORS[0];

fn sign_ecdsa(c: &mut Criterion) {
    let signer = EcdsaSigner::from(&SecretKey::from_bytes(TEST_VECTOR.sk).unwrap());

    c.bench_function("secp256k1: ECDSA signer", move |b| {
        b.iter(|| {
            let _: FixedSignature = signer.sign_msg_digest(TEST_VECTOR.msg);
        })
    });
}

fn verify_ecdsa(c: &mut Criterion) {
    let signature = FixedSignature::from_bytes(TEST_VECTOR.sig).unwrap();
    let public_key =
        PublicKey::from_compressed_point(GenericArray::clone_from_slice(TEST_VECTOR.pk)).unwrap();
    let verifier = EcdsaVerifier::from(&public_key);

    c.bench_function("secp256k1: ECDSA verifier", move |b| {
        b.iter(|| {
            verifier
                .verify_msg_digest(TEST_VECTOR.msg, &signature)
                .unwrap();
        })
    });
}

criterion_group! {
    name = ecdsa;
    config = Criterion::default();
    targets = sign_ecdsa, verify_ecdsa
}

criterion_main!(ecdsa);
