"""Deterministic multi-agent coordination for local bounded memory capabilities."""

class Swarm:
    def __init__(self, registry, memory):
        self.registry, self.memory = registry, memory
    def execute(self, task_id, payload):
        capability = payload['capability']
        ids = payload['agent_ids']
        if not 1 <= len(ids) <= 8 or len(set(ids)) != len(ids):
            raise ValueError('task requires 1–8 distinct agents')
        agents = [self.registry.get(identifier) for identifier in ids]
        if any(capability not in agent.capabilities for agent in agents):
            raise ValueError('agent capability denied')
        def operation(db):
            events = [{'agent_id': agent.id, 'result': agent.run(capability, payload, self.memory, db)} for agent in agents]
            return {'task_id': task_id, 'state': 'completed', 'events': events}
        return self.memory.execute_once(task_id, payload, operation)
