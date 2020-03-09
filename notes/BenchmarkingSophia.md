# Benchmarking Sophia

> Adapters in this repo are slow

Solution : Actually compile in release mode.


> But it is still kinda slow but ... there is a twist !


For every test with PRIWA we use Datasetcorearry as the used dataset


| Name                    | Load  | Count all quads with a certain po |
| ----------------------- | ----- | --------------------------------- |
| sophia_rs (graph)       | 7.69  | 0.0019                            |
| sophia_rs (dataset)     | 8.06  | 0.15                              |
| PRIWA (rdf js style)    | 15.17 | 0.29                              |
| PRIWA (sophia_rs style) | 15.36 | 0.14                              |
| n3js                    | 8.76  | 0.067                             |
| graphy                  | 6.83  | 0.34                              [

FastDS takes 20 sec to load

But the catch is that the FastDataset is useless for querying PO (the tested
match). The indexing is only used if we only query an object. n3js is
optimized for PO and not P / O.


| Name                     | P    | O    | PO   |
| ------------------------ | ---- | ---- | ---- |
| n3js                     | 0.26 | 0.26 | 0.07 |
| priwa array              | 0.33 | 0.24 | 0.29 |
| priwa fastDS into array  | 0.26 | 0.12 | 0.30 |
| Graphy                   | 0.30 | 0.32 | 0.36 |
| priwa fastdataset x2     | 0.69 | 0.53 | 0.72 |


n3js is way faster for PO because it uses an alternative indexes if there are
more than 1 keys.






