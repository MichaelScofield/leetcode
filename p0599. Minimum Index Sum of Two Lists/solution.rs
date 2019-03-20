impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let list1_with_index =
            &mut list1.iter().enumerate().collect::<Vec<(usize, &String)>>();
        list1_with_index.sort_by_key(|(_i, restaurant)| *restaurant);
        let list2_with_index =
            &mut list2.iter().enumerate().collect::<Vec<(usize, &String)>>();
        list2_with_index.sort_by_key(|(_i, restaurant)| *restaurant);
        let mut restaurants = Vec::new();
        let mut i = 0;
        let mut j = 0;
        let mut min_index_sum = std::usize::MAX;
        let mut candidate_indices = Vec::new();
        while i < list1_with_index.len() && j < list2_with_index.len() {
            let x = list1_with_index[i];
            let y = list2_with_index[j];
            if x.1 == y.1 {
                if x.0 + y.0 <= min_index_sum {
                    min_index_sum = x.0 + y.0;
                    candidate_indices.push((x.0, y.0));
                }
                i += 1;
                j += 1;
            } else if x.1 < y.1 {
                i += 1;
            } else {
                j += 1;
            }
        }
        for (i, j) in candidate_indices {
            if i + j == min_index_sum {
                restaurants.push(list1[i].clone());
            }
        }
        restaurants
    }
}
