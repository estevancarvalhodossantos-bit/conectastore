use conectastore::{Product, StoreGraph};

#[test]
fn fluxo_completo_de_compra_e_recomendacao() {
    let mut store = StoreGraph::new();
    for (id, name) in [("A", "Camera"), ("B", "Cartao de memoria"), ("C", "Tripé")] {
        assert!(store.add_product(Product { id: id.into(), name: name.into(), category: "Fotografia".into() }));
    }
    assert!(store.connect_similar_products("A", "B", 10));
    assert!(store.connect_similar_products("A", "C", 6));
    assert!(store.record_purchase("cliente-7", "A"));
    let recommendations = store.recommend("cliente-7", 2);
    assert_eq!(recommendations.len(), 2);
    assert_eq!(recommendations[0].id, "B");
}
