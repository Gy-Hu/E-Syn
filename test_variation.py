import argparse
import subprocess
import threading
import sys

def run_command(i, delay):
    command = f"./abc/abc -c \"read_eqn /data/cchen/E-Brush/test_data_beta_runner/raw_circuit.eqn; read_lib /data/cchen/E-Brush/asap7_clean.lib;st;ifraig;scorr;dc2;dretime;retime -o -D {delay};st; &get -n; &dch -f; &nf -D {delay}; &put; buffer; upsize -D {delay};dnsize -D {delay}; stime -p\""
    subprocess.run(command, shell=True)
    print("----------------------------------------------------------------------------------------")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Test Variation Script")
    parser.add_argument("delay", type=int, help="目标延迟值 以ps为单位")
    args = parser.parse_args()

    sys.stdout = open('log_var.txt', 'w')  # 将标准输出重定向到文件

    threads = []
    increment = (args.delay // (15*3))

    for i in range(30):
        curr_delay = args.delay + (i - 14) * increment
        print(i , curr_delay)
        thread = threading.Thread(target=run_command, args=(i, curr_delay))
        threads.append(thread)
        thread.start()

    for thread in threads:
        thread.join()

    sys.stdout.close()
    sys.stdout = sys.__stdout__  # 恢复标准输出