from parsing import collect_actions

def refactor_property_term(skills: list[dict]) -> list[dict]:
  dirty_skills = []
  for i, skill in enumerate(skills):
    if refactor_action( skill.get("action") ):
      dirty_skills.append(skill)
  return dirty_skills


def refactor_action(action:dict):
  properties = action.get("properties",[])
  dirty = False
  for i, prop in enumerate(properties):
    if refactor_property(prop):
      dirty = True
      properties[i] = prop
  rules = action.get("rules", [])
  for i, rule in enumerate(rules):
    prop = rule.get("property")
    if prop is None:
      continue
    if refactor_property(prop):
      dirty = True
      rules[i] = rule
  return dirty


def refactor_property(data:dict) -> bool:
  term = {}
  dirty = False
  dirty = dirty | move_key_to_obj(data,"title",term)
  dirty = dirty | move_key_to_obj(data,"keywordId",term)
  data["term"] = term
  return dirty


def move_key_to_obj(data:dict,key:str,object:dict):
  value = data.pop(key, None)
  if value is not None:
    object[key] = value
    return True
  return False
