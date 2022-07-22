fn main() {
    // Higher Order Functions takes one of more functions
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    let upper = 1000;
    let mut acc = 0;
    
    // imperitive style
    for n in 0.. { // infinity
        let n_square = n * n;
        if n_square >= upper {
            break;
        } else if is_odd(n_square){
            acc += n_square;
        }
    }
    println!("imperitive style : {}", acc);

    // Functional approach
    let sum_of_squared_odd_number: u32 = 
        (0..).map(|n| n*n)
            .take_while(|&n_sqaured| n_sqaured < upper)
            .filter(|&n_sqaured| is_odd(n_sqaured))
            .fold(0, |acc, n_sqaured| acc + n_sqaured);
    println!("funcational approach : {}", sum_of_squared_odd_number);
}
