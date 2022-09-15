#![allow(unused_imports)]
use crate::test_vectors;
use crate::sec1::Sec1EncodePoint;
use crate::fields::{Fr, Fq};
use crate::curves::*;
use ark_std::{
    rand::Rng,
    test_rng,
    string::String,
    vec::Vec,
    io::BufReader,
};
use ark_algebra_test_templates::{
    fields::*,
    curves::*,
    msm::test_var_base_msm,
    groups::group_test,
};
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_std::string::ToString;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use ark_ff::{
    vec,
    biginteger::BigInteger320,
    ToBytes,
    FromBytes,
    ToConstraintField,
    PrimeField,
    BigInteger,
};
use hex::ToHex;

#[test]
fn test_fr() {
    let mut rng = test_rng();
    for _ in 0..5 {
        let a: Fr = rng.gen();

        sqrt_field_test(a);
        fft_field_test::<Fr>();
        primefield_test::<Fr>();

        let b: Fr = rng.gen();
        field_test::<Fr>(a, b);
    }
}

#[test]
fn test_fq() {
    let mut rng = test_rng();
    for _ in 0..5 {
        let a: Fq = rng.gen();

        sqrt_field_test(a);
        fft_field_test::<Fq>();
        primefield_test::<Fq>();
        let b: Fq = rng.gen();
        field_test::<Fq>(a, b);
    }
}

#[test]
fn test_secp256k1_curve() {
    let mut rng = ark_std::test_rng();
    let a: Projective = rng.gen();
    let b: Projective = rng.gen();
    group_test(a, b);

    curve_tests::<Projective>();

    test_var_base_msm::<Affine>();

    // Fails in arkworks 0.3.0 but the next version should have a fix
    sw_tests::<Secp256k1Parameters>();

    test_var_base_msm::<Affine>();
}

#[test]
fn test_secp256k1_generator() {
    let generator = Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_serialization() {
    let generator = Affine::prime_subgroup_generator();
    let mut w = Vec::<u8>::new();
    generator.x.serialize(&mut w).unwrap();
    let serialized_x = w.as_slice();
    assert_eq!(hex::encode(Vec::from(serialized_x.clone())), "9817f8165b81f259d928ce2ddbfc9b02070b87ce9562a055acbbdcf97e66be79");

    let mut w = Vec::<u8>::new();
    generator.y.serialize(&mut w).unwrap();
    let serialized_y = w.as_slice();
    assert_eq!(hex::encode(Vec::from(serialized_y.clone())), "b8d410fb8fd0479c195485a648b417fda808110efcfba45d65c4a32677da3a48");

    let reader_x = BufReader::new(serialized_x.clone());
    let x = Fq::deserialize(reader_x).unwrap();
    assert_eq!(x.to_string(), generator.x.to_string());

    let reader_y = BufReader::new(serialized_y.clone());
    let y = Fq::deserialize(reader_y).unwrap();
    assert_eq!(y.to_string(), generator.y.to_string());
}

#[test]
fn test_point_exponentiation() {
    let vectors = test_vectors::addition_test_vectors();

    let generator = Affine::prime_subgroup_generator();

    for vector in vectors {
        let mut k_bytes = vector.0;
        k_bytes.reverse();
        let k = BigInteger320::read(k_bytes.as_slice()).unwrap();

        let point = generator.mul(k).into_affine();

        let mut x_bytes = vec![];
        let _ = point.x.write(&mut x_bytes);
        x_bytes.reverse();

        assert_eq!(x_bytes, vector.1);

        let mut y_bytes = vec![];
        let _ = point.y.write(&mut y_bytes);
        y_bytes.reverse();

        assert_eq!(y_bytes, vector.2);
    }
}

#[test]
fn test_point_sec1_encoding() {
    let vectors = test_vectors::encoding_test_vectors();

    let generator = Affine::prime_subgroup_generator();

    for vector in vectors {
        let k = vector.0;
        let point = generator.mul(k).into_affine();

        let encoded_compressed = point.to_encoded_point(true);
        assert_eq!(encoded_compressed, vector.1);

        let encoded_uncompressed = point.to_encoded_point(false);
        assert_eq!(encoded_uncompressed, vector.2);
    }
}
