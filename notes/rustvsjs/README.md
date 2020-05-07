# Benchmarking

In this folder, we try to study the performances of the wasm exported datasets
and the proposed TreeDataset.

The benchmark mostly follows the https://github.com/pchampin/sophia_benchmark
infrastructure with some minor changes.


The tested queries are :

- query1 : A request on POG, output size is linear (with the dataset size)
- query2 : A reuqest on SG, output size is constant (with the dataset size)
- query3 : query1 but on PO only
- query5 : query2 but on S only

The dataset is the persondata dataset.



