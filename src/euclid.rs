// Greatest Common Divisor (Maior Divisor Comum)
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    // Garante que não é zero
    assert!(n != 0 && m != 0);
    while m != 0 {
        // Se m for menor que n, trocam os valores
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n; // Calcula o resto da divisão de m por n e atribui este valor a m.
    }
    n
}

#[test] // O #[test] faz com que isso seja pulado ao compilar normalmente
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
