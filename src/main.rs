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
fn all_positions_of(m : &Matrix, limit_sym:bool) -> Vec<u64> {
    let two : u64 = 2;
    let l = shapes_of(&m);
    let mut result : Vec<u64> = Vec::new();
    for s in l {
        let h = s.len();
        let w = s[0].len();
        let mut x : usize;
        let mut y : usize;
        let x1 : usize = if limit_sym { 6 } else { 10 };
        let y1 : usize = if limit_sym { 4 } else { 6 };
        for dy in 0..(y1-h+1) {
            for dx in 0..(x1-w+1) {
                let mut n : u64 = 0;
                y = dy;
                for row in &s  {
                    x = dx;
                    for cell in row  {
                        if *cell {
                            let p : u32 = (y*10+x) as u32;
                            n += two.pow(p);
                        }
                        x+=1;
                    }
                    y+=1;
                }
                result.push(n);
            }
        }
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

    println!("Shapes");
    let mut i = "FXYPTWNZLUVI".char_indices();
    let mut ordered : Vec<(usize,char,Vec<u64>)> = Vec::new();
    for pentomino in &pentominos {
        let (_,c) = i.next().unwrap();
        let limit = c == 'X';
        let positions = all_positions_of(&pentomino, limit);
        println!("{} {:?}", c, positions);
        let tuple = (positions.len(),c,positions);
        ordered.push(tuple);
    }
    ordered.sort();

    println!("");
    println!("Order");
    for (n,c,_) in &ordered {
        println!("{} {}", c, n);
    }

    let compiter = get_compiter();
    let bitcounter = get_bitcounter();

    // Max items in solution vec is 12, capacity 20 prevents pre-allocation
    find_solutions(&compiter,bitcounter,ordered,&mut Vec::with_capacity(20),0);
}

fn get_compiter() -> Vec<(u64,u64)> {
    let two : u64 = 2;
    let mut compiter : Vec<(u64,u64)> = Vec::new();
    for y in 0..6 {
        for x in 0..10 {
            let bit : u64 = two.pow((y*10+x) as u32);
            let mut around : u64 = 0;
            if x>0 { around |= two.pow((y*10+x-1) as u32); }
            if y>0 { around |= two.pow(((y-1)*10+x) as u32); }
            compiter.push( (bit,around) );
        }
    }
    compiter
}

fn get_bitcounter() -> [u8;256] {
    let mut result : [u8; 256] = [0; 256];
    for i in 0..256 {
        let mut n = i;
        let mut bits = 0;
        while n>0 {
            if 0!=(n&1) {
                bits += 1;
            }
            n = n >> 1;
        }
        result[i] = bits;
    }
    result
}

//
// Walk the solution space
//
fn find_solutions(
        compiter : &Vec<(u64,u64)>,
        bitcounter : [u8;256],
        remaining : Vec<(usize,char,Vec<u64>)>,
        solution : &mut Vec<(char,u64)>,
        current : u64) {
    if remaining.is_empty() {
        println!("Solution {:?}", solution);
    } else {
        let (_,letter,candidates) = &remaining[0];
        let others = &remaining[1..];
        for candidate in candidates {
            if 0 == (candidate & current) {
                let next = current | candidate;
                if holes_five(compiter, bitcounter, next) {
                    solution.push( (*letter, *candidate) );
                    find_solutions(compiter, bitcounter, others.to_vec(), solution, next);
                    solution.pop();
                }
            }
        }
    }
}

fn holes_of(compiter : &Vec<(u64,u64)>, board:u64) -> Vec<u64> {
    let mut existing : Vec<u64> = Vec::new();
    for (bit,around) in compiter {
        if 0 == (board & bit) {
            let (mut truthy,falsy) = existing.iter().partition(
                |r| 0 != (*r & around)
            );
            existing = falsy;
            truthy.push(*bit);
            let region = truthy.iter().copied()
                    .reduce(|a,b| a|b).unwrap();
            existing.push(region);
            // untouched, tomerge = existing.iter().partition(...)
            // existing = untouched
            // newregion = tomerge+[region].reduce(a|b)
            // existing.push(newregion)
        }
    }
    existing
}
fn holes_five(compiter:&Vec<(u64,u64)>, bitcounter:[u8;256], board:u64) -> bool {
    let existing = holes_of(compiter, board);
    let mut fiveonly : bool = true;
    for hole in &existing {
        let mut n = 0;
        let mut chk = *hole;
        while chk != 0 {
            n += bitcounter[(chk & 255) as usize];
            chk = chk >> 8;
        }
        if n%5 != 0 {
            fiveonly = false;
            break;
        }
    }
    // check that existing contains only 5 bits multiples
    return fiveonly;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_holes() {
        let compiter = get_compiter();
        let bitcounter = get_bitcounter();
        let board = board_from(
                "....X.....",
                "..X.X.....",
                "XX..X.....",
                "..XXX.....",
                "...X......",
                "....X....."
            );
        assert!(holes_five(&compiter, bitcounter, 0));
        assert!(! holes_five(&compiter, bitcounter, board));
        let b2 = board_from(
                "...XX.....",
                "..X.X.....",
                "XX..XXXXXX",
                "X..XX.....",
                ".XXXX.....",
                "....X....."
            );
        assert!(holes_five(&compiter, bitcounter, b2));
    }

    fn board_from(s1:&str,s2:&str,s3:&str,s4:&str,s5:&str,s6:&str) -> u64 {
        let two : u64 = 2;
        let mut n : u64 = 0;
        n |= conv_board_row(s1,two.pow(0));
        n |= conv_board_row(s2,two.pow(10));
        n |= conv_board_row(s3,two.pow(20));
        n |= conv_board_row(s4,two.pow(30));
        n |= conv_board_row(s5,two.pow(40));
        n |= conv_board_row(s6,two.pow(50));
        n
    }

    fn conv_board_row(row:&str,n:u64) -> u64 {
        let mut result : u64 = 0;
        let mut i : u64 = n;
        for c in row.chars() {
            if 'X' == c {
                result += i;
            }
            i*=2;
        }
        result
    }

}

