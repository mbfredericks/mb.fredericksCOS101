fn main () {
	let toshiba_quantity : f64 = 2.0;
	let toshiba_amount : f64 = 450_000.00;
	let mac_quantity : f64 = 1.0;
	let mac_amount : f64 = 1_500_000.00;
	let hp_quantity : f64 = 3.0;
	let hp_amount : f64 = 750_000.00;
    let dell_quantity : f64 = 3.0;
    let dell_amount : f64 = 2_850_000.00;
    let acer_quantity : f64 = 1.0;
    let acer_amount : f64 = 250_000.00;
    // sum
    let q = toshiba_quantity + mac_quantity + hp_quantity + dell_quantity + acer_quantity;
    let  a = toshiba_amount + mac_amount + hp_amount + dell_amount + acer_amount;
    println!(" Quantity is {} and Amount is {:.2}", q,a);
    //average
    let avg = q / a;
    println!("Average is {:.2}", avg);

}