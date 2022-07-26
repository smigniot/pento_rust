//
// A naive 2d matrix
//
type Matrix = Vec<Vec<bool>>;

//
// Add rows and columns of false around the matrix
// inflating its dimensions by +1 in each direction
//
fn inflate(source : &Matrix) -> Matrix {
    let mut result : Matrix = source.iter().map(|row| {
        let mut cloned = row.to_vec();
        cloned.push(false);
        cloned.insert(0, false);
        return cloned;
    }).collect();
    let n = result[0].len();
    result.push(vec![false;n]);
    result.insert(0,vec![false;n]);
    return result;
}

//
// Compute the next shapes generation
//
fn next_generation(current : &Vec<Matrix>) -> Vec<Matrix> {
    return current.iter().map(|shape| {
        inflate(&shape)
    }).collect();
}

fn matrix_to_string(m:&Matrix) -> String {
    let rows : Vec<String> =  m.iter().map(|row| {
        let line : Vec<String> = row.iter().map(|b| {
            (if *b { "X" } else { "." }).to_string()
        }).collect();
        return line.join("");
    }).collect();
    return rows.join("\n");
}

fn main() {
    let atom = vec![vec![true;1];1];
    let gen2 = next_generation(&vec![atom.to_vec();1]);
    let gen3 = next_generation(&gen2);

    println!("Generation 1 = {:?}", 
            matrix_to_string(&atom));
    let l2:Vec<String> = gen2.iter().map(matrix_to_string).collect();
    println!("Generation 2 = {:?}", l2);
    let l3:Vec<String> = gen3.iter().map(matrix_to_string).collect();
    println!("Generation 3 = {:?}", l3);
}
