use akar_newton::newton_raphson::find_root_with_history;
use plotters::prelude::*;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    //let f = |x: f64| x * x - 2.0;
    //let df = |x: f64| 2.0 * x;

    let f = |x: f64| x*x - 1.0;
    let df = |x: f64| 2.0*x;

    // input initial_guess
    println!("Masukkan tebakan awal (initial guess): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Gagal membaca input");
    let initial_guess: f64 = input.trim().parse().expect("Input harus berupa angka");

    // input tolerance
    println!("Masukkan toleransi error (misal 1e-7): ");
    input.clear();
    io::stdin().read_line(&mut input).expect("Gagal membaca input");
    let tolerance: f64 = input.trim().parse().expect("Input harus berupa angka");

    // input max_iter
    println!("Masukkan jumlah iterasi maksimum: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("Gagal membaca input");
    let max_iter: usize = input.trim().parse().expect("Input harus berupa angka");

    println!(
        "\nMencari akar dari persamaan x^2 - 2 = 0\n\
        Tebakan awal: {}\n\
        Toleransi: {}\n\
        Iterasi maksimum: {}\n",
        initial_guess, tolerance, max_iter
    );

    let (result, history) = find_root_with_history(f, df, initial_guess, tolerance, max_iter);

    for (i, x) in history.iter().enumerate() {
        println!(
            "Iterasi {}: x = {:.10}, f(x) = {:.10}",
            i + 1,
            x,
            f(*x)
        );
    }

    match result {
        Some(root) => println!("Akar ditemukan: x = {:.10}", root),
        None => println!("Gagal menemukan akar."),
    }

    // Buat grafik
    let root_area = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
    root_area.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root_area)
        .caption("Newton-Raphson Method", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-2.0..4.0, -4.0..4.0)?;

    chart.configure_mesh().draw()?;

    // plot fungsi f(x)
    chart.draw_series(LineSeries::new(
        (-200..200).map(|x| x as f64 * 0.03).map(|x| (x, f(x))),
        &BLUE,
    ))?
    .label("f(x)")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));

    // plot titik-titik iterasi
    chart.draw_series(history.iter().map(|x| {
        let y = f(*x);
        Circle::new((*x, y), 5, RED.filled())
    }))?
    .label("Iterasi")
    .legend(|(x, y)| Circle::new((x, y), 5, RED.filled()));

    // tampilkan legend
    chart.configure_series_labels().border_style(&BLACK).draw()?;

    println!("Grafik disimpan sebagai plot.png");

    Ok(())
}
