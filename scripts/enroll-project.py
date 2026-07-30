#!/usr/bin/env python3
"""Alias: enroll-project.py → enroll-product.py (projects, not products)."""
from pathlib import Path
import runpy
runpy.run_path(str(Path(__file__).with_name("enroll-product.py")), run_name="__main__")
