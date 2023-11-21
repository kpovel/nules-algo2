use binary_tree::{BinaryNode, Student};
use chrono::{Utc, TimeZone};

mod binary_tree;

fn main() {
    let mut binary_tree_student = BinaryNode::new(Student {
        first_name: "first",
        last_name: "first",
        birthday: Utc.with_ymd_and_hms(1969, 2, 3, 5, 6, 7).unwrap(),
    });

    binary_tree_student.insert(Student {
        first_name: "second",
        last_name: "second",
        birthday: Utc.with_ymd_and_hms(1968, 2, 3, 5, 6, 7).unwrap(),
    });

    binary_tree_student.insert(Student {
        first_name: "third",
        last_name: "third",
        birthday: Utc.with_ymd_and_hms(1967, 2, 3, 5, 6, 7).unwrap(),
    });

    binary_tree_student.insert(Student {
        first_name: "third",
        last_name: "third",
        birthday: Utc::now(),
    });

    binary_tree_student.print();

    println!(" {:?}", binary_tree_student.students_younger_than_20());
}
