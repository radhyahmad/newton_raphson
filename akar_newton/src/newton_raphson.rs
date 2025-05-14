pub fn find_root_with_history<F, DF>(
    f: F,
    df: DF,
    initial_guess: f64,
    tolerance: f64,
    max_iter: usize,
) -> (Option<f64>, Vec<f64>)
where
    F: Fn(f64) -> f64,
    DF: Fn(f64) -> f64,
{
    let mut x = initial_guess;
    let mut history = vec![x];

    for _ in 0..max_iter {
        let fx = f(x);
        let dfx = df(x);

        if dfx.abs() < 1e-12 {
            println!("Turunan mendekati nol. Berhenti.");
            return (None, history);
        }

        let x_new = x - fx / dfx;
        history.push(x_new);

        if (x_new - x).abs() < tolerance {
            return (Some(x_new), history);
        }

        x = x_new;
    }

    println!("Tidak konvergen setelah {} iterasi.", max_iter);
    (None, history)
}
