impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;

        let mut subdomain_counts: HashMap<String, i32> = HashMap::new();
        for cpdomain in cpdomains {
            let mut iter = cpdomain.split_whitespace();
            let count = iter.next().unwrap().parse::<i32>().unwrap();
            let subdomains: Vec<&str> = iter.next().unwrap().split('.').collect();
            let l = subdomains.len();
            for i in 0..l {
                let mut subdomain = String::new();
                for j in i..l {
                    subdomain.push_str(&subdomains[j]);
                    if j < l - 1 {
                        subdomain.push_str(&".");
                    }
                }
                let value = subdomain_counts.entry(subdomain.to_string()).or_insert(0);
                *value += count;
            }
        }

        let mut cpdomains = Vec::with_capacity(subdomain_counts.len());
        for (k, v) in subdomain_counts {
            cpdomains.push(v.to_string() + " " + &k);
        }
        cpdomains
    }
}
