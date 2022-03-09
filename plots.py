import csv
from traceback import print_list
import matplotlib.pyplot as plt
import argparse
import pandas as pd
import numpy as np

def convert_bandwidth(bandwidths):
    return bandwidths.apply(lambda x: float(x.split('MB')[0]))

def plot_buf_size(file):
    
    df = pd.read_csv(file)
    df['bandwidth'] = convert_bandwidth(df['bandwidth'])
    t = df.groupby('buffer_size').mean()

    plt.plot(t.index, t['bandwidth'], '-o')
    plt.xlabel('Buffer Size')
    plt.ylabel('Bandwidth (MB/s)')
    plt.title(file)
    for i,j in zip(t.index,t['bandwidth']):
        plt.annotate(str(int(j)) + "," + str(i),xy=(i,j))
    plt.savefig(file.split('.')[0] + '_buffer.png')
    print("Saved to " + file.split('.')[0] + '_buffer.png')
    plt.close()

def plot_vector_len(file):
    df = pd.read_csv(file)
    df['bandwidth'] = convert_bandwidth(df['bandwidth'])
    t = df.groupby('vector_len').mean()
    plt.plot(t.index, t['bandwidth'], '-o')
    plt.xlabel('Vector Length')
    plt.ylabel('Bandwidth (MB/s)')
    for i,j in zip(t.index,t['bandwidth']):
        plt.annotate(str(int(j)) + "," + str(i),xy=(i,j))
    plt.title(file)
    plt.savefig(file.split('.')[0] + '_vec_len.png')
    print("Saved to " + file.split('.')[0] + '_vec_len.png')
    plt.close()

def plot_num_concurrent(file):
    df = pd.read_csv(file)
    df['bandwidth'] = convert_bandwidth(df['bandwidth'])
    t = df.groupby('num_concurrent').mean()
    plt.plot(t.index, t['bandwidth'], '-o')
    plt.xlabel('Number of Concurrent Requests')
    plt.ylabel('Bandwidth (MB/s)')
    for i,j in zip(t.index,t['bandwidth']):
        plt.annotate(str(int(j)) + "," + str(i),xy=(i,j))
    plt.title(file)
    plt.savefig(file.split('.')[0] + '_num_concurrent.png')
    print("Saved to " + file.split('.')[0] + '_num_concurrent.png')
    plt.close()


def plot_max_bw(files):
    setting = []
    max_bw = []
    for file in files:
        df = pd.read_csv(file)
        df['bandwidth'] = convert_bandwidth(df['bandwidth'])
        max_bw.append(df['bandwidth'].max())
        setting.append(file.split('.')[0])
        idx = df['bandwidth'].idxmax()
        print("Max value for ", file, "occurs at ", df.iloc[idx])
    
    plt.rcParams.update({'font.size': 6})
    plt.bar(setting, max_bw)
    plt.xlabel('Setting')
    plt.ylabel('Max Bandwidth (MB/s)')
    plt.title('Max Bandwidth')
    plt.savefig('max_bw.png')
    print("Saved to max_bw.png")
    plt.close()




def main():
    parser = argparse.ArgumentParser(description='Plot the results of the benchmark')

    read_async_random_csv = 'data/read-async-random.csv'
    read_async_seq_csv = 'data/read-async-seq.csv'
    read_sync_random_csv = 'data/read-sync-random.csv'
    read_sync_seq_csv = 'data/read-sync-seq.csv'
    write_sync_seq_csv = 'data/write-sync-seq.csv'
    write_async_seq_csv = 'data/write-async-seq.csv'

    plot_buf_size(read_async_random_csv)
    plot_buf_size(read_async_seq_csv)
    plot_buf_size(read_sync_random_csv)
    plot_buf_size(read_sync_seq_csv)
    plot_buf_size(write_sync_seq_csv)
    plot_buf_size(write_async_seq_csv)

    plot_vector_len(read_async_random_csv)
    plot_vector_len(read_async_seq_csv)
    plot_vector_len(write_async_seq_csv)

    plot_num_concurrent(read_async_random_csv)
    plot_num_concurrent(read_async_seq_csv)
    plot_num_concurrent(write_async_seq_csv)

    plot_max_bw([read_async_random_csv, read_async_seq_csv, read_sync_random_csv, read_sync_seq_csv, write_sync_seq_csv, write_async_seq_csv])




if __name__ == "__main__":
    main()