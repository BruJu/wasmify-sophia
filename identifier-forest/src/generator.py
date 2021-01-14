

f = open("src/generated.rs", "w")


def print(text = ""):
    f.write(text)
    f.write("\n")

data = [
    ("Subject", "S", 0),
    ("Predicate", "P", 1),
    ("Object", "O", 2),
    ("Graph", "G", 3)
]

methods = [
    { "name": "exists", "return": "-> bool ", "parameter": "", "forward": ""},
    {
        "name": "index_conformance",
        "return": "-> Option<usize>",
        "parameter": ", can_build: bool, pattern_layout: &[Option<I>; 4]",
        "forward": "can_build, &pattern_layout"
    },
    {
        "name": "initialize",
        "return": "",
        "parameter": ", iter: OnceTreeSetIterator<'a, I>",
        "forward": "iter",
        "template": "'a"
    },
    {  
        "name": "get_quads",
        "return": "-> OnceTreeSetIterator<'a, I>",
        "parameter": ", pattern: [Option<I>; 4]",
        "forward": "pattern",
        "template": "'a",
        "refkind": "&'a "
    },
    {
        "name": "insert",
        "return": "-> Option<bool>",
        "parameter": ", quad: &[I; 4]",
        "forward": "&quad",
        "refkind": "&mut ",
        "refkindsoft": "&mut "
    },
    {
        "name": "size",
        "return": "-> Option<usize>",
        "parameter": "",
        "forward": "",
    }
]

constructors = [
    "new", "new_instanciated"
]



print("use crate::identifier_quad::*;")
print()

def every_order():
    retval = []
    for a in data:
        for b in data:
            for c in data:
                for d in data:
                    if a != b and a != c and a != d and b != c and b != d and c != d:
                        retval.append([a, b, c, d])

    return retval


all = every_order()

print("pub enum TreeForFour<I>")
print("where I: Identifier")
print("{")


for permutation in all:
    print("    {}(OnceTreeSet<I{}>),".format(
        "".join([x[1] for x in permutation]),
        "".join([", " + x[0] for x in permutation])
    ))

print("}")
print()
print("impl<I> TreeForFour<I>")
print("where I: Identifier {")


for constructor in constructors:
    print("    pub fn "+ constructor + "(order: &[usize; 4]) -> Option<TreeForFour<I>> {")
    print("        match order {")

    for permutation in all:
        print("            [{}] => Some(Self::{}( OnceTreeSet::{}())),".format(
            ", ".join([str(x[2]) for x in permutation]),
            "".join([x[1] for x in permutation]),
            constructor
        ))

    print("            [_, _, _, _] => None,")
    print("        }")
    print("    }")
    print("")

for method in methods:

    template = ("<" + method["template"] + ">") if "template" in method else ""
    refkind = method["refkind"] if "refkind" in method else "&"
    refkindsoft = method["refkindsoft"] if "refkindsoft" in method else "&"

    print("    pub fn "+ method["name"] + template + "("+refkind+"self"+method["parameter"]+") " + method["return"] + "{")
    print("        match "+refkindsoft+"self {")

    for permutation in all:
        print("            Self::{}(tree) => tree.{}({}),".format(
            "".join([x[1] for x in permutation]),
            method["name"],
            method["forward"]
        ))

    print("        }")
    print("    }")
    print("")

print("}")
