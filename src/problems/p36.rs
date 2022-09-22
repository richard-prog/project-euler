pub fn p36() -> u64 {
    let decimal_palindromes = enumerate_palindromes(6);
    sum_binary_palindromes(decimal_palindromes)
}

fn enumerate_palindromes(num_digits: usize) -> Vec<u32> {
    let mut palindrome_vec: Vec<Vec<u32>> = Vec::with_capacity(num_digits);
    palindrome_vec.push((1..=9).collect());
    palindrome_vec.push((1..=9).map(|x| x * 11).collect());

    for cur_digits in 3..=num_digits {
        let inner_palindromes = &palindrome_vec[cur_digits - 3];
        let mut new_palindromes: Vec<u32> = Vec::with_capacity(inner_palindromes.len() * 10);
        let base_palindrome = 10_u32.pow(cur_digits as u32 - 1) + 1;
        let base_palindromes: Vec<u32> = (1..=9).map(|x| base_palindrome * x).collect();
        let shift = 10;
        for &p in &base_palindromes {
            new_palindromes.push(p);
            for q in inner_palindromes {
                new_palindromes.push(p + q * shift);
            }
        }

        palindrome_vec.push(new_palindromes);
    }

    palindrome_vec.into_iter().flatten().collect()
}

fn sum_binary_palindromes(decimal_palindromes: Vec<u32>) -> u64 {
    let mut even_masks: Vec<u32> = Vec::new();
    let mut odd_masks = Vec::from([1]);
    let mut checking_odd = true;
    let mut num_bits = 1;
    let mut upper_limit = 2;
    let mut sum = 0;

    for d in decimal_palindromes {
        if d >= upper_limit {
            let new_mask = (1 << num_bits) + 1;
            num_bits += 1;
            upper_limit <<= 1;
            checking_odd = !checking_odd;
            if checking_odd {
                update_masks(&mut odd_masks, new_mask);
            } else {
                update_masks(&mut even_masks, new_mask);
            }
        }
        if checking_odd && is_binary_palindrome(d, &odd_masks) {
            sum += d;
        }
        if !checking_odd && is_binary_palindrome(d, &even_masks) {
            sum += d;
        }
    }
    sum as u64
}

fn update_masks(masks: &mut Vec<u32>, new_mask: u32) {
    for mask in masks.iter_mut() {
        *mask <<= 1;
    }
    masks.push(new_mask);
}

fn is_binary_palindrome(num: u32, masks: &Vec<u32>) -> bool {
    for &mask in masks {
        let masked_num = num & mask;
        if masked_num != 0 && masked_num != mask {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        assert_eq!(p36(), 872187);
    }

    #[test]
    fn test_is_binary_palindrome() {
        let masks = Vec::from([
            0b0000110000,
            0b0001001000,
            0b0010000100,
            0b0100000010,
            0b1000000001,
        ]);
        assert!(is_binary_palindrome(585, &masks));
        assert!(!is_binary_palindrome(584, &masks));
    }

    #[test]
    fn test_is_binary_palindrome2() {
        let masks = Vec::from([0b1001, 0b0110]);
        assert!(!is_binary_palindrome(11, &masks));
    }
}
