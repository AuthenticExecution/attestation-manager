import os
import sys
import re
from os.path import join, exists

TARGET_FILE = join("src", "init.rs")
PLACEHOLDER = "-----BEGIN PUBLIC KEY[\s\S]*END PUBLIC KEY-----\n"
KEY = sys.argv[1] if len(sys.argv) > 1 else join("keys", "sp_pubkey.pem")

if not exists(TARGET_FILE):
    raise Exception("target file {} does not exist".format(TARGET_FILE))

if not exists(KEY):
    raise Exception("Key {} does not exist".format(KEY))

with open(TARGET_FILE, "r") as f:
    file = f.read()

with open(KEY, "r") as f:
    key = f.read()

new_file = re.sub(PLACEHOLDER, key, file)

with open(TARGET_FILE, "w") as f:
    f.write(new_file)

print("Done")
