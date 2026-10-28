import argparse
import math
import os
import subprocess
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description="Tribonacci test runner")
    parser.add_argument('--mode', type=str, default='python', help='Which mode to run')
    parser.add_argument('--leaderboard', action='store_true', help="Run leaderboard config to get final values")
    parser.add_argument('--test', action='store_true', help="Run as test")

    args = parser.parse_args()
    FILE_PATH = 'output.txt'
    mode = args.mode
    run_test = args.test
    run_lead = args.leaderboard

    if run_lead:
        N = 5
        sum, line_avg = 0, 0
        for n in range(N):
            line, res = run_once(mode, FILE_PATH)
            if n == 0:
                validate(FILE_PATH)
            sum += math.log2(res)
            line_avg += line
        sum = sum / N
        line_avg = line_avg / N
        print(f"Average Sum: 2^{sum}\nAverage Lines: {line_avg}")
        return None

    line, res = run_once(mode, FILE_PATH)
    if res > 10 ^ 10:
        print(f"Result: line {line}, 2^{math.log2(res):.2f}, aka: {str(res)[:20]}...")
    else:
        print(f"Result: {res}")

    if run_test:
        res = validate(FILE_PATH)
        if res is False:
            print("Test failed")
            return None
        print("Success")
        return None

    return res



def run_once(mode, FILE_PATH):
    Path(FILE_PATH).unlink(missing_ok=True)

    with open(FILE_PATH, 'w') as f:
        if mode == 'python':
            subprocess.call(['timeout', '--kill-after=1s', '1s', 'python3', 'test.py'], stdout=f, stderr=f)
        if mode == 'python-cache':
            subprocess.call(['timeout', '--kill-after=1s', '1s', 'python3', 'cached_test.py'], stdout=f, stderr=f)
        if mode == 'rust':
            subprocess.call(['timeout --kill-after=1s 1s rustc rust_test.r && ./rust'], stdout=f, stderr=f)
        if mode == 'c':
            pass

    # Get last line of file
    with open(FILE_PATH, 'r') as f:
        lines = f.readlines()
        return len(lines), int(lines[-1])


def validate(FILE_PATH):
    with open(FILE_PATH, 'r') as f:
        lines = f.readlines()
        if int(lines[0]) != 0 and int(lines[1]) != 0 and int(lines[2]) != 1:
            return False
        for idx, line in enumerate(lines):
            if idx < 3:
                continue
            if int(line) != int(lines[idx - 1]) + int(lines[idx - 2]) + int(lines[idx - 3]):
                print(f"Failed with {int(line)}, {int(lines[idx - 1])}, {int(lines[idx - 2])}, {int(lines[idx - 3])}")
                return False
    return True


if __name__ == '__main__':
    main()
