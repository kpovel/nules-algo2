use chrono::{DateTime, Datelike, Utc};

#[derive(Debug, PartialEq)]
pub struct Student<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub birthday: DateTime<Utc>,
}

#[derive(Debug, PartialEq)]
pub struct BinaryNode<'a> {
    pub value: Student<'a>,
    left: Option<Box<BinaryNode<'a>>>,
    right: Option<Box<BinaryNode<'a>>>,
}

impl<'a> BinaryNode<'a> {
    pub fn new(value: Student<'a>) -> Self {
        BinaryNode {
            value,
            left: None,
            right: None,
        }
    }

    pub fn print(&self) {
        println!("{:#?}", self);
    }

    pub fn insert(&mut self, new_value: Student<'a>) {
        if new_value.birthday < self.value.birthday {
            match &mut self.left {
                Some(left_node) => left_node.insert(new_value),
                None => {
                    self.left = Some(Box::new(BinaryNode::new(new_value)));
                }
            }
            return;
        }

        match &mut self.right {
            Some(right_node) => right_node.insert(new_value),
            None => {
                self.right = Some(Box::new(BinaryNode::new(new_value)));
            }
        }
    }

    pub fn count_nodex(&self) -> usize {
        let left_count = self.left.as_ref().map_or(0, |node| node.count_nodex());
        let right_count = self.right.as_ref().map_or(0, |node| node.count_nodex());

        1 + left_count + right_count
    }

    pub fn common_age(&self) -> i32 {
        let left_age = self.left.as_ref().map_or(0, |node| node.common_age());
        let right_age = self.right.as_ref().map_or(0, |node| node.common_age());

        let now = Utc::now();

        let current_age = now.year()
            - self.value.birthday.year()
            - if now.ordinal() < self.value.birthday.ordinal() {
                1
            } else {
                0
            };

        current_age + left_age + right_age
    }

    pub fn youngest_student(&self) -> &Student {
        let mut youngest = &self.value;

        if let Some(ref left) = self.left {
            let left_youngest = left.youngest_student();
            if left_youngest.birthday > youngest.birthday {
                youngest = left_youngest;
            }
        }

        if let Some(ref right) = self.right {
            let right_youngest = right.youngest_student();
            if right_youngest.birthday > youngest.birthday {
                youngest = right_youngest;
            }
        }

        youngest
    }

    pub fn count_leaf_nodes(&self) -> u32 {
        match (&self.left, &self.right) {
            (None, None) => 1,
            (Some(left), None) => left.count_leaf_nodes(),
            (None, Some(right)) => right.count_leaf_nodes(),
            (Some(left), Some(right)) => left.count_leaf_nodes() + right.count_leaf_nodes(),
        }
    }

    pub fn students_younger_than_20(&self, current_date: DateTime<Utc>) -> Vec<&Student> {
        let mut students = Vec::new();

        let age_years = current_date.year()
            - self.value.birthday.year()
            - if current_date.ordinal() < self.value.birthday.ordinal() {
                1
            } else {
                0
            };

        if age_years < 20 {
            students.push(&self.value);
        }

        if let Some(ref left) = self.left {
            students.extend(left.students_younger_than_20(current_date));
        }
        if let Some(ref right) = self.right {
            students.extend(right.students_younger_than_20(current_date));
        }

        students
    }
}
