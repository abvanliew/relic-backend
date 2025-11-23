GROUPED_KEYS = [ "text", "term", "roll" ]

def refactor(skills: list[dict]):
  refactored_skills=[]
  for skill in skills:
    new_skill = refactor_skill(skill)
    if new_skill != skill:
      refactored_skills.append(new_skill)
  return refactored_skills

def refactor_skill(skill:dict):
  refactored_skill = {}
  action = skill.get("action")
  if action is not None:
    refactored_skill["action"] = refactor_action(action)
  
  sub_actions = skill.get("subActions")
  if sub_actions is not None:
    refactored_sub_actions = []
    for sub_action in sub_actions:
      refactored_sub_actions.append( refactor_action(sub_action) )
    refactored_skill["subActions"] = refactored_sub_actions
  return {**skill,**refactored_skill}

def refactor_action(action:dict):
  refactored_action = {}
  condition = action.get("condition")
  if condition is not None:
    refactored_action["condition"] = group_rules_block(condition)
  rules = action.get("rules")
  if rules is not None:
    refactored_action["rules"] = group_rules_block(rules)
  return {**action,**refactored_action}

def group_rules_block(elements: list[dict]):
  refactored = []
  current_set = []
  for element in elements:
    if overlap(GROUPED_KEYS, element.keys()):
      current_set.append(element)
    else:
      if len(current_set) > 0:
        refactored.append({"block": current_set})
        current_set = []
      refactored.append( element )
  if len(current_set) > 0:
    refactored.append({"block": current_set})
  return refactored

def overlap(left: list, right: list)->bool:
  for l in left:
    for r in right:
      if l == r:
        return True
  return False