pub mod algorithms;
use algorithms::graphv::Graph;
use algorithms::graphv::NodeMatrix;

pub fn sortv<T: Ord + Clone + std::fmt::Debug>(input: &Vec<T>, merge: bool) -> Vec<T> {
    if merge {
        algorithms::sortv::merge_sort::<T>(&input)
    } else {
        algorithms::sortv::quick_sort::<T>(&input)
    }
}

pub fn searchv<T: Ord + Clone>(input: &Vec<T>, target: T) -> usize {
    algorithms::searchv::binary_search::<T>(&input, target)
}

pub fn graphv<T: Clone + std::fmt::Debug + std::fmt::Display + std::cmp::PartialEq>(
    node_count: usize,
) -> NodeMatrix<T>
//where
    //`for<'a> &'a T: PartialEq<T>,
   // T: std::fmt::Debug + std::fmt::Display + std::cmp::PartialEq,
   // T: Clone,
{
    let new_graph: NodeMatrix<T> = Graph::<T>::create(node_count);
    new_graph
}
