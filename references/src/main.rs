use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by: {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn dereference() {
    let x = 10;
    let r = &x;
    // assert_eq!(r, 10) // doesn't work
    assert_eq!(*r, 10);

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(*m == 64);
}

fn ref_of_ref() {
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    
    assert!(*rrx == rx);
    assert!(rry > rrx);
}

fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The calling of St. Mathew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Persues with the head of medusa".to_string(), "a salt cellar".to_string()]);

    show(&table);

    dereference();
    ref_of_ref();
}
