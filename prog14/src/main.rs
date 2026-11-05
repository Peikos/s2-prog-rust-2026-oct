fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // Iterator met map (transformatie per waarde (lazy) en for_each (statement op elke waarde)
    v.iter().map(|i| i * i).for_each(|i| {
        println!("{}", i);
    });

    // Iterator is lazy; zonder .collect gebeurt er niets.
    // Collect zet het resultaat van de map / transformatie in een datastructuur.
    let v2: Vec<_> = v.iter().map(|g| g * 3).collect();
    dbg!(v2);

    dbg!(&v);

    // Een for loop is onder water een into_iter() -> neemt eigenaarschap

    // for i in v {
    //     println!("{}", &i)
    // }
    //
    // dbg!(v); // dit kan niet meer, `v` is geconsumeerd door for loop

    // Merk op: dit is een array ipv Vec; maakt voor iterator niet uit
    let interessantere_input = [27, 3, 30, 26, 9, 13, 67];

    let interessante_output: Vec<_> = interessantere_input // Type hier gegeven voor `collect()`
        .iter()
        // .map(|i| *i % 2 == 0) // tranformeer naar bool obv even/odd
        .filter(|i| *i % 2 == 0) // filter obv even/odd (behoud even)
        .collect();
    // .collect::<Vec<_>>(); // Alternatief voor type bij `let`; collect::<...>() aka turbofish

    dbg!(interessante_output);

    // Hier is de turbofish nodig - we maken geen variabele dus er is geen let om te typeren.
    // dbg!(v.iter().map(|x| x * 2).collect()); // Nope
    dbg!(v.iter().map(|x| x * 2).collect::<Vec<_>>());

    let mut m = vec![1, 2, 3, 4, 5];

    // Mutable iterators passen de waardes in place aan:

    m.iter_mut().for_each(|i| {
        *i = *i * *i; // Ik moet hier de waarde van referentie i gebruiken, niet i zelf.
    });

    dbg!(m);
}

// # Lifetimes

// Deze code werkt niet met references, omdat niet bekend is hoe lang de return-string moet leven
// Owned values blijven leven zolang als de owner, maar die is hier buiten beeld.
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// Dit zou werken als de return string net zo lang leeft als de input. Welke?
// De enige manier om tot een geldig type te komen, is als ze alledrie even lang leven.
#[allow(unused)]
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Met maar een parameter / zonder de if..else is de lifetime duidelijk en hoeven we die
// niet aan te geven.
#[allow(unused)]
fn simple(x: &str) -> &str {
    x
}
