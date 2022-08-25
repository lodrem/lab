use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct FileNode {
    is_dir: bool,
    content: String,
    children: HashMap<String, Rc<RefCell<FileNode>>>,
}

impl FileNode {
    fn new_dir() -> Self {
        Self {
            is_dir: true,
            content: Default::default(),
            children: Default::default(),
        }
    }

    fn new_file(content: String) -> Self {
        Self {
            is_dir: false,
            content,
            children: Default::default(),
        }
    }

    fn search_node(&self, path_parts: &[String]) -> Rc<RefCell<FileNode>> {
        let node = self.children.get(&path_parts[0]).unwrap();

        if path_parts.len() == 1 {
            return node.clone();
        } else {
            return node.borrow().search_node(&path_parts[1..]);
        }
    }

    fn create_dir_node(&mut self, path_parts: &[String]) -> Rc<RefCell<FileNode>> {
        let node = match self.children.get(&path_parts[0]) {
            Some(node) => node.clone(),
            None => {
                let node = Rc::new(RefCell::new(Self::new_dir()));
                self.children.insert(path_parts[0].clone(), node.clone());

                node
            }
        };

        if path_parts.len() == 1 {
            return node;
        }
        let mut node = node.borrow_mut();
        node.create_dir_node(&path_parts[1..])
    }
}

struct FileSystem {
    root: Rc<RefCell<FileNode>>,
}

fn parse_path(path: String) -> Vec<String> {
    path.split('/').map(|s| s.to_owned()).collect()
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl FileSystem {
    pub fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(FileNode::new_dir())),
        }
    }

    pub fn ls(&self, path: String) -> Vec<String> {
        let parts = &parse_path(path)[1..];
        let mut node = self.root.clone();
        if !parts.is_empty() && parts[0] != "" {
            node = self.root.borrow().search_node(parts);
        }

        let node = node.borrow();

        if node.is_dir {
            let children_keys = node.children.keys();
            let mut keys = Vec::with_capacity(children_keys.len());

            for k in children_keys {
                keys.push(k.clone());
            }
            keys.sort();
            keys
        } else {
            vec![parts[parts.len() - 1].clone()]
        }
    }

    pub fn mkdir(&self, path: String) {
        let parts = &parse_path(path)[1..];

        if parts.is_empty() || parts[0] == "" {
            return;
        }

        self.root.borrow_mut().create_dir_node(parts);
    }

    pub fn add_content_to_file(&self, file_path: String, content: String) {
        let parts = &parse_path(file_path)[1..];

        let dir_node = {
            let parts = &parts[..parts.len() - 1];
            if parts.is_empty() || parts[0] == "" {
                self.root.clone()
            } else {
                self.root.borrow_mut().create_dir_node(parts)
            }
        };

        let file_name = parts[parts.len() - 1].to_string();

        let mut dir_node = dir_node.borrow_mut();

        match dir_node.children.get(&file_name) {
            Some(file_node) => {
                file_node.borrow_mut().content.push_str(&content);
            }
            None => {
                let file = Rc::new(RefCell::new(FileNode::new_file(content)));

                dir_node.children.insert(file_name, file);
            }
        }
    }

    pub fn read_content_from_file(&self, file_path: String) -> String {
        let parts = &parse_path(file_path)[1..];
        let file_node = self.root.borrow_mut();
        file_node.search_node(parts).borrow().content.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::FileSystem;
    #[test]
    fn it_works() {
        {
            // ["FileSystem","ls","mkdir","addContentToFile","ls","readContentFromFile"]
            // [[],["/"],["/a/b/c"],["/a/b/c/d","hello"],["/"],["/a/b/c/d"]]

            let fs = FileSystem::new();

            assert!(fs.ls("/".to_string()).is_empty());
            fs.mkdir("/a/b/c".to_string());
            fs.add_content_to_file("/a/b/c/d".to_string(), "hello".to_string());
            assert_eq!(vec!["a"], fs.ls("/".to_string()));
            assert_eq!(
                "hello".to_string(),
                fs.read_content_from_file("/a/b/c/d".to_string())
            );
        }

        {
            // ["FileSystem","ls","mkdir","mkdir","mkdir","mkdir","ls","addContentToFile","readContentFromFile","addContentToFile"]
            // [[],         ["/"],["/gh"],["/e"],["/jfo"],["/gh/znflyvnd"],["/gh"],["/mhdmck","v"],["/mhdmck"],["/bbigs","kzdi"]]

            let fs = FileSystem::new();

            assert!(fs.ls("/".to_string()).is_empty());
            fs.mkdir("/gh".to_string());
            fs.mkdir("/e".to_string());
            fs.mkdir("/jfo".to_string());
            fs.mkdir("/gh/znflyvnd".to_string());
            assert_eq!(vec!["znflyvnd"], fs.ls("/gh".to_string()));
            fs.add_content_to_file("/mhdmck".to_string(), "v".to_string());
            assert_eq!(
                "v".to_string(),
                fs.read_content_from_file("/mhdmck".to_string())
            );
            fs.add_content_to_file("/bbigs".to_string(), "kzdi".to_string());
        }

        {
            // ["FileSystem","mkdir","ls","ls","mkdir","ls","ls","addContentToFile","ls","ls","ls"]
            // [[],["/goowmfn"],["/goowmfn"],["/"],["/z"],["/"],["/"],["/goowmfn/c","shetopcy"],["/z"],["/goowmfn/c"],["/goowmfn"]]
            let fs = FileSystem::new();

            fs.mkdir("/goowmfn".to_string());
            assert!(fs.ls("/goowmfn".to_string()).is_empty());
            assert_eq!(vec!["goowmfn"], fs.ls("/".to_string()));
            fs.mkdir("/z".to_string());
            assert_eq!(vec!["goowmfn", "z"], fs.ls("/".to_string()));
            assert_eq!(vec!["goowmfn", "z"], fs.ls("/".to_string()));
            fs.add_content_to_file("/goowmfn/c".to_string(), "shetopcy".to_string());
            assert!(fs.ls("/z".to_string()).is_empty());
            assert_eq!(vec!["c"], fs.ls("/goowmfn/c".to_string()));
            assert_eq!(vec!["c"], fs.ls("/goowmfn".to_string()));
        }
    }
}
