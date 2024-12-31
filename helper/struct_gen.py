# Simple tool to help make rust struct from api response.
# Not perfect. Human need to adjust result.

import re
import json

# JSON string
# make json_str.py and define json_str
from json_str import json_str

dict = json.loads(json_str)

def camel_to_snake(name):
    # Find all uppercase letters and prepend an underscore, then convert the whole string to lowercase
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    s2 = re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1)
    return s2.lower()

def rust_type(x):
    if isinstance(x, str):
        return "String"
    elif isinstance(x, bool):
        return "bool"
    elif isinstance(x, int):
        return "i32"
    elif isinstance(x, float):
        return "f64"
    else:
        return "Unknown"


print("""#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct * {""")

for key in dict:
    if key.islower():
        field = key
    else:
        field = camel_to_snake(key)
        print(f'    #[serde(rename = "{key}")]')
    print(f'    pub {field}: {rust_type(dict[key])},')

print('}')
