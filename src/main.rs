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
// Return all different shapes of a matrix
//
fn shapes_of(m: &Matrix) -> Vec<Matrix> {
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

    let mut set: HashSet<Matrix> = HashSet::new();
    let mut result : Vec<Matrix> = Vec::new();
    for shape in &all {
        if set.insert(shape.to_vec()) {
            result.push(shape.to_vec());
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
                if (!m[y][x]) & (m[y - 1][x] | m[y + 1][x] | m[y][x - 1] |
                        m[y][x + 1]) {
                    let mut m2: Matrix = m.iter().map(|row|
                            row.to_vec()).collect();
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
// Max impl
//
fn maxi(a:usize,b:usize) -> usize {
    if a>b {a} else {b}
}

//
// Print a generation
//
fn print_gen(heading: &str, l: &Vec<Matrix>) {
    println!("{}", heading);
    for m in l {
        let l2 = shapes_of(&m);
        let mut full_h = 0;
        for s in &l2 { full_h = maxi(full_h,s.len()); }
        let mut text : Vec<String> = vec!["".to_string();full_h];
        for s in l2 {
            let h = s.len();
            let w = s[0].len();
            for y in 0..full_h {
                for x in 0..w {
                    if y<h {
                        let c = if s[y][x] { "X" } else { "." };
                        text[y] += c;
                    } else {
                        text[y] += " ";
                    }
                }
                text[y] += " ";
            }
        }
        println!("{}\n", text.join("\n"));
    }
}

//
// Return all board positions of all shapes of a matrix
//
fn all_positions_of(m : &Matrix) -> Vec<u64> {
    let two : u64 = 2;
    let l = shapes_of(&m);
    let mut result : Vec<u64> = Vec::new();
    for s in l {
        let mut y = 0;
        let mut x;
        let mut n : u64 = 0;
        for row in s  {
            x = 0;
            for cell in row  {
                if cell {
                    n += two.pow(y*10+x);
                }
                x+=1;
            }
            y+=1;
        }
        result.push(n);
    }
    return result;
}

//
// Main
//
fn main() {
    // FXYPTWNZLUVI
    let pentominos = 
             next_generation(
            &next_generation(
            &next_generation(
            &next_generation(
            &vec![vec![vec![true; 1]; 1]; 1]
            ))));
    print_gen("Pentominos", &pentominos);
    for pentomino in &pentominos {
        let positions = all_positions_of(&pentomino);
        println!("Shapes {:?}", positions);
    }
}
