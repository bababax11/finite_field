#[derive(Debug)]
pub struct RBTree<K, V>
where
    K: Ord,
{
    root: Option<Box<Node<K, V>>>,
    change: bool,
}
#[derive(PartialEq, Debug)]
enum Color {
    R,
    B,
    ErrC,
}
#[derive(Debug)]
struct Node<K, V>
where
    K: Ord,
{
    color: Color,
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}
impl<K, V> Node<K, V>
where
    K: Ord,
{
    fn new(color: Color, key: K, value: V) -> Self {
        Self {
            color: color,
            key: key,
            value: value,
            left: None,
            right: None,
        }
    }
    // root の右側のノードが新たな根となるように木を回転させ、新たな根を返す。
    fn rotate_left(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut new_root = root.right.unwrap();
        root.right = new_root.left;
        new_root.left = Some(root);
        new_root
    }
    // root の左側のノードが新たな根となるように木を回転させ、新たな根を返す。
    fn rotate_right(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut new_root = root.left.unwrap();
        root.left = new_root.right;
        new_root.right = Some(root);
        new_root
    }
    fn rotate_lr(mut t: Box<Node<K, V>>) -> Box<Node<K, V>> {
      t.left = Some(Node::rotate_left(t.left.unwrap()));
      Node::rotate_right(t)
    }
    fn rotate_rl(mut t: Box<Node<K, V>>) -> Box<Node<K, V>> {
      t.right = Some(Node::rotate_right(t.right.unwrap()));
      Node::rotate_left(t)
    }
}
fn check_r<K, V>(child: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>>
where
  K: Ord,
{
  match child {
    None => None,
    Some(t) => {
      if t.color == Color::R {
        Some(t)
      } else {
        None
      }
    }
  }
}
impl<K, V> RBTree<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        Self {
            root: None,
            change: false,
        }
    }
    pub fn insert(&mut self, key: K, x: V) {
      let mut _root = self._insert(self.root, key, x);
      _root.color = Color::B;
      self.root = Some(_root);
    }
    fn _insert(&mut self, t: Option<Box<Node<K, V>>>, key: K, x: V) -> Box<Node<K, V>> {
      match t {
        None => {
          self.change = true;
          Box::new(Node::new(Color::R, key, x))
        }
        Some(mut t_) => {
          if key < t_.key {
            t_.left = Some(self._insert(t_.left, key, x));
            self.balance(t_)
          } else if key > t_.key {
            t_.right = Some(self._insert(t_.right, key, x));
            self.balance(t_)
          } else {
            self.change = false;
            t_.value = x;
            t_
          }
        }
      }
    }
    fn balance(&mut self, t: Box<Node<K, V>>) -> Box<Node<K, V>> {
      if !self.change {
        return t;
      }
      if t.color != Color::B {
        return t;
      }
      if let Some(tl) = check_r(t.left) {
        if let Some(_) = check_r(tl.left) {
          t = Node::rotate_right(t);
          t.left.unwrap().color = Color::B;
          return t;
        }
        if let Some(_) = check_r(tl.right) {
          t = Node::rotate_lr(t);
          t.left.unwrap().color = Color::B;
          return t;
        }
      }
      if let Some(tr) = check_r(t.right) {
        if let Some(_) = check_r(tr.left) {
          t = Node::rotate_rl(t);
          t.right.unwrap().color = Color::B;
          return t;
        }
        if let Some(_) = check_r(tr.right) {
          t = Node::rotate_right(t);
          t.right.unwrap().color = Color::B;
          return t;
        }
      }
      self.change = false;
      t
    }
}
#[test]
fn new_tree() {
    let tree = RBTree::<i32, f64>::new();
    assert!(!tree.change);
}
