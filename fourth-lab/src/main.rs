mod linked_list;

use chrono::{Utc, Months};
use linked_list::{Student, StudentLinkedList};

fn main() {
    let mut student_linked_list = StudentLinkedList::new();
    student_linked_list.prepend(Student::new("first", "a", Utc::now()));
    student_linked_list.prepend(Student::new("second", "b", Utc::now()));
    student_linked_list.prepend(Student::new("John", "Doe", Utc::now() - Months::new(12 * 69)));

    student_linked_list.display();

    student_linked_list.sort_by_age_desc();

    student_linked_list.insert_in_sorted_list_by_last_name(Student::new("fourth", "a", Utc::now()));
    student_linked_list.remove_over_20();

    student_linked_list.display();

}
