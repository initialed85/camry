import sys
import os

sys.path.append(os.path.join(os.getcwd(), "object_detector"))
sys.path.append(os.path.join(os.getcwd(), "object_detector", "api"))

from .object_detector import run


if __name__ == "__main__":
    run()
