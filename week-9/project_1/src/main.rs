use std::fs;
use std::io::Write;

fn main() -> io::Result<()> {
    let headers = ["Lager", "Stout", "Non-Alcoholic"];

    let rows: [&[&str]; 3] = [
        &["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"],
        &["Legend", "Turbo King", "Williams"],
        &["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"],
    ];

    let mut file = std::fs::File::create("drinks.csv").expect("create failed");

    // Header row
    writeln!(out, "{}", headers.join(","))?;

    // Data rows: one per index, up to the longest column
    let max_rows = columns.iter().map(|c| c.len()).max().unwrap_or(0);
    for i in 0..max_rows {
        let row: Vec<&str> = columns
            .iter()
            .map(|col| col.get(i).copied().unwrap_or(""))
            .collect();
        writeln!(out, "{}", row.join(","))?;
    }

    out.flush()?;
    println!("Saved drink categories to drinks.csv");
    Ok(())
}