import os
import subprocess

def run_benchmark(command):
    print(command)
    result = subprocess.run(command.split(), stderr=subprocess.PIPE, stdout=subprocess.PIPE)
    if result.stderr:
        print("error", result.stderr)
        print("out", result.stdout)
    return int(result.returncode), str(result.stdout.decode('utf-8'))

clearCacheCmd = "sync; echo 3 | sudo tee /proc/sys/vm/drop_caches;"

inp_files = [ '/users/ak5/read.bin']
buf_sizes = [1024, 4096, 16384, 65536, 262144, 1048576]
vector_lens = [1, 4, 16, 64, 256, 1024, 4096, 16384]
num_concurrents = [1, 4, 8, 16, 32, 64, 128, 256]
time = 4


''' Small params
inp_files = [ '/home/ubuntu/read_small.bin']
buf_sizes = [1024]
vector_lens = [2]
num_concurrents = [4]
time = 3
'''

with open('read-async-seq.csv', 'w') as f:
    f.write('input_file,buffer_size,num_concurrent,vector_len,type,read_size,bandwidth\n')
    for inp_file in inp_files:
        for buf_size in buf_sizes:
            for num_concurrent in num_concurrents:
                for v in vector_lens:
                    subprocess.Popen(clearCacheCmd.split(), shell=True)
                    cmd = "./target/release/async-io -i {0} -b {1} -n {2} -r -t {3} -v {4}".format(inp_file, buf_size, num_concurrent, time, v)
                    e, out = run_benchmark(cmd)

                    if e == 0:
                        f.write('{0},{1},{2},{3},{4}\n'.format(inp_file, buf_size, num_concurrent, v, out))


with open('read-async-random.csv', 'w') as f:
    f.write('input_file,buffer_size,num_concurrent,vector_len,type,read_size,bandwidth\n')
    for inp_file in inp_files:
        for buf_size in buf_sizes:
            for num_concurrent in num_concurrents:
                for v in vector_lens:
                    subprocess.Popen(clearCacheCmd.split(), shell=True)
                    cmd = "./target/release/async-io -i {0} -b {1} -n {2} -r --random -t {3} -v {4}".format(inp_file, buf_size, num_concurrent, time, v)
                    e, out = run_benchmark(cmd)
                    if e == 0:
                        f.write('{0},{1},{2},{3},{4}\n'.format(inp_file, buf_size, num_concurrent, v, out))


with open('read-sync-seq.csv', 'w') as f:
    f.write('input_file,buffer_size,type,read_size,bandwidth\n')
    for inp_file in inp_files:
        for buf_size in buf_sizes:
                subprocess.Popen(clearCacheCmd.split(), shell=True)
                cmd = "./target/release/sync-io -i {0} -b {1} -r -t {2}".format(inp_file, buf_size, time)
                e, out = run_benchmark(cmd)
                if e == 0:
                    f.write('{0},{1},{2}\n'.format(inp_file, buf_size, out))

with open('read-sync-random.csv', 'w') as f:
    f.write('input_file,buffer_size,type,read_size,bandwidth\n')
    for inp_file in inp_files:
        for buf_size in buf_sizes:
                subprocess.Popen(clearCacheCmd.split(), shell=True)
                cmd = "./target/release/sync-io -i {0} -b {1} -r --random -t {2}".format(inp_file, buf_size, time)
                e, out = run_benchmark(cmd)
                if e == 0:
                    f.write('{0},{1},{2}\n'.format(inp_file, buf_size, out))

with open('write-sync-seq.csv', 'w') as f:
    f.write('input_file,buffer_size,type,read_size,bandwidth\n')
    for inp_file in inp_files:
        for buf_size in buf_sizes:
                subprocess.Popen(clearCacheCmd.split(), shell=True)
                cmd = "./target/release/sync-io -o {0} -b {1} -w -t {2}".format("temp", buf_size, time)
                e, out = run_benchmark(cmd)
                if e == 0:
                    f.write('{0},{1},{2}\n'.format(inp_file, buf_size, out))
                subprocess.Popen("rm temp".split(), shell=True)


with open('write-async-seq.csv', 'w') as f:
    f.write('input_file,buffer_size,num_concurrent,vector_len,type,read_size,bandwidth\n')
    for inp_file in inp_files:
        for buf_size in buf_sizes:
            for num_concurrent in num_concurrents:
                for v in vector_lens:
                    subprocess.Popen(clearCacheCmd.split(), shell=True)
                    cmd = "./target/release/async-io -o {0} -b {1} -n {2} -w -t {3} -v {4}".format("temp", buf_size, num_concurrent, time, v)
                    e, out = run_benchmark(cmd)
                    if e == 0:
                        f.write('{0},{1},{2},{3},{4}\n'.format("temp", buf_size, num_concurrent, v, out))
                    subprocess.Popen("rm temp".split(), shell=True)