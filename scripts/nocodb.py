from requests import Session

class Nocodb():
  def __init__(
    self,
    token: str,
    host: str = "http://localhost:8080",
  ) -> None:
    self.session = Session()
    self.session.headers = { "xc-token": token }
    self.host = host

  def get(
    self,
    table: str,
    view: str|None = None,
    where: str = "",
    offset: int = 0,
    limit: int = 100
  ) -> dict|None:
    params = [
      f"where={where}",
      f"offset={offset}",
      f"limit={limit}",
    ]
    if view is not None:
      params.append( f"viewId={view}" )
    url = f"{self.host}/api/v2/tables/{table}/records?{"&".join(params)}"
    res = self.session.get( url )
    return dict( res.json() ).get( "list", {} )