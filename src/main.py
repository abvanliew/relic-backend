import json
from property_term import refactor_property_term

def load_json_lines(file_path: str)->list[dict]:
  items=[]
  with open(file_path, mode= "r") as file_io:
    for line in file_io:
      items.append(json.loads(line))
  return items

def write_json_objects(file_path:str, objects:list[dict]):
  with open(file_path,"w") as file_io:
    for object in objects:
      file_io.write(f"{json.dumps(object)}\n")


if __name__ == "__main__":
  skills=load_json_lines("./data/mongo/backup/skills.jsonl")
  refactors = refactor_property_term(skills)
  write_json_objects("upsert.json",refactors)
