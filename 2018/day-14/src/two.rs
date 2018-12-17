const INPUT: &'static str = include_str!("input.txt");

fn main() {

    let target_recipe = INPUT.trim()
        .chars()
        .map(|c| c.to_digit(10))
        .map(Option::unwrap)
        .map(|d| d as usize)
        .collect::<Vec<_>>();
    let target_len = target_recipe.len();

    let mut a = 0;
    let mut b = 1;
    let mut recipes = vec![3, 7];

    loop {
        let mut new = 0;
        for recipe in (recipes[a] + recipes[b]).to_string()
            .chars()
            .map(|c| c.to_digit(10))
            .map(Option::unwrap) {
            new += 1;
            recipes.push(recipe as usize);
        }

        let len = recipes.len();
        if len > target_len {
            for offset in 0..new {
                if recipes[len - offset - target_len..len - offset] == target_recipe[..] {
                    println!("{}", recipes.len() - offset - target_len);
                    return
                }
            }
        }

        a = (a + 1 + recipes[a]) % recipes.len();
        b = (b + 1 + recipes[b]) % recipes.len();
    }
}
