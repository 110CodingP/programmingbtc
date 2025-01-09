use rug::{integer::Order, Integer};

use crate::{s256_field::Signature, traits::Serializer, EllipticCurve};

impl Serializer for EllipticCurve {
    fn sec(&self, is_compressed: bool) -> String {
        // Uncompressed format serialization of a a pubkey
        let serialized_x = self.x.clone().unwrap().num().to_string_radix(16);

        if is_compressed {
            let prefix = if self.y.clone().unwrap().num().is_odd() {
                "03"
            } else {
                "02"
            };
            prefix.to_string() + &pad_or_truncate_data(serialized_x)
        } else {
            let prefix = "04";
            let serialized_y = self.y.clone().unwrap().num().to_string_radix(16);
            prefix.to_string() + &pad_or_truncate_data(serialized_x) + &pad_or_truncate_data(serialized_y)
        }
    }
}

fn pad_or_truncate_data(input: String) -> String {
    let mut data = input;
    if data.len() < 64 {
        let padding = "0".repeat(64 - data.len());
        padding + &data
    } else if data.len() > 64 {
        data.truncate(64);
        data
    } else {
        data
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rug::{ops::Pow, Integer};

    use crate::{s256_field::secp_generator_point, traits::Serializer};

    #[test]
    fn test_uncompressed_sec() {
        // get the generator point
        let point = secp_generator_point()
            .scalar_mul(Integer::from(5000));

        let serialized = point.sec(false);
        assert_eq!(
            serialized,
            "04ffe558e388852f0120e46af2d1b370f85854a8eb0841811ece0e3e03d282d57c315dc72890a4f10a1481c031b03b351b0dc79901ca18a00cf009dbdb157a1d10",
            "Serialized point is incorrect"
        );

        let exponent = Integer::from(2018).pow(5);
        let point = secp_generator_point().scalar_mul(exponent);
        let serialized = point.sec(false);
        assert_eq!(
            serialized,
            "04027f3da1918455e03c46f659266a1bb5204e959db7364d2f473bdf8f0a13cc9dff87647fd023c13b4a4994f17691895806e1b40b57f4fd22581a4f46851f3b06",
            "Serialized point is incorrect"
        );

        let point = secp_generator_point()
            .scalar_mul(
                Integer::from_str_radix("deadbeef12345", 16).unwrap()
            );
        let serialized = point.sec(false);
        assert_eq!(
            serialized,
            "04d90cd625ee87dd38656dd95cf79f65f60f7273b67d3096e68bd81e4f5342691f842efa762fd59961d0e99803c61edba8b3e3f7dc3a341836f97733aebf987121",
            "Serialized point is incorrect"
        );
    }

    #[test]
    fn test_compressed() {
        let mut point = secp_generator_point()
            .scalar_mul(Integer::from(5001));
        let mut serialized = point.sec(true);
        assert_eq!(
            serialized,
            "0357a4f368868a8a6d572991e484e664810ff14c05c0fa023275251151fe0e53d1",
            "Serialized point is incorrect"
        );

        let exponent = Integer::from(2019).pow(5);
        point = secp_generator_point().scalar_mul(exponent);
        serialized = point.sec(true);
        assert_eq!(
            serialized,
            "02933ec2d2b111b92737ec12f1c5d20f3233a0ad21cd8b36d0bca7a0cfa5cb8701",
            "Serialized point is incorrect"
        );

        point = secp_generator_point()
            .scalar_mul(
                Integer::from_str_radix("deadbeef54321", 16).unwrap()
            );
        serialized = point.sec(true);
        assert_eq!(
            serialized,
            "0296be5b1292f6c856b3c5654e886fc13511462059089cdf9c479623bfcbe77690",
            "Serialized point is incorrect"
        );
    }
}