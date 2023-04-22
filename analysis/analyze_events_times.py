import os

import matplotlib.pyplot as plt
import numpy

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

    # For each stating position, graph the average time between events, averaging over all runs
    metrics = []
    for pos, all_runs in times_per_pos.items():
        # Average and standard deviation of all runs
        avg_per_run = [numpy.mean(run) for run in all_runs]

        print(f"Position: {pos}")
        print(f"Average time between events: {numpy.mean(avg_per_run)}")
        print(f"Standard deviation: {numpy.std(avg_per_run)}")
        print(f"Average frequency of events: {1 / numpy.mean(avg_per_run)}")
        print()
        metrics.append((pos, avg_per_run))

    for pos, avg_per_run in metrics:
        plt.errorbar(
            pos,
            numpy.mean(avg_per_run),
            yerr=numpy.std(avg_per_run),
            fmt="bx",
            ecolor="r",
            capsize=5,
        )
    plt.xlabel("Posición inicial en Y de la bola blanca (m)")
    plt.ylabel("Tiempo medio entre eventos (s)")
    plt.savefig("./analysis/results/Mean_Time_Between_Events.png")

    plt.clf()

    for pos, avg_per_run in metrics:
        plt.errorbar(
            pos,
            1 / numpy.mean(avg_per_run),
            yerr=numpy.std(avg_per_run),
            fmt="bx",
            ecolor="r",
            capsize=5,
        )
    plt.xlabel("Posición inicial en Y de la bola blanca (m)")
    plt.ylabel("Frecuencia media de eventos (1/s)")
    plt.savefig("./analysis/results/Mean_Event_Frequency.png")


def main():
    times_per_offsets = read_from_dir(DIRECTORY_PATH)
    graph_events_times(times_per_offsets)


if __name__ == "__main__":
    main()
