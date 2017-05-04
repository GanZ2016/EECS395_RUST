use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::mem;

#[derive(Debug)]
struct BST <K, V> (Link<K, V>);
#[derive(Debug)]
struct Node <K, V>{
    key: K,
    value: V,
    left: Link<K, V>,
    right: Link<K,V>,
}

type Link<K,V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
struct CursorMut <'a, K: 'a, V: 'a>(Option<&'a mut Node<K,V>>);
impl <'a K, V> CursorMut<'a K, V> {
    fn left(&mut self)  {
        self.0.take().map(|node| {
            self.0 = node.left.as_mut().map(|node| &mut **node);
        });
    }
    fn right(&mut self) {
        self.0.take().map(|node| {
            self.0 = node.right.as_mut().map(|node| &mut **node);
        });
    }

    fn key(&mut self) -> Option<&K> {
        self.0.as_ref().map(|n| &n.key)
    }

    fn into_value() -> Option<&'a mut V> {
        self.0.map(|n| &mut n.value)
    }
}
//recrsive get length
impl <K:Ord, V> Node<K,V>{
    fn len(ptr: &Link<K,V>) -> usize {
        if let &Some(ref n) = ptr {
            1 + Node::len(&n.left) + Node::len(&n.right)
        }
        else {
            0
        }
    }

    fn find_rec<'a, 'b>(ptr: &a' Link<K, V>, key: &'b k) -> Option<&'a V> {
        if let &Some(ref n) = ptr{
            match key.cmp(&n.key) {
                Less => Node::find_rec(&n.left, key),
                Greater => Node::find_rec(&n.right, key),
                Equal => Some(&n.value),
            }
        }
        else{
            None
        }

    }

    fn find_iter<'a, 'b>(mut ptr: &'a Link<K,V>, key: &'b K) -> Option<&'a V> {
        while let &Some(ref n) = ptr {
            match key.cmp(&n.key) {
                Less => {ptr = &n.left;}
                Greater => {ptr = &n.right;}
                Equal => {return Some(&n.value);}
            }
        }
        return None;
    }
}