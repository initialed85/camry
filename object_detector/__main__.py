import sys
import os

try:
    sys.path.index(os.path.join(os.getcwd(), "object_detector"))
except ValueError:
    sys.path.append(os.path.join(os.getcwd(), "object_detector"))

from .object_detector import run


if __name__ == "__main__":
    run()
