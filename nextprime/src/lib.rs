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

pub fn is_prime(nbr: u64) -> bool {
    if nbr <= 1 {
        return false;
    }
    if nbr == 2 {
        return true;
    }
    for i in 2..nbr-1 {
        if nbr%i == 0{
            return false
        }
    }
    return true;
}