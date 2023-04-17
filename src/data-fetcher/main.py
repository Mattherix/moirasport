from requests import get
from os import getenv
from sys import stderr
from pprint import pprint

BASE_URL = "https://api.sportmonks.com/v3/football"
FIXTURES_URL = BASE_URL + "/fixtures"
TOKEN = getenv("TOKEN")
if not TOKEN:
    stderr.write("No token in TOKEN environnement variable\n")
    quit(1)


response = get(FIXTURES_URL, headers={"Authorization": TOKEN}).json()
# response = get(FIXTURES_URL + "?api_token=" + TOKEN).json()
pprint(response)
