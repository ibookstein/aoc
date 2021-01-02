use aoc2020::aoc_input::get_input;
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl FromStr for Food {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trim = s.trim_end_matches(')');
        let mut split = trim.split(" (contains ");
        let ingredients = split.next().ok_or("No ingredients")?;
        let allergens = split.next().ok_or("No allergens")?;
        if !split.next().is_none() {
            return Err("Trailing garbage");
        }

        let ingredients = ingredients.split(" ").map(|s| s.to_string()).collect();
        let allergens = allergens.split(", ").map(|s| s.to_string()).collect();
        Ok(Food {
            ingredients,
            allergens,
        })
    }
}

fn match_allergens(foods: &[Food]) -> HashMap<String, String> {
    let mut candidates = HashMap::<&String, HashSet<&String>>::new();

    for food in foods {
        for allergen in &food.allergens {
            match candidates.entry(allergen) {
                Entry::Occupied(e) => {
                    e.into_mut().retain(|s| food.ingredients.contains(*s));
                }
                Entry::Vacant(e) => {
                    e.insert(food.ingredients.iter().collect());
                }
            }
        }
    }

    let mut assignments = HashMap::<String, String>::new();
    loop {
        let res = candidates.iter().find(|(_, v)| v.len() == 1);
        let allergen = match res {
            None => break,
            Some((k, _)) => (*k).clone(),
        };
        let ingredient = candidates.remove(&allergen).unwrap();
        let ingredient = ingredient.iter().copied().next().unwrap().clone();

        for set in candidates.values_mut() {
            set.remove(&ingredient);
        }

        assignments.insert(ingredient, allergen);
    }

    assignments
}

fn main() {
    let input = get_input(21);
    let foods: Vec<Food> = input.lines().map(|line| line.parse().unwrap()).collect();
    let assignments = match_allergens(&foods);

    let mut no_allergen_count = 0;
    for food in &foods {
        for ingredient in &food.ingredients {
            if !assignments.contains_key(ingredient) {
                no_allergen_count += 1;
            }
        }
    }
    dbg!(no_allergen_count);

    let mut v: Vec<_> = assignments.iter().collect();
    v.sort_by_key(|(_, allergen)| *allergen);
    let dangerous: Vec<_> = v.iter().map(|(ing, _)| (*ing).clone()).collect();
    let dangerous = dangerous.join(",");
    dbg!(dangerous);
}
