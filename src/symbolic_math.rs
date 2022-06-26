pub fn symbolic_add(v: &Vec<u8>, u: &Vec<u8>) -> Vec<u8> {
    let mut top = v.clone();
    let mut bottom = u.clone();
    top.reverse();
    bottom.reverse();
    while top.len() < bottom.len() {
	top.push(b'0');
    }
    while bottom.len() < top.len() {
	bottom.push(b'0');
    }
    let mut result = Vec::with_capacity(top.len() + 1);
    let mut carry = 0;
    for i in 0..top.len() {
	let next = (top[i] - b'0') + (bottom[i] - b'0') + carry;
	result.push(next % 10 + b'0');
	if next >= 10 {
	    carry = 1;
	} else {
	    carry = 0;
	}
    }
    if carry == 1 {
	result.push(b'1');
    }
    result.reverse();
    result
}

pub fn symbolic_multiply(u: &Vec<u8>, v: &Vec<u8>) -> Vec<u8> {
    let mut result = vec![];
    let mut bottom = v.clone();
    bottom.reverse();
    for i in 0..bottom.len() {
	let mut digit_product = single_digit_symbolic_multiply(&u, bottom[i]);
	for _ in 0..i {
	    digit_product.push(b'0')
	}
	result = symbolic_add(&result, &digit_product);
    }
    result
}

pub fn symbolic_factorial(n: u32) -> Vec<u8> {
    let mut cur_product = vec![b'1'];
    for i in 2..=n {
	cur_product = symbolic_multiply(&cur_product,
					&num_to_symbolic_num(i.into()));
    }
    cur_product
}

pub fn sum_symbolic_digits(digits: &Vec<u8>) -> u64 {
    let mut sum = 0;
    for digit in digits {
	sum += (digit - b'0') as u64;
    }
    sum
}

fn num_to_symbolic_num(num: u64) -> Vec<u8> {
    let mut result = Vec::<u8>::new();
    let mut n = num;
    while n > 0 {
	let digit: u8 = (n % 10) as u8;
	result.push(digit + b'0');
	n /= 10;
    }
    result.reverse();
    result
}
    

fn single_digit_symbolic_multiply(multiplier: &Vec<u8>, digit: u8) -> Vec<u8> {
    if digit == b'0' {
	return vec![b'0'];
    }
    let m = digit - b'0';
    let mut top = multiplier.clone();
    top.reverse();
    let mut result = Vec::with_capacity(top.len() + 1);
    let mut carry = 0;
    for d in top {
	let next_value = (d - b'0') * m + carry;
	result.push(next_value % 10 + b'0');
	carry = next_value / 10;
    }
    if carry != 0 {
	result.push(carry + b'0');
    }
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbolic_add() {
	let v = vec![b'1', b'2', b'3'];
	let u = vec![b'9', b'9', b'9'];
	assert_eq!(symbolic_add(&v, &u), vec![b'1', b'1', b'2', b'2']);
	let w = vec![];
	assert_eq!(symbolic_add(&v, &w), v);
    }

    #[test]
    fn test_single_digit_symbolic_multiply() {
	let multiplier = vec![b'9', b'8'];
	let multiplicand = b'9';
	assert_eq!(single_digit_symbolic_multiply(&multiplier, multiplicand),
		   vec![b'8', b'8', b'2']);
	let multiplier = vec![b'8', b'8'];
	let multiplicand = b'0';
	assert_eq!(single_digit_symbolic_multiply(&multiplier, multiplicand),
		   vec![b'0']);
    }

    #[test]
    fn test_symbolic_multiply() {
	let multiplier = vec![b'9', b'8'];
	let multiplicand = vec![b'7', b'6'];
	assert_eq!(symbolic_multiply(&multiplier, &multiplicand),
		   vec![b'7', b'4', b'4', b'8']);
	let multiplier = vec![b'7', b'3'];
	let multiplicand = vec![b'5', b'7'];
	assert_eq!(symbolic_multiply(&multiplier, &multiplicand),
		   vec![b'4', b'1', b'6', b'1']);
    }

    #[test]
    fn test_num_to_symbolic_num() {
	let num = 93534;
	assert_eq!(num_to_symbolic_num(num),
		   vec![b'9', b'3', b'5', b'3', b'4']);
    }

    #[test]
    fn test_symbolic_factorial() {
	assert_eq!(symbolic_factorial(10),
		   num_to_symbolic_num(3628800));
    }

    #[test]
    fn test_sum_symbolic_digits() {
	assert_eq!(sum_symbolic_digits(&num_to_symbolic_num(12345)),
		   15);
    }
}
