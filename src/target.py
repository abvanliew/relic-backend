from main import load_json_lines

def matched_list( items: list[dict], match_key: str ):
  for item in items:
    yield from matched_values( item, match_key )

def matched_values( data: dict, match_key: str ):
  if isinstance(data, list):
    for item in data:
      yield from matched_values( item, match_key )
  elif isinstance(data, dict):
    for (key, value) in data.items():
      if key == match_key:
        yield value
      else:
        yield from matched_values( value, match_key )

if __name__ == "__main__":
  skills = load_json_lines("./data/mongo/backup/skills.jsonl")
  targets = list( matched_values( skills, "target" ) )
  classes = set( matched_values( targets, "class" ) )
  for target in targets:
    print( target )
  print( classes )
  # for item in data:
  #   print( item )
  # print( list ( data ) )
