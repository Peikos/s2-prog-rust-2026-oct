// Voorbeeld met tests: wanneer de functie largest wordt aangepast, kan het zijn dat tests
// niet langer slagen.

pub fn largest(list: &[i32]) -> i32 {
    let mut biggest = list[0];
    for &item in list {
        if item > biggest { biggest = item; }
    }
    biggest
}

#[cfg(test)]
mod tests {         // Test module binnen dezelfde file
    use super::*;   // Importeer alles uit de module
    #[test]
    fn largest_is_last() {
        assert_eq!(largest(&[1, 2, 3]), 3); // assert_eq macro -> panic als niet gelijk
    }                                           // panic -> test faalt
    #[test]
    fn largest_single_element() {
        assert_eq!(largest(&[42]), 42);
    }
}