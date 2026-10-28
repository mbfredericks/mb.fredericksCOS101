fn main() {

    //a list of nos
    let v = vec![15,25,35,45,55];
    print_vector(v);
    println!("{}",print_vector); // this line would give an error if v[0] was put in the place of print_vector
}

fn print_vector(x:Vec<i32>){

    println!("Inside print_vector function {:?}",x);
}
