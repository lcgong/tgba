from packaging.specifiers import SpecifierSet

s = SpecifierSet("~= 1.1.0")
assert '1.1' in s
assert '1.1.0' in s
assert '1.1.2' in s
assert '1.1.post1' in s
assert '1.1.1.dev1' not in s 
assert '1.1.1rc1' not in s 
assert '1.2' not in s 
assert '1.0' not in s 


# versions = ["1.10", "1.11", "1.12"]



# for v in versions:
#     print(v, specifier.contains(v))