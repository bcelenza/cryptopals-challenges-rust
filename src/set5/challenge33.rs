#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use crate::cipher::diffie_hellman;

    #[test]
    fn test_solution() {
        // start with prime base and modulus
        let p = BigUint::from_bytes_be(b"ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca237327ffffffffffffffff");
        let g = BigUint::from_bytes_be(b"2");

        // generate public/private keys for a and b
        let a_private = diffie_hellman::new_private_key(1024);
        let a_public = diffie_hellman::new_public_key(&a_private, &g, &p);

        let b_private = diffie_hellman::new_private_key(1024);
        let b_public = diffie_hellman::new_public_key(&b_private, &g, &p);

        // generate a shared key
        let shared_key = diffie_hellman::generate_shared_key(&a_private, &b_public, &g);

        // the shared key should match if generated in the other direction
        assert_eq!(shared_key, diffie_hellman::generate_shared_key(&b_private, &a_public, &g));
    }
}
