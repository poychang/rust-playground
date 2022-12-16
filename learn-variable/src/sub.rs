pub fn immutable() {
    /*
        Rust 的變數預設是 immutable 的，也就是說，一旦綁定之後，就無法再被修改
        但是，如果想要讓變數可以被修改，可以在綁定時加上 mut 關鍵字
    */
    let x = 10;
    // x = 20; // 錯誤訊息：cannot assign twice to immutable variable
    println!("x = {}", x);

    // 使用 mut 關鍵字將變數宣告為 mutable 之後，就可以被修改
    // 但這樣做也容易產生 bug，因此在 Rust 中，盡量避免使用 mutable 變數
    let mut y: i32 = 10;
    let address = format!("{:p}", &y);
    println!("y = {}, address = {}", y, address);
    y = 20;
    println!("y = {}, address = {}", y, address);
}

pub fn binding() {
    /*
       Rust 的變數是透過綁定（binding）的方式來宣告的，不同於其他程式語言是使用指定（assign）的概念
       因此，Rust 的變數綁定是可以被遮蔽（shadowed），這代表一個變數綁定之外有同樣名稱的變數綁定時，在有效範圍內，將會覆蓋前一個綁定
       也因此無法在取得前一次的綁定值
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

pub fn var_type() {
    /*
       預設使用 i32 作為整數型別，整數的型別如下：

       | 長度    | 帶正負號 | 不帶帶正負號 |
       | ------- | -------- | ------------ |
       | 8-bit   | i8       | u8           |
       | 16-bit  | i16      | u16          |
       | 32-bit  | i32      | u32          |
       | 64-bit  | i64      | u64          |
       | 128-bit | i128     | u128         |
       | arch    | isize    | usize        |
    */

    let x: i32 = 10; // 宣告時指定型別
    println!("x = {}", x);
    let y = 10_000; // 可以用下底線作為數字的分隔符號，提升可讀性
    println!("y = {}", y);

    /*
       預設使用 f64 作為浮點數型別，浮點數的型別如下：

       | 長度    | 型別 |
       | ------- | ---- |
       | 32-bit  | f32  |
       | 64-bit  | f64  |
    */
    let float = 10_000.1001_f64; // 在數字後面加上 _f64 來指定型別
    println!("float = {}", float);
}

pub fn range() {
    let list: std::ops::Range<i32> = 1..10; // 產生一個 1 到 10 的範圍
    println!("list = {:?}", list);
    for i in list {
        println!("i = {}", i);
    }

    let sum: i32 = (1..10).sum();
    println!("sum = {}", sum);
}
