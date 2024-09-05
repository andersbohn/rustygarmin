from garminconnect import Garmin
import datetime
# import pandas as pd
# import matplotlib.pyplot as plt

import argparse
import json

parser = argparse.ArgumentParser("simple_example")
parser.add_argument("pwd", help="Garmin Connect password", type=str)
args = parser.parse_args()

username = 'andersbohn@icloud.com'
password = args.pwd

# connect to the API
try:
  api = Garmin(username, password)
  success = api.login()

  start_date = datetime.date(1972, 1, 1)
  end_date = datetime.date(2025, 9, 30)

  activities = api.get_activities_by_date(start_date.isoformat(),end_date.isoformat())

  file_name = f"gc-{start_date}-{end_date}.json"
  print(f"login {success} garmin connect {len(activities)} activities between {start_date} and {end_date} saved as '{file_name}'") 
  
  with open(file_name, 'w') as file:
    json_str = json.dumps(activities, indent=2)
    file.write(json_str)

except Exception as e:
  print(f"An error occurred while initializing the API: {e}")

