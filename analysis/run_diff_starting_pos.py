import os
import subprocess

import numpy as np

Y_MAX_POS = 0.56
Y_MIN_POS = 0.42
RUNS_PER_OFFSET = 10
Y_STEP = 0.014


RUNS_PER_POS = 50


RESULTS_PATH = f"./analysis/results/diff_starting_pos/pos{{}}/"


for y_pos in np.arange(Y_MIN_POS, Y_MAX_POS, Y_STEP):
    y_pos_str = str(round(y_pos, 3))
    y_offset_str = str(round(Y_MAX_POS - y_pos, 3))
    print(f"Starting runs for Y position {y_pos_str} and offset {y_offset_str}")
    os.makedirs(RESULTS_PATH.format(y_pos_str), exist_ok=True)
    for run_idx in range(RUNS_PER_POS):
        subprocess.run(
            [
                "./target/release/billiards-simulation",
                "-w" + y_offset_str,
                "-t",
                RESULTS_PATH.format(y_pos_str) + f"pos{y_pos_str}_run{run_idx}.txt",
            ]
        )
