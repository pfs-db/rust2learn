// Problem definition
// The Collatz Sequence is defined as follows, for an arbitrary n1 greater than zero:

// If ni is 1, then the sequence terminates at ni.
// If ni is even, then ni+1 = ni / 2.
// If ni is odd, then ni+1 = 3 * ni + 1.
// For example, beginning with n1 = 3:

// 3 is odd, so n2 = 3 * 3 + 1 = 10;
// 10 is even, so n3 = 10 / 2 = 5;
// 5 is odd, so n4 = 3 * 5 + 1 = 16;
// 16 is even, so n5 = 16 / 2 = 8;
// 8 is even, so n6 = 8 / 2 = 4;
// 4 is even, so n7 = 4 / 2 = 2;
// 2 is even, so n8 = 1; and
// the sequence terminates.
// Write a function to calculate the length of the collatz sequence for a given initial n.

/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    if n == 1 {
        n.try_into().unwrap()
    } else if (n % 2) == 0 {
        (n / 2).try_into().unwrap()
    } else if ((n + 1) % 2) == 0 {
        (3 * n + 1).try_into().unwrap()
    } else {
        todo!("Implement")
    }
}

fn main() {
    println!("ni: {}", collatz_length(5));
}
