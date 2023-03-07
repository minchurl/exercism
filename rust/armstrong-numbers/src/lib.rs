pub fn is_armstrong_number(num: u32) -> bool {
    let mut x: u64 = num as u64;
    let mut sum:u64 = 0;
    let mut cnt = 0;

    while x != 0 {
        cnt = cnt + 1;
        x /= 10;
    }

    x = num as u64;

    while x != 0 {
        sum = sum + (x % 10).pow(cnt);
        x /= 10;
    }

    num as u64 == sum
}
