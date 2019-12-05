fn digits(mut n: u32) -> Vec<u32> {
    let mut ds = vec![];
    loop {
        ds.push(n % 10);
        n /= 10;

        if n == 0 {
            return ds;
        }
    }
}

#[test]
fn test_digits() {
    assert_eq!(digits(0), vec![0]);
    assert_eq!(digits(1), vec![1]);
    assert_eq!(digits(9), vec![9]);
    assert_eq!(digits(10), vec![0, 1]);
    assert_eq!(digits(1234), vec![4,3,2,1]);
}

fn p1(n: u32) -> bool {
    let ds = digits(n);
    let mut two_consecutive = false;
    for i in 1 .. ds.len() {
        if ds[i] > ds[i-1] {
            return false;
        }
        two_consecutive = two_consecutive || (ds[i] == ds[i-1]);
    }
    return two_consecutive;
}

fn p2(n: u32) -> bool {
    let ds = digits(n);
    let mut consecutive = [0; 10];
    for i in 1 .. ds.len() {
        if ds[i] > ds[i-1] {
            return false;
        }
        if ds[i] == ds[i-1] {
            consecutive[ds[i] as usize] += 1;
        }
    }
    return consecutive.iter().any(|x| *x == 1);
}

#[test]
fn test_p1() {
    assert_eq!(true, p1(111111));
    assert_eq!(false, p1(223450));
    assert_eq!(false, p1(123789));
}

#[test]
fn test_p2() {
    assert_eq!(true, p2(112233));
    assert_eq!(false, p2(123444));
    assert_eq!(true, p2(111122));
}

fn main() {
    let okay = (234208 ..= 765869).filter(|x| p1(*x)).count();
    let double_okay = (234208 ..= 765869).filter(|x| p2(*x)).count();
    println!("{} {}", okay, double_okay);
}
