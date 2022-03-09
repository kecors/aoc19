use pest::Parser;
use pest_derive::Parser;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "recipe.pest"]
struct ReactionParser;

#[derive(Debug)]
struct Relation {
    product: String,
    ingredient: String,
}

impl Relation {
    fn new(product: String, ingredient: String) -> Relation {
        Relation {
            product,
            ingredient,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Ingredient {
    chemical: String,
    quantity: u32,
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &format!("{} {}", self.quantity, self.chemical))
    }
}

impl Ingredient {
    fn new(chemical: String, quantity: u32) -> Ingredient {
        Ingredient { chemical, quantity }
    }
}

#[derive(Debug)]
struct Recipe {
    ingredients: Vec<Ingredient>,
    product_quantity: u32,
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for (index, ingredient) in self.ingredients.iter().enumerate() {
            if index > 0 {
                result.push_str(", ");
            }
            result.push_str(&format!("{}", ingredient));
        }

        result.push_str(&format!(" => {}", self.product_quantity));

        write!(f, "{}", result)
    }
}

#[derive(Debug)]
struct Nanofactory {
    recipes: HashMap<String, Recipe>,
}

impl fmt::Display for Nanofactory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for (product_chemical, recipe) in self.recipes.iter() {
            result.push_str(&format!("{} {}\n", recipe, product_chemical));
        }

        write!(f, "{}", result)
    }
}

impl Nanofactory {
    fn new(input: &str) -> Nanofactory {
        let pairs = ReactionParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        let mut ingredient_quantity = 0;
        let mut ingredients = Vec::new();
        let mut product_quantity = 0;
        let mut recipes = HashMap::new();

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::ingredient_quantity => {
                    ingredient_quantity = text.parse::<u32>().unwrap();
                }
                Rule::ingredient_chemical => {
                    ingredients.push(Ingredient::new(text.clone(), ingredient_quantity));
                }
                Rule::product_quantity => {
                    product_quantity = text.parse::<u32>().unwrap();
                }
                Rule::product_chemical => {
                    let recipe = Recipe {
                        ingredients,
                        product_quantity,
                    };
                    ingredients = Vec::new();
                    recipes.insert(text.clone(), recipe);
                }
                _ => {
                    panic!("Unknown rule {:?} with {:?}", rule, text);
                }
            }
        }

        Nanofactory { recipes }
    }

    fn determine_reduction_order(&self) -> Vec<String> {
        let mut reduction_order: Vec<String> = Vec::new();

        let mut relations: Vec<Relation> = Vec::new();

        for (product_chemical, recipe) in self.recipes.iter() {
            for ingredient in recipe.ingredients.iter() {
                relations.push(Relation::new(
                    product_chemical.to_string(),
                    ingredient.chemical.to_string(),
                ));
            }
        }

        while !relations.is_empty() {
            let mut products: HashSet<String> = HashSet::new();
            let mut ingredients: HashSet<String> = HashSet::new();

            for relation in relations.iter() {
                products.insert(relation.product.clone());
                ingredients.insert(relation.ingredient.clone());
            }

            let non_ingredients: HashSet<String> =
                products.difference(&ingredients).cloned().collect();

            for product in non_ingredients.iter().cloned() {
                reduction_order.push(product);
            }

            relations = relations
                .into_iter()
                .filter(|relation| !non_ingredients.contains(&relation.product))
                .collect();
        }

        reduction_order
    }

    fn reduce(&mut self) -> u32 {
        let reduction_order = self.determine_reduction_order();

        let mut workspace: HashMap<String, u32> = HashMap::new();
        workspace.insert(String::from("FUEL"), 1);

        for chemical in reduction_order.iter() {
            if let Some(quantity) = workspace.remove(chemical) {
                if let Some(recipe) = self.recipes.get(chemical) {
                    let product_quantity = quantity / recipe.product_quantity
                        + if quantity % recipe.product_quantity == 0 {
                            0
                        } else {
                            1
                        };
                    for ingredient in recipe.ingredients.iter() {
                        let ingredient_quantity = ingredient.quantity * product_quantity;
                        let o = workspace.entry(ingredient.chemical.clone()).or_insert(0);
                        *o += ingredient_quantity;
                    }
                }
            }
        }

        if let Some(&quantity) = workspace.get("ORE") {
            quantity
        } else {
            panic!("Reduction did not product ORE");
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut nanofactory = Nanofactory::new(&input);
    println!(
        "Part 1: the minimum amount of ore is {}",
        nanofactory.reduce()
    );
}
