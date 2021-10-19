//n - 1 = (2^k) * q
//a^q と a^((2^k) * q)を調べる
fn main() {
    loop {
        let n = read::<i128>();
        if n == 0 {
            break;
        }
        let mut a = Vec::new();
        if 100 < n - 1 {
            for i in 1..100 {
                a.push(i as i128);
            }
        } else {
            for i in 1..n {
                a.push(i as i128);
            }
        }
        //println!("{:?}", a);
        let k = max_beki(n);
        let  q = (n - 1) / (pow(2, max_beki(n)));
        let mut cnt = 0;
        let mut switch = false;
        for i in a {
            if mod_pow(i, q, n) != 1 {
                for j in 0..k {
                    if mod_pow(i, pow(2, j as i128) * q, n) != n - 1 {
                        cnt += 1;
                    }
                }
                if cnt == k {
                    switch = true;
                }
            }
            cnt = 0;
        }
        if switch {
            println!("{}", "not prime!");
        } else {
            println!("{}", "prime!!");
        }
    }
}
fn mod_pow(a: i128, b: i128, m: i128) -> i128 {
    let mut res = 1;
    let mut x = a % m;
    let mut n = b % m;
    while n > 0 {
        if n % 2 == 1 {
            res *= x;
        }
        res %= m;
        x *= x;
        x %= m;
        n >>= 1;
    }
    res
}
fn pow(a: i128, b: i128) -> i128 {
    let mut res = 1;
    let mut x = a;
    let mut n = b;
    while n > 0 {
        if n % 2 == 1 {
            res *= x;
        }
        x *= x;
        n >>= 1;
    }
    res
}
fn max_beki(n: i128) -> i128 {
    let mut m = n - 1;
    let mut res = 0;
    while m >= 0 {
        if m % 2 == 0 {
            res += 1;
            m = m / 2;
        } else {
            break;
        }
    }
    res
}
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
