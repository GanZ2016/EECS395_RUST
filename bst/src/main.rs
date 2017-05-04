use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::mem;

#[derive(Debug)]
struct BST <K, V> (Link<K, V>);
#[derive(Debug)]
struct Node {
    field: Type
}