from math import sqrt
from matplotlib import pyplot as plt
from utils import load
import pandas as pd
import seaborn as sns
import numpy as np

iters, temps, dists = load("../simulated-annealing/results/base51.txt")
d = {"temp": temps, "dists": dists, "iteraction": iters}
df = pd.DataFrame(data=d)

sns.lineplot(data=df, x="iteraction", y="dists")

plt.show()