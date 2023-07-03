use num_bigint::{BigInt};

// y^2 = x^3 + ax + b (mod p)
#[derive(Clone)]
struct ECurve {
    a: BigInt,
    b: BigInt,
    p: BigInt,
}

impl ECurve {
    pub fn create(a: BigInt, b: BigInt, p: BigInt) -> Self {
        Self { a, b, p }
    }
}

#[derive(Clone)]
struct ECPoint {
    x: BigInt,
    y: BigInt,
    curve: ECurve,
}

impl ECPoint {
    pub fn create(x: BigInt, y: BigInt, curve: ECurve) -> Self {
        Self { x, y, curve }
    }

    pub fn is_point_on_curve(&self) -> bool {
        let one: BigInt = BigInt::from(1);

        let x = &self.x;
        let y = &self.y;

        let a = &self.curve.a;
        let b = &self.curve.b;
        let p = &self.curve.p;

        let left = y.pow(2).modpow(&one, p);
        let right = (x.pow(3) + a * x + b).modpow(&one, p);

        left.eq(&right)
    }

    pub fn add_point(&self, point: &ECPoint) -> Self {
        let one: BigInt = BigInt::from(1);

        let p = &self.curve.p;
        let a = &self.curve.a;

        let x1 = &self.x;
        let y1 = &self.y;

        let x2 = &point.x;
        let y2 = &point.y;

        let mut num = BigInt::from(0);
        let mut den = BigInt::from(0);
        if x1.eq(x2) && y1.eq(y2) {
            num = (BigInt::from(3) * (x1.pow(2)) + a).modpow(&one, p);
            den = (BigInt::from(2) * y1).modpow(&BigInt::from(p - 2), p);
        } else {
            num = (y2 - y1).modpow(&one, p);
            den = (x2 - x1).modpow(&BigInt::from(p - 2), p);
        }
        let m = (&num * &den).modpow(&one, p);

        let x3 = (m.pow(2) - x1 - x2).modpow(&one, p);
        let y3 = (m * (x1 - &x3) - y1).modpow(&one, p);

        let new_point = ECPoint::create(x3, y3, self.curve.clone());
        if !new_point.is_point_on_curve() {
            panic!("New point isn't on curve")
        }

        new_point
    }

    pub fn multiply_point(&self, mut scalar: BigInt) -> Self {
        let one: BigInt = BigInt::from(1);

        let mut result = self.clone();
        let current = self.clone();

        if scalar == one {
            return result;
        }
    
        while scalar > one {
            result = result.add_point(&current);

            scalar = scalar - &one;
        }
    
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points() {
        // https://learn.ztu.edu.ua/pluginfile.php/196084/mod_resource/content/1/%D0%9B%D0%B5%D0%BA%D1%86%D1%96%D1%8F12.pdf

        // y^2 = x^3 + a*x + b (mod p)
        // y^2 = x^3 + x + 1 (mod 23)
        let curve = ECurve::create(BigInt::from(1), BigInt::from(1), BigInt::from(23));

        // P(3,10); Q(9,7); P+Q = R(17,20)
        let p = ECPoint::create(BigInt::from(3), BigInt::from(10), curve.clone());
        let q = ECPoint::create(BigInt::from(9), BigInt::from(7), curve.clone());
        let r = p.add_point(&q);
        assert_eq!(r.x, BigInt::from(17));
        assert_eq!(r.y, BigInt::from(20));

        // P(12,19); Q(5,4); P+Q = R(12,4)
        let p = ECPoint::create(BigInt::from(12), BigInt::from(19), curve.clone());
        let q = ECPoint::create(BigInt::from(5), BigInt::from(4), curve.clone());
        let r = p.add_point(&q);
        assert_eq!(r.x, BigInt::from(12));
        assert_eq!(r.y, BigInt::from(4));

        // P(5,4); P*P = R(17,20)
        let p = ECPoint::create(BigInt::from(5), BigInt::from(4), curve.clone());
        let p2 = p.add_point(&p);
        assert_eq!(p2.x, BigInt::from(17));
        assert_eq!(p2.y, BigInt::from(20));

        // P(5,4); 2*P+P = R(13,16)
        let p3 = p.add_point(&p2);
        assert_eq!(p3.x, BigInt::from(13));
        assert_eq!(p3.y, BigInt::from(16));

        // P(5,4); 2*P = R(17,20)
        let p_mul_2 = p.multiply_point(BigInt::from(2));
        assert_eq!(p_mul_2.x, BigInt::from(17));
        assert_eq!(p_mul_2.y, BigInt::from(20));

        // P(5,4); 3*P = R(13,16)
        let p_mul_3 = p.multiply_point(BigInt::from(3));
        assert_eq!(p_mul_3.x, BigInt::from(13));
        assert_eq!(p_mul_3.y, BigInt::from(16));

        // Check is P on curve
        let p = ECPoint::create(BigInt::from(4), BigInt::from(4), curve.clone());
        assert_eq!(p.is_point_on_curve(), false);
    }

    #[test]
    fn test_diffie_hellman() {
        // https://learn.ztu.edu.ua/pluginfile.php/196084/mod_resource/content/1/%D0%9B%D0%B5%D0%BA%D1%86%D1%96%D1%8F12.pdf

        // y^2 = x^3 + a*x + b (mod p)
        // y^2 = x^3 + -2 * x + 15 (mod 23)
        let curve = ECurve::create(BigInt::from(-2), BigInt::from(15), BigInt::from(23));

        // g(4,5)
        let g = ECPoint::create(BigInt::from(4), BigInt::from(5), curve.clone());

        let user_a_private_key = BigInt::from(3);
        let user_b_private_key = BigInt::from(7);

        // <user_a_private_key>*G = <user_a_public_key>(13,22)
        let user_a_public_key = g.multiply_point(user_a_private_key.clone());
        assert_eq!(user_a_public_key.x, BigInt::from(13));
        assert_eq!(user_a_public_key.y, BigInt::from(22));

        // <user_b_private_key>*G = <user_b_public_key>(13,22)
        let user_b_public_key = g.multiply_point(user_b_private_key.clone());
        assert_eq!(user_b_public_key.x, BigInt::from(17));
        assert_eq!(user_b_public_key.y, BigInt::from(8));

        // Main assertion
        // <user_b_public_key>*<user_a_private_key> = <user_a_private_key_diffie>(15,5)
        let user_a_private_key_diffie = user_b_public_key.multiply_point(user_a_private_key);
        assert_eq!(user_a_private_key_diffie.x, BigInt::from(15));
        assert_eq!(user_a_private_key_diffie.y, BigInt::from(5));

        // <user_A_public_key>*<user_b_private_key> = <user_b_private_key_diffie>(15,5)
        let user_b_private_key_diffie = user_a_public_key.multiply_point(user_b_private_key);
        assert_eq!(user_b_private_key_diffie.x, BigInt::from(15));
        assert_eq!(user_b_private_key_diffie.y, BigInt::from(5));
    }
}

// cargo test -- ec_wrapper --nocapture