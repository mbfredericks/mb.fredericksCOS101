fn main() {

    //Name vector
    let name = vec!["Mary","Sam","Sally","Greg","Ade","Mark","June","Ife"];

    //Age vector
    let age = vec![16,17,18,19,20,21,22,23];

    println!("\nAge allocation: \n");

    //loop to iterate elements in vector
    for i in 0..age.len()
    {
         //iterating through i on the vector
         print!("{} is {} years old\n",name[i],age[i]); 
    }
}
