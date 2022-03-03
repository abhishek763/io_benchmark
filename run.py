import os
import subprocess

def run_benchmark(command):
    process = subprocess.Popen(cmd.split(), shell=True)
    output, error = process.communicate()
    if error:
        print(error)
    return output

clearCacheCmd = "sync; echo 3 | sudo tee /proc/sys/vm/drop_caches;"

inp_files = [ '3G', '5G']
buf_sizes = [1024, 4096, 16384, 65536, 262144, 1048576]
num_concurrents = [1, 4, 8, 16, 32, 64, 128, 256]


inp_files = [ '3G']
buf_sizes = [1024]
num_concurrents = [4]


with open('read-async.csv', 'w') as f:
    f.write('input_file,buffer_size,num_concurrent,bandwidth\n')
    for inp_file in inp_files:
        for buf_size in buf_sizes:
            for num_concurrent in num_concurrents:
                subprocess.Popen(clearCacheCmd.split(), shell=True)
                cmd = "./target/release/async-io -i {0} -b {1} -n {2} -r -a".format(inp_file, buf_size, num_concurrent)
                out = run_benchmark(cmd)
                f.write('{0},{1},{2},{3}\n'.format(inp_file, buf_size, num_concurrent, out))

with open('write-async.csv', 'w') as f:
    f.write('input_file,buffer_size,num_concurrent,bandwidth\n')
    for inp_file in inp_files:
        for buf_size in buf_sizes:
            for num_concurrent in num_concurrents:
                subprocess.Popen(clearCacheCmd.split(), shell=True)
                cmd = "./target/release/async-io -o {0} -b {1} -n {2} -w -a".format(inp_file + "_temp", buf_size, num_concurrent)
                out = run_benchmark(cmd)
                f.write('{0},{1},{2},{3}\n'.format(inp_file, buf_size, num_concurrent, out))



with open('read-sync.csv', 'w') as f:
    f.write('input_file,buffer_size,bandwidth\n')
    for inp_file in inp_files:
        for buf_size in buf_sizes:
                subprocess.Popen(clearCacheCmd.split(), shell=True)
                cmd = "./target/release/sync-io -i {0} -b {1} -r".format(inp_file, buf_size)
                out = run_benchmark(cmd)
                f.write('{0},{1},{2}\n'.format(inp_file, buf_size, out))

# Will have to do this manually for now
# because buf size needs to be a compile time constant
with open('write-sync.csv', 'w') as f:
    f.write('input_file,buffer_size,bandwidth\n')
    for inp_file in inp_files:
        for buf_size in buf_sizes:
                subprocess.Popen(clearCacheCmd.split(), shell=True)
                cmd = "./target/release/sync-io -o {0} -b {1} -w".format(inp_file + "_temp", buf_size)
                out = run_benchmark(cmd)
                f.write('{0},{1},{2}\n'.format(inp_file, buf_size, out))