# ConectaStore — Sistema de Recomendação baseado em Grafos

Autor: Estevan Carvalho dos Santos

## Objetivo
Demonstrar como um grafo pode representar relações entre clientes e produtos para sugerir itens relacionados ao histórico de compras.

## Tecnologias e estruturas
- Rust (edition 2021), sem dependências externas.
- `HashMap`: consulta de produtos por identificador e armazenamento da lista de adjacência.
- `Vec`: lista de arestas de cada vértice.
- `HashSet`: histórico de compras e controle de vértices visitados/duplicatas.
- `VecDeque`: fila FIFO usada pela busca em largura (BFS).
- `Instant`: medição de tempo de construção do grafo e de recomendação.

## Arquitetura
O grafo é não direcionado. Produtos e clientes são vértices; arestas representam compras ou similaridade entre produtos. Cada aresta possui peso positivo. As arestas de compra recebem peso 1; as de similaridade recebem um peso informado na conexão. O cadastro de produtos é indexado por ID.

## Pré-requisitos
Instale o Rust e o Cargo pelo site oficial: https://www.rust-lang.org/tools/install

## Compilar e executar
Na pasta `conectastore`:

```bash
cargo run --release
```


## Exemplo de uso (API)
```rust
let mut store = StoreGraph::new();
store.add_product(Product { id: "P001".into(), name: "Notebook".into(), category: "Eletronicos".into() });
store.add_product(Product { id: "P002".into(), name: "Mouse".into(), category: "Acessorios".into() });
store.connect_similar_products("P001", "P002", 5);   // aresta de similaridade (peso 5)
store.record_purchase("C001", "P001");               // aresta de compra cliente-produto
store.recommend("C001", 3);                          // recomendação a partir de um cliente
store.recommend_for_product("P001", 3);              // recomendação a partir de um produto
store.get_product("P001");                           // consulta por ID
```

## Exemplo de saída (`cargo run --release`)
```
Cliente C001 comprou: Notebook Pro (P001)
Recomendacoes:
1. Mouse sem fio [P002] - categoria: Acessorios
2. Teclado mecanico [P003] - categoria: Acessorios
3. Monitor 27 polegadas [P005] - categoria: Eletronicos
```

## Resultados dos testes de desempenho
Grafo esparso: ~5 similaridades por produto e 1 cliente a cada 10 produtos (3 compras cada). Compilado com `--release`; recomendação = mediana de 5 execuções (top 10). Os tempos variam conforme a máquina.

| Produtos | Arestas de similaridade | Construção do grafo | Recomendação (BFS) |
|---------:|------------------------:|--------------------:|-------------------:|
| 1.000    | 4.997                   | 4,14 ms             | 410,70 µs          |
| 10.000   | 49.996                  | 44,18 ms            | 7,31 ms           |
| 100.000  | 499.993                 | 620,48 ms              | 190,68 ms          |
| 500.000  | 2.499.995               | 3,92 s              | 1,63 s             |

O tempo cresce de forma próxima à linear com V + E, como esperado para BFS, com variações causadas por cache de memória e tabelas hash maiores (de 10 mil para 100 mil produtos, 10× mais dados levaram ~26× mais tempo de recomendação). Como a BFS atual percorre todo o componente conectado, uma melhoria natural é limitar a profundidade ou parar ao reunir candidatos suficientes.

## Testes
```bash
cargo test
```
Testes unitários (7, em `src/lib.rs`) e de integração (`tests/`) cobrem cadastro/consulta, exclusão de itens já comprados, ausência de duplicatas com vários caminhos, recomendação por produto, filtragem colaborativa via outro cliente e entradas inválidas.

## Complexidade
- Cadastro/consulta por ID: O(1) esperado por operação de `HashMap`.
- Inserção de aresta: O(grau do vértice) nesta implementação, pois verifica se a conexão já existe.
- BFS: O(V + E) no grafo percorrido.
- Espaço: O(V + E), para vértices e arestas, além dos índices auxiliares.

## Limitações e melhorias futuras
Este protótipo utiliza dados em memória e exemplos pequenos. Para produção, seria necessário persistência, importação de dados reais, avaliação de relevância das recomendações, privacidade, tratamento de catálogo distribuído e testes de desempenho com grafos maiores. A BFS prioriza distância; o peso é usado como critério de desempate entre candidatos com a mesma distância.

## Vídeo pitch
**[COLE AQUI O LINK DO YOUTUBE]**

## Repositório
**[COLE AQUI O LINK DO GITHUB]**
