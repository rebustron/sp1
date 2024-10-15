#![no_main]
sp1_zkvm::entrypoint!(main);

// use p256::elliptic_curve::group::prime::PrimeCurveAffine;
// use p256::elliptic_curve::group::GroupEncoding;
// use p256::elliptic_curve::{CurveArithmetic, PrimeCurveArithmetic};
// use p256::primeorder::{point_arithmetic, PrimeCurveParams};
// use hex_literal::hex;
// use p256::elliptic_curve::sec1::{FromEncodedPoint, ToCompactEncodedPoint, ToEncodedPoint};
// use p256::{AffinePoint, EncodedPoint};
// use p256::{Scalar, U256};
use sp1_curves::params::FieldParameters;
use sp1_zkvm::lib::secp256r1::Secp256r1Point;
// use sp1_zkvm::syscalls::syscall_secp256r1_add;

pub fn main() {
    // generator.
    // 48439561293906451759052585252797914202762949526041747995844080717082404635286
    // 36134250956749795798585127919587881956611106672985015071877198253568414405109

    const A: [u8; 64] = [
        150, 194, 152, 216, 69, 57, 161, 244, 160, 51, 235, 45, 129, 125, 3, 119, 242, 64, 164, 99,
        229, 230, 188, 248, 71, 66, 44, 225, 242, 209, 23, 107, 245, 81, 191, 55, 104, 64, 182,
        203, 206, 94, 49, 107, 87, 51, 206, 43, 22, 158, 15, 124, 74, 235, 231, 142, 155, 127, 26,
        254, 226, 66, 227, 79,
    ];

    // 2 * generator.
    // 56515219790691171413109057904011688695424810155802929973526481321309856242040
    // 3377031843712258259223711451491452598088675519751548567112458094635497583569
    const B: [u8; 64] = [
        120, 153, 102, 71, 252, 72, 11, 166, 53, 27, 242, 119, 226, 105, 137, 192, 195, 26, 181, 4,
        3, 56, 82, 138, 126, 79, 3, 141, 24, 123, 242, 124, 209, 115, 120, 34, 157, 183, 4, 158,
        41, 130, 233, 60, 230, 173, 125, 186, 219, 48, 116, 159, 198, 154, 61, 41, 64, 208, 142,
        219, 16, 85, 119, 7,
    ];

    // 3 * generator.
    // 42877656971275811310262564894490210024759287182177196162425349131675946712428
    // 61154801112014214504178281461992570017247172004704277041681093927569603776562
    const C: [u8; 64] = [
        108, 253, 231, 198, 27, 102, 65, 251, 133, 169, 173, 239, 33, 183, 198, 230, 101, 241, 75,
        29, 149, 239, 247, 200, 68, 10, 51, 166, 209, 228, 203, 94, 50, 80, 125, 162, 39, 177, 121,
        154, 61, 184, 79, 56, 54, 176, 42, 216, 236, 162, 100, 26, 206, 6, 75, 55, 126, 255, 152,
        73, 12, 100, 52, 135,
    ];

    // Tests A + B == C, sum of points of infinity, A + A == 2 * A, and A + (-A) == infinity.
    common_test_utils::weierstrass_add::test_weierstrass_add::<
        Secp256r1Point,
        { sp1_lib::secp256r1::N },
    >(&A, &B, &C, sp1_curves::weierstrass::secp256r1::Secp256r1BaseField::MODULUS);
}
