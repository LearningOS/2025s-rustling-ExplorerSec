/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
*/
// I AM DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;
use std::ptr::swap;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
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
        node.prev = self.end;
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
	pub fn reverse(&mut self) where T:Clone{
		
        // 本想写一个较高性能的实现，只交换每个节点的前驱和后继指针，但出了一些问题，暂未解决
        // 先放一个开销较大的实现，它对所有节点进行了重新的构造
        let mut list = LinkedList::<T>::new();
        let mut np =&self.end;
        while let Some(np_node) = *np{
            let val = unsafe{(*np_node.as_ptr()).val.clone()};
            list.add(val);
            np = unsafe{&(*np_node.as_ptr()).prev};
        }
        *self = list;

        /* 下面的代码有问题 */
        // /* 交换所有节点的前驱和后继 */
        // // 标记当前节点的奇偶，确定下一个是“前”或“后”
        // let mut flag:bool = false; 
        // // 一共修改 length 个节点，就是交换 length 对的数据
        // let mut np_now = &self.start; 
        // //for _ in 1..=self.length{
        //  //  let np_node_now = (*np_now).unwrap(); // 拿节点
        // while let Some(np_node_now) = (*np_now){
        //    unsafe{
        //     let next;
        //     match (*np_node_now.as_ptr()).next{
        //         Some(x) => next = Some(x),
        //         None => next = None
        //     }
        //     let prev;
        //     match (*np_node_now.as_ptr()).prev{
        //         Some(x) => prev = Some(x),
        //         None => prev = None
        //     }
        //     (*np_node_now.as_ptr()).prev =next;
        //     (*np_node_now.as_ptr()).next =prev;
        //     //let prev =(*np_node_now.as_ptr()).prev;
        //     //(*np_node_now.as_ptr()).next = prev; 
        //     //(*np_node_now.as_ptr()).prev = Some(next);
        //    }
        //    
        //    if flag{
        //     unsafe{np_now =&(*np_node_now.as_ptr()).next}  
        //    }else{
        //     unsafe{np_now =&(*np_node_now.as_ptr()).prev} 
        //    }
        //    flag = !flag; // 取反奇偶
        // }
        // /* 交换双向链表上记录的首和尾 */
        // let tmp = self.start; // 引用是实现了 Copy 的
        // self.start = self.end;
        // self.end = tmp;
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
    fn test_reverse_linked_list_1() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![2,3,5,11,9,7];
		let reverse_vec = vec![7,9,11,5,3,2];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}

	#[test]
	fn test_reverse_linked_list_2() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![34,56,78,25,90,10,19,34,21,45];
		let reverse_vec = vec![45,21,34,19,10,90,25,78,56,34];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}
}