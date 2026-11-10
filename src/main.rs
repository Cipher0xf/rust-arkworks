use ark_bn254::Fr;
use ark_ff::Field;

fn evaluate(coeffs: &[Fr], x: Fr) -> Fr {
    coeffs
        .iter()
        .rev()
        .fold(Fr::from(0u64), |acc, &c| acc * x + c)
}

fn main() {
    let a = Fr::from(3u64);
    let b = Fr::from(5u64);

    let sum = a + b;
    let product = a * b;
    let power = a.pow([4u64]);

    assert_eq!(sum, Fr::from(8u64));
    assert_eq!(product, Fr::from(15u64));
    assert_eq!(power, Fr::from(81u64));

    let inverse = a.inverse().expect("3 is nonzero");
    assert_eq!(a * inverse, Fr::from(1u64));

    println!("All field checks passed.");
}
