/*
 *
 *       Graph with Depth First Seach and Breadth First Search
 *
 *
 */

// Create a struct that will hold the data of the Graph
// The matrix is a single dimension vector, an algorithm will be used to calculate
// the values of the succeeding columns:
// true_value = vec.len() * row + col;
pub struct NodeMatrix<T> {
    nodes: Vec<T>,
    matrix: Vec<bool>,
}

// Implementation of the struct helper functions to be used later on our Graph
// implementaion.
impl<T> NodeMatrix<T>
where
    //for<'a> &'a T: PartialEq<T>,
    T: std::cmp::PartialEq + std::fmt::Debug,
    T: Clone,
{
    // This function will find the true index where the given valuse is stored
    // based on the given row (src) and col (dest) nodes.
    fn get_index(&self, src: T, dest: T) -> (bool, usize) {
        // Delcare all the necessary iterators and variables.
        // Since usize is being used, -1 cannot be used to initialize the iterators.
        // Instead len() + 1 will be used to make it invalid due to out of bound
        let mut found = 0;
        let mut src_ndx = self.matrix.len() + 1;
        let mut dest_ndx = self.matrix.len() + 1;
        let mut matrix_ndx = self.matrix.len() + 1;
        let mut is_found = false;

        // Enumerate through the storage of nodes to get the indexes of the
        // given row (src) node and col (dest) node.
        // If src or dest is found, increment the iterator found variable.
        for (ndx, &ref item) in self.nodes.iter().enumerate() {
            if *item == src {
                src_ndx = ndx;
                found += 1;
            }

            if *item == dest {
                dest_ndx = ndx;
                found += 1;
            }

            if found == 2 {
                break;
            }
        }

        // If both src and dest is found and valid, the found iterator should
        // be equals to 2.
        if found == 2 {
            // Apply the true_value function to get the true position of the value in the
            // matrix.
            matrix_ndx = self.nodes.len() * src_ndx + dest_ndx;
            is_found = true;
        }

        // Return boolean to indicate if found and the true matrix position.
        (is_found, matrix_ndx)
    }

    // This is the helper function that will be used as a recursive function for the
    // depth first search function in the Graph.
    fn dfs_helper(&self, src_ndx: usize, mut is_hit: &mut Vec<bool>, mut hit: &mut Vec<T>) {
        // Check if the node is already visited
        if is_hit[src_ndx] {
            return;
        } else {
            // If first visit, flip to true and add push the value at the end
            is_hit[src_ndx] = true;
            hit.push(self.nodes[src_ndx].clone());
            println!("{:?}", hit);
        }

        // Since the matrix full length is equal to the square on the length
        // of the vector of nodes. Then iterating through the vector of nodes and Then
        // apply it to the true_value function to compensate for the column (ndx) part.
        for ndx in 0..self.nodes.len() {
            let matrix_ndx = self.nodes.len() * src_ndx + ndx;

            // If the value at the matrix_ndx is a true, meaning an edge is found,
            // recurse the function using the index of the column (ndx).
            if self.matrix[matrix_ndx] {
                self.dfs_helper(ndx, &mut is_hit, &mut hit);
            }
        }
    }
}

// Declare a trait called Graph to implement functions to be used using the
// NodeMatrix
pub trait Graph<T> {
    fn create(node_count: usize) -> Self;

    fn node(&mut self, name: T);
    fn edge(&mut self, src: T, dest: T) -> bool;

    fn check(&self, src: T, dest: T) -> bool;
    fn print(&self);
    fn dfs(&self, src: T) -> Vec<T>;
    fn bfs(&self, src: T) -> Vec<T>;
}

