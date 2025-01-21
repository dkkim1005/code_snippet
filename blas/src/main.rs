use blas::dgemm;

fn main() {
    let a: Vec<f64> = vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    ];
    let b: Vec<f64> = vec![
        9.0, 8.0, 7.0,
        6.0, 5.0, 4.0,
        3.0, 2.0, 1.0,
    ];
    let mut c: Vec<f64> = vec![0.0; 9];

    let m = 3;
    let k = 3;
    let n = 3;
    let alpha = 1.0;
    let beta = 0.0;

    unsafe {
        dgemm(
            b'T',
            b'T',
            m,
            n,
            k,
            alpha,
            &a,
            m,
            &b,
            k,
            beta,
            &mut c,
            m,
        );
    }

    let c = transpose(&c, m.try_into().unwrap(), k.try_into().unwrap());
    print_matrix(&c, m.try_into().unwrap(), k.try_into().unwrap());
    // println!("Resulting matrix C: {:?}", c);
}

fn transpose(mat: &Vec<f64>, rows: usize, cols: usize) -> Vec<f64> {
    let mut ret_mat: Vec<f64> = vec![0.0; rows * cols];
    for i in 0..rows {
        for j in 0..cols {
            ret_mat[j * rows + i] = mat[i * cols + j];
        }
    }
    ret_mat
}

fn print_matrix(mat: &Vec<f64>, rows: usize, cols: usize) {
    for i in 0..rows {
        for j in 0..cols {
            print!("{} ", mat[i * cols + j]);
        }
        println!("");
    }
}
