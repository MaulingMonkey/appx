fn main() {
    println!("Families");
    println!("--------");
    for fam in appx::repository::families().unwrap() {
        println!("{}", fam);
    }
    println!();

    println!("Packages");
    println!("--------");
    for pkg in appx::repository::packages().unwrap() {
        println!("{}", pkg);
    }
    println!();
}
