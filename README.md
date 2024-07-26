# pypies: scatter pie plot in python
A collection of various tools for working with scatter pie. `pypies` is very much a work in progress, so suggestions for new tools, features and capabilities is very much welcome, as are contributions as PRs.
## Current tools
* `scatterpie`: Create scatterpie plots.
## Installation
Download the released binary package and install via pip:
`pip install xxxx.whl`
# Usage
For users from pandas:
```
import pandas as pd
import polars as pl
import pypies
df = pl.read_csv("datasets/iris.csv")
pypies.scatterpie(df, "Sepal.Length", "Sepal.Width", ["Petal.Length","Petal.Width"])
```

