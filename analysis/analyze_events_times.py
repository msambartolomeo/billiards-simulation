import os

import matplotlib.pyplot as plt
import numpy
import numpy as np

DIRECTORY_PATH = "./analysis/results/diff_starting_pos/"
Y_STARTING_POS = 0.56


def read_from_dir(directory) -> dict[float, list[list[float]]]:
    times_per_pos: dict[float, list[list[float]]] = {}

    for pos_dir in os.listdir(directory):
        # Directory names follow the format "pos<Y_POS>"
        pos = float(pos_dir[3:])

        for file in os.listdir(directory + pos_dir):
            if file.endswith(".txt"):
                # Read the file and get the times
                with open(directory + pos_dir + "/" + file, "r") as f:
                    # Skip the first line, containing the amount of events
                    f.readline()

                    times = []
                    for line in f:
                        times.append(float(line.split(" ")[0]))

                    # Add the times to the dict
                    if pos in times_per_pos:
                        times_per_pos[pos].append(times)
                    else:
                        times_per_pos[pos] = [times]

    # Sort by position ascending
    return dict(sorted(times_per_pos.items()))


def graph_events_times(times_per_pos: dict[float, list[list[float]]]):
    plt.rcParams["font.family"] = "serif"
    plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams.update({"font.size": 16})

    # For each stating position, graph the average time between events, averaging over all runs
    metrics = []
    for pos, all_runs in times_per_pos.items():
        # Average and standard deviation of all runs
        avg_per_run = [numpy.mean(run) for run in all_runs]

        # print(f"Position: {pos}")
        # print(f"Average time between events: {numpy.mean(avg_per_run)}")
        # print(f"Standard deviation: {numpy.std(avg_per_run)}")
        # print(f"Average frequency of events: {1 / numpy.mean(avg_per_run)}")
        # print()
        metrics.append((pos, avg_per_run))

    for pos, avg_per_run in metrics:
        plt.errorbar(
            pos * 100,
            numpy.mean(avg_per_run),
            yerr=numpy.std(avg_per_run) / numpy.sqrt(len(avg_per_run)),
            fmt="bx",
            ecolor="r",
            capsize=5,
        )
    plt.yscale("log")
    plt.xlabel("Posición inicial en Y de la bola blanca (cm)", fontsize=18)
    plt.ylabel("Tiempo medio entre eventos (s)", fontsize=18)
    plt.savefig("./analysis/results/Mean_Time_Between_Events.png")

    plt.clf()

    for pos, avg_per_run in metrics:
        # Convert to frequency
        avg_per_run = [1 / time for time in avg_per_run]

        plt.errorbar(
            pos * 100,
            numpy.mean(avg_per_run),
            yerr=numpy.std(avg_per_run) / numpy.sqrt(len(avg_per_run)),
            fmt="bx",
            ecolor="r",
            capsize=5,
        )
    plt.yscale("log")
    plt.xlabel("Posición inicial en Y de la bola blanca (cm)", fontsize=18)
    plt.ylabel("Frecuencia media de eventos (1/s)", fontsize=18)
    plt.savefig("./analysis/results/Mean_Event_Frequency.png")


def plot_histogram(run_times_per_pos: dict[float, list[float]]):
    plt.rcParams["font.family"] = "serif"
    plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams.update({"font.size": 16})

    max_value = 0.5
    step = 0.00015
    range_value = int(max_value / step)

    for pos, run_times in run_times_per_pos.items():
        heights, edges = np.histogram(
            run_times,
            bins=[i * step for i in range(range_value)],
            range=(0, 0.5),
            density=True,
        )

        x = edges[:-1] + np.diff(edges) / 2
        plt.plot(x, heights, "-", label=f"Y = {round(pos * 100)} cm")

    plt.yscale("log")
    plt.xscale("log")
    plt.xlabel("Tiempo medio entre eventos (s)", fontsize=14)
    plt.ylabel("Densidad de probabilidad", fontsize=14)
    plt.legend()
    plt.savefig("./analysis/results/Event_Frequency_Histogram.png")


def main():
    times_per_offsets = read_from_dir(DIRECTORY_PATH)
    graph_events_times(times_per_offsets)

    # For positions 0.56, 0.42, and 0.49, get a dict with the times of all runs
    all_runs_in_offset = {}
    for offset in [0.56, 0.42, 0.49]:
        all_runs_in_offset[offset] = []
        for run in times_per_offsets[offset]:
            all_runs_in_offset[offset].extend(run)

    plot_histogram(all_runs_in_offset)


if __name__ == "__main__":
    main()
