use std::collections::HashSet;

//
// A naive 2d matrix
//
type Matrix = Vec<Vec<bool>>;

//
// Add rows and columns of false around the matrix
// inflating its dimensions by +1 in each direction
//
fn inflate(source: &Matrix) -> Matrix {
    let mut result: Matrix = source
        .iter()
        .map(|row| {
            let mut cloned = row.to_vec();
            cloned.push(false);
            cloned.insert(0, false);
            return cloned;
        })
        .collect();
    let n = result[0].len();
    result.push(vec![false; n]);
    result.insert(0, vec![false; n]);
    return result;
}

//
// Reduce all falsy border lines and columns
//
fn deflate(source: &mut Matrix) {
    while !source[0].iter().copied().reduce(|a, b| a | b).unwrap() {
        source.remove(0);
    }
    while !source[source.len() - 1]
        .iter()
        .copied()
        .reduce(|a, b| a | b)
        .unwrap()
    {
        source.remove(source.len() - 1);
    }
    loop {
        let col_0 = source
            .iter()
            .map(|row| row[0])
            .reduce(|a, b| a | b)
            .unwrap();
        if !col_0 {
            for row in source.iter_mut() {
                row.remove(0);
            }
        }
        if col_0 {
            break;
        }
    }
    loop {
        let col_n = source
            .iter()
            .map(|row| row[row.len() - 1])
            .reduce(|a, b| a | b)
            .unwrap();
        if !col_n {
            for row in source.iter_mut() {
                row.remove(row.len() - 1);
            }
        }
        if col_n {
            break;
        }
    }
}

//
// Rotate
//
fn rotate(m: &Matrix) -> Matrix {
    let h = m.len();
    let w = m[0].len();
    let mut result: Matrix = Vec::new();
    for x in 0..w {
        result.push(Vec::new());
        for y in 0..h {
            result[x].push(m[y][w - x - 1]);
        }
    }
    return result;
}

//
// Return a rotation and symmetry invariant matrix
//
fn canonical(m: &Matrix) -> Matrix {
    let m0 = m.iter().map(|r| r.to_vec()).collect();
    let m1 = rotate(&m0);
    let m2 = rotate(&m1);
    let m3 = rotate(&m2);
    let m4 = m.iter().map(|r| r.to_vec()).rev().collect();
    let m5 = rotate(&m4);
    let m6 = rotate(&m5);
    let m7 = rotate(&m6);

    let mut all = vec![m0, m1, m2, m3, m4, m5, m6, m7];
    all.sort();
    return all.pop().unwrap();
}

//
// Compute the next shapes generation
//
fn next_generation(current: &Vec<Matrix>) -> Vec<Matrix> {
    let mut result: Vec<Matrix> = Vec::new();
    let mut set: HashSet<Matrix> = HashSet::new();
    for shape in current {
        let h = shape.len();
        let w = shape[0].len();
        let m = inflate(&inflate(&shape));
        for y in 1..(h + 3) {
            for x in 1..(w + 3) {
                if (!m[y][x]) & (m[y - 1][x] | m[y + 1][x]
                        | m[y][x - 1] | m[y][x + 1]) {
                    let mut m2: Matrix = 
                            m.iter().map(|row| row.to_vec()).collect();
                    m2[y][x] = true;
                    deflate(&mut m2);
                    let m3 = canonical(&m2);
                    if set.insert(m3.to_vec()) {
                        result.push(m3);
                    }
                }
            }
        }
    }
    return result.to_vec();
}

//
// Return a nice displayable string
//
fn matrix_to_string(m: &Matrix) -> String {
    let rows: Vec<String> = m
        .iter()
        .map(|row| {
            let line: Vec<String> = row
                .iter()
                .map(|b| (if *b { "X" } else { "." }).to_string())
                .collect();
            return line.join("");
        })
        .collect();
    return rows.join("\n");
}

//
// Print a generation
//
fn print_gen(heading: &str, l: &Vec<Matrix>) {
    println!("{}", heading);
    for shape in l {
        let s = matrix_to_string(&shape);
        println!("{}", s);
        println!("");
    }
}

//
// Main
//
fn main() {
    let pentominos = 
        next_generation(
        &next_generation(
        &next_generation(
        &next_generation(
        &vec![vec![vec![true; 1]; 1];1]
        ))));

    print_gen("Pentominos", &pentominos);
}
