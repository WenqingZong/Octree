import numpy as np


if __name__ == "__main__":
    MAX_COORDINATE = 100000
    NUM_POINTS = 100000
    FILE_PATH = './data/points.txt'

    np.random.seed(1701)

    points = np.random.rand(NUM_POINTS, 3) * MAX_COORDINATE
    np.savetxt(FILE_PATH, points, fmt='%.4f')

    print(NUM_POINTS, "points are written to", FILE_PATH)
