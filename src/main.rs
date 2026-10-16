use conectastore::{Product, StoreGraph};
use std::time::Instant;

fn main() {
    let mut store = StoreGraph::new();
    let products = vec![
        Product { id: "P001".into(), name: "Notebook Pro".into(), category: "Eletronicos".into() },
        Product { id: "P002".into(), name: "Mouse sem fio".into(), category: "Acessorios".into() },
        Product { id: "P003".into(), name: "Teclado mecanico".into(), category: "Acessorios".into() },
        Product { id: "P004".into(), name: "Suporte para notebook".into(), category: "Acessorios".into() },
        Product { id: "P005".into(), name: "Monitor 27 polegadas".into(), category: "Eletronicos".into() },
    ];
    for p in products { store.add_product(p); }
    store.connect_similar_products("P001", "P002", 5);
    store.connect_similar_products("P001", "P003", 4);
    store.connect_similar_products("P001", "P004", 3);
    store.connect_similar_products("P001", "P005", 4);
    store.connect_similar_products("P002", "P003", 2);
    store.record_purchase("C001", "P001");

    println!("=== ConectaStore: recomendacao de produtos ===");
    println!("Catalogo cadastrado: {} produtos", store.product_count());
    println!("Cliente C001 comprou: Notebook Pro (P001)");
    println!("Recomendacoes:");
    for (i, p) in store.recommend("C001", 3).iter().enumerate() {
        println!("{}. {} [{}] - categoria: {}", i + 1, p.name, p.id, p.category);
    }

    // Benchmark: constroi grafos esparsos (~5 similaridades por produto, 1 cliente a cada 10
    // produtos com 3 compras) e mede construcao e recomendacao (mediana de 5 execucoes).
    println!("\n=== Medicao de desempenho (use `cargo run --release`) ===");
    println!("{:>9} {:>10} {:>12} {:>16}", "produtos", "arestas", "construcao", "recomendacao");
    for n in [1_000usize, 10_000, 100_000, 500_000] {
        let t = Instant::now();
        let mut g = StoreGraph::new();
        for i in 0..n {
            g.add_product(Product { id: format!("P{i}"), name: format!("Produto {i}"), category: format!("Cat{}", i % 50) });
        }
        let mut seed: u64 = 42;
        let mut next = || { seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); (seed >> 33) as usize };
        let mut edges = 0;
        for i in 0..n {
            for _ in 0..5 {
                let j = next() % n;
                if g.connect_similar_products(&format!("P{i}"), &format!("P{j}"), (next() % 9 + 1) as u32) { edges += 1; }
            }
        }
        for c in 0..n / 10 {
            for _ in 0..3 { g.record_purchase(&format!("C{c}"), &format!("P{}", next() % n)); }
        }
        let build = t.elapsed();
        let mut times: Vec<_> = (0..5).map(|_| { let t = Instant::now(); let r = g.recommend("C0", 10); assert!(!r.is_empty()); t.elapsed() }).collect();
        times.sort();
        println!("{:>9} {:>10} {:>12.2?} {:>16.2?}", n, edges, build, times[2]);
    }
}
