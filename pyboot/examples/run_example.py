#!/usr/bin/env python
"""Runner script for examples - adds src to path automatically."""

import sys
from pathlib import Path

# Add src to path
src_path = Path(__file__).parent.parent / "src"
sys.path.insert(0, str(src_path))

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python run_example.py <example_name>")
        print("\nAvailable examples:")
        examples_dir = Path(__file__).parent
        for f in examples_dir.glob("*_example.py"):
            print(f"  - {f.stem}")
        sys.exit(1)
    
    example_name = sys.argv[1]
    if not example_name.endswith("_example"):
        example_name += "_example"
    
    example_file = Path(__file__).parent / f"{example_name}.py"
    if not example_file.exists():
        print(f"Example not found: {example_file}")
        sys.exit(1)
    
    # Execute the example
    exec(example_file.read_text(), {"__name__": "__main__"})
