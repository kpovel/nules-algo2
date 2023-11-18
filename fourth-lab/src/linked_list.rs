use chrono::{DateTime, Utc};

#[derive(Debug, Copy, Clone)]
pub struct Student<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub birthday: DateTime<Utc>,
}

impl Student<'_> {
    pub fn new<'a>(
        first_name: &'a str,
        last_name: &'a str,
        birthday: DateTime<Utc>,
    ) -> Student<'a> {
        Student {
            first_name,
            last_name,
            birthday,
        }
    }
}

#[derive(Debug, Clone)]
struct StudentNode<'a> {
    value: Student<'a>,
    next: Option<Box<StudentNode<'a>>>,
}

#[derive(Debug)]
pub struct StudentLinkedList<'a> {
    pub len: usize,
    head: Option<Box<StudentNode<'a>>>,
}

impl<'a> StudentLinkedList<'a> {
    pub fn new() -> Self {
        StudentLinkedList { len: 0, head: None }
    }

    pub fn display(&self) {
        println!("{:#?}", self);
    }

    pub fn prepend(&mut self, value: Student<'a>) {
        let new_node = Box::new(StudentNode {
            value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<Student> {
        if self.head.is_none() {
            return None;
        }

        let mut cursor = &mut self.head;
        while cursor.as_ref()?.next.is_some() {
            cursor = &mut cursor.as_mut()?.next;
        }

        self.len -= 1;
        return cursor.take().map(|node| node.value);
    }

    pub fn sort_by_last_name_asc(&mut self) {
        let mut vec = Vec::new();
        while let Some(node) = self.head.take() {
            vec.push(node.value);
            self.head = node.next;
        }

        vec.sort_by(|a, b| a.last_name.cmp(b.last_name));

        self.len = vec.len();
        for student in vec.into_iter().rev() {
            let new_node = Box::new(StudentNode {
                value: student,
                next: self.head.take(),
            });
            self.head = Some(new_node);
        }
    }

    pub fn sort_by_age_desc(&mut self) {
        let mut vec = Vec::new();
        while let Some(node) = self.head.take() {
            vec.push(node.value);
            self.head = node.next;
        }

        vec.sort_by(|a, b| a.last_name.cmp(b.last_name));

        self.len = vec.len();
        for student in vec.into_iter().rev() {
            let new_node = Box::new(StudentNode {
                value: student,
                next: self.head.take(),
            });
            self.head = Some(new_node);
        }
    }

    pub fn insert_in_sorted_list_by_last_name(&mut self, value: Student<'a>) {
        let mut new_node = Box::new(StudentNode { value, next: None });
        let mut cursor = &mut self.head;

        while let Some(ref mut current) = cursor {
            if new_node.value.last_name <= current.value.last_name {
                new_node.next = current.next.take();
                current.next = Some(new_node);
                self.len += 1;
                return;
            }
            cursor = &mut current.next;
        }

        *cursor = Some(new_node);
        self.len += 1;
    }

    pub fn remove_over_20(&mut self) {
        let mut cursor = &mut self.head;

        while cursor.is_some() {
            let mut remove_node = false;

            if let Some(node) = &cursor {
                if let Some(next_node) = &node.next {
                    let age = Utc::now()
                        .signed_duration_since(next_node.value.birthday)
                        .num_days()
                        / 365;
                    if age > 20 {
                        remove_node = true;
                    }
                }
            }

            if remove_node {
                let next_next = cursor.as_mut().unwrap().next.as_mut().unwrap().next.take();
                cursor.as_mut().unwrap().next = next_next;
                self.len -= 1;
            } else {
                cursor = &mut cursor.as_mut().unwrap().next;
            }
        }

        if self.head.is_some() {
            let age = Utc::now()
                .signed_duration_since(self.head.as_ref().unwrap().value.birthday)
                .num_days()
                / 365;
            if age > 20 {
                self.head = self.head.take().and_then(|node| node.next);
                self.len -= 1;
            }
        }
    }
}
