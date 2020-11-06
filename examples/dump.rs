fn main() {
    println!("Families");
    println!("--------");
    for fam in appx::repository::families().unwrap() {
        println!("{}", fam);
        for pkg in appx::repository::packages_for_family(&fam).unwrap() {
            println!("    {}", pkg);
        }
    }
    println!();

    println!("Packages");
    println!("--------");
    for pkg in appx::repository::packages().unwrap() {
        println!("{}", pkg);
    }
    println!();
}
