use crate::{fq::Fq, fr::Fr};
use ark_ec::{
    short_weierstrass_jacobian::{GroupAffine, GroupProjective},
    ModelParameters, SWModelParameters,
};
use ark_ff::field_new;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Secp256k1Parameters;

pub type Affine = GroupAffine<Secp256k1Parameters>;
pub type Projective = GroupProjective<Secp256k1Parameters>;

pub type BaseField = Fq;
pub type ScalarField = Fr;

impl ModelParameters for Secp256k1Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;
    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[1];

    /// COFACTOR_INV = 1
    const COFACTOR_INV: Fr = field_new!(Fr, "1");

}

impl SWModelParameters for Secp256k1Parameters {
    /// COEFF_A = 0
    const COEFF_A: Fq = field_new!(Fq, "0");

    /// COEFF_B = 7
    const COEFF_B: Fq = field_new!(Fq, "7");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G_GENERATOR_X, G_GENERATOR_Y);
}

/// G_GENERATOR_X = 55066263022277343669578718895168534326250603453777594175500187360389116729240
pub const G_GENERATOR_X: Fq = field_new!(
    Fq,
    "55066263022277343669578718895168534326250603453777594175500187360389116729240"
);

/// G_GENERATOR_Y = 32670510020758816978083085130507043184471273380659243275938904335757337482424
pub const G_GENERATOR_Y: Fq = field_new!(
    Fq,
    "32670510020758816978083085130507043184471273380659243275938904335757337482424"
);
