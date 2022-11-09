
def load(fp: str):

    iters = []
    temps = []
    dists = []
    with open(fp, "r") as txt_file:
        iter = 1
        for line in txt_file.readlines():
            raw = line.split(" ")
            temp = raw[0]
            dist = raw[1]

            iters.append(iter)
            temps.append(temp)
            dists.append(dist)

            iter += 1
    
    return iters, temps, dists