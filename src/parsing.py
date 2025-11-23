def get_path(data:dict, path:list[str]):
  for key in path:
    data = data.get(key, {})
  return data

def collect_actions(data:dict) -> list[dict]:
  actions = []
  action = data.get("action")
  if action is not None:
    actions.append(action)
  sub_actions = data.get("subActions")
  if sub_actions is not None:
    actions.extend(sub_actions)
  return actions