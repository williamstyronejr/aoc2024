use std::fs;

fn get_permutations(values: &Vec<i32>) ->  Vec<Vec<i32>> {
  let mut premutations = Vec::new();
  let operation_count = values.len() - 1;
  
  // 1 = add 2 = multiple
  // Add same operation cases
  premutations.push(vec![1;operation_count]);
  premutations.push(vec![2;operation_count]);
  
  if operation_count > 1 {
    for opers_to_add in 1..=operation_count {
      let mut swap_idx = opers_to_add - 1;
      let mut signs= vec![1; operation_count];

      signs.reverse();


      // 1, 2, 3
      // place opers in order
      // then move x placed oper until the end of the vec
      // then x = x - 1 till x is 0

      premutations.push(signs);
    }
  }

  premutations
}

fn validate_equations(equations: &Vec<(i32, Vec::<i32>)>) -> Vec<i32> { 
  let mut valid_equations = Vec::new();

  for eq in equations.clone() {
    let premutations = get_permutations(&eq.1);

    for operations in premutations {
      let mut result = 0;
      
      for (idx, value) in eq.1.iter().enumerate() {
        if idx == 0 {
          result += *value; 
          continue;
        }

        let operation = *operations.get(idx - 1).unwrap();
        if operation == 1 {
          result += *value;
        } else if operation == 2 {
          result = result * value;
        }
      }

      if result == eq.0 {
        valid_equations.push(eq.0);
        break;
      }
    }
  }

  valid_equations
} 

pub fn prob1() {
  let mut equations = Vec::new();
  let binding = fs::read_to_string("src/day7/input.txt").unwrap();

  for line in binding.lines() {
    let text_split = line.split(":").collect::<Vec<_>>();
    let test_values = text_split[1].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

    equations.push((text_split[0].parse::<i32>().unwrap(), test_values));
  }

  println!("{:?}", equations);

  let valid_eqs = validate_equations(&equations);


  let part1_ans = 0 + valid_eqs.iter().sum::<i32>();
  let part2_ans = 0;


  // 
  println!("Part 1: {}", part1_ans);

  // 
  println!("Part 2: {}", part2_ans);
}