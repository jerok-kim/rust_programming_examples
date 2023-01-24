﻿use std::cell::RefCell;
use std::rc::{Rc, Weak};

// 양방향 연결 리스트의 각 노드가 될 구조체
pub struct Node {
    data: isize,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

// 양방향 연결 리스트 자체를 나타낼 구조체
pub struct List {
    head: Option<Rc<RefCell<Node>>>,
    foot: Option<Rc<RefCell<Node>>>,
}

// List 구조체의 메서드 구현
impl List {
    pub fn new() -> Self {
        Self { head: None, foot: None }
    }

    fn new_node(v: isize) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            data: v,
            next: None,
            prev: None,
        }))
    }

    // 리스트의 끝에 값을 추가
    pub fn push(&mut self, v: isize) {
        let n = List::new_node(v);
        match self.foot.take() {
            None => {
                self.foot = Some(Rc::clone(&n));
                self.head = Some(n);
            },
            Some(old_foot) => {
                self.foot = Some(Rc::clone(&n));
                n.borrow_mut().prev = Some(Rc::downgrade(&old_foot));
                old_foot.borrow_mut().next = Some(n);
            },
        }
    }

    // 리스트의 앞에 값을 추가
    pub fn unshift(&mut self, v: isize) {
        let n = List::new_node(v);
        match self.head.take() {
            None => {
                self.foot = Some(Rc::clone(&n));
                self.head = Some(n);
            }
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&old_head));
                n.borrow_mut().next = Some(old_head);
                self.head = Some(n);
            }
        }
    }

    // 반복자를 반환하는 메서드
    pub fn iter(&mut self) -> ListIter {
        match &self.head {
            None => ListIter { cur: None },
            Some(head) => {
                let head = Rc::clone(&head);
                ListIter { cur: Some(head) }
            }
        }
    }
}

// 반복자를 위한 구조체
pub struct ListIter {
    pub cur: Option<Rc<RefCell<Node>>>,
}

// 반복자 구현
impl Iterator for ListIter {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cur.take() {
            None => None,
            Some(cur) => {
                let cb = cur.borrow();
                let data = cb.data;  // 현재 값을 얻음
                match &cb.next {  // 커서를 다음으로 옮김
                    None => self.cur = None,
                    Some(next) => self.cur = Some(Rc::clone(&next)),
                }
                Some(data)
            }
        }
    }
}