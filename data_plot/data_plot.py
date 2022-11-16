from math import sqrt
from matplotlib import pyplot as plt
from utils import load_convergence, load_runs
import pandas as pd
import seaborn as sns

base = "base100"

iters, temps, dists = load_convergence(f'../simulated-annealing/results/{base}.txt', max_itens=200005)

d = {"temp": temps, "Activation Function": dists, "Iteractions": iters}
df = pd.DataFrame(data=d)

sns.lineplot(data=df, x="Iteractions", y="Activation Function")

plt.show()

plt.clf()

runs = load_runs(f'../simulated-annealing/results/{base}_runs')

d_runs = {"runs": runs}
df_runs = pd.DataFrame(data = d_runs)

print(df_runs.describe())

ax = sns.boxplot(x=runs)

ax.set(xlabel="Final Result")

plt.show()