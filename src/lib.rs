//! # TableMap
//!
//! **this name may not be final**
//!
//! HashMap, BTreeMap, IndexMap needs a lot of memory in case of String based keys, and large number of data.
//!
//! This is a simple library that tries to memory efficiently provide a `IndexMap` with a String key like functionality using vecs, that might have a large number of data with string keys. There might be other better solutions in the wild.
//!
//! As the String keys are mapped to vec index we are storing the string keys only once, instead of we keep the best of both worlds. I have not benchmarked it, so can not say anything about performance.
//!
//! Simple macros are provided for easy assigning of data.
//!
//! #### To do
//!
//! - [ ] fill to end row by index
//! - [ ] Removing a row
//! - [ ] Clear data
//!
//!
//! ## Initializing and inserting
//!
//! ```rust
//! use dyn_col_map::table_map::TableMap;
//! use dyn_col_map::{push, update_row};
//! let mut cm = TableMap::new();
//! cm.add_columns(vec!["c0", "c1", "c2", "c3"]);
//! // single insert
//! cm.insert("c1", "Something").unwrap();
//! // single insert using macro, will not change row
//! update_row! { cm, "c0", "c0v" }
//! // multiple inserts using macro, will not add a new row
//! update_row! {
//!     cm,
//!     "c1", "Something",
//!     "c2", "v2",
//!     "c3", "32"
//! }
//! // this will create a new row and insert
//! push! {
//!     cm,
//!     "c0", "Another thing",
//!     "c1", "second column",
//!     "c2", "another value",
//!     "c3", "final column"
//! }
//! // getting a value from current row
//! let v = cm.get_column_value("c1").unwrap();
//! assert_eq!(v, "second column");
//! // getting a value from another row
//! let v = cm.get_column_value_by_index(0, "c1").unwrap();
//! assert_eq!(v, "Something");
//!
//! ```
//!
//! This also provides benefit with different datasets, which may not have similar columns.
//! So, in case of one dataset with columns `c1` and `c2` another with `c5` and `c6`
//!
//! ```rust
//! use dyn_col_map::{push, update_row};
//! use dyn_col_map::table_map::TableMap;
//! let mut cm = TableMap::new();
//!  // first dataset, but you can add all of the columns beforehand as usual
//!  // cm.add_columns(vec!["c0", "c1", "c4", "c5"]);
//!
//! cm.add_columns(vec!["c0", "c1"]);
//! // insert data for first dataset
//! push! {
//!         cm,
//!         "c0", "c0v",
//!         "c1", "Something"
//!     }
//! // now another dataset found
//! cm.add_columns(vec!["c4", "c5"]);
//! // insert data for second dataset
//! push! {
//!         cm,
//!         "c4", "v2",
//!         "c5", "32"
//!     }
//!
//! // another dataset with mixed columns, as names are already added,
//! // no new columns will be added and the sequence will stay
//! // the same
//! cm.add_columns(vec!["c1", "c5"]);
//! push! {
//!         cm,
//!         "c1", "another set",
//!         "c5", "mixed dataset"
//!     }
//!
//! assert_eq!(
//!     cm.get_vec(),
//!     &vec![
//!         vec!["c0v", "Something"],  // NOTE: this is not filled up
//!         vec!["", "", "v2", "32"],
//!         vec!["", "another set", "", "mixed dataset"],
//!     ]
//! );
//!
//! ```
//!
//! One issue is, as noted in the example above, any rows inserted before a new column is added,
//! will not be filled up, and cause error when we try to get value for the new column from those
//! rows. Any rows added after will have them.
//!
//! To solve this issue, `fill_to_end` method should be used for each row as necessary.
//!
//! Following example attempts to clarify the issue, and provide solution.
//!
//! ```rust
//!     use dyn_col_map::{push, update_row};
//!     use dyn_col_map::table_map::TableMap;
//!     let mut cm = TableMap::new();
//!     cm.add_columns(vec!["c0", "c1", "c2", "c3"]);
//!
//!     update_row! {
//!             cm,
//!             "c0", "r1d0",
//!             "c2", "r1d2"
//!         }
//!
//!     // now a new column is added
//!     cm.add_column("c4");
//!
//!     // this will cause a NoDataSet error, cause column c4 was created after setting
//!     // this row, and it does not exists
//!     let n = cm.get_column_value("c4");
//!     assert!(n.is_err());
//!
//!     // fill the row with default value
//!     cm.fill_to_end();
//!     // now it will be okay
//!     let n = cm.get_column_value("c4");
//!     assert!(n.is_ok());
//!
//!     // all the next rows will have all the columns
//!     push! {
//!             cm,
//!             "c0", "r2d0",
//!             "c2", "r2d2"
//!         }
//!
//!     // this will work without filling up
//!     let n = cm.get_column_value("c4");
//!     assert!(n.is_ok());
//!
//! ```
//!
//! ## What this crate tries to solve?
//!
//! It is trying to maintain the lower memory usage of a vec and ordered key based accessing of an IndexMap.
//!
//! In my own testing, with a dataset of 947300 rows,
//! * HashMap/IndexMap implementation was out of memory on my 64GB machine,
//! * TableMap was 37GB.
//! * Interestingly Python was only 27GB.
//!
//! As I understand, HashMap/IndexMap, stores all the keys for each row, and in addition to that, they provide performance for the price of high memory usage. Unfortunately, It was not suitable for my task and I have not found any other solutions online. So here's what I devised.
//!
//! `fill_to_end` may not be optimal. If I ever find a better way, I will try to incorporate it.

