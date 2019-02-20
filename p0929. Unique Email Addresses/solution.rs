impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        use std::collections::HashSet;

        let mut local_names = HashSet::new();
        for email in emails {
            let split: Vec<&str> = email.split('@').collect();
            let local_name = split[0];
            let domain = split[1];

            let split: Vec<&str> = local_name.split('+').collect();
            let mut local_name = split[0].to_string();
            local_name.retain(|c| c != '.');

            local_names.insert(local_name + domain);
        }
        return local_names.len() as i32;
    }
}
