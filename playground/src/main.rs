fn main() {
    /*
        Rust 的變數是透過綁定（binding）的方式來宣告的，不同於其他程式語言是使用指定（assign）的概念
        因此，Rust 的變數綁定是可以被遮蔽（shadowed），這代表一個變數綁定之外有同樣名稱的變數綁定時，在有效範圍內，將會覆蓋前一個綁定
     */
    let x = 10;
    println!("x = {}", x);
    let address = format!("{:p}", &x);
    println!("address of first x = {}", address);

    let x = 20;
    println!("x = {}", x);
    let address = format!("{:p}", &x);
    println!("address of second x = {}", address);

    // 同樣都是 x 的變數綁定，但實際上是不同的變數，因此其記憶體位置也不同
}
