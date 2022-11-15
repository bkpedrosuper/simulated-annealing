def load_runs(fp: str):
    runs = []
    with open(fp, 'r') as file:
        for line in file.readlines():
            runs.append(float(line))

        return runs

def load_convergence(fp: str, max_itens: int = 100006):

    iters = []
    temps = []
    dists = []
    with open(fp, "r") as txt_file:
        iter = 1
        for line in txt_file.readlines():
            raw = line.split(" ")
            temp = raw[0]
            dist = raw[1]

            iters.append(int(iter))
            temps.append(float(temp))
            dists.append(float(dist))

            iter += 1
    
    return iters[:max_itens], temps[:max_itens], dists[:max_itens]