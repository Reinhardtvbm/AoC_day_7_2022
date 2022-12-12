use std::{cell::RefCell, fmt, process::Child, rc::Rc, vec};

pub type Link = Rc<RefCell<TreeNode>>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Type {
    Directory,
    File,
}

#[derive(Clone)]
pub struct TreeNode {
    system_type: Type,
    value: (usize, String),
    parent: Option<Link>,
    children: Vec<Link>,
}

impl TreeNode {
    pub fn new(parent: Option<Link>, value: (usize, String), system_type: Type) -> Self {
        Self {
            system_type,
            value,
            parent,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, link: &Link) {
        self.children.push(Rc::clone(link));

        let value_to_add = link.borrow().value.0;

        self.value.0 += value_to_add;

        let mut traverse_node = match &self.parent {
            Some(node) => Some(Rc::clone(node)),
            None => None,
        };

        while traverse_node.is_some() {
            traverse_node.clone().unwrap().borrow_mut().value.0 += value_to_add;
            traverse_node = traverse_node.unwrap().borrow_mut().get_parent();
        }
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    pub fn get_parent(&self) -> Option<Link> {
        self.parent.clone()
    }

    pub fn get_children(&self) -> Option<Vec<Link>> {
        if self.children.len() == 0 {
            None
        } else {
            Some(
                self.children
                    .iter()
                    .map(|child| Rc::clone(child))
                    .collect::<Vec<Link>>(),
            )
        }
    }

    pub fn get_name(&self) -> &str {
        self.value.1.as_str()
    }

    pub fn get_size(&self) -> usize {
        self.value.0
    }
}

pub fn part_1(link: Option<Link>) -> usize {
    if let Some(tree_node_ptr) = link {
        let mut answer = 0;

        if let Some(children) = tree_node_ptr.borrow().get_children() {
            for child in &children {
                let value_to_add = child.borrow().value.0;

                if value_to_add <= 100_000 && child.borrow().system_type == Type::Directory {
                    answer += value_to_add;
                }

                let inner_value = part_1(Some(Rc::clone(child)));
                answer += inner_value;
            }
        }

        answer
    } else {
        0
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum TreeError {
    NoCurrReference,
    InvalidChildDirectory,
    NoParentDirectory,
    NoHeadDirectory,
}

#[derive(Debug)]
pub struct Tree {
    pub head: Option<Link>,
    curr_node_ptr: Option<Link>,
}

impl Tree {
    pub fn new(head: (usize, String)) -> Self {
        let head_ptr = Rc::new(RefCell::new(TreeNode::new(None, head, Type::Directory)));

        Self {
            head: Some(Rc::clone(&head_ptr)),
            curr_node_ptr: Some(Rc::clone(&head_ptr)),
        }
    }

    pub fn move_out(&mut self) -> Result<(), TreeError> {
        if let Some(ptr) = self.curr_node_ptr.clone() {
            if ptr.borrow().has_parent() {
                self.curr_node_ptr = Some(Rc::clone(&ptr.borrow().get_parent().unwrap()));

                Ok(())
            } else {
                Err(TreeError::NoParentDirectory)
            }
        } else {
            Err(TreeError::NoCurrReference)
        }
    }

    pub fn move_into_directory(&mut self, directory: String) -> Result<(), TreeError> {
        if let Some(ptr) = self.curr_node_ptr.clone() {
            if let Some(children) = &ptr.borrow().get_children() {
                for child in children {
                    if child.borrow().get_name() == directory.as_str() {
                        self.curr_node_ptr = Some(Rc::clone(child));

                        return Ok(());
                    }
                }
            } else {
                return Err(TreeError::InvalidChildDirectory);
            }
        }

        Err(TreeError::InvalidChildDirectory)
    }

    pub fn return_to_head(&mut self) -> Result<(), TreeError> {
        if let Some(head_ptr) = &self.head {
            self.curr_node_ptr = Some(Rc::clone(head_ptr));

            Ok(())
        } else {
            Err(TreeError::NoHeadDirectory)
        }
    }

    pub fn add_child(
        &mut self,
        value: (usize, String),
        system_type: Type,
    ) -> Result<(), TreeError> {
        if let Some(curr_ptr) = &self.curr_node_ptr {
            curr_ptr
                .borrow_mut()
                .add_child(&Rc::new(RefCell::new(TreeNode::new(
                    Some(Rc::clone(&curr_ptr)),
                    value,
                    system_type,
                ))));

            Ok(())
        } else {
            Err(TreeError::NoCurrReference)
        }
    }
}

pub fn get_directories(link: Option<Link>) -> Vec<usize> {
    let mut directories = vec![];

    if let Some(tree_node_ptr) = link {
        if tree_node_ptr.borrow().system_type == Type::Directory {
            directories.push(tree_node_ptr.borrow().get_size());
        }

        if let Some(children) = tree_node_ptr.borrow().get_children() {
            for child in children {
                directories.extend(get_directories(Some(child)));
            }
        }
    }

    directories
}

impl fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TreeNode")
            .field("value", &self.value)
            .field("children", &self.children)
            .field("type", &self.system_type)
            .finish()
    }
}
