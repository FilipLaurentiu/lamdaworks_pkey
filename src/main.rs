use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::{short_weierstrass::curves::bls12_381::curve::BLS12381Curve, traits::IsEllipticCurve};
use lambdaworks_math::unsigned_integer::element::UnsignedInteger;

/*
Using lambdaworks, compute the public key associated with the secret key 0x6C616D6264617370 with the BLS12-381 elliptic curve.
 */
fn main() {
    let priv_key: UnsignedInteger<2> = UnsignedInteger::from("0x6C616D6264617370");
    let G = BLS12381Curve::generator();
    let pub_key = G.operate_with_self(priv_key).to_affine();
    println!("Pub key:");
    println!("({},{})", pub_key.x(), pub_key.y());
}
