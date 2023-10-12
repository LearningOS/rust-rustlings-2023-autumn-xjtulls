// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let mut vec0 = Vec::new();

    let mut vec1 = fill_vec(&mut vec0);

    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(v: &mut Vec<i32>) -> Vec<i32> {
    v.push(22);
    v.push(44);
    v.push(66);

    vec![v[0],v[1],v[2]]
}
