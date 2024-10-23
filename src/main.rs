#![allow(unused_imports, dead_code)]
#![forbid(unsafe_code)]

pub mod ledger;


fn main() {



}


fn read(filepath: &str) -> Result<(), serde_yaml::Error> {

    let point = Point { x: 1.0, y: 2.0 };

    let yaml = serde_yaml::to_string(&point)?;
    assert_eq!(yaml, "x: 1.0\ny: 2.0\n");

    let deserialized_point: Point = serde_yaml::from_str(&yaml)?;
    assert_eq!(point, deserialized_point);
    Ok(())
}

