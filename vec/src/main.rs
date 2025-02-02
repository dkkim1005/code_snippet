fn main() {
    // from vector slices to vector
    let mut v1 = vec![];
    let v2 = vec![1, 2, 3, 4, 5];
    v1.extend_from_slice(&v2[2..5]);
    v1.extend_from_slice(&v2[0..2]);
    println!("{v1:?}");

    // from array to vector
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let v1: Vec<i32> = a.to_vec();
    println!("{v1:?}");

    // iterator to vector
    let v2: Vec<i32> = a.iter().map(|x| x.pow(2)).collect();
    println!("{v2:?}");

    // iterator with enumerate
    for (i, item) in v2.iter().enumerate() {
        println!("(i: {}, item: {})", i, item);
    }

    // sort for tuples
    let mut v: Vec<(f32, u32)> = vec![(0.4, 0), (0.5, 1), (0.1, 2), (0.3, 3), (0.9, 4)];
    v.sort_by(|a, b| b.partial_cmp(&a).unwrap()); // The reason why we use 'partial_cmp' method here is that float value can be NaN.
    println!("{v:?}");

    // sort for struct
    #[derive(Debug)]
    struct Temp {
        value: f32,
        index: u32,
    };

    impl Temp {
        fn new(v: f32, i: u32) -> Self {
            Self {
                value: v,
                index: i,
            }
        }
    }

    let mut v: Vec<Temp> = vec![
        Temp::new(0.4, 0),
        Temp::new(0.5, 1),
        Temp::new(0.1, 2),
        Temp::new(0.3, 3),
        Temp::new(0.9, 4),
    ];
    v.sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap());
    println!("{v:?}");
}
