# pypies: scatter pie plot in python
A collection of various tools for working with scatter pie. It is written in pure rust and provide python API through [PyO3](https://github.com/PyO3/pyo3).

Pypies is built upon [polars](https://github.com/pola-rs/polars) and [plotters](https://github.com/plotters-rs/plotters), It inherited the advantages of both, so it can be very fast for large dataset and can supports various types of back-ends potentially.

`pypies` is very much a work in progress, so suggestions for new tools, features and capabilities is very much welcome, as are contributions as PRs.
## Current tools
* `scatterpie`: Create scatterpie plots.
## Installation
You can install it with  pip:

`pip install pypies`
## Gallery
Here is an example of scatter pie plot:

![scatter pie](examples/visium.png)
## Usage
For users from pandas:
```
import pandas as pd
import polars as pl
import pypies

df = pd.read_csv("datasets/iris.csv")
df = pl.from_pandas(df)
pypies.scatterpie(df, "Sepal.Length", "Sepal.Width", ["Petal.Length","Petal.Width"])
```
For users from polars:
```
import polars as pl
import pypies

df = pl.read_csv("datasets/iris.csv")
pypies.scatterpie(df, "Sepal.Length", "Sepal.Width", ["Petal.Length","Petal.Width"])
```
The plot will be saved in the currently directory. To learn more, please read the help documentaion using `help(pypies.scatterpie)`.
## Limitations
- The plot is not interactive and must be saved locally.
## License
This project is licensed under the [MIT license](LICENSE)