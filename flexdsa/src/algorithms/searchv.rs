/*
 *
 * Binary Search
 *
 *
 */

// A search function that can be used to find or check a value if the array is sorted.
pub fn binary_search<T: Ord + Clone>(input: &Vec<T>, target: T) -> usize {
    // Create iterators for the low end and high end
    let mut low_iter: usize = 0;
    let mut high_iter: usize = input.len() - 1;

    // Loop through the input array until the low has meet the high iterator
    while low_iter <= high_iter {
        // Calculate the middle iterator and get its value
        let middle_iter: usize = low_iter + (high_iter - low_iter) / 2;
        let value = input[middle_iter].clone();

        // if the middle is lower than the target, halve the array by setting the new
        // low iterator to the middle iterator + 1.
        if value < target {
            low_iter = middle_iter + 1;
        } else if value > target {
            // if the value is higher, halve the array by setting the high iterator
            // to the middle - 1.
            high_iter = middle_iter - 1;
        } else {
            // Return iterator if found
            return middle_iter;
        }
    }

    // if not found, return a value that is more than the input length
    return input.len() + 1;
}

/*
 *
 *
 *  Interpolation Search
 *
 */

// Jandarl Note 02082025: I will make this when I have grasp Rust generics better.

// An improved function over Binary Search for uniformly distributed guesses where
// a value might be based on calculated probe result if probe is incorrect,
// search are is narrowed and a new probe is calculated.
/*
pub fn interpolation_search<T: Ord + Clone + std::ops::Sub + std::ops::Div>(
    input: &Vec<T>,
    target: T,
) -> usize {
    // Create iterators for low end and high end
    let mut low_iter: usize = 0;
    let mut high_iter: usize = input.len() - 1;

    // Loop through the input until the iterators meet
    while target >= input[low_iter] && target <= input[high_iter] && low_iter <= high_iter {
        // Compute for the probe to use as a reference to slice the input
        let probe = low_iter
            + ((high_iter - low_iter) / (input[high_iter] - input[low_iter]))
                * (target - input[low_iter]);

        // If value at iterator probe hits the target, return.
        if input[probe] == target {
            return probe;
        } else if input[probe] > target {
            // If probe is higher, adjust low iterator to probe + 1
            low_iter = probe + 1;
        } else {
            // If probe is lower, adjust higher iterator to probe - 1
            high_iter = probe - 1;
        }
    }

    // If not found, return a value higher than the length of input
    return input.len() + 1;
}*/
