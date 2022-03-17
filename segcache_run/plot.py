import matplotlib.pyplot as plt

def plot_throughput():
    
    mqps = [0.16, 0.16, 0.16, 0.16, 0.16, 0.16, 1.0, 2.15, 4.01, 4.08, 4.20, 4.1, 3.93, 4.14]
    ram =  [2,     3,     4,    5,    6,    7,   8,  10,   15,   20,   25,   30,   35,  41]
    plt.plot(ram, mqps, '-o')
    plt.xlabel('RAM Size (GB)')
    plt.ylabel('Bandwidth (MQPS)')
    plt.title('Throughput')
    for i,j in zip(ram, mqps):
        plt.annotate(str(i) + "," + str(j),xy=(i,j))
    plt.savefig("throughput" + '.png')
    print("Saved to " + "throughput" + '.png')
    plt.close()
    
plot_throughput()