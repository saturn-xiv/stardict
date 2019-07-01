extern crate stardict;

use std::path::Path;

#[test]
fn it_works() {
    let mut st = stardict::StarDict::new(
        Path::new("/mnt")
            .join("cbeta")
            .join("GoldenDict")
            .join("13Dicts"),
    )
    .unwrap();
    for it in st.info() {
        println!("{:?}", it);
    }
    for it in st.search("hello") {
        println!("{} v{} \n {:?}", it.info.name, it.info.version, it.results);
    }
}
