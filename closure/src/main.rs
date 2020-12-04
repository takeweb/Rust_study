fn main() {
    // スレッド数
    const N_THREADS : usize = 3;

    // 処理する整数のRange
    let series_range = 0..30;
    let add = 1;

    let chunks = (0..N_THREADS)
        .map(|ii| series_range.clone().skip(ii).step_by(N_THREADS));

    let handles : Vec<_> = chunks
        .map(|vv| std::thread::spawn(move || {
            vv.for_each(|nn| print!("{},", nn + add));
        })).collect();
    handles.into_iter().for_each(|hh| hh.join().unwrap());
}
