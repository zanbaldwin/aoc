// Take from the Wikipedia page for Greatest Common Divisor
pub fn gcd(a: i64, b: i64) -> i64 {
    let mut b = b.wrapping_abs() as u32;
    if a == 0 {
        return b as i64;
    }
    let mut a = a.wrapping_abs() as u32;
    if b == 0 {
        return a as i64;
    }
    let gcd_exponent_on_two = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    b >>= b.trailing_zeros();

    while a != b {
        if a < b {
            core::mem::swap(&mut a, &mut b);
        }
        a -= b;
        a >>= a.trailing_zeros();
    }
    (a << gcd_exponent_on_two) as i64
}

pub fn lcm_n(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm_n(&nums[1..]);
    (a * b) / gcd(a, b)
}