pub mod table_map;
pub mod table_map_errors;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table_map::TableMap;
    use crate::table_map_errors::TableMapErrors;

    #[test]
    fn test_macro() {
        let mut tm = TableMap::new();
        tm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        update_row! { tm, "c0", "c0v" }
        update_row! {
            tm,
            "c1", "Something",
            "c2", "v2",
            "c3", "32"
        }
        // get all the columns, sequence is maintained
        assert_eq!(tm.get_columns(), vec!["c0", "c1", "c2", "c3"]);
        assert_eq!(tm.get_vec(), &vec![vec!["c0v", "Something", "v2", "32"]]);
    }

    #[test]
    fn test_macro_obj() {
        #[derive(Clone, Default, PartialEq, Debug)]
        struct TestStruct {
            val: i32,
        }
        let ar = vec![
            TestStruct { val: 30 },
            TestStruct { val: 100 },
            TestStruct { val: 1230 },
            TestStruct { val: 800 },
        ];
        let mut tm = TableMap::new();
        tm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        update_row! { tm, "c0", ar[0].clone() }
        update_row! {
            tm,
            "c1", ar[1].clone(),
            "c2", ar[2].clone(),
            "c3", ar[3].clone()
        }
        assert_eq!(tm.get_vec(), &vec![ar])
    }

    #[test]
    fn test_insert_randomly() {
        let mut tm = TableMap::new();
        tm.add_columns(vec!["c0", "c1", "c2", "c3"]);

        update_row! {
            tm,
            "c1", "Something",
            "c3", "Another thing",
            "c2", "First thing"
        }
        assert_eq!(tm.get_column_value("c1").unwrap(), "Something");
        assert!(tm.get_column_value("c10").is_err());
        assert_eq!(
            tm.get_vec(),
            &vec![vec!["", "Something", "First thing", "Another thing"]]
        );
    }

    #[test]
    fn test_extending_with_new_column() {
        let mut tm = TableMap::new();
        tm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        update_row! {
            tm,
            "c1", "Something",
            "c3", "Another thing",
            "c2", "First thing"
        }
        tm.add_column("c5");
        tm.insert("c0", "First First thing").unwrap();
        // no matter how the data is inserted, the sequence of column is maintained
        assert_eq!(
            tm.get_vec(),
            &vec![vec![
                "First First thing",
                "Something",
                "First thing",
                "Another thing",
                "",
            ]]
        );
    }

    #[test]
    fn test_multiple_row_with_empty_column() {
        let mut tm = TableMap::new();
        tm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        push! {
            tm,
            "c0", "c0v",
            "c1", "Something",
            "c2", "v2",
            "c3", "32"
        }
        push! {
            tm,
            "c0", "c0v",
            "c2", "v2",
            "c3", "32"
        }
        push! {
            tm,
            "c0", "c0v",
            "c1", "Something",
            "c2", "v2"
        }
        assert_eq!(
            tm.get_vec(),
            &vec![
                vec!["c0v", "Something", "v2", "32"],
                vec!["c0v", "", "v2", "32"],
                vec!["c0v", "Something", "v2", ""],
            ]
        );
    }

    #[test]
    fn test_multi_datasets_csv() {
        let mut tm = TableMap::new();
        tm.add_columns(vec!["c0", "c1"]);
        // insert data for first dataset
        push! {
            tm,
            "c0", "c0v",
            "c1", "Something"
        }
        tm.add_columns(vec!["c4", "c5"]);
        // insert data for second dataset
        push! {
            tm,
            "c4", "v2",
            "c5", "32"
        }
        // mixture of dataset is possible
        tm.add_columns(vec!["c1", "c5"]);
        push! {
            tm,
            "c1", "another set",
            "c5", "mixed dataset"
        }
        assert_eq!(
            tm.get_vec(),
            &vec![
                vec!["c0v", "Something"],
                vec!["", "", "v2", "32"],
                vec!["", "another set", "", "mixed dataset"],
            ]
        );
    }

    // testing unset columns
    fn setup_for_unset_columns() -> TableMap<String> {
        let mut tm = TableMap::new();
        tm.add_columns(vec!["c0", "c1", "c2", "c3"]);
        update_row! {
            tm,
            "c0", "r1d0".into(),
            "c2", "r1d2".into()
        }
        tm
    }

    #[test]
    fn test_unset_column_value_should_be_empty() {
        let mut tm = setup_for_unset_columns();
        // this will be an empty value, as inserted row does not set "c3" column
        assert_eq!(tm.get_column_value("c3").unwrap(), "");
    }

    #[test]
    fn test_accessing_rows_added_before_additional_column_returns_error() {
        let mut tm = setup_for_unset_columns();
        tm.add_column("c4");
        // this will cause a NoDataSet error, cause column c4 was created after setting *this* row
        assert!(tm.get_column_value("c4").is_err());
    }

    #[test]
    fn test_filling_unset_columns() {
        let mut tm = setup_for_unset_columns();
        tm.add_column("c4");
        tm.fill_to_end();
        assert!(tm.get_column_value("c4").is_ok());
    }

    #[test]
    fn test_before_moving_to_next_row_will_not_fill_up_current_row() {
        let mut tm = setup_for_unset_columns();
        tm.add_column("c4");
        tm.next_row();
        println!("{:?}", tm.get_vec());
        assert!(tm.get_column_value_by_index(0, "c4").is_err());
    }
}
