use std::fmt::Formatter;

#[derive(Clone)]
pub struct List {
    value: i32,
    next: Option<Box<List>>,   // None = einde van de lijst
}

impl List {
    pub fn new(value: i32) -> List {            // Singleton lijst: 1 element
        List { value, next: None }
    }

    pub fn prepend(self, value: i32) -> List {  // Waarde vooraan toevoegen
        List { value, next: Some(Box::new(self)) }
    }

    pub fn append(mut self, value: i32) -> List { // Iteratief
        let mut cursor = &mut self;

        while cursor.next.is_some() {
            cursor = cursor.next.as_deref_mut().unwrap();
        }

        cursor.next = Some(Box::new(List::new(value)));
        self
    }

    pub fn appendr(mut self, value: i32) -> List { // Recursief
        if let Some(next) = self.next {
            return next.appendr(value).prepend(self.value)  // niet vergeten de afgeknipte items terug te zetten (prepend)
        }

        self.next = Some(Box::new(List::new(value)));
        self
    }

    pub fn print(&self) {                       // Recursief printen
        println!("{}", self.value);
        if let Some(next) = &self.next {
            next.print();
        }
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)?;
        if let Some(next) = &self.next {
            write!(f, " -> {}", next)?;
        }
        Ok(())
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        if self.value == other.value {  // Zijn de eerste values hetzelfde?
            self.next == other.next     // dan zijn de lijsten gelijk als de rest dat ook is
        } else {                        //    - Some(rest) vs None is sowieso niet gelijk
            false                       // Ongelijke waardes -> lijsten niet hetzelfde
        }
    }
}

fn main() {
    let mut l = List::new(3).prepend(2).prepend(1);
    let l2 = l.clone();
    println!("{l} == {l2}");  // Clones zijn hetzelfde
    dbg!(l == l2);

    l = l.append(4);
    println!("[[{l}]]");
    println!("{l} == {l2}");  // Na aanpassen niet meer
    dbg!(l == l2);
}