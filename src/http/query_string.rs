use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    pub data: HashMap<&'buf str, Value<'buf>>
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i+1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => *existing = Value::Multiple(vec![prev_val, val]),
                    Value::Multiple(vec) => vec.push(val)
                }).or_insert(Value::Single(val));
        }
        
        QueryString{data}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_correctnes_of_single_value_type(option_value: Option<&Value>, expected: &str) {
        if let Some(value) = option_value {
            if let Value::Single(str) = value {
                assert_eq!(str, &expected);
            } else {
                assert!(false, "Wrong Value type!")
            }
        } else {
            assert!(false, "No key in map!")
        }
    }

    fn check_correctnes_of_multiple_type_value(option_value: Option<&Value>, expected: &Vec<&str>) {
        if let Some(value) = option_value {
            if let Value::Multiple(vec_of_str) = value {
                assert_eq!(vec_of_str, expected);
            } else {
                assert!(false, "Wrong Value type!")
            }
        } else {
            assert!(false, "No key in map!")
        }
    }

    #[test]
    fn query_string_should_be_created_from_str() {
        let query = "a=1&b=2&c&d=&e===&d=7&d=abc";
        let query_string = QueryString::from(query);
        
        check_correctnes_of_single_value_type(
            query_string.data.get("a"), "1");

        check_correctnes_of_single_value_type(
            query_string.data.get("c"), "");

        check_correctnes_of_single_value_type(
            query_string.data.get("e"), "==");

        check_correctnes_of_multiple_type_value(
            query_string.data.get("d"), &vec!["", "7", "abc"]);
    }
}