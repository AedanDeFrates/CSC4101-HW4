use std::collections::HashMap;

/// Returns the first `n` Fibonacci numbers.
pub fn fib(_n: u32) -> Vec<u32> 
{
    // TODO: implement
    if _n == 0 {return vec![];}
    if _n == 1 {return vec![0];}
    else{
        let mut v: Vec<u32> = vec![0,1];

        for i in 2.._n
        {
        let next: u32 = v[i as usize -1] + v[i as usize -2];
        v.push(next);
        }
        v
    }
}

/// Returns true if `n` is a palindrome, false otherwise.
pub fn is_palindrome(_n: u32) -> bool {
    // TODO: implement
    let s = _n.to_string();
    let rev = s.chars().rev().collect::<String>();
    let rev_n: u32 = rev.parse().unwrap();
    
    if rev_n == _n {return true;}
    else {return false;}
}

/// Returns the nth largest element in `a`, or None if it does not exist.
pub fn nthmax(_n: usize, _a: &[i32]) -> Option<i32> {
    // TODO: implement
    if _a .is_empty() ||_n >= _a.len() {return None;}

    let mut v =_a.to_vec();
    v.sort();
    v.reverse();

    return Some(v[_n]);
}

/// Returns a one-character String containing the most frequent character in `s`.
pub fn freq(_s: &str) -> String {
    // TODO: implement
    if _s.is_empty() {return "".to_string();}

    let mut map: HashMap<char, u32> = HashMap::new();
    for c in _s.chars()
    {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    return map.into_iter().max_by_key(|&(_, count)| count).unwrap().0.to_string();
}

/// Zips two slices into a HashMap, mapping arr1[i] -> arr2[i].
pub fn zip_hash(_arr1: &[String], _arr2: &[String]) -> Option<HashMap<String, String>> {
    // TODO: implement
    if _arr1.len() != _arr2.len() {return None;}
    let mut map : HashMap<String, String> = HashMap::new();
    for i in 0.._arr1.len()
    {
        map.insert(_arr1[i].clone(), _arr2[i].clone());
    }
    return Some(map);
}

/// Converts a HashMap into a Vec of (key, value) pairs.
pub fn hash_to_array(_map: &HashMap<String, String>) -> Vec<(String, String)> {
    // TODO: implement
    let mut v: Vec<(String, String)> = Vec::new();
    for (k, val) in _map.iter()
    {
        v.push((k.clone(), val.clone()));
    }
    v.sort_by(|a, b| a.0.cmp(&b.0));
    return v;
}

// ========================
// Part 2: PhoneBook
// ========================

/// A single phone book entry.
#[derive(Debug, Clone)]
pub struct PhoneEntry {
    pub name: String,
    pub number: String,
    pub is_listed: bool,
}

/// PhoneBook holds name/number pairs and whether each is listed.
#[derive(Debug, Default)]
pub struct PhoneBook {
    // You are free to change this internal representation if you want.
    pub entries: Vec<PhoneEntry>,
}

impl PhoneBook {
    /// Constructor: create an empty PhoneBook.
    pub fn new() -> Self {
        // You may also use `Self::default()`
        PhoneBook {
            entries: Vec::new(),
        }
    }

    /// Attempts to add a new entry.
    ///
    /// Rules:
    /// 1. If the name already exists, return false.
    /// 2. If the number is not in the format NNN-NNN-NNNN, return false.
    /// 3. A number can be unlisted any number of times, but listed at most once.
    ///    - If the number already exists as listed, adding another listed entry
    ///      with the same number must return false.
    ///
    /// Returns true if the entry was successfully added.
    pub fn add(
        &mut self,
        _name: String,
        _number: String,
        _is_listed: bool,
    ) -> bool {
        // TODO: implement
        if _number.len() != 12 || _number.chars().nth(3) != Some('-') || _number.chars().nth(7) != Some('-') {return false;}
        else{
            for entry in &self.entries
            {
                if entry.name == _name {return false;}
                if entry.number == _number && entry.is_listed && _is_listed {return false;}
            }
            let new_entry = PhoneEntry{name: _name,number: _number,is_listed: _is_listed,};
            self.entries.push(new_entry);
            return true;
        }
    }

    /// Looks up `name` and returns the number ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup(&self, _name: &str) -> Option<String> {
        // TODO: implement
        for i in &self.entries
        {
            if i.name == _name && i.is_listed {return Some(i.number.clone());}
            else {continue;}
        }
        return None;
    }

    /// Looks up `num` and returns the associated name ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup_by_num(&self, _num: &str) -> Option<String> {
        // TODO: implement
        for i in &self.entries
        {
            if i.number == _num && i.is_listed {return Some(i.name.clone());}
            else {continue;}
        }
        return None;
    }

    /// Returns all names (listed and unlisted) whose numbers begin with `areacode`.
    pub fn names_by_ac(&self, _areacode: &str) -> Vec<String> {
        // TODO: implement
        let mut searchArr = Vec::new();
        for i in &self.entries
        {
            if _areacode == &i.number[0..3]
            {
                searchArr.push(i.name.clone());
            }
        }
        return searchArr;
    }
}