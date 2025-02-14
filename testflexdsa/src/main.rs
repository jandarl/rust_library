use flexdsa::algorithms::graphv::Graph;
use flexdsa::graphv;
use flexdsa::searchv;
use flexdsa::sortv;

fn main() {
    //env::set_var("RUST_BACKTRACE", "full");

    let mut input: Vec<i32> = Vec::new();
    input.push(5);
    input.push(3);
    input.push(1);
    input.push(4);
    input.push(7);
    input.push(2);
    input.push(0);
    input.push(6);

    println!("{:?}", input);

    let output = sortv::<i32>(&input, false);

    println!("{:?}", output);

    let ndx_output = searchv::<i32>(&output, 7);

    println!("{ndx_output}");

    let mut matrix = graphv::<char>(7);
    matrix.node('A');
    matrix.node('B');
    matrix.node('C');
    matrix.node('D');
    matrix.node('E');
    matrix.node('F');
    matrix.node('G');

    matrix.edge('A', 'C');
    matrix.edge('A', 'D');
    matrix.edge('B', 'E');
    matrix.edge('C', 'F');
    matrix.edge('D', 'C');
    matrix.edge('E', 'A');
    matrix.edge('F', 'E');
    matrix.edge('G', 'B');

    matrix.print();

    println!("A,C edge is {}", matrix.check('A', 'C'));
    println!("C,A edge is {}", matrix.check('C', 'A'));

    let dfs = matrix.dfs('A');
    println!("{:?}", dfs);

    let bfs = matrix.bfs('A');
    println!("{:?}", bfs);
}
