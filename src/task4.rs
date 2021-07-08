const MAX_SIZE: usize = 1000;
#[allow(dead_code)]
pub fn nth_prime(number: u32) -> u32 {
    if number > MAX_SIZE as u32 {
        return 0;
    }
    let mut prime_array: [u32; MAX_SIZE] = [0; MAX_SIZE];

    let mut is_prime_number = true;
    let mut candidate = 2;
    let mut index = 0;

    loop {
        for j in 2..candidate / 2 + 1 {
            if candidate % j == 0 {
                is_prime_number = false;
            }
        }
        if is_prime_number {
            prime_array[index] = candidate;
            if index == number as usize {
                return prime_array[index];
            }
            index += 1;
        }
        candidate += 1;
        is_prime_number = true;
    }
}
