from inspect import signature
from cacophony.clatter import Clatter


c = Clatter()
print(signature(Clatter).from_callable(c.get))
for k in c.__dict__:
    title = k.replace("_", " ").title()
    print(title)
    v = c.__dict__[k]
    if isinstance(v, list):
        if isinstance(v[0], str):
            print(f"\t{v}")
        elif isinstance(v[0], int) or isinstance(v[0], float):
            print(f"\t{v[0]}-{v[-1]}")
        else:
            raise Exception(k, v)
    else:
        print(f"\t{c.__dict__[k]}")
