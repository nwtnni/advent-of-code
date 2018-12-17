const INPUT: &'static str = include_str!("input.txt");

fn main() {

    let max_recipes = INPUT.trim().parse::<usize>().unwrap();
    let mut a = 0;
    let mut b = 1;
    let mut recipes = vec![3, 7];

    while recipes.len() < max_recipes + 10 {
        for recipe in (recipes[a] + recipes[b]).to_string()
            .chars()
            .map(|c| c.to_digit(10))
            .map(Option::unwrap) {
            recipes.push(recipe as usize);
        }

        a = (a + 1 + recipes[a]) % recipes.len();
        b = (b + 1 + recipes[b]) % recipes.len();
    }

    let last = &recipes[max_recipes..max_recipes + 10];

    println!("{:?}", last);

}
