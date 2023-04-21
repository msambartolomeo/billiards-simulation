import os
import subprocess

import numpy as np

Y_MAX_OFFSET = 0.0
Y_MIN_OFFSET = -6.0
Y_STEP = 0.6

RUNS_PER_POS = 50
X_STARTING_POS = 0.56

RESULTS_PATH = f"./analysis/results/diff_starting_pos/offset{{}}/"


for y_offset in np.arange(Y_MIN_OFFSET, Y_MAX_OFFSET + Y_STEP, Y_STEP):
    os.makedirs(RESULTS_PATH.format(round(y_offset, 2)), exist_ok=True)
    for run_idx in range(RUNS_PER_POS):
        subprocess.run(
            [
                "./target/release/billiards-simulation",
                "-w" + str(round(y_offset, 2)),
                "-t",
                RESULTS_PATH.format(round(y_offset, 2))
                + f"offset{round(y_offset, 2)}_run{run_idx}.txt",
            ]
        )
