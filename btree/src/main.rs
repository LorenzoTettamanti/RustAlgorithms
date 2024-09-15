// Definisce la struttura del BTreeNode
#[derive(Clone)]
struct BTreeNode {
    t: i32,
    leaf: bool,
    keys: Vec<String>,
    children: Vec<BTreeNode>,
}

/* implementazioni */
impl BTreeNode {
    /* costruttore */
    fn new(t: i32, leaf: bool) -> Self {
        BTreeNode {
            t,
            leaf,
            keys: Vec::new(),
            children: Vec::new(),
        }
    }

    /* implementazioni algoritmo self balancing */
    fn insert_non_full(&mut self, k: String) {
        let mut i = (self.keys.len() as isize) - 1;
        if self.leaf {
            // Inserimento in un nodo foglia
            while i >= 0 && self.keys[i as usize] > k {
                i -= 1;
            }
            self.keys.insert((i + 1) as usize, k);
        } else {
            // Inserimento in un nodo non foglia
            while i >= 0 && self.keys[i as usize] > k {
                i -= 1;
            }
            let mut i = (i + 1) as usize;
            let mut child = self.children[i].clone();
            if child.keys.len() == ((2 * self.t - 1) as usize) {
                self.split_child(i, &mut child);
                if self.keys[i] < k {
                    i += 1;
                }
            }
            self.children[i].insert_non_full(k);
        }
    }

    /* funzione per splittare i child */
    fn split_child(&mut self, i: usize, y: &mut BTreeNode) {
        let t = self.t;
        let mut z = BTreeNode::new(t, y.leaf);

        let split_index = (t - 1) as usize;
        z.keys = y.keys.split_off(split_index + 1);
        if !y.leaf {
            z.children = y.children.split_off(t as usize);
        }

        self.children.insert(i + 1, z);
        self.keys.insert(i, y.keys.remove(split_index));
    }

    /* funzione per cercare una chiave */
    fn search(&self, k: String) -> Option<String> {
        let mut i = 0;
        while i < self.keys.len() && k > self.keys[i] {
            i += 1;
        }
        if i < self.keys.len() && k == self.keys[i] {
            return Some(self.keys[i].clone());
        }
        if self.leaf {
            return None;
        }
        self.children[i].search(k)
    }
}

struct BTree {
    root: BTreeNode,
    t: i32,
}

impl BTree {
    // Crea un nuovo BTree
    fn new(t: i32) -> Self {
        let root = BTreeNode::new(t, true);
        BTree { t, root }
    }

    // Inserisce una chiave nel BTree
    fn insert(&mut self, k: String) {
        let root = &mut self.root;
        if root.keys.len() == (2 * self.t - 1) as usize {
            // Se la radice è piena, crea una nuova radice
            let mut new_root = BTreeNode::new(self.t, false);
            new_root.children.push(root.clone());
            
            // Divide il vecchio nodo radice
            new_root.split_child(0, root);
            
            // Inserisce nel figlio appropriato
            let i = if new_root.keys[0] < k { 1 } else { 0 };
            new_root.children[i].insert_non_full(k);

            // Imposta la nuova radice
            self.root = new_root;
        } else {
            // Inserimento diretto se la radice non è piena
            root.insert_non_full(k);
        }
    }

    // Cerca una chiave nel BTree
    fn search(&self, k: String) -> Option<String> {
        self.root.search(k)
    }
}

fn main() {
    // Crea un nuovo BTree con grado minimo 2
    let mut btree = BTree::new(2);

    // Inserisce alcune chiavi
    btree.insert(String::from("Key1"));
    btree.insert(String::from("Key2"));
    btree.insert(String::from("Key3"));
    btree.insert(String::from("Key4"));

    // Cerca una chiave
    match btree.search(String::from("Key2")) {
        Some(key) => println!("Trovato: {}", key),
        None => println!("Non trovato"),
    }
}
