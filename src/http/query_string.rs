use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, QueryStringValue<'buf>>,
}

#[derive(Debug)]
pub enum QueryStringValue<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&QueryStringValue> {
        self.data.get(key)
    }
}


impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for substr in s.split('&') {
            let mut key = substr;
            let mut val = "";

            if let Some(i) = substr.find('=') {
                key = &substr[..i];
                val = &substr[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut QueryStringValue| match existing {
                    QueryStringValue::Single(prev_value) => {
                        *existing = QueryStringValue::Multiple(vec![prev_value, val])
                    }
                    QueryStringValue::Multiple(vec) => vec.push(val)
                })
                .or_insert(QueryStringValue::Single(val));
        }

        QueryString { data }
    }
}