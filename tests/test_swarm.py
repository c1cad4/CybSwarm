import unittest
from cybswarm import Swarm
class SwarmTests(unittest.TestCase):
    def test_duplicate_agents_rejected_before_execution(self):
        with self.assertRaises(ValueError):Swarm(None,None).execute('id',{'capability':'memory.recall','agent_ids':['a','a']})
    def test_bounded_team(self):
        with self.assertRaises(ValueError):Swarm(None,None).execute('id',{'capability':'memory.recall','agent_ids':[str(i) for i in range(9)]})
