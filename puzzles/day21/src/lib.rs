// Day 21: Allergen Assessment

use std::collections::{HashMap, HashSet};

use shared::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FoodLabel<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

lazy_static! {
    static ref FOOD_LABEL_REGEX: Regex =
        Regex::new(r"^([a-z ]+?) \(contains ([a-z, ]+)\)$").unwrap();
    static ref TEST_INPUT: Vec<FoodLabel<'static>> =
        puzzle_input::lines(include_str!("test_input.txt"))
            .into_iter()
            .map(|x| FoodLabel::parse(x))
            .collect::<Result<_, _>>()
            .unwrap();
    static ref PUZZLE_INPUT: Vec<FoodLabel<'static>> =
        puzzle_input::lines(include_str!("puzzle_input.txt"))
            .into_iter()
            .map(|x| FoodLabel::parse(x))
            .collect::<Result<_, _>>()
            .unwrap();
}

impl FoodLabel<'_> {
    pub fn parse<'a>(s: &'a str) -> anyhow::Result<FoodLabel<'a>> {
        let captures = FOOD_LABEL_REGEX.captures(s);
        if let Some(captures) = captures {
            let ingredients = captures.get(1).unwrap().as_str().split(" ").collect();
            let allergens = captures.get(2).unwrap().as_str().split(", ").collect();
            Ok(FoodLabel {
                ingredients,
                allergens,
            })
        } else {
            Err(anyhow!("invalid label: {}", s))
        }
    }
}

#[derive(Default)]
struct Solution<'a> {
    allergens_for_ingredients_map: HashMap<&'a str, Option<&'a str>>,
}
impl Solution<'_> {
    fn solve<'a>(labels: &'a [FoodLabel<'a>]) -> anyhow::Result<Solution<'a>> {
        let allergen_map = labels
            .iter()
            .flat_map(
                |FoodLabel {
                     allergens,
                     ingredients,
                 }| {
                    allergens.iter().copied().map(move |allergen| {
                        (
                            allergen,
                            ingredients.iter().copied().collect::<HashSet<&str>>(),
                        )
                    })
                },
            )
            .fold(
                HashMap::new(),
                |mut result: HashMap<&str, HashSet<&str>>,
                 (allergen, ingredients): (&str, HashSet<&str>)| {
                    result
                        .entry(allergen)
                        .and_modify(|existing| {
                            *existing = existing.intersection(&ingredients).copied().collect()
                        })
                        .or_insert(ingredients);

                    result
                },
            );

        // HashSets aren't super useful anymore, convert them to vecs
        let allergen_map: HashMap<&str, Vec<&str>> = allergen_map
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();

        fn iterate_solutions<'a>(
            allergen_map: HashMap<&'a str, Vec<&'a str>>,
        ) -> Option<HashMap<&'a str, Vec<&'a str>>> {
            let solved_allergens: HashSet<&str> = allergen_map
                .iter()
                .filter_map(|(allergen, possible_ingredients)| {
                    if possible_ingredients.len() == 1 {
                        Some(allergen)
                    } else {
                        None
                    }
                })
                .copied()
                .collect();
            let solved_ingredients: HashSet<&str> = solved_allergens
                .iter()
                .map(|allergen| allergen_map.get(allergen).unwrap().first().unwrap())
                .copied()
                .collect();

            if solved_allergens.len() == allergen_map.len() {
                Some(allergen_map)
            } else {
                // weed out all the ingredients from possibilities that have already been solved
                let new_map = allergen_map
                    .iter()
                    .map(|(&allergen, possible_ingredients)| {
                        if solved_allergens.contains(allergen) {
                            (allergen, possible_ingredients.clone())
                        } else {
                            let possible_ingredients = possible_ingredients
                                .into_iter()
                                .filter(|&x| !solved_ingredients.contains(x))
                                .copied()
                                .collect();
                            (allergen, possible_ingredients)
                        }
                    })
                    .collect();

                if new_map != allergen_map {
                    iterate_solutions(new_map)
                } else {
                    None
                }
            }
        }

        let resulting_map = iterate_solutions(allergen_map);
        let resulting_map = resulting_map.ok_or(anyhow!("Couldn't find a solution"))?;
        let inverted_map: HashMap<&str, &str> = resulting_map
            .into_iter()
            .map(|(allergen, ingredients)| (*ingredients.first().unwrap(), allergen))
            .collect();

        let all_ingredients = labels
            .iter()
            .flat_map(|FoodLabel { ingredients, .. }| ingredients)
            .copied();

        let result = all_ingredients
            .map(|ingredient| (ingredient, inverted_map.get(ingredient).copied()))
            .collect();

        Ok(Solution {
            allergens_for_ingredients_map: result,
        })
    }
}

pub struct SafeIngredients<'a> {
    pub ingredients: HashSet<&'a str>,
    pub count: usize,
}
impl SafeIngredients<'_> {
    pub fn solve<'a>(labels: &'a [FoodLabel<'a>]) -> anyhow::Result<SafeIngredients<'a>> {
        let solution = Solution::solve(labels)?;
        let safe_ingredients: HashSet<&str> = solution
            .allergens_for_ingredients_map
            .iter()
            .filter_map(|(&ingredient, &allergen)| match allergen {
                Some(_) => None,
                None => Some(ingredient),
            })
            .collect();

        let count = labels
            .iter()
            .flat_map(|label| label.ingredients.iter().copied())
            .filter(|ingredient| safe_ingredients.contains(ingredient))
            .count();

        Ok(SafeIngredients {
            ingredients: safe_ingredients,
            count,
        })
    }
}

#[cfg(test)]
mod part_one {

    use super::*;

    #[test]
    fn can_find_solution() {
        let solution = Solution::solve(TEST_INPUT.as_slice());
        assert!(solution.is_ok());
    }

    #[test]
    fn test_case() {
        let result = SafeIngredients::solve(TEST_INPUT.as_slice()).unwrap();
        assert_eq!(
            result.ingredients,
            vec!["kfcds", "nhms", "sbzzf", "trh"].into_iter().collect()
        );
        assert_eq!(result.count, 5);
    }

    #[test]
    fn answer() {
        let result = SafeIngredients::solve(PUZZLE_INPUT.as_slice()).unwrap();
        assert_eq!(result.count, 2072);
    }
}

// #[cfg(test)]
// mod part_two {
//     use super::*;
//     #[test]
//     fn test_cases() {}
//     #[test]
//     fn answer() {}
// }
