use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let food = process_input(&buf);
    
    let answer = get_answer(&food);
    let answer2 = get_answer2(&food);

    println!("{}\n{}", answer, answer2);
}

fn process_input(input: &str) -> Vec<Food> {
    input.lines().map(|s| {
        let mut iter = s.split(" (contains ");
        let ingredients = iter.next().unwrap().split(" ").collect();
        let allergens = iter.next().unwrap().strip_suffix(')').unwrap().split(", ").collect();
        Food {ingredients, allergens}
    }).collect()
}

fn get_answer(food: &[Food]) -> usize {
    let all_ingredients = Food::all_ingredients(food);
    
    let allergen_map = Food::get_allergen_map(food);
    
    let bad_ingredients = allergen_map.values().flat_map(HashSet::iter).cloned().collect();
    let good_ingredients: HashSet<_> = all_ingredients.difference(&bad_ingredients).collect();

    let mut count = 0;
    for Food {ingredients, allergens: _} in food {
        count += ingredients.iter().filter(|i| good_ingredients.contains(i)).count();
    }
    count
}

fn get_answer2(food: &[Food]) -> String {

    let allergen_map = Food::get_allergen_map(food);
    let mut allergen_map: Vec<_> = allergen_map.into_iter().collect();

    for index in 1..allergen_map.len() {
        allergen_map.sort_by_key(|(_allergen, ingredients)| ingredients.len());

        let (single_ingredients, multi_ingredients) = allergen_map.split_at_mut(index);

        if let Some(current_ingredient) = single_ingredients.last().unwrap().1.iter().next() {
            for ingredients in multi_ingredients {
                ingredients.1.retain(|i| i != current_ingredient);
            }
        }
    }

    allergen_map.sort_by_key(|(allergen, _ingredients)| *allergen);

    let mut display = String::new();
    for (_allergen, ingredients) in allergen_map {
        let ingredient = ingredients.iter().next().unwrap();
        display.push_str(ingredient);
        display.push_str(",");
    }

    display
}

#[derive(Clone, Debug)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

impl<'a> Food<'a> {
    fn all_ingredients(food: &[Self]) -> HashSet<&'a str> {
        food.iter().flat_map(|food| {
            food.ingredients.iter()
        //We are cloning a `&&str` to make a `&str`
        }).cloned().collect()
    }

    fn all_allergens(food: &[Self]) -> HashSet<&'a str> {
        food.iter().flat_map(|food| {
            food.allergens.iter()
        //We are cloning a `&&str` to make a `&str`
        }).cloned().collect()
    }

    fn get_allergen_map(food: &[Self]) -> HashMap<&str, HashSet<&str>> {
        let all_allergens = Food::all_allergens(food);
        let all_ingredients = Food::all_ingredients(food);
        
        all_allergens.iter().map(|current_allergen| {
            let mut current_ingredients = all_ingredients.clone();
    
            for Food {ingredients, allergens} in food {
                if allergens.contains(current_allergen) {
                    current_ingredients.retain(|i| ingredients.contains(i));
                }
            }
    
            (*current_allergen, current_ingredients)
        }).collect()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer() {
        let food = process_input(INPUT);
        assert_eq!(5, get_answer(&food));
    }

    #[test]
    fn test_answer2() {
        let food = process_input(INPUT);
        assert_eq!("mxmxvkd,sqjhc,fvjkl,", get_answer2(&food));
    }

    const INPUT: &'static str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

}
