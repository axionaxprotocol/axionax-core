# deai/bridge_test.py
# A simple script to test the PyO3 bridge.

def test_rust_bridge():
    """
    Tests if the Rust bridge is accessible and functions can be called.
    """
    print("--- Testing Rust-Python Bridge ---")
    try:
        # This import will work once the Rust code is compiled as a Python module
        # and placed in the correct path.
        import axionax_bridge

        print("Successfully imported 'axionax_bridge' module.")

        # Test the get_version() function
        version = axionax_bridge.get_version()
        print(f"Rust Core Version: {version}")

        assert version is not None
        assert isinstance(version, str)

        print("\n--- Bridge Test Passed! ---")
        return True

    except ImportError:
        print("\n!!! FAILED to import 'axionax_bridge'.")
        print("This likely means the Rust bridge has not been compiled or is not in the PYTHONPATH.")
        return False
    except Exception as e:
        print(f"\n!!! An unexpected error occurred: {e}")
        return False

if __name__ == "__main__":
    if not test_rust_bridge():
        # Exit with a non-zero code to indicate failure, which can be useful for CI/CD
        import sys
        sys.exit(1)
