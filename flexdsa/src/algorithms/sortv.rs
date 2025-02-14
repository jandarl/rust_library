/*
 *
 *
 *        MERGE SORT FUNCTION
 *
 *
 *
 */

// First let us create a merge function that will merge the two sorted halves.
// An input of the left sorted Vector and right sorted Vector will then be
// joined in a sorted manner.

// We are going to make this function a polymorph to let it reusable
// for many different data types. We add Ord and Clone as we would need to use
// ordering and data cloning for this function with the use of the left and right
// vectors.
fn merge<T: Ord + Clone>(left: &Vec<T>, right: &Vec<T>) -> Vec<T> {
    // Let us define the iterators for both the left and right vectors
    // to help us track where we are currently on each of the vectors.
    let mut l_iter = 0;
    let mut r_iter = 0;

    // Here let us define the vector that we will use to join the two vectors
    // and return at the end.
    let mut result: Vec<T> = Vec::new();

    // Loop while the left and right iterators has not reached
    // the end of their respective vectors.
    while l_iter < left.len() && r_iter < right.len() {
        // Test if left value is lower than right value
        // if lower, then push left value to the result and increase left iterator
        // if not, then push right value to the result and increase right iterator
        if left[l_iter] < right[r_iter] {
            result.push(left[l_iter].clone());
            l_iter += 1;
        } else {
            result.push(right[r_iter].clone());
            r_iter += 1;
        }
    }

    // If right vector is empty but left is not, iterate through left and push
    // all the contents to the result until its empty
    while l_iter < left.len() {
        result.push(left[l_iter].clone());
        l_iter += 1;
    }

    // If left vector is empty but right is not, iterate through right and push
    // all the contents to the result until its empty.
    while r_iter < right.len() {
        result.push(right[r_iter].clone());
        r_iter += 1;
    }

    // Return sorted result vector
    result
}

// The merge sort function will be a polymorph that can be used on different data types
// We add the Ord and Clone as Ordering and Cloning will be needed by the function
pub fn merge_sort<T: Ord + Clone>(input: &Vec<T>) -> Vec<T> {
    // Test for an edge, if input length is less than 2, no need to sort, just return.
    if input.len() < 2 {
        input.to_vec()
    } else {
        // Get the middle of the input to seperate the left and right
        let middle = input.len() / 2;

        // Recursively merge sort the left side and return the merge sorted result
        let left = merge_sort::<T>(&input[..middle].to_vec());

        // Recursively merge sort the right side and return the merge sorted result
        let right = merge_sort::<T>(&input[middle..].to_vec());

        // Merge the left and the right halves and return the merged vector
        let merged = merge::<T>(&left, &right);

        // Return merged sorted vector
        merged
    }
}

/*
 *
 *          QUICKSORT FUNCTION
 *
 *
 */

// We will partition the input into smaller parts and sort them using a chosen
// pivot. Every partition will return the end index as a new pivot for the next partition

pub fn quick_sort<T: Ord + Clone>(input: &Vec<T>) -> Vec<T> {
    // Test for an edge, if input length is less than 2, just return
    if input.len() < 2 {
        input.to_vec()
    } else {
        // Use the first 0th index value as the pivot
        let pivot = input[0].clone();

        // Define the left and right storage for each partition
        let mut left: Vec<T> = Vec::new();
        let mut right: Vec<T> = Vec::new();

        // Loop through the input and push the lower value into left partition
        // and higher into right partition
        for ndx in 1..input.len() {
            if input[ndx] < pivot {
                left.push(input[ndx].clone());
            } else {
                right.push(input[ndx].clone());
            }
        }

        // Create a storage for the return result
        let mut result: Vec<T> = Vec::new();

        // Recurse through using the left partition and append
        // the return vector into the result.
        result.append(&mut quick_sort::<T>(&left));

        // Push the pivot used into the result as the middle value
        result.push(pivot);

        // Recurse through using the right partition and append
        // the reutrn vector into the result.
        result.append(&mut quick_sort::<T>(&right));

        // Return the result.
        result
    }
}
