import unittest
import subprocess


class Full(unittest.TestCase):

    def run_command(self, args):
        return subprocess.Popen(args, stdout=subprocess.PIPE).stdout.read().decode('utf-8')

    def setUp(self):
        self.run_command(['rm', '-rf', '/Users/ammont/.gee'])

    def test_gee_list(self):
        self.setUp()
        out = self.run_command(['gee', 'list'])
        self.assertIn(
            'you do not have any repositories currently installed with gee.',
            out,
            'gee list with no repositories installed returned an invalid response.'
        )
        self.run_command(['gee', 'clone', 'https://github.com/human37/stockbot.git'])
        out = self.run_command(['gee', 'list'])
        self.assertIn(
            '==================\nindex   repository\n==================\n[ 1 ]   stockbot \n',
            out,
            'gee list with one repository installed returned an invalid response.'
        )

    def test_gee_clone(self):
        self.setUp()
        out = self.run_command(['gee', 'clone', 'https://github.com/human37/stockbot.git'])
        self.assertIn(
            'done. cloning repository was successful.',
            out,
            'running gee clone on one repository did not give the correct output.'
        )

if __name__ == '__main__':
    tests = unittest.defaultTestLoader.loadTestsFromTestCase(Full)
    unittest.TextTestRunner().run(tests)
