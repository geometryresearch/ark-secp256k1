use ark_ec::short_weierstrass_jacobian::GroupAffine;
use ark_std::string::String;
use ark_std::vec;
use ark_ff::ToBytes;
use hex;

pub trait Sec1EncodePoint {
    fn to_encoded_point(&self, compress: bool) -> String;
}

impl<P: ark_ec::SWModelParameters> Sec1EncodePoint for GroupAffine<P> {
    fn to_encoded_point(&self, compress: bool) -> String {
        if self.infinity {
            let b = &[0u8];
            return String::from(hex::encode(b));
        }

        let mut x_bytes = vec![];
        let _ = self.x.write(&mut x_bytes);
        x_bytes.reverse();
        let x_bytes = x_bytes.as_slice()[x_bytes.len() - 32..].to_vec();

        let mut y_bytes = vec![];
        let _ = self.y.write(&mut y_bytes);
        y_bytes.reverse();
        let y_bytes = y_bytes.as_slice()[y_bytes.len() - 32..].to_vec();

        if compress {
            // Check if y is odd
            let mut y_bytes = vec![];
            let _ = self.y.write(&mut y_bytes);
            y_bytes.reverse();

            let y_is_odd = y_bytes[y_bytes.len() - 1] % 2u8 == 1u8;

            if y_is_odd {
                let b = [vec![3u8], x_bytes].concat();
                return String::from(hex::encode_upper(b));
            } else {
                let b = [vec![2u8], x_bytes].concat();
                return String::from(hex::encode_upper(b));
            }
        } else {
            let b = [vec![4u8], x_bytes, y_bytes].concat();
            return String::from(hex::encode_upper(b));
        }
    }
}
