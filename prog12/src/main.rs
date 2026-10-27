// For loop op characters in een String
fn print_chars(string: &String) -> () {
    for c in string.chars() {
        println!("{}", c);
    }
}

// Sort de characters - via Vec<string>
fn sort(string: &String) -> String {
    // let mut result = list.clone();
    let mut result: Vec<char> = string.chars().collect(); // Conversie naar Vec<char>
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..result.len() {
            if result[i-1] > result[i] {
                result.swap(i - 1, i);
                swapped = true;
            }
        }
    }
    result.iter().collect() // Conversie terug
}

// Voorbeeld enum
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum User {
    Gebruiker(String, u32),
    Admin { naam: String, pass: String }
}


fn main() {
    println!("Hello, world!");
    let string = String::from("Hallo 锈");
    print_chars(&string); // Per karakter, niet per byte

    // Match op Option / letterlijke waardes
    let x: Option<u8> = Some(42);

    match x {
        None => println!("Minder dan niets"),
        Some(0) => println!("Niet veel"),
        Some(1) => println!("Vrij weinig"),
        Some(val) => println!("Te veel - {val}")
    }

    // Oefening: long vowels (functie, hier de aanroep)
    match long_vowels('x') {
        None => println!("Geen vowel"),
        Some(val) => println!("Het is een {val}")
    }

    if let Some(y) = long_vowels('x') {
        println!("{y}")
    }

    // Linked list voor Box (komt terug in PROG-VI)
    test_list();

    // Oefening: maak zelf een enum
    let g = User::Gebruiker("Harry".to_string(), 0);
    let a = User::Admin { naam: "Mathilde".to_string(), pass: "geheim123".to_string() };

    // Printbaar door #[derive(Debug)]
    dbg!(&g);
    println!("{:?}", &a);

    g.do_the_thing();
    a.do_the_thing();

    // Voorbeeld: anyhow en ?
    let port = get_port();
    dbg!(port);
}

// Deze functie levert meerdere error types op in het tussenproduct
// Met anyhow::Result (`cargo add anyhow` om te installeren) kunnen we `?` gebruiken om de errors
// te converteren.
fn get_port() -> anyhow::Result<u16> {
    println!("Geef een port?");
    let mut buffer = String::new();
    let stdin = std::io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;
    let port = buffer.trim().parse::<u16>()?;
    Ok(port)
}

// Oefening
impl User {
    fn do_the_thing(&self) {
        match self {
            User::Gebruiker(n, s) => println!("Welkom {n}, uw score is {s}"),
            User::Admin { naam: n, pass: _} => println!("Welkom {n}, geeft uw wachtwoord!")
        }
    }
}

//Oefening
fn long_vowels(c: char) -> Option<String> {
    match c {
        'a' => Some(String::from("aa")),
        'e' => Some("ee".to_string()),
        'i' => Some("ie".into()),
        'o' => Some(String::from("oo")),
        'u' => Some("uu".to_string()),
        _ => None,
    }
}

// Voorbeeld List / Box
struct List {
    value: u8,
    next: Option<Box<List>>,                                   // Mogelijk een volgend item op de heap
}

pub fn test_list() {
    let three = List { value: 3, next: None };                 // Opbouwen van achter naar voren
    let two = List { value: 2, next: Some(Box::new(three)) };
    let one = List { value: 1, next: Some(Box::new(two)) };

    let mut cursor = &one;                                     // Loop van voor naar achteren door lijst
    while cursor.next.is_some() {
        println!("{}", cursor.value);
        cursor = &cursor.next.as_ref().unwrap();               // Unwrap om de waarde eruit te krijgen
    }                                                          // Maar unwrap wordt owner, en jat dan de link van de lijst...
    println!("{}\nEND", cursor.value);                         // Dus unwrap mag owner worden van een referentie
}