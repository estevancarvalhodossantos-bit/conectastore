use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub category: String,
}

#[derive(Clone, Debug)]
struct Edge {
    to: String,
    weight: u32,
}

#[derive(Default)]
pub struct StoreGraph {
    products: HashMap<String, Product>,
    adjacency: HashMap<String, Vec<Edge>>,
    purchases: HashMap<String, HashSet<String>>,
}

impl StoreGraph {
    pub fn new() -> Self { Self::default() }

    pub fn add_product(&mut self, product: Product) -> bool {
        if self.products.contains_key(&product.id) { return false; }
        self.adjacency.entry(product.id.clone()).or_default();
        self.products.insert(product.id.clone(), product);
        true
    }

    pub fn get_product(&self, id: &str) -> Option<&Product> { self.products.get(id) }

    pub fn product_count(&self) -> usize { self.products.len() }

    pub fn connect_similar_products(&mut self, a: &str, b: &str, weight: u32) -> bool {
        if a == b || weight == 0 || !self.products.contains_key(a) || !self.products.contains_key(b) { return false; }
        self.add_edge(a, b, weight);
        self.add_edge(b, a, weight);
        true
    }

    pub fn record_purchase(&mut self, customer_id: &str, product_id: &str) -> bool {
        if !self.products.contains_key(product_id) { return false; }
        let customer_node = format!("cliente:{customer_id}");
        let product_node = product_id.to_string();
        self.adjacency.entry(customer_node.clone()).or_default();
        self.adjacency.entry(product_node.clone()).or_default();
        let inserted = self.purchases.entry(customer_id.to_string()).or_default().insert(product_node.clone());
        if inserted {
            self.add_edge(&customer_node, &product_node, 1);
            self.add_edge(&product_node, &customer_node, 1);
        }
        inserted
    }

    fn add_edge(&mut self, from: &str, to: &str, weight: u32) {
        let edges = self.adjacency.entry(from.to_string()).or_default();
        if let Some(edge) = edges.iter_mut().find(|e| e.to == to) {
            edge.weight = edge.weight.max(weight);
        } else {
            edges.push(Edge { to: to.to_string(), weight });
        }
    }

    /// Recomenda produtos a partir de um cliente (BFS no vertice do cliente).
    /// Produtos ja comprados sao excluidos e o resultado nao contem duplicatas.
    pub fn recommend(&self, customer_id: &str, limit: usize) -> Vec<Product> {
        let start = format!("cliente:{customer_id}");
        let bought = self.purchases.get(customer_id).cloned().unwrap_or_default();
        self.bfs_rank(&start, &bought, limit)
    }

    /// Recomenda produtos relacionados a um produto (BFS no vertice do produto).
    pub fn recommend_for_product(&self, product_id: &str, limit: usize) -> Vec<Product> {
        let mut excluded = HashSet::new();
        excluded.insert(product_id.to_string());
        self.bfs_rank(product_id, &excluded, limit)
    }

    /// BFS em camadas. Cada produto alcancado recebe (distancia, peso da aresta
    /// que o descobriu). Ordenacao: menor distancia, maior peso, id (determinismo).
    fn bfs_rank(&self, start: &str, excluded: &HashSet<String>, limit: usize) -> Vec<Product> {
        if !self.adjacency.contains_key(start) || limit == 0 { return Vec::new(); }
        let mut queue = VecDeque::new();
        let mut visited: HashSet<&str> = HashSet::new();
        let mut ranked: Vec<(usize, u32, &str)> = Vec::new();
        queue.push_back((start, 0usize));
        visited.insert(start);
        while let Some((node, depth)) = queue.pop_front() {
            if let Some(edges) = self.adjacency.get(node) {
                for edge in edges {
                    let to = edge.to.as_str();
                    if visited.insert(to) {
                        if self.products.contains_key(to) && !excluded.contains(to) {
                            ranked.push((depth + 1, edge.weight, to));
                        }
                        queue.push_back((to, depth + 1));
                    }
                }
            }
        }
        ranked.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| b.1.cmp(&a.1)).then_with(|| a.2.cmp(b.2)));
        ranked.into_iter().take(limit).filter_map(|(_, _, id)| self.products.get(id).cloned()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn product(id: &str, name: &str) -> Product { Product { id: id.into(), name: name.into(), category: "Eletronicos".into() } }

    #[test]
    fn cadastra_e_consulta_produto() {
        let mut g = StoreGraph::new();
        assert!(g.add_product(product("p1", "Notebook")));
        assert_eq!(g.get_product("p1").unwrap().name, "Notebook");
        assert!(!g.add_product(product("p1", "Duplicado")));
    }
    #[test]
    fn recomenda_similares_sem_repetir_comprados() {
        let mut g = StoreGraph::new();
        g.add_product(product("p1", "Notebook"));
        g.add_product(product("p2", "Mouse"));
        g.add_product(product("p3", "Teclado"));
        g.connect_similar_products("p1", "p2", 5);
        g.connect_similar_products("p1", "p3", 3);
        g.record_purchase("c1", "p1");
        let result = g.recommend("c1", 10);
        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|p| p.id != "p1"));
        assert_eq!(result[0].id, "p2");
    }
    #[test]
    fn cliente_sem_historico_nao_quebra() {
        let g = StoreGraph::new();
        assert!(g.recommend("inexistente", 5).is_empty());
    }

    #[test]
    fn sem_duplicatas_com_varios_caminhos() {
        let mut g = StoreGraph::new();
        for id in ["a", "b", "c", "d"] { g.add_product(product(id, id)); }
        g.connect_similar_products("a", "d", 2);
        g.connect_similar_products("b", "d", 9);
        g.connect_similar_products("c", "d", 4);
        g.record_purchase("c1", "a");
        g.record_purchase("c1", "b");
        g.record_purchase("c1", "c");
        let r = g.recommend("c1", 10);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].id, "d");
    }
    #[test]
    fn recomenda_por_produto_e_exclui_o_proprio() {
        let mut g = StoreGraph::new();
        for id in ["a", "b", "c"] { g.add_product(product(id, id)); }
        g.connect_similar_products("a", "b", 3);
        g.connect_similar_products("b", "c", 8);
        let r = g.recommend_for_product("a", 10);
        let ids: Vec<_> = r.iter().map(|p| p.id.as_str()).collect();
        assert_eq!(ids, vec!["b", "c"]); // b esta a distancia 1, c a distancia 2
    }
    #[test]
    fn filtragem_colaborativa_via_outro_cliente() {
        let mut g = StoreGraph::new();
        for id in ["a", "b"] { g.add_product(product(id, id)); }
        g.record_purchase("c1", "a");
        g.record_purchase("c2", "a");
        g.record_purchase("c2", "b");
        let r = g.recommend("c1", 5);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].id, "b");
    }
    #[test]
    fn entradas_invalidas_sao_rejeitadas() {
        let mut g = StoreGraph::new();
        g.add_product(product("a", "a"));
        assert!(!g.connect_similar_products("a", "a", 1));
        assert!(!g.connect_similar_products("a", "x", 1));
        assert!(!g.connect_similar_products("a", "a", 0));
        assert!(!g.record_purchase("c1", "x"));
        assert!(g.record_purchase("c1", "a"));
        assert!(!g.record_purchase("c1", "a"));
        assert!(g.recommend("c1", 0).is_empty());
    }
}
