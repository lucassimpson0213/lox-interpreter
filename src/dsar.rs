use std::cmp::Ordering;
fn main() {
    println!("Hello, world!");
}

fn gcd(a: u32, b: u32) -> u32 {
    //basically find the bigger one and divide by the smaller option_env!("
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }
    let bigger_number = a;
    match a.cmp(&b) {
        Ordering::Greater => {
            let greater_num = a;
            let lesser_num = b;
            if greater_num / lesser_num > 0 && greater_num != 0 && lesser_num != 0 {
                gcd(lesser_num, greater_num % lesser_num)
            } else {
                eprintln!("ing u32::max it does not meet conditions");
                u32::MAX
            }
        }
        _ => {
            eprintln!("Number is not less, equal or greater. who knows what is is ");
            u32::MAX
        }
    }
}
/*
*  insert
remove
contains
len
is_empty
clear
iter

Then the real set logic:

union
intersection
difference
symmetric_difference
is_subset
is_superset
is_disjoint
*
*/
mod discretemath {
    use std::error::Error;

    #[derive(PartialEq, Eq, Clone, PartialOrd, Ord)]
    pub enum SetElem {
        Int(u32),
        AnotherSet(Box<Set>),
    }

    #[derive(PartialEq, Eq, Clone, PartialOrd, Ord)]
    pub struct Set {
        set_vec: Vec<SetElem>,
    }

    enum InsertError {
        DuplicateKey(usize),
        CapacityExceeded,
    }
    impl Set {
        pub fn new(slice: &[SetElem]) -> Self {
            Self {
                set_vec: Vec::from(slice),
            }
        }

        pub fn insert(&mut self, elem: SetElem) -> Result<(), InsertError> {
            //ensure uniqueness
            //
            match self.set_vec.binary_search(&elem) {
                Ok(indexofelem) => {
                    return Err(InsertError::DuplicateKey(indexofelem));
                }
                Err(proposed_index_sorted_order) => {
                    self.set_vec.insert(proposed_index_sorted_order, elem);
                }
            }

            Ok(())
        }
        pub fn is_empty(&self) -> bool {
            self.set_vec.len() == 0 as usize
        }

        pub fn is_subset(&self, set: &Set) -> bool {
            //something is a subset if every member of that set, exists in the corresponding set
            //something is a proper subset if every member in that set also exists in the other set
            //and they have the same number of elements

            let parent_and_child_empty = self.set_vec.is_empty() && set.set_vec.is_empty();

            let parent_empty_child_not = self.set_vec.is_empty() && !set.set_vec.is_empty();

            if parent_and_child_empty {
                return true;
            } else if parent_empty_child_not {
                return false;
            }
            let setcount: usize = set.set_vec.len();
            let mut is_in_array = vec![false; setcount];

            for (index, item) in self.set_vec.iter().enumerate() {
                for set_val in set.set_vec.iter() {
                    if set_val == item {
                        is_in_array[index] = true;
                    }
                }
            }

            let num_trues = is_in_array.into_iter().filter(|item| *item).count();
            num_trues == setcount
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::discretemath::Set;
    use crate::discretemath::SetElem;
    use crate::gcd;
    #[test]
    pub fn common_test() {
        let val = gcd(270, 192);

        assert_eq!(val, 6);
    }

    #[test]
    pub fn is_subset_empty_set_empty_subset() {
        let set1 = Set::new(&[]);
        let set2 = Set::new(&[]);
        let is = set1.is_subset(&set2);

        assert!(is);
    }

    #[test]
    pub fn is_subset_parent_empty_child_not() {
        let child = Set::new(&[SetElem::Int(1), SetElem::Int(2)]);
        let parent = Set::new(&[]);

        assert!(!parent.is_subset(&child));
    }

    #[test]
    pub fn ensure_uniqueness() {}
}
