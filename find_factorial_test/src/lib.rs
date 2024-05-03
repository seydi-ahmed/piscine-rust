pub fn factorial(num: u64) -> u64 {
    if num == 1{
        return 1;
    }
    if num == 0{
        return 0;
    }

    let mut res : u64 = 1;
    
    for nbr in 1..num+1{
        res = res * nbr;
    }
    return res;
}