use rand::seq::IteratorRandom;

pub fn random_choice<'a, T>(items: &'a [T]) -> Option<&'a T> {
    let mut rng = rand::rng();
    items.iter().choose(&mut rng)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_choice_empty() {
        let items: [i32; 0] = [];
        assert!(random_choice(&items).is_none());
    }

    #[test]
    fn test_random_choice_single() {
        let items = [42];
        assert_eq!(random_choice(&items), Some(&42));
    }

    #[test]
    fn test_random_choice_multiple() {
        let items = [1, 2, 3, 4, 5];
        let result = random_choice(&items);
        assert!(result.is_some());
        assert!(items.contains(result.unwrap()));
    }
}