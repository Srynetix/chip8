import os
import sys

rc = os.system("cargo clippy")
sys.exit(rc)