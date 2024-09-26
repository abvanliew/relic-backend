import json

from nocodb import Nocodb

session = Nocodb( "CnPgb3PJlgDXwg13jdgQxO4jG_xtAyUu1uVuIFdS" )
data = session.get( "mjo8242pgg9ajmq", view = "vweh0w6tq1w324q1" )

props = {
  "skills": {
    "table": "mjo8242pgg9ajmq",
    "view": "vweh0w6tq1w324q1",
    "limit": 500,
  },
  "skills-initiate": {
    "table": "mjo8242pgg9ajmq",
    "view": "vwviodkiw455e0rg",
  },
  "skills-export": {
    "table": "mjo8242pgg9ajmq",
    "view": "vwcnx1f7f2v4ma8z",
  },
  "keywords": {
    "table": "m0gt5mgnfbvp7ol",
    "view": "vwlusrr7k5xub0zv",
  },
}

data = session.get( **props.get( "skills" ) )
with open( "skills.json", "w" ) as file:
  json.dump( data, file )