// Implement the trait functions for the NodeMatrix
impl<T> Graph<T> for NodeMatrix<T>
where
    //for<'a> &'a T: PartialEq<T>,
    T: std::fmt::Debug + std::fmt::Display + std::cmp::PartialEq,
    T: Clone,
{
    // This function will create a NodeMatrix.
    // nodes vector will be dynamic for now while matrix will have a capacity of
    // square of node_count. Return the create NodeMatrix after creation is done.
    fn create(node_count: usize) -> NodeMatrix<T> {
        NodeMatrix::<T> {
            nodes: Vec::new(),
            matrix: vec![false; node_count * node_count],
        }
    }

    // Add a Node
    fn node(&mut self, name: T) {
        self.nodes.push(name);
    }

    // Add an Edge
    fn edge(&mut self, src: T, dest: T) -> bool {
        // Check and Get the true matrix index
        let (found, matrix_ndx) = self.get_index(src, dest);

        if found {
            // Set to true if found
            self.matrix[matrix_ndx] = true;
            return true;
        }

        false
    }

    // Check if an Edge is present
    fn check(&self, src: T, dest: T) -> bool {
        // Check and Get the true matrix index
        let (found, matrix_ndx) = self.get_index(src, dest);

        if found {
            // Return true / false if found / not found.
            if self.matrix[matrix_ndx] {
                return true;
            }
        }

        false
    }

    // Print the whoule matrix of the graph
    fn print(&self) {
        // Print the whole nodes as legend for the column.
        print!(" ");
        for item in self.nodes.iter() {
            print!("  {item}");
        }
        //println!("  {:?}", self.nodes);
        println!(" ");

        // Create iterators for the matrix and nodes.
        let mut matrix_iter = 0;
        let mut leg_iter = 0;

        // Iterate while the matrix and node iterators are valid.
        while matrix_iter < self.matrix.len() && leg_iter < self.nodes.len() {
            // Get the stop index to be used for slicing the matrix values.
            let stop_ndx = matrix_iter + self.nodes.len();

            // Get the value of the whole column by using the current iterator and
            // the stop index using the vector slice.
            let col = self.matrix[matrix_iter..stop_ndx].to_vec();

            // Get the nodes value of the iterator to be used as legend on printing rows
            let legend = self.nodes[leg_iter].clone();

            // Increment both the iterators.
            matrix_iter += self.nodes.len();
            leg_iter += 1;

            let mut printable: Vec<i32> = vec![];

            // Save the sliced vector into a printable version
            // of 1/0 instead of true/false
            for &item in col.iter() {
                printable.push(if item { 1 } else { 0 });
            }

            println!("{legend} {:?}", printable);
        }
    }

    // Implement the Depth First Search
    fn dfs(&self, src: T) -> Vec<T> {
        // Create the mutable check if a node is visited and the storage for all visited
        let mut is_hit: Vec<bool> = vec![false; self.nodes.len()];
        let mut hit: Vec<T> = vec![];

        // Check and Get the true matrix index
        let (found, matrix_ndx) = self.get_index(self.nodes[0].clone(), src);

        if found {
            // Apply dfs helper if found to travel through the matrix
            self.dfs_helper(matrix_ndx, &mut is_hit, &mut hit);
        }

        hit
    }

    // Implement the Breadth First Search
    fn bfs(&self, src: T) -> Vec<T> {
        // Create the mutable storage for the visited checker, the visited index queue
        // and the return vector
        let mut retval: Vec<T> = vec![];
        let mut hit_queue: Vec<usize> = vec![];
        let mut is_hit: Vec<bool> = vec![false; self.nodes.len()];

        // Check and Get the true matrix index. This time use the first node as row
        // and the src as column.
        let (found, matrix_ndx) = self.get_index(self.nodes[0].clone(), src);

        if found {
            // If found push the matrix index at the end of the queue and set the
            // visited as true.
            hit_queue.push(matrix_ndx);
            is_hit[matrix_ndx] = true;

            // Iterate until the queue is not empty
            while hit_queue.len() != 0 {
                // Get the matrix index at the top of the queue and push it at the
                // end of the return vector. Remove the index from the queue.
                let matrix_ndx = hit_queue[0].clone();
                hit_queue.remove(0);
                retval.push(self.nodes[matrix_ndx].clone());
                println!("{:?}", retval);

                // Since the matrix full length is equal to the square on the length
                // of the vector of nodes. Then iterating through the vector of
                // nodes and then apply it to the true_value function to compensate
                // for the column (ndx) part.
                for ndx in 0..self.nodes.len() {
                    let matrix_ndx = self.nodes.len() * matrix_ndx + ndx;

                    // If an Edge is found and is not yet visited, push the column (ndx)
                    // at the end of the visited queue and set the visited to true.
                    if self.matrix[matrix_ndx] && is_hit[ndx] == false {
                        hit_queue.push(ndx);
                        is_hit[ndx] = true;
                    }
                }
            }
        }

        retval
    }
}
