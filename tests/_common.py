def test_main():
    from tap.runner import TAPTestRunner
    import unittest

    runner = TAPTestRunner()
    runner.set_stream(True)
    unittest.main(testRunner=runner)
