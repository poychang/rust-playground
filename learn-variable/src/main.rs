mod sub;

fn main() {
    sub::immutable();
    println!("---------------------");
    sub::binding();
    println!("---------------------");
    sub::var_type();
    println!("---------------------");
    sub::range();
    println!("---------------------");
}
