import matplotlib.pyplot as plt
import numpy

# Data needs to be created manually
DIRECTORY_PATH = "./analysis/results/deterministic/"


def read_mean_time_from_file(path):
    times: list[float] = []

    # Read the file and get the times
    with open(DIRECTORY_PATH + path, "r") as f:
        # Skip the first line, containing the amount of events
        f.readline()

        for line in f:
            times.append(float(line.split(" ")[0]))

    return sum(times) / len(times)


def main():
    plt.rcParams["font.family"] = "serif"
    plt.figure(figsize=(1280 / 108, 720 / 108), dpi=108)
    plt.rcParams.update({"font.size": 16})
    plt.ylabel("Tiempo medio entre eventos (s)", fontsize=18)
    plt.ylim(4.5, 5)

    double_1 = read_mean_time_from_file("double_1.txt")
    double_2 = read_mean_time_from_file("double_2.txt")
    float_1 = read_mean_time_from_file("float_1.txt")
    float_2 = read_mean_time_from_file("float_2.txt")

    values = [double_1, double_2, float_1, float_2]
    labels = ["Doble 1", "Doble 2", "Simple 1", "Simple 2"]

    plt.bar(
        labels,
        values,
        color=["#1f77b4", "#1f77b4", "#ff7f0e", "#ff7f0e"],
    )

    plt.savefig("./analysis/results/deterministic_times.png")


if __name__ == "__main__":
    main()
