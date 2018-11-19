#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import time
import sys

folders = [(10, "Day1-9"), (20, "Day10-19"), (26, "Day20-25")]

# Form:
#   extension: (compile_cmd, run_cmd, name)
languages = {
    "py": (None, "python3 {}", "Python"),
    # 'hs': 'Haskell',
    "cpp": ("g++ -Ofast -march=native {} -o {} -lboost_regex", "./{} ", "C++"),
    "c": ("gcc -Ofast -pthread -march=native  -lpthread {} -o {}", "./{}", "C"),
    # 'rb': 'Ruby',
    # 'swift': 'Swift',
    # 'java': 'Java',
    # 'js': 'Javascript',
    "rs": ("rustc -O {0} -o {1}", "./{0}", "Rust"),
    "go": ("go build -o {1} {0}", "./{}", "Go"),
}


def benchmark(cmd):
    times = []
    bench_start = time.perf_counter()
    for i in range(10000):
        # Halt if benchmark takes more than 1 seconds
        if time.perf_counter() - bench_start > 1:
            break
        start = time.perf_counter()
        cmd_pipe = os.popen(cmd)
        # Force the command to complete
        cmd_pipe.read()
        result = cmd_pipe.close()
        end = time.perf_counter()

        if result and result != 0:
            raise Exception("Command failed: {}".format(cmd))

        times.append((end - start))
    return sum(times) / len(times) * 1000


def benchmark_day(day):
    fdr = next(x[1] for x in folders if x[0] > day)
    if not os.path.isdir(fdr):
        return
    os.chdir(fdr)

    if not os.path.exists("bin"):
        os.makedirs("bin")

    printed_header = False

    for fn in os.listdir("."):

        file_day = None
        try:
            name, ext = fn.split(".")
            file_day = next(
                int(name[i:]) for i in range(len(name)) if name[i:].isdigit()
            )
        except StopIteration:
            continue
            # print("Coundn't parse: %s" % fn)
        except ValueError:
            continue
        if ext in languages and day == file_day:
            if not printed_header:
                print(25 * "-")
                print("Day {}:".format(day))
                printed_header = True

            comp, run, lang = languages[ext]
            if not comp:
                comp = "cp {} {}"
            bin_file = "bin/{}-{}".format(day, ext) if comp else fn

            compile_command = comp.format(fn, bin_file)
            compile_result = os.system(compile_command)
            if compile_result != 0:
                raise Exception("Compile failed for command {}".format(compile_command))
            runtime = benchmark(run.format(bin_file))
            print("\t{}: {:.2f}ms".format(lang, runtime))

    os.chdir("..")


if __name__ == "__main__":
    if len(sys.argv) == 2:
        benchmark_day(int(sys.argv[-1]))
    else:
        for day in range(1, 26):
            benchmark_day(day)
