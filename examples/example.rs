use constexpr_edn::edn;
use edn_rs::{map, set, Edn, List, Map, Set, Vector};

fn main() {
    // Compile time error on any invalid EDN
    let edn: Edn = edn!([ 1 2 3 #{4 5 6}]);
    // Vector(Vector([UInt(1), UInt(2), UInt(3), Set(Set({UInt(4), UInt(5), UInt(6)}))]))
    println!("{edn:?}");

    let edn: Edn = edn!(#{ 1 2 3 #{4 5 6}});
    // Set(Set({Set(Set({UInt(4), UInt(5), UInt(6)})), UInt(1), UInt(2), UInt(3)}))
    println!("{edn:?}");

    let edn: Edn = edn!({:foo #{42 43 44} :bar (1 2 3)});
    // Map(Map({":bar": List(List([UInt(1), UInt(2), UInt(3)])), ":foo": Set(Set({UInt(42), UInt(43), UInt(44)}))}))
    println!("{edn:?}");

    let edn: Edn = edn!([42, "foobar",,,,, ,, ; 424242
                         #_ ignoreme
                         ,, yaycats 16r9001]);
    // Vector(Vector([UInt(42), Str("foobar"), Symbol("yaycats"), UInt(36865)]))
    println!("{edn:?}");

    let edn: Edn = edn!(42 43 44);
    // UInt(42)
    println!("{edn:?}");
}
