pub fn next_prime(nbr: u64) -> u64 {
    if is_prime(nbr){
        return nbr;
    }
    let mut res = nbr;
    while !is_prime(res){
        res += 1;
    }
    res
}


fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut i = 3;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}