/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
// I AM DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;
use std::cmp::PartialOrd;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self 
    where T:PartialOrd + Copy // T 必须能比较大小，这里为了方便指定了 Copy，更多样例应该用 Clone
	{
        // Option(NonNull(Node(T)))
		let mut list = LinkedList::<T>::new();
        let mut np_a = &list_a.start;
        let mut np_b = &list_b.start;
        // 当两个链表都有数据时，依次取小的值构造节点，放入新链表
        while let (Some(np_node_a),Some(np_node_b)) =(*np_a, *np_b){
            let val_a = unsafe{(*np_node_a.as_ptr()).val};
            let val_b = unsafe{(*np_node_b.as_ptr()).val};
            
            if val_a <val_b{
                list.add(val_a);
                np_a = unsafe{&(*np_node_a.as_ptr()).next};
            }
            else{
                list.add(val_b);
                np_b = unsafe{&(*np_node_b.as_ptr()).next};
            }
        }
        // 可能剩下 a 中有数据，由于有序，直接放在新链表后面
        while let Some(np_node_a) =*np_a{
            let val_a = unsafe{(*np_node_a.as_ptr()).val};
            list.add(val_a);
            np_a =unsafe{&(*np_node_a.as_ptr()).next};
        }
        // 可能剩下 b 中有数据，由于有序，直接放在新链表后面
        while let Some(np_node_b) =*np_b{
            let val_b = unsafe{(*np_node_b.as_ptr()).val};
            list.add(val_b);
            np_b =unsafe{&(*np_node_b.as_ptr()).next};
        }

        return list;

        
        /* 下面是早期理解偏差，写的不同版本不能通过编译的代码  */
        /*  while let (Some(pa),Some(pb)) = (list_a.start,list_b.start){
            list_a.end =None;
            list_b.end =None;
            
            let mut p_less:NonNull<Node<T>> = NonNull<Node::<T>::new(0)>;
            let mut p_less_next:NonNull<Node<T>> = None;
            let list_used = &list_a;
            unsafe{
                if (*pa.as_ptr()).val < (*pb.as_ptr()).val{
                    p_less =Some(pa);
                    p_less_next = (*pa.as_ptr()).next;
                }else{
                    p_less =Some(pb);
                    p_less_next = (*pb.as_ptr()).next;
                    list_used = &list_b;
                }
            
            // 增加 list 长度
            p_less.next =None;
            match list.end {
                None => list.start = p_less,
                Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = p_less },
            }
            list.end = p_less;
            list.length += 1;
            // 消减源 list 长度
            list_used.start = p_less_next;
            list_used.length -=1;}
        }
        return list;*/
            /*
            if pa.as_ref().val < pb.as_ref().val {
                // list add
                let mut ptr = Box::new(pa);
                let mut node_next = Box::new(ptr.next);
                node.next = None;
                let node_ptr = Some(ptr);
                match list.end {
                    None => list.start = node_ptr,
                    Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
                }
                list.end = node_ptr;
                list.length += 1;
                // list_a sub
                list_a.start = Some(unsafe{ NonNull::new_unchecked(Box::into_raw(node_next)) });
                list_a.length -=1;

            }else{
                // list add
                let mut node = Box::new(pb);
                let mut node_next = Box::new(node.next);
                node.next = None;
                let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
                match list.end {
                    None => list.start = node_ptr,
                    Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
                }
                list.end = node_ptr;
                list.length += 1;
                // list_a sub
                list_b.start = Some(unsafe{ NonNull::new_unchecked(Box::into_raw(node_next)) });
                list_b.length -=1;
            }
            
        }
        return list; 
        */
        
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